//! Convert OpenTofu/Terraform test JSON into local, reviewable evidence.
//!
//! ```text
//! infra-test-evidence --junit report.xml --evidence-dir evidence tofu-test.jsonl
//! ```

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path, process};

const VERSION: &str = "0.1.0";
const USAGE: &str = "infra-test-evidence 0.1.0\n\nUsage:\n  infra-test-evidence [--json] [--junit <report.xml>] [--evidence-dir <directory>] <terraform-test.jsonl>\n\nReads strict JSON or OpenTofu/Terraform `test -json` event streams. It writes\nstandards-compatible JUnit XML and/or a self-contained, redacted static evidence\npage when requested. The legacy portable evidence record remains supported.\n\nOptions:\n  --json                    Print a machine-readable validation summary\n  --junit <report.xml>      Write a JUnit XML test report\n  --evidence-dir <dir>      Write index.html and evidence.json for reviewers\n  -h, --help                Show this help\n\nExit 0 is valid, 2 is invalid/unreadable input or output failure, and 64 is\nincorrect usage. No data leaves this machine.";

#[derive(Debug, Clone, PartialEq)]
struct Case {
    name: String,
    class_name: String,
    status: String,
    duration_seconds: Option<f64>,
    inputs: Vec<String>,
    assertions: Vec<String>,
    failure: Option<String>,
}

#[derive(Debug, Clone)]
struct Report {
    source_kind: &'static str,
    cases: Vec<Case>,
    diagnostics: Vec<String>,
    plan_summary: Vec<String>,
    input_sha256: String,
}

fn string_field(object: &Map<String, Value>, field: &str) -> Option<String> {
    object.get(field).and_then(Value::as_str).map(str::trim).filter(|value| !value.is_empty()).map(ToOwned::to_owned)
}

fn normalized_status(value: &str) -> Option<&'static str> {
    match value.trim().to_ascii_lowercase().as_str() {
        "pass" | "passed" | "success" => Some("pass"),
        "fail" | "failed" => Some("fail"),
        "error" => Some("error"),
        "skip" | "skipped" | "pending" => Some("skip"),
        _ => None,
    }
}

fn sha256(input: &str) -> String { format!("{:x}", Sha256::digest(input.as_bytes())) }

fn parse_legacy(object: &Map<String, Value>, digest: String) -> Result<Report, Vec<String>> {
    let mut errors = Vec::new();
    for field in ["run", "environment", "recordedAt"] {
        if string_field(object, field).is_none() { errors.push(format!("missing a non-empty string `{field}`")); }
    }
    let Some(checks) = object.get("checks").and_then(Value::as_array) else {
        errors.push("missing a non-empty `checks` array".to_owned());
        return Err(errors);
    };
    if checks.is_empty() { errors.push("missing a non-empty `checks` array".to_owned()); }
    let mut cases = Vec::new();
    for (index, check) in checks.iter().enumerate() {
        let Some(check) = check.as_object() else { errors.push(format!("check {} must be an object", index + 1)); continue };
        let name = string_field(check, "name");
        let check_status = string_field(check, "status").and_then(|value| normalized_status(&value).map(str::to_owned));
        if name.is_none() { errors.push(format!("check {} needs a non-empty name", index + 1)); }
        if check_status.is_none() { errors.push(format!("check {} needs a supported status (pass, fail, error, or skip)", index + 1)); }
        if let (Some(name), Some(check_status)) = (name, check_status) {
            let duration_seconds = check.get("durationMs").and_then(Value::as_f64).map(|value| value / 1000.0);
            let failure = if check_status == "pass" { None } else { string_field(check, "message").map(|message| redact_inline(&message)) };
            cases.push(Case { name, class_name: string_field(object, "run").unwrap_or_else(|| "legacy-evidence".to_owned()), status: check_status, duration_seconds, inputs: vec![format!("environment={}", string_field(object, "environment").unwrap_or_default())], assertions: Vec::new(), failure });
        }
    }
    if errors.is_empty() { Ok(Report { source_kind: "portable-evidence", cases, diagnostics: Vec::new(), plan_summary: Vec::new(), input_sha256: digest }) } else { Err(errors) }
}

fn value_strings(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::String(text)) if !text.trim().is_empty() => vec![text.trim().to_owned()],
        Some(Value::Array(values)) => values.iter().filter_map(Value::as_str).map(str::trim).filter(|text| !text.is_empty()).map(ToOwned::to_owned).collect(),
        _ => Vec::new(),
    }
}

fn sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    ["password", "secret", "token", "api_key", "apikey", "private", "credential", "authorization", "access_key", "client_key", "identifier", "resource", "address", "arn", "_id"].iter().any(|needle| key.contains(needle)) || key == "id" || key.starts_with("id_")
}

fn redact_inline(value: &str) -> String {
    let lowered = value.to_ascii_lowercase();
    if ["password", "secret", "token", "api_key", "apikey", "private_key", "credential", "authorization", "access_key", "client_key"].iter().any(|needle| lowered.contains(needle)) {
        "[REDACTED]".to_owned()
    } else {
        value.to_owned()
    }
}

fn redact(value: &Value, key_hint: Option<&str>) -> Value {
    if key_hint.is_some_and(sensitive_key) { return Value::String("[REDACTED]".to_owned()); }
    match value {
        Value::Object(object) => Value::Object(object.iter().map(|(key, value)| (key.clone(), redact(value, Some(key)))).collect()),
        Value::Array(values) => Value::Array(values.iter().map(|value| redact(value, None)).collect()),
        Value::String(text) => Value::String(redact_inline(text)),
        _ => value.clone(),
    }
}

fn compact_redacted(value: &Value) -> String { serde_json::to_string(&redact(value, None)).unwrap_or_else(|_| "[unserializable]".to_owned()) }

fn terraform_report(events: &[Value], digest: String) -> Result<Report, Vec<String>> {
    let mut cases = Vec::new();
    let mut diagnostics = Vec::new();
    let mut plan_summary = Vec::new();
    for event in events {
        let Some(event) = event.as_object() else { return Err(vec!["every event must be a JSON object".to_owned()]) };
        let event_type = string_field(event, "type").unwrap_or_default();
        if event_type == "test_run" {
            let Some(run) = event.get("test_run").and_then(Value::as_object) else { return Err(vec!["a test_run event is missing its test_run object".to_owned()]) };
            let Some(run_status) = string_field(run, "status").and_then(|value| normalized_status(&value).map(str::to_owned)) else { continue };
            let Some(name) = string_field(run, "run").or_else(|| string_field(run, "name")).or_else(|| string_field(run, "path")) else { return Err(vec!["a completed test_run needs run, name, or path".to_owned()]) };
            let path = string_field(run, "path");
            let mut assertions = value_strings(run.get("assertion_path"));
            assertions.extend(value_strings(run.get("assertions")));
            let failure = string_field(run, "error_message").or_else(|| string_field(run, "message")).map(|message| redact_inline(&message));
            let duration_seconds = run.get("elapsed").and_then(Value::as_f64).or_else(|| run.get("duration_ms").and_then(Value::as_f64).map(|value| value / 1000.0));
            let inputs = path.clone().map(|value| vec![format!("test_file={value}")]).unwrap_or_default();
            cases.push(Case { name, class_name: path.unwrap_or_else(|| "terraform-test".to_owned()), status: run_status, duration_seconds, inputs, assertions, failure });
        } else if event_type == "diagnostic" {
            if let Some(diagnostic) = event.get("diagnostic") { diagnostics.push(compact_redacted(diagnostic)); }
        } else if event_type == "planned_change" || event_type == "resource_planned_change" {
            let action = event.get("change").and_then(Value::as_object).and_then(|change| string_field(change, "action")).or_else(|| string_field(event, "action")).unwrap_or_else(|| "planned change".to_owned());
            plan_summary.push(format!("{action}: resource identifier redacted"));
        }
    }
    if cases.is_empty() { return Err(vec!["no completed OpenTofu/Terraform test_run events were found".to_owned()]); }
    let fallback = diagnostics.first().cloned();
    for case in &mut cases { if case.status != "pass" && case.failure.is_none() { case.failure = fallback.clone(); } }
    Ok(Report { source_kind: "terraform-test-json", cases, diagnostics, plan_summary, input_sha256: digest })
}

fn parse_report(input: &str) -> Result<Report, Vec<String>> {
    let digest = sha256(input);
    if let Ok(value) = serde_json::from_str::<Value>(input) {
        let Some(object) = value.as_object() else { return Err(vec!["input must be a JSON object or JSON-lines event stream".to_owned()]) };
        if object.contains_key("run") || object.contains_key("checks") || object.contains_key("environment") { return parse_legacy(object, digest); }
        return terraform_report(&[value], digest);
    }
    let mut events = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.trim().is_empty() { continue }
        let event: Value = serde_json::from_str(line).map_err(|error| vec![format!("invalid JSON on line {}: {error}", index + 1)])?;
        events.push(event);
    }
    if events.is_empty() { return Err(vec!["input is empty".to_owned()]); }
    terraform_report(&events, digest)
}

fn escape(value: &str) -> String { value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('\"', "&quot;").replace('\'', "&apos;") }

fn junit(report: &Report) -> String {
    let failures = report.cases.iter().filter(|case| case.status == "fail").count();
    let errors = report.cases.iter().filter(|case| case.status == "error").count();
    let skipped = report.cases.iter().filter(|case| case.status == "skip").count();
    let duration = report.cases.iter().filter_map(|case| case.duration_seconds).sum::<f64>();
    let mut xml = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuite name=\"infra-test-evidence\" tests=\"{}\" failures=\"{failures}\" errors=\"{errors}\" skipped=\"{skipped}\" time=\"{duration:.3}\">\n", report.cases.len());
    for case in &report.cases {
        xml.push_str(&format!("  <testcase classname=\"{}\" name=\"{}\" time=\"{:.3}\">", escape(&case.class_name), escape(&case.name), case.duration_seconds.unwrap_or(0.0)));
        let message = case.failure.as_deref().unwrap_or(match case.status.as_str() { "fail" => "test assertion failed", "error" => "test execution error", _ => "" });
        match case.status.as_str() {
            "fail" => xml.push_str(&format!("<failure message=\"{}\">{}</failure>", escape(message), escape(message))),
            "error" => xml.push_str(&format!("<error message=\"{}\">{}</error>", escape(message), escape(message))),
            "skip" => xml.push_str("<skipped />"),
            _ => {}
        }
        xml.push_str("</testcase>\n");
    }
    xml.push_str("</testsuite>\n"); xml
}

fn artifact_value(report: &Report) -> Value {
    let cases: Vec<Value> = report.cases.iter().map(|case| json!({ "name": case.name, "status": case.status, "durationSeconds": case.duration_seconds, "inputs": case.inputs, "assertionPaths": case.assertions, "failure": case.failure })).collect();
    json!({ "schemaVersion": 1, "provenance": { "tool": format!("infra-test-evidence {VERSION}"), "sourceKind": report.source_kind, "inputSha256": report.input_sha256, "redaction": "Recursive secret and resource-identifier redaction is applied by default." }, "testCases": cases, "assertionPaths": report.cases.iter().flat_map(|case| case.assertions.clone()).collect::<Vec<_>>(), "planSummary": report.plan_summary, "failures": report.cases.iter().filter(|case| case.status != "pass").map(|case| json!({"test": case.name, "status": case.status, "message": case.failure})).collect::<Vec<_>>(), "diagnostics": report.diagnostics })
}

fn artifact_html(artifact: &Value) -> String {
    // Keep untrusted JSON from closing the inline data script in a file opened directly.
    let payload = serde_json::to_string(artifact).expect("artifact serializes").replace("</", "<\\/").replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029");
    format!("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Infrastructure test evidence</title><style>body{{margin:0;background:#f7f2e9;color:#1d2933;font:16px/1.5 system-ui,sans-serif}}main{{max-width:1000px;margin:auto;padding:32px 20px}}h1{{color:#193b58}}section{{background:#fffdf8;border-left:5px solid #193b58;padding:18px;margin:18px 0}}pre{{overflow:auto;background:#e9e1d4;padding:14px}}.fail{{color:#8f261c;font-weight:700}}.pass{{color:#18543b;font-weight:700}}</style></head><body><main><p>LOCAL, REDACTED REVIEW ARTIFACT</p><h1>Infrastructure test evidence</h1><p>This artifact is static. It makes no network requests.</p><section><h2>Provenance</h2><pre id=\"provenance\"></pre></section><section><h2>Test-case inputs</h2><div id=\"cases\"></div></section><section><h2>Assertion paths</h2><pre id=\"assertions\"></pre></section><section><h2>Redacted plan summary</h2><pre id=\"plan\"></pre></section><section><h2>Failures</h2><pre id=\"failures\"></pre></section></main><script>const evidence={payload};const write=(id,value)=>document.getElementById(id).textContent=JSON.stringify(value,null,2);write('provenance',evidence.provenance);write('assertions',evidence.assertionPaths);write('plan',evidence.planSummary);write('failures',evidence.failures);document.getElementById('cases').replaceChildren(...evidence.testCases.map(c=>{{const el=document.createElement('article');el.innerHTML='<h3></h3><p></p><pre></pre>';el.querySelector('h3').textContent=c.name;el.querySelector('h3').className=c.status==='pass'?'pass':'fail';el.querySelector('p').textContent='Status: '+c.status;el.querySelector('pre').textContent=JSON.stringify({{inputs:c.inputs,assertionPaths:c.assertionPaths,failure:c.failure}},null,2);return el;}}));</script></body></html>")
}

fn write_outputs(report: &Report, junit_path: Option<&str>, evidence_dir: Option<&str>) -> Result<(), String> {
    if let Some(path) = junit_path { fs::write(path, junit(report)).map_err(|error| format!("cannot write JUnit report {path}: {error}"))?; }
    if let Some(directory) = evidence_dir {
        fs::create_dir_all(directory).map_err(|error| format!("cannot create evidence directory {directory}: {error}"))?;
        let artifact = artifact_value(report);
        fs::write(Path::new(directory).join("evidence.json"), serde_json::to_string_pretty(&artifact).expect("artifact serializes")).map_err(|error| format!("cannot write evidence JSON: {error}"))?;
        fs::write(Path::new(directory).join("index.html"), artifact_html(&artifact)).map_err(|error| format!("cannot write evidence page: {error}"))?;
    }
    Ok(())
}

fn print_result(json_output: bool, report: &Result<Report, Vec<String>>, path: &str) {
    match report {
        Ok(report) if json_output => println!("{}", json!({"valid": true, "checks": report.cases.len(), "errors": []})),
        Err(errors) if json_output => println!("{}", json!({"valid": false, "checks": 0, "errors": errors})),
        Ok(report) => println!("Valid evidence: {} check(s) recorded in {path}", report.cases.len()),
        Err(errors) => eprintln!("Invalid evidence in {path}: {}.", errors.join("; ")),
    }
}

fn main() {
    let mut json_output = false; let mut junit_path = None; let mut evidence_dir = None; let mut input_path = None;
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "-h" | "--help" => { println!("{USAGE}"); return; }
            "--json" => json_output = true,
            "--junit" => match args.next() { Some(path) if !path.starts_with('-') => junit_path = Some(path), _ => { eprintln!("--junit needs an output path\n\n{USAGE}"); process::exit(64); } },
            "--evidence-dir" => match args.next() { Some(path) if !path.starts_with('-') => evidence_dir = Some(path), _ => { eprintln!("--evidence-dir needs an output directory\n\n{USAGE}"); process::exit(64); } },
            value if value.starts_with('-') => { eprintln!("Unknown option: {value}\n\n{USAGE}"); process::exit(64); }
            value if input_path.is_none() => input_path = Some(value.to_owned()),
            _ => { eprintln!("{USAGE}"); process::exit(64); }
        }
    }
    let Some(path) = input_path else { eprintln!("{USAGE}"); process::exit(64); };
    let input = match fs::read_to_string(&path) { Ok(value) => value, Err(error) => { let report = Err(vec![format!("cannot read {path}: {error}")]); print_result(json_output, &report, &path); process::exit(2); } };
    let report = parse_report(&input);
    if let Ok(report) = &report {
        if let Err(error) = write_outputs(report, junit_path.as_deref(), evidence_dir.as_deref()) { let failed = Err(vec![error]); print_result(json_output, &failed, &path); process::exit(2); }
    }
    let valid = report.is_ok(); print_result(json_output, &report, &path); process::exit(if valid { 0 } else { 2 });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_documented_legacy_record_strictly() {
        let report = parse_report(r#"{"run":"deploy-42","environment":"staging","recordedAt":"2026-08-27T10:00:00Z","checks":[{"name":"health","status":"pass"}]}"#).unwrap();
        assert_eq!(report.cases.len(), 1);
        assert_eq!(report.cases[0].status, "pass");
        assert!(parse_report(r#"{"run":"x","environment":"prod","recordedAt":"now","checks":[{}]"#).is_err());
        assert!(parse_report(r#"{"run":"","environment":"","recordedAt":"","checks":[{}]}"#).is_err());
        let failure = parse_report(r#"{"run":"x","environment":"prod","recordedAt":"now","checks":[{"name":"x","status":"fail","message":"token=do-not-leak"}]}"#).unwrap();
        assert_eq!(failure.cases[0].failure.as_deref(), Some("[REDACTED]"));
    }
    #[test]
    fn converts_tofu_events_to_junit_and_redacted_artifact() {
        let input = r#"{"@level":"info","type":"test_run","test_run":{"path":"tests/network.tftest.hcl","run":"denies_public_ingress","status":"fail","elapsed":0.4,"assertion_path":"aws_security_group.web","error_message":"token=shh"}}
{"@level":"info","type":"planned_change","change":{"action":"update","address":"aws_instance.secret[0]","password":"shh"}}"#;
        let report = parse_report(input).unwrap();
        let xml = junit(&report);
        assert!(xml.contains("tests=\"1\" failures=\"1\""));
        assert!(xml.contains("denies_public_ingress"));
        let rendered = serde_json::to_string(&artifact_value(&report)).unwrap();
        assert!(rendered.contains("resource identifier redacted"));
        assert!(!rendered.contains("aws_instance.secret[0]"));
        assert!(!rendered.contains("token=shh"));
    }
    #[test]
    fn creates_a_self_contained_review_page() {
        let report = parse_report(r#"{"type":"test_run","test_run":{"run":"works","status":"pass"}}"#).unwrap();
        let page = artifact_html(&artifact_value(&report));
        assert!(page.contains("Test-case inputs"));
        assert!(page.contains("Redacted plan summary"));
        assert!(page.contains("Provenance"));
    }
}
