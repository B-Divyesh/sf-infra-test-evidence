//! Local OpenTofu/Terraform test evidence converter.
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs, path::Path, process};

const VERSION: &str = "0.1.0";
const USAGE: &str = "infra-test-evidence 0.1.0\n\nUsage:\n  infra-test-evidence [--json] [--junit <report.xml>] [--evidence-dir <directory>] <terraform-test.jsonl>\n\nReads strict JSON or complete OpenTofu/Terraform test JSON event streams.\n\nOptions:\n  --json                    Print a machine-readable validation summary\n  --junit <report.xml>      Write a JUnit XML test report\n  --evidence-dir <dir>      Write index.html and evidence.json for reviewers\n  -h, --help                Show this help\n\nExit 0 is valid, 2 is invalid/unreadable input or output failure, and 64 is incorrect usage.";
#[derive(Clone, Debug, PartialEq)]
struct Case {
    name: String,
    class_name: String,
    status: String,
    duration: Option<f64>,
    inputs: Vec<String>,
    assertions: Vec<String>,
    failure: Option<String>,
}
#[derive(Clone, Debug)]
struct Report {
    source: &'static str,
    cases: Vec<Case>,
    diagnostics: Vec<String>,
    plans: Vec<String>,
    digest: String,
}
#[derive(Default)]
struct Context {
    path: Option<String>,
    run: Option<String>,
    inputs: Vec<String>,
    assertions: Vec<String>,
    diagnostics: Vec<String>,
    plans: Vec<String>,
}

fn field(object: &Map<String, Value>, key: &str) -> Option<String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(ToOwned::to_owned)
}
fn status(value: &str) -> Option<&'static str> {
    match value.trim().to_ascii_lowercase().as_str() {
        "pass" | "passed" | "success" => Some("pass"),
        "fail" | "failed" => Some("fail"),
        "error" => Some("error"),
        "skip" | "skipped" | "pending" => Some("skip"),
        _ => None,
    }
}
fn digest(input: &str) -> String {
    format!("{:x}", Sha256::digest(input.as_bytes()))
}
fn secret(value: &str) -> bool {
    let v = value.to_ascii_lowercase();
    [
        "password",
        "secret",
        "token",
        "api_key",
        "apikey",
        "private_key",
        "credential",
        "authorization",
        "access_key",
        "client_key",
        "db_pass",
        "passwd",
    ]
    .iter()
    .any(|word| v.contains(word))
}
fn sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    secret(&key)
        || ["identifier", "address", "arn", "_id"]
            .iter()
            .any(|word| key.contains(word))
        || key == "id"
        || key.starts_with("id_")
}
fn redact_text(value: &str) -> String {
    if secret(value) {
        "[REDACTED]".to_owned()
    } else {
        value.to_owned()
    }
}
fn has_secret(value: &Value) -> bool {
    match value {
        Value::Object(values) => values
            .iter()
            .any(|(key, value)| secret(key) || has_secret(value)),
        Value::Array(values) => values.iter().any(has_secret),
        Value::String(value) => secret(value),
        _ => false,
    }
}
fn redact(value: &Value, hint: Option<&str>) -> Value {
    if hint.is_some_and(sensitive_key) {
        return Value::String("[REDACTED]".to_owned());
    }
    match value {
        Value::Object(values) => Value::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), redact(value, Some(key))))
                .collect(),
        ),
        Value::Array(values) => {
            Value::Array(values.iter().map(|value| redact(value, None)).collect())
        }
        Value::String(value) => Value::String(redact_text(value)),
        _ => value.clone(),
    }
}
fn compact(value: &Value) -> String {
    serde_json::to_string(&redact(value, None)).unwrap_or_else(|_| "[unserializable]".to_owned())
}
fn diagnostic(value: &Value) -> String {
    if has_secret(value) {
        "[REDACTED SENSITIVE DIAGNOSTIC]".to_owned()
    } else {
        value
            .as_object()
            .and_then(|value| field(value, "detail").or_else(|| field(value, "summary")))
            .unwrap_or_else(|| compact(value))
    }
}
fn duration(value: Option<f64>, label: &str, errors: &mut Vec<String>) -> Option<f64> {
    match value {
        Some(value) if value.is_finite() && value >= 0.0 => Some(value),
        Some(_) => {
            errors.push(format!("{label} must be a non-negative finite number"));
            None
        }
        None => None,
    }
}

fn legacy(object: &Map<String, Value>, input_digest: String) -> Result<Report, Vec<String>> {
    let mut errors = Vec::new();
    for key in ["run", "environment", "recordedAt"] {
        if field(object, key).is_none() {
            errors.push(format!("missing a non-empty string {key}"));
        }
    }
    let Some(checks) = object.get("checks").and_then(Value::as_array) else {
        return Err(vec!["missing a non-empty checks array".to_owned()]);
    };
    if checks.is_empty() {
        errors.push("missing a non-empty checks array".to_owned());
    }
    let mut cases = Vec::new();
    for (i, value) in checks.iter().enumerate() {
        let Some(check) = value.as_object() else {
            errors.push(format!("check {} must be an object", i + 1));
            continue;
        };
        let name = field(check, "name");
        let state = field(check, "status").and_then(|v| status(&v).map(str::to_owned));
        if name.is_none() {
            errors.push(format!("check {} needs a non-empty name", i + 1));
        }
        if state.is_none() {
            errors.push(format!(
                "check {} needs a supported status (pass, fail, error, or skip)",
                i + 1
            ));
        }
        let time = duration(
            check.get("durationMs").and_then(Value::as_f64),
            &format!("check {} durationMs", i + 1),
            &mut errors,
        )
        .map(|v| v / 1000.0);
        if let (Some(name), Some(state)) = (name, state) {
            cases.push(Case {
                name,
                class_name: field(object, "run").unwrap_or_else(|| "legacy-evidence".to_owned()),
                duration: time,
                failure: (state != "pass")
                    .then(|| field(check, "message").map(|v| redact_text(&v)))
                    .flatten(),
                status: state,
                inputs: vec![format!(
                    "environment={}",
                    field(object, "environment").unwrap_or_default()
                )],
                assertions: Vec::new(),
            });
        }
    }
    if errors.is_empty() {
        Ok(Report {
            source: "portable-evidence",
            cases,
            diagnostics: Vec::new(),
            plans: Vec::new(),
            digest: input_digest,
        })
    } else {
        Err(errors)
    }
}

fn event_value(
    event: &Map<String, Value>,
    payload: Option<&Map<String, Value>>,
    keys: &[&str],
) -> Option<String> {
    keys.iter().find_map(|key| {
        field(event, key).or_else(|| payload.and_then(|payload| field(payload, key)))
    })
}
fn key(event: &Map<String, Value>, payload: Option<&Map<String, Value>>) -> Option<String> {
    Some(format!(
        "{}::{}",
        event_value(event, payload, &["@testfile", "test_file", "path"])?,
        event_value(event, payload, &["@testrun", "run", "name"])?
    ))
}
fn identity(
    context: &mut Context,
    event: &Map<String, Value>,
    payload: Option<&Map<String, Value>>,
) {
    context.path = context
        .path
        .clone()
        .or_else(|| event_value(event, payload, &["@testfile", "test_file", "path"]));
    context.run = context
        .run
        .clone()
        .or_else(|| event_value(event, payload, &["@testrun", "run", "name"]));
}
fn paths(value: &Value) -> Vec<String> {
    value
        .pointer("/snippet/values")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_object)
        .filter_map(|value| field(value, "traversal"))
        .map(|value| {
            if secret(&value) {
                "[REDACTED SENSITIVE ASSERTION]".to_owned()
            } else {
                value
            }
        })
        .collect()
}
fn plan(value: &Value, context: &mut Context) {
    let redacted = redact(value, None);
    let Some(plan) = redacted.as_object() else {
        context.plans.push("test plan: [REDACTED]".to_owned());
        return;
    };
    for name in ["variables", "inputs", "outputs"] {
        if let Some(value) = plan.get(name) {
            context.inputs.push(format!(
                "{name}={}",
                serde_json::to_string(value).unwrap_or_else(|_| "[unserializable]".to_owned())
            ));
        }
    }
    for name in ["resource_changes", "planned_changes", "changes"] {
        if let Some(Value::Array(changes)) = plan.get(name) {
            for change in changes {
                let action = change
                    .pointer("/change/actions/0")
                    .and_then(Value::as_str)
                    .or_else(|| change.pointer("/change/action").and_then(Value::as_str))
                    .unwrap_or("planned change");
                context
                    .plans
                    .push(format!("{action}: resource identifier redacted"));
            }
        }
    }
    context.plans.push(format!(
        "test_plan={}",
        serde_json::to_string(&redacted).unwrap_or_else(|_| "[unserializable]".to_owned())
    ));
}

fn stream(events: &[Value], input_digest: String) -> Result<Report, Vec<String>> {
    let mut errors = Vec::new();
    let mut contexts = BTreeMap::<String, Context>::new();
    let mut finished = Vec::<(String, String, String, Option<f64>, Option<String>)>::new();
    let mut diagnostics = Vec::new();
    let mut global_plans = Vec::new();
    let mut summary = None;
    for value in events {
        let Some(event) = value.as_object() else {
            errors.push("every event must be a JSON object".to_owned());
            continue;
        };
        let Some(kind) = field(event, "type") else {
            errors.push("every event needs a non-empty type".to_owned());
            continue;
        };
        match kind.as_str() {
            "test_plan" => {
                let payload = event.get("test_plan").and_then(Value::as_object);
                let Some(run_key) = key(event, payload) else {
                    errors.push("a test_plan event needs test file and run identity".to_owned());
                    continue;
                };
                let context = contexts.entry(run_key).or_default();
                identity(context, event, payload);
                if let Some(payload) = event.get("test_plan") {
                    plan(payload, context);
                } else {
                    errors.push("a test_plan event is missing its test_plan object".to_owned());
                }
            }
            "test_run" => {
                let Some(run) = event.get("test_run").and_then(Value::as_object) else {
                    errors.push("a test_run event is missing its test_run object".to_owned());
                    continue;
                };
                let Some(raw) = field(run, "status") else {
                    errors.push("a test_run event needs a non-empty status".to_owned());
                    continue;
                };
                let Some(state) = status(&raw).map(str::to_owned) else {
                    errors.push(format!("test_run has an unsupported status {raw}"));
                    continue;
                };
                let Some(run_key) = key(event, Some(run)) else {
                    errors.push("a completed test_run needs test file and run identity".to_owned());
                    continue;
                };
                let context = contexts.entry(run_key.clone()).or_default();
                identity(context, event, Some(run));
                let elapsed = duration(
                    run.get("elapsed").and_then(Value::as_f64).or_else(|| {
                        run.get("duration_ms")
                            .and_then(Value::as_f64)
                            .map(|v| v / 1000.0)
                    }),
                    "test_run elapsed",
                    &mut errors,
                );
                finished.push((
                    run_key,
                    context
                        .run
                        .clone()
                        .unwrap_or_else(|| "terraform-test".to_owned()),
                    state,
                    elapsed,
                    field(run, "error_message")
                        .or_else(|| field(run, "message"))
                        .map(|v| redact_text(&v)),
                ));
            }
            "diagnostic" => {
                let item = event.get("diagnostic");
                let payload = item.and_then(Value::as_object);
                let rendered = item.map(diagnostic);
                if let Some(rendered) = &rendered {
                    diagnostics.push(rendered.clone());
                }
                if let (Some(run_key), Some(item)) = (key(event, payload), item) {
                    let context = contexts.entry(run_key).or_default();
                    identity(context, event, payload);
                    context.assertions.extend(paths(item));
                    if let Some(rendered) = rendered {
                        context.diagnostics.push(rendered);
                    }
                }
            }
            "test_summary" => {
                let Some(item) = event.get("test_summary").and_then(Value::as_object) else {
                    errors
                        .push("a test_summary event is missing its test_summary object".to_owned());
                    continue;
                };
                let Some(raw) = field(item, "status") else {
                    errors.push("a test_summary event needs a non-empty status".to_owned());
                    continue;
                };
                let Some(state) = status(&raw).map(str::to_owned) else {
                    errors.push(format!("test_summary has an unsupported status {raw}"));
                    continue;
                };
                if summary.replace(state).is_some() {
                    errors.push("event stream contains more than one test_summary".to_owned());
                }
            }
            "planned_change" | "resource_planned_change" => {
                let action = event
                    .get("change")
                    .and_then(Value::as_object)
                    .and_then(|change| field(change, "action"))
                    .or_else(|| field(event, "action"))
                    .unwrap_or_else(|| "planned change".to_owned());
                global_plans.push(format!("{action}: resource identifier redacted"));
            }
            _ => {}
        }
    }
    if finished.is_empty() {
        errors.push("no completed OpenTofu/Terraform test_run events were found".to_owned());
    }
    let Some(summary) = summary else {
        errors.push("event stream ended without a final test_summary".to_owned());
        return Err(errors);
    };
    if !errors.is_empty() {
        return Err(errors);
    }
    let failed = finished
        .iter()
        .any(|(_, _, state, _, _)| state == "fail" || state == "error");
    if (summary == "pass" && failed) || ((summary == "fail" || summary == "error") && !failed) {
        return Err(vec![
            "test_summary status does not match completed test_run results".to_owned(),
        ]);
    }
    let mut cases = Vec::new();
    let mut plans = global_plans;
    for (run_key, name, state, elapsed, inline_failure) in finished {
        let context = contexts.remove(&run_key).unwrap_or_default();
        let mut inputs = context.inputs;
        if let Some(path) = context.path.clone() {
            inputs.insert(0, format!("test_file={path}"));
        }
        let failure = if state == "pass" {
            None
        } else {
            inline_failure.or_else(|| context.diagnostics.first().cloned())
        };
        plans.extend(context.plans);
        cases.push(Case {
            name,
            class_name: context.path.unwrap_or_else(|| "terraform-test".to_owned()),
            status: state,
            duration: elapsed,
            inputs,
            assertions: context.assertions,
            failure,
        });
    }
    Ok(Report {
        source: "terraform-test-json",
        cases,
        diagnostics,
        plans,
        digest: input_digest,
    })
}
fn parse(input: &str) -> Result<Report, Vec<String>> {
    let input_digest = digest(input);
    if let Ok(value) = serde_json::from_str::<Value>(input) {
        let Some(object) = value.as_object() else {
            return Err(vec![
                "input must be a JSON object or JSON-lines event stream".to_owned(),
            ]);
        };
        return if object.contains_key("run")
            || object.contains_key("checks")
            || object.contains_key("environment")
        {
            legacy(object, input_digest)
        } else {
            stream(&[value], input_digest)
        };
    }
    let mut events = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if !line.trim().is_empty() {
            events.push(
                serde_json::from_str(line)
                    .map_err(|error| vec![format!("invalid JSON on line {}: {error}", i + 1)])?,
            );
        }
    }
    if events.is_empty() {
        Err(vec!["input is empty".to_owned()])
    } else {
        stream(&events, input_digest)
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
fn junit(report: &Report) -> String {
    let failures = report
        .cases
        .iter()
        .filter(|case| case.status == "fail")
        .count();
    let errors = report
        .cases
        .iter()
        .filter(|case| case.status == "error")
        .count();
    let skipped = report
        .cases
        .iter()
        .filter(|case| case.status == "skip")
        .count();
    let duration = report
        .cases
        .iter()
        .filter_map(|case| case.duration)
        .sum::<f64>();
    let mut xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuite name=\"infra-test-evidence\" tests=\"{}\" failures=\"{failures}\" errors=\"{errors}\" skipped=\"{skipped}\" time=\"{duration:.3}\">\n",
        report.cases.len()
    );
    for case in &report.cases {
        xml.push_str(&format!(
            "  <testcase classname=\"{}\" name=\"{}\" time=\"{:.3}\">",
            escape(&case.class_name),
            escape(&case.name),
            case.duration.unwrap_or(0.0)
        ));
        let message = case
            .failure
            .as_deref()
            .unwrap_or(match case.status.as_str() {
                "fail" => "test assertion failed",
                "error" => "test execution error",
                _ => "",
            });
        match case.status.as_str() {
            "fail" => xml.push_str(&format!(
                "<failure message=\"{}\">{}</failure>",
                escape(message),
                escape(message)
            )),
            "error" => xml.push_str(&format!(
                "<error message=\"{}\">{}</error>",
                escape(message),
                escape(message)
            )),
            "skip" => xml.push_str("<skipped />"),
            _ => {}
        }
        xml.push_str("</testcase>\n");
    }
    xml.push_str("</testsuite>\n");
    xml
}
fn artifact(report: &Report) -> Value {
    let cases: Vec<Value> = report.cases.iter().map(|case| json!({"name":case.name,"status":case.status,"durationSeconds":case.duration,"inputs":case.inputs,"assertionPaths":case.assertions,"failure":case.failure})).collect();
    json!({"schemaVersion":1,"provenance":{"tool":format!("infra-test-evidence {VERSION}"),"sourceKind":report.source,"inputSha256":report.digest,"redaction":"Sensitive values and resource identifiers are redacted by default."},"testCases":cases,"assertionPaths":report.cases.iter().flat_map(|case|case.assertions.clone()).collect::<Vec<_>>(),"planSummary":report.plans,"failures":report.cases.iter().filter(|case|case.status!="pass").map(|case|json!({"test":case.name,"status":case.status,"message":case.failure})).collect::<Vec<_>>(),"diagnostics":report.diagnostics})
}
fn page(artifact: &Value) -> String {
    let payload = serde_json::to_string(artifact)
        .expect("artifact serializes")
        .replace("</", "<\\/")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Infrastructure test evidence</title><style>body{{margin:0;background:#f7f2e9;color:#1d2933;font:16px/1.5 system-ui,sans-serif}}main{{max-width:1000px;margin:auto;padding:32px 20px}}h1{{color:#193b58}}section{{background:#fffdf8;border-left:5px solid #193b58;padding:18px;margin:18px 0}}pre{{overflow:auto;background:#e9e1d4;padding:14px}}pre:focus{{outline:3px solid #193b58;outline-offset:3px}}.fail{{color:#8f261c;font-weight:700}}.pass{{color:#18543b;font-weight:700}}</style></head><body><main><p>LOCAL, REDACTED REVIEW ARTIFACT</p><h1>Infrastructure test evidence</h1><p>This artifact is static. It makes no network requests.</p><section><h2>Provenance</h2><pre id=\"provenance\" tabindex=\"0\" aria-label=\"Evidence provenance\"></pre></section><section><h2>Test-case inputs</h2><div id=\"cases\"></div></section><section><h2>Assertion paths</h2><pre id=\"assertions\" tabindex=\"0\" aria-label=\"Assertion paths\"></pre></section><section><h2>Redacted plan summary</h2><pre id=\"plan\" tabindex=\"0\" aria-label=\"Redacted plan summary\"></pre></section><section><h2>Failures</h2><pre id=\"failures\" tabindex=\"0\" aria-label=\"Test failures\"></pre></section></main><script>const evidence={payload};const write=(id,value)=>document.getElementById(id).textContent=JSON.stringify(value,null,2);write('provenance',evidence.provenance);write('assertions',evidence.assertionPaths);write('plan',evidence.planSummary);write('failures',evidence.failures);document.getElementById('cases').replaceChildren(...evidence.testCases.map(c=>{{const el=document.createElement('article');el.innerHTML='<h3></h3><p></p><pre tabindex=\"0\" aria-label=\"Test case details\"></pre>';el.querySelector('h3').textContent=c.name;el.querySelector('h3').className=c.status==='pass'?'pass':'fail';el.querySelector('p').textContent='Status: '+c.status;el.querySelector('pre').textContent=JSON.stringify({{inputs:c.inputs,assertionPaths:c.assertionPaths,failure:c.failure}},null,2);return el;}}));</script></body></html>"
    )
}
fn output(
    report: &Report,
    junit_path: Option<&str>,
    evidence_dir: Option<&str>,
) -> Result<(), String> {
    if let Some(path) = junit_path {
        fs::write(path, junit(report))
            .map_err(|error| format!("cannot write JUnit report {path}: {error}"))?;
    }
    if let Some(dir) = evidence_dir {
        fs::create_dir_all(dir)
            .map_err(|error| format!("cannot create evidence directory {dir}: {error}"))?;
        let artifact = artifact(report);
        fs::write(
            Path::new(dir).join("evidence.json"),
            serde_json::to_string_pretty(&artifact).expect("artifact serializes"),
        )
        .map_err(|error| format!("cannot write evidence JSON: {error}"))?;
        fs::write(Path::new(dir).join("index.html"), page(&artifact))
            .map_err(|error| format!("cannot write evidence page: {error}"))?;
    }
    Ok(())
}
fn print(json_output: bool, report: &Result<Report, Vec<String>>, path: &str) {
    match report {
        Ok(report) if json_output => println!(
            "{}",
            json!({"valid":true,"checks":report.cases.len(),"errors":[]})
        ),
        Err(errors) if json_output => {
            println!("{}", json!({"valid":false,"checks":0,"errors":errors}))
        }
        Ok(report) => println!(
            "Valid evidence: {} check(s) recorded in {path}",
            report.cases.len()
        ),
        Err(errors) => eprintln!("Invalid evidence in {path}: {}.", errors.join("; ")),
    }
}
fn main() {
    let mut json_output = false;
    let mut junit_path = None;
    let mut evidence_dir = None;
    let mut input_path = None;
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "-h" | "--help" => {
                println!("{USAGE}");
                return;
            }
            "--json" => json_output = true,
            "--junit" => match args.next() {
                Some(path) if !path.starts_with('-') => junit_path = Some(path),
                _ => {
                    eprintln!("--junit needs an output path\n\n{USAGE}");
                    process::exit(64)
                }
            },
            "--evidence-dir" => match args.next() {
                Some(path) if !path.starts_with('-') => evidence_dir = Some(path),
                _ => {
                    eprintln!("--evidence-dir needs an output directory\n\n{USAGE}");
                    process::exit(64)
                }
            },
            value if value.starts_with('-') => {
                eprintln!("Unknown option: {value}\n\n{USAGE}");
                process::exit(64)
            }
            value if input_path.is_none() => input_path = Some(value.to_owned()),
            _ => {
                eprintln!("{USAGE}");
                process::exit(64)
            }
        }
    }
    let Some(path) = input_path else {
        eprintln!("{USAGE}");
        process::exit(64)
    };
    let input = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(error) => {
            let report = Err(vec![format!("cannot read {path}: {error}")]);
            print(json_output, &report, &path);
            process::exit(2)
        }
    };
    let report = parse(&input);
    if let Ok(report) = &report
        && let Err(error) = output(report, junit_path.as_deref(), evidence_dir.as_deref())
    {
        let failed = Err(vec![error]);
        print(json_output, &failed, &path);
        process::exit(2)
    };
    let valid = report.is_ok();
    print(json_output, &report, &path);
    process::exit(if valid { 0 } else { 2 });
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../examples/opentofu-real-stream.jsonl");
    #[test]
    fn legacy_is_strict() {
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"pass"}]}"#).is_ok());
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"mystery"}]}"#).is_err());
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"pass","durationMs":-1}]}"#).is_err());
    }
    #[test]
    fn real_stream_is_scoped_and_redacted() {
        let report = parse(FIXTURE).unwrap();
        assert_eq!(report.cases.len(), 2);
        assert_eq!(
            report.cases[0].failure.as_deref(),
            Some("expected production environment")
        );
        assert_eq!(
            report.cases[1].failure.as_deref(),
            Some("[REDACTED SENSITIVE DIAGNOSTIC]")
        );
        assert!(
            report.cases[0]
                .inputs
                .iter()
                .any(|value| value.contains("variables"))
        );
        assert!(
            report.cases[0]
                .assertions
                .iter()
                .any(|value| value == "var.environment")
        );
        let rendered = serde_json::to_string(&artifact(&report)).unwrap();
        assert!(!rendered.contains("s3cr3t-sentinel"));
        assert!(
            report
                .plans
                .iter()
                .any(|value| value.starts_with("create:"))
        );
    }
    #[test]
    fn incomplete_and_unknown_streams_fail_closed() {
        assert!(
            parse(
                r#"{"type":"test_run","@testfile":"x","@testrun":"a","test_run":{"status":"pass"}}"#
            )
            .is_err()
        );
        assert!(parse("{\"type\":\"test_run\",\"@testfile\":\"x\",\"@testrun\":\"a\",\"test_run\":{\"status\":\"pass\"}}\n{\"type\":\"test_run\",\"@testfile\":\"x\",\"@testrun\":\"b\",\"test_run\":{\"status\":\"mystery\"}}\n{\"type\":\"test_summary\",\"test_summary\":{\"status\":\"fail\"}}").is_err());
    }
    #[test]
    fn review_page_is_keyboard_scrollable() {
        let html = page(&artifact(&parse(FIXTURE).unwrap()));
        assert!(html.contains("id=\"provenance\" tabindex=\"0\""));
        assert!(html.contains("<pre tabindex=\"0\" aria-label=\"Test case details\""));
    }
}
