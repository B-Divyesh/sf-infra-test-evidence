//! Local OpenTofu/Terraform test evidence converter.
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

const VERSION: &str = "0.1.0";
const USAGE: &str = "infra-test-evidence 0.1.0\n\nUsage:\n  infra-test-evidence [--json] [--junit <report.xml>] [--evidence-dir <directory>] <terraform-test.jsonl>\n  infra-test-evidence --demo\n\nReads strict JSON or complete OpenTofu/Terraform test JSON event streams.\n\nOptions:\n  --demo                    Run the bundled sample in a new temporary directory\n  --json                    Print a machine-readable validation summary\n  --junit <report.xml>      Write a JUnit XML test report\n  --evidence-dir <dir>      Write index.html and evidence.json for reviewers\n  -h, --help                Show this help\n\nExit 0 is valid, 2 is invalid/unreadable input or output failure, and 64 is incorrect usage.";
const DEMO_FIXTURE: &str = include_str!("../examples/tofu-test.jsonl");
#[derive(Clone, Debug, PartialEq)]
struct Case {
    name: String,
    class_name: String,
    status: String,
    duration: Option<f64>,
    inputs: Vec<String>,
    assertions: Vec<String>,
    plans: Vec<String>,
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
fn redacted_field(object: &Map<String, Value>, key: &str) -> Option<String> {
    field(object, key).map(|value| redact_text(&value))
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
fn aws_arn(value: &str) -> bool {
    value
        .as_bytes()
        .windows(b"arn:aws".len())
        .any(|window| window == b"arn:aws")
}
fn ec2_instance_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    for index in 0..bytes.len().saturating_sub(2) {
        let starts_identifier = bytes[index] == b'i'
            && bytes[index + 1] == b'-'
            && (index == 0
                || !matches!(
                    bytes[index - 1],
                    b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-'
                ));
        if !starts_identifier {
            continue;
        }
        let mut end = index + 2;
        while end < bytes.len() && bytes[end].is_ascii_hexdigit() {
            end += 1;
        }
        // Real EC2 IDs use 8 or 17 hexadecimal characters. Accept six here
        // because test runners commonly shorten fixture identifiers.
        if end - (index + 2) >= 6 {
            return true;
        }
    }
    false
}
fn resource_identifier(value: &str) -> bool {
    aws_arn(value) || ec2_instance_id(value)
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
    if secret(value) || resource_identifier(value) {
        "[REDACTED]".to_owned()
    } else {
        value.to_owned()
    }
}
const REDACTED: &str = "[REDACTED]";

/// OpenTofu and Terraform use both an inline `sensitive: true` wrapper and
/// structural masks such as `after_sensitive` and `sensitive_values`. A
/// marker is security metadata, not a hint: an unrecognised marker shape must
/// stop conversion before a shareable artifact can be written.
fn sensitivity_mask(value: &Value, label: &str) -> Result<(), String> {
    match value {
        Value::Bool(_) => Ok(()),
        Value::Array(values) => values
            .iter()
            .enumerate()
            .try_for_each(|(index, value)| sensitivity_mask(value, &format!("{label}[{index}]"))),
        Value::Object(values) => values
            .iter()
            .try_for_each(|(key, value)| sensitivity_mask(value, &format!("{label}.{key}"))),
        _ => Err(format!(
            "cannot safely interpret sensitivity marker {label}; expected a boolean or a matching boolean mask"
        )),
    }
}
fn marker_for<'a>(values: &'a Map<String, Value>, key: &str) -> Option<(&'static str, &'a Value)> {
    match key {
        "before" => values
            .get("before_sensitive")
            .map(|value| ("before_sensitive", value)),
        "after" => values
            .get("after_sensitive")
            .map(|value| ("after_sensitive", value)),
        "values" => values
            .get("sensitive_values")
            .map(|value| ("sensitive_values", value)),
        _ => None,
    }
}
fn redact_masked(value: &Value, mask: &Value, label: &str) -> Result<Value, String> {
    match mask {
        Value::Bool(true) => Ok(Value::String(REDACTED.to_owned())),
        Value::Bool(false) => redact(value, None),
        Value::Object(mask_values) => {
            let Value::Object(values) = value else {
                return Err(format!(
                    "cannot safely apply sensitivity marker {label} to a non-object value"
                ));
            };
            let mut redacted = Map::new();
            for (key, child) in values {
                let child = match mask_values.get(key) {
                    Some(mask) => redact_masked(child, mask, &format!("{label}.{key}"))?,
                    None => redact(child, Some(key))?,
                };
                redacted.insert(key.clone(), child);
            }
            for key in mask_values.keys() {
                if !values.contains_key(key) {
                    return Err(format!(
                        "cannot safely apply sensitivity marker {label}; it names missing value {key}"
                    ));
                }
            }
            Ok(Value::Object(redacted))
        }
        Value::Array(masks) => {
            let Value::Array(values) = value else {
                return Err(format!(
                    "cannot safely apply sensitivity marker {label} to a non-array value"
                ));
            };
            if values.len() != masks.len() {
                return Err(format!(
                    "cannot safely apply sensitivity marker {label}; array lengths differ"
                ));
            }
            values
                .iter()
                .zip(masks)
                .enumerate()
                .map(|(index, (value, mask))| {
                    redact_masked(value, mask, &format!("{label}[{index}]"))
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array)
        }
        _ => Err(format!(
            "cannot safely interpret sensitivity marker {label}; expected a boolean or a matching boolean mask"
        )),
    }
}
fn redact(value: &Value, hint: Option<&str>) -> Result<Value, String> {
    if hint.is_some_and(sensitive_key) {
        return Ok(Value::String(REDACTED.to_owned()));
    }
    match value {
        Value::Object(values) => {
            if let Some(marker) = values.get("sensitive") {
                match marker {
                    Value::Bool(true) => return Ok(Value::String(REDACTED.to_owned())),
                    Value::Bool(false) => {}
                    _ => {
                        return Err("cannot safely interpret explicit sensitive marker; expected true or false".to_owned());
                    }
                }
            }
            for (key, marker) in values {
                if ["before_sensitive", "after_sensitive", "sensitive_values"]
                    .contains(&key.as_str())
                {
                    sensitivity_mask(marker, key)?;
                }
            }
            let mut redacted = Map::new();
            for (key, child) in values {
                let child = match marker_for(values, key) {
                    Some((label, marker)) => redact_masked(child, marker, label)?,
                    None => redact(child, Some(key))?,
                };
                redacted.insert(key.clone(), child);
            }
            Ok(Value::Object(redacted))
        }
        Value::Array(values) => values
            .iter()
            .map(|value| redact(value, None))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Value::String(value) => Ok(Value::String(redact_text(value))),
        _ => Ok(value.clone()),
    }
}
fn compact(value: &Value) -> Result<String, String> {
    serde_json::to_string(&redact(value, None)?)
        .map_err(|_| "cannot serialize redacted diagnostic".to_owned())
}
fn diagnostic(value: &Value) -> Result<String, String> {
    let redacted = redact(value, None)?;
    if redacted != *value {
        Ok("[REDACTED SENSITIVE DIAGNOSTIC]".to_owned())
    } else {
        Ok(value
            .as_object()
            .and_then(|value| field(value, "detail").or_else(|| field(value, "summary")))
            .unwrap_or(compact(value)?))
    }
}
fn duration(value: Option<&Value>, label: &str, errors: &mut Vec<String>) -> Option<f64> {
    match value {
        Some(value)
            if value
                .as_f64()
                .is_some_and(|value| value.is_finite() && value >= 0.0) =>
        {
            value.as_f64()
        }
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
        let name = redacted_field(check, "name");
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
            check.get("durationMs"),
            &format!("check {} durationMs", i + 1),
            &mut errors,
        )
        .map(|v| v / 1000.0);
        if let (Some(name), Some(state)) = (name, state) {
            cases.push(Case {
                name,
                class_name: redacted_field(object, "run")
                    .unwrap_or_else(|| "legacy-evidence".to_owned()),
                duration: time,
                failure: (state != "pass")
                    .then(|| field(check, "message").map(|v| redact_text(&v)))
                    .flatten(),
                status: state,
                inputs: vec![format!(
                    "environment={}",
                    redacted_field(object, "environment").unwrap_or_default()
                )],
                assertions: Vec::new(),
                plans: Vec::new(),
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
fn redacted_object(value: &Value, label: &str) -> Result<Map<String, Value>, String> {
    match redact(value, None)? {
        Value::Object(value) => Ok(value),
        _ => Err(format!(
            "cannot safely read {label}; an explicit sensitivity marker conceals required fields"
        )),
    }
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
fn paths(value: &Value) -> Result<Vec<String>, String> {
    let redacted = redact(value, None)?;
    if redacted != *value {
        return Ok(vec!["[REDACTED SENSITIVE ASSERTION]".to_owned()]);
    }
    let redact_path = |value: String| {
        if redact_text(&value) != value {
            "[REDACTED SENSITIVE ASSERTION]".to_owned()
        } else {
            value
        }
    };
    let mut found = Vec::new();
    if let Some(value) = redacted
        .as_object()
        .and_then(|value| field(value, "assertion_path"))
    {
        found.push(redact_path(value));
    }
    if let Some(values) = redacted
        .as_object()
        .and_then(|value| value.get("assertion_paths"))
        .and_then(Value::as_array)
    {
        found.extend(
            values
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .map(&redact_path),
        );
    }
    found.extend(
        redacted
            .pointer("/snippet/values")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_object)
            .filter_map(|value| field(value, "traversal"))
            .map(redact_path),
    );
    Ok(found)
}
fn plan(value: &Value, context: &mut Context) -> Result<(), String> {
    let redacted = redact(value, None)?;
    let Some(plan) = redacted.as_object() else {
        context.plans.push("test plan: [REDACTED]".to_owned());
        return Ok(());
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
    Ok(())
}

fn stream(events: &[Value], input_digest: String) -> Result<Report, Vec<String>> {
    let mut errors = Vec::new();
    let mut contexts = BTreeMap::<String, Context>::new();
    let mut finished = Vec::<(String, String, String, Option<f64>, Option<String>)>::new();
    let mut diagnostics = Vec::new();
    let mut global_plans = Vec::new();
    let mut summary = None;
    for (event_index, value) in events.iter().enumerate() {
        let Some(raw_event) = value.as_object() else {
            errors.push("every event must be a JSON object".to_owned());
            continue;
        };
        let event = match redacted_object(value, "event") {
            Ok(event) => event,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        let Some(kind) = field(&event, "type") else {
            errors.push("every event needs a non-empty type".to_owned());
            continue;
        };
        match kind.as_str() {
            "test_plan" => {
                let Some(raw_payload) = raw_event.get("test_plan") else {
                    errors.push("a test_plan event is missing its test_plan object".to_owned());
                    continue;
                };
                let payload = match redacted_object(raw_payload, "test_plan") {
                    Ok(payload) => payload,
                    Err(error) => {
                        errors.push(error);
                        continue;
                    }
                };
                let Some(run_key) = key(&event, Some(&payload)) else {
                    errors.push("a test_plan event needs test file and run identity".to_owned());
                    continue;
                };
                let context = contexts.entry(run_key).or_default();
                identity(context, &event, Some(&payload));
                if let Err(error) = plan(&Value::Object(payload), context) {
                    errors.push(error);
                }
            }
            "test_run" => {
                let Some(raw_run) = raw_event.get("test_run") else {
                    errors.push("a test_run event is missing its test_run object".to_owned());
                    continue;
                };
                let run = match redacted_object(raw_run, "test_run") {
                    Ok(run) => run,
                    Err(error) => {
                        errors.push(error);
                        continue;
                    }
                };
                let Some(raw) = field(&run, "status") else {
                    errors.push("a test_run event needs a non-empty status".to_owned());
                    continue;
                };
                let Some(state) = status(&raw).map(str::to_owned) else {
                    errors.push(format!("test_run has an unsupported status {raw}"));
                    continue;
                };
                let Some(run_key) = key(&event, Some(&run)) else {
                    errors.push("a completed test_run needs test file and run identity".to_owned());
                    continue;
                };
                let context = contexts.entry(run_key.clone()).or_default();
                identity(context, &event, Some(&run));
                if raw_run.as_object().is_some_and(|raw_run| {
                    raw_run.contains_key("assertion_path")
                        || raw_run.contains_key("assertion_paths")
                }) {
                    match paths(raw_run) {
                        Ok(paths) => context.assertions.extend(paths),
                        Err(error) => errors.push(error),
                    }
                }
                let elapsed = if run.contains_key("elapsed") {
                    duration(run.get("elapsed"), "test_run elapsed", &mut errors)
                } else {
                    duration(run.get("duration_ms"), "test_run duration_ms", &mut errors)
                        .map(|value| value / 1000.0)
                };
                finished.push((
                    run_key,
                    context
                        .run
                        .clone()
                        .unwrap_or_else(|| "terraform-test".to_owned()),
                    state,
                    elapsed,
                    field(&run, "error_message")
                        .or_else(|| field(&run, "message"))
                        .map(|v| redact_text(&v)),
                ));
            }
            "diagnostic" => {
                let item = raw_event.get("diagnostic");
                let payload = event.get("diagnostic").and_then(Value::as_object);
                let rendered = match item.map(diagnostic).transpose() {
                    Ok(rendered) => rendered,
                    Err(error) => {
                        errors.push(error);
                        None
                    }
                };
                if let Some(rendered) = rendered.as_ref() {
                    diagnostics.push(rendered.clone());
                }
                if let (Some(run_key), Some(item)) = (key(&event, payload), item) {
                    let context = contexts.entry(run_key).or_default();
                    identity(context, &event, payload);
                    match paths(item) {
                        Ok(paths) => context.assertions.extend(paths),
                        Err(error) => errors.push(error),
                    }
                    if let Some(rendered) = rendered {
                        context.diagnostics.push(rendered);
                    }
                }
            }
            "test_summary" => {
                let Some(raw_item) = raw_event.get("test_summary") else {
                    errors
                        .push("a test_summary event is missing its test_summary object".to_owned());
                    continue;
                };
                let item = match redacted_object(raw_item, "test_summary") {
                    Ok(item) => item,
                    Err(error) => {
                        errors.push(error);
                        continue;
                    }
                };
                let Some(raw) = field(&item, "status") else {
                    errors.push("a test_summary event needs a non-empty status".to_owned());
                    continue;
                };
                let Some(state) = status(&raw).map(str::to_owned) else {
                    errors.push(format!("test_summary has an unsupported status {raw}"));
                    continue;
                };
                if summary.replace((state, event_index)).is_some() {
                    errors.push("event stream contains more than one test_summary".to_owned());
                }
            }
            "planned_change" | "resource_planned_change" => {
                let action = event
                    .get("change")
                    .and_then(Value::as_object)
                    .and_then(|change| field(change, "action"))
                    .or_else(|| field(&event, "action"))
                    .unwrap_or_else(|| "planned change".to_owned());
                global_plans.push(format!("{action}: resource identifier redacted"));
            }
            _ => {}
        }
    }
    if finished.is_empty() {
        errors.push("no completed OpenTofu/Terraform test_run events were found".to_owned());
    }
    let Some((summary, summary_index)) = summary else {
        errors.push("event stream ended without a final test_summary".to_owned());
        return Err(errors);
    };
    if summary_index + 1 != events.len() {
        errors.push("test_summary must be the final event in the stream".to_owned());
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let failed = finished
        .iter()
        .any(|(_, _, state, _, _)| state == "fail" || state == "error");
    let summary_matches_results = match summary.as_str() {
        "pass" => !failed,
        "fail" | "error" => failed,
        "skip" => finished.iter().all(|(_, _, state, _, _)| state == "skip"),
        _ => false,
    };
    if !summary_matches_results {
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
        let case_plans = context.plans;
        plans.extend(case_plans.clone());
        cases.push(Case {
            name,
            class_name: context.path.unwrap_or_else(|| "terraform-test".to_owned()),
            status: state,
            duration: elapsed,
            inputs,
            assertions: context.assertions,
            plans: case_plans,
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
    let cases: Vec<Value> = report.cases.iter().map(|case| json!({"name":case.name,"status":case.status,"durationSeconds":case.duration,"inputs":case.inputs,"assertionPaths":case.assertions,"planSummary":case.plans,"failure":case.failure})).collect();
    json!({"schemaVersion":1,"provenance":{"tool":format!("infra-test-evidence {VERSION}"),"sourceKind":report.source,"inputSha256":report.digest,"redaction":"Sensitive values and resource identifiers are redacted by default."},"testCases":cases,"assertionPaths":report.cases.iter().flat_map(|case|case.assertions.clone()).collect::<Vec<_>>(),"planSummary":report.plans,"failures":report.cases.iter().filter(|case|case.status!="pass").map(|case|json!({"test":case.name,"status":case.status,"message":case.failure})).collect::<Vec<_>>(),"diagnostics":report.diagnostics})
}
fn page(artifact: &Value) -> String {
    let payload = serde_json::to_string(artifact)
        .expect("artifact serializes")
        .replace("</", "<\\/")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Infrastructure test evidence</title><style>body{{margin:0;background:#f7f2e9;color:#1d2933;font:16px/1.5 system-ui,sans-serif}}main{{max-width:1000px;margin:auto;padding:32px 20px}}h1{{color:#193b58}}section{{background:#fffdf8;border-left:5px solid #193b58;padding:18px;margin:18px 0}}pre{{overflow:auto;background:#e9e1d4;padding:14px}}pre:focus{{outline:3px solid #193b58;outline-offset:3px}}.fail{{color:#8f261c;font-weight:700}}.pass{{color:#18543b;font-weight:700}}</style></head><body><main><p>LOCAL, REDACTED REVIEW ARTIFACT</p><h1>Infrastructure test evidence</h1><p>This artifact is static. It makes no network requests.</p><section><h2>Provenance</h2><pre id=\"provenance\" tabindex=\"0\" aria-label=\"Evidence provenance\"></pre></section><section><h2>Test-case inputs</h2><div id=\"cases\"></div></section><section><h2>Assertion paths</h2><pre id=\"assertions\" tabindex=\"0\" aria-label=\"Assertion paths\"></pre></section><section><h2>Redacted plan summary</h2><pre id=\"plan\" tabindex=\"0\" aria-label=\"Redacted plan summary\"></pre></section><section><h2>Failures</h2><pre id=\"failures\" tabindex=\"0\" aria-label=\"Test failures\"></pre></section></main><script>const evidence={payload};const write=(id,value)=>document.getElementById(id).textContent=JSON.stringify(value,null,2);write('provenance',evidence.provenance);write('assertions',evidence.assertionPaths);write('plan',evidence.planSummary);write('failures',evidence.failures);document.getElementById('cases').replaceChildren(...evidence.testCases.map(c=>{{const el=document.createElement('article');el.innerHTML='<h3></h3><p></p><pre tabindex=\"0\" aria-label=\"Test case details\"></pre>';el.querySelector('h3').textContent=c.name;el.querySelector('h3').className=c.status==='pass'?'pass':'fail';el.querySelector('p').textContent='Status: '+c.status;el.querySelector('pre').textContent=JSON.stringify({{inputs:c.inputs,assertionPaths:c.assertionPaths,planSummary:c.planSummary,failure:c.failure}},null,2);return el;}}));</script></body></html>"
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

fn create_demo_directory() -> Result<PathBuf, String> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("cannot create a unique demo directory: {error}"))?
        .as_nanos();
    for attempt in 0..100 {
        let path = env::temp_dir().join(format!(
            "infra-test-evidence-demo-{}-{timestamp}-{attempt}",
            process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "cannot create demo directory {}: {error}",
                    path.display()
                ));
            }
        }
    }
    Err("cannot create a unique demo directory after 100 attempts".to_owned())
}

fn run_demo() -> Result<(), String> {
    let directory = create_demo_directory()?;
    let sample_path = directory.join("tofu-test.jsonl");
    let junit_path = directory.join("report.xml");
    let evidence_dir = directory.join("evidence");
    fs::write(&sample_path, DEMO_FIXTURE).map_err(|error| {
        format!(
            "cannot write bundled sample {}: {error}",
            sample_path.display()
        )
    })?;
    let report = parse(DEMO_FIXTURE)
        .map_err(|errors| format!("bundled sample is invalid: {}", errors.join("; ")))?;
    let junit_output = junit_path.to_string_lossy().into_owned();
    let evidence_output = evidence_dir.to_string_lossy().into_owned();
    output(&report, Some(&junit_output), Some(&evidence_output))?;
    println!("Demo complete: {} checks converted", report.cases.len());
    println!("Demo directory: {}", directory.display());
    println!("Sample input: {}", sample_path.display());
    println!("JUnit report: {}", junit_path.display());
    println!(
        "Reviewer page: {}",
        evidence_dir.join("index.html").display()
    );
    println!(
        "Evidence JSON: {}",
        evidence_dir.join("evidence.json").display()
    );
    Ok(())
}

fn main() {
    let mut demo = false;
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
            "--demo" => demo = true,
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
    if demo {
        if json_output || junit_path.is_some() || evidence_dir.is_some() || input_path.is_some() {
            eprintln!("--demo cannot be combined with an input or output options\n\n{USAGE}");
            process::exit(64)
        }
        if let Err(error) = run_demo() {
            eprintln!("Demo failed: {error}.");
            process::exit(2)
        }
        return;
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
    const EXPLICIT_SENSITIVE_FIXTURE: &str =
        include_str!("../examples/explicit-sensitive-output.jsonl");
    const FIXTURE: &str = include_str!("../examples/opentofu-real-stream.jsonl");
    const RESOURCE_IDENTIFIER_FIXTURE: &str =
        include_str!("../tests/fixtures/verification-9-resource-identifiers.jsonl");
    const LEGACY_IDENTIFIER_FIXTURE: &str =
        include_str!("../tests/fixtures/verification-9-legacy-identifiers.json");
    const DURATION_STRING_FIXTURE: &str =
        include_str!("../tests/fixtures/verification-9-duration-string.json");
    const ELAPSED_STRING_FIXTURE: &str =
        include_str!("../tests/fixtures/verification-9-elapsed-string.jsonl");
    const SKIPPED_SUMMARY_FIXTURE: &str =
        include_str!("../tests/fixtures/verification-9-skipped-summary.jsonl");
    #[test]
    fn legacy_is_strict() {
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"pass"}]}"#).is_ok());
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"mystery"}]}"#).is_err());
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"pass","durationMs":-1}]}"#).is_err());
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"pass","durationMs":"fast"}]}"#).is_err());
        assert!(parse(r#"{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"ok","status":"pass","durationMs":false}]}"#).is_err());
    }
    #[test]
    fn verification_9_counterexamples_are_rejected_or_redacted() {
        for fixture in [DURATION_STRING_FIXTURE, ELAPSED_STRING_FIXTURE] {
            let errors = parse(fixture).unwrap_err();
            assert!(
                errors
                    .iter()
                    .any(|error| error.contains("must be a non-negative finite number"))
            );
        }
        let errors = parse(SKIPPED_SUMMARY_FIXTURE).unwrap_err();
        assert!(
            errors
                .iter()
                .any(|error| error.contains("test_summary status does not match"))
        );

        for fixture in [RESOURCE_IDENTIFIER_FIXTURE, LEGACY_IDENTIFIER_FIXTURE] {
            let report = parse(fixture).unwrap();
            for output in [
                junit(&report),
                serde_json::to_string(&artifact(&report)).unwrap(),
                page(&artifact(&report)),
            ] {
                for identifier in ["arn:aws", "i-0abc123", "aws_instance.web"] {
                    assert!(!output.contains(identifier));
                }
            }
        }

        let demo = parse(DEMO_FIXTURE).unwrap();
        assert_eq!(demo.cases.len(), 2);
        assert!(
            demo.cases.iter().all(|case| {
                case.assertions == vec!["aws_security_group.web.ingress".to_owned()]
            })
        );
        assert!(
            serde_json::to_string(&artifact(&demo))
                .unwrap()
                .contains(REDACTED)
        );
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
    fn explicit_sensitive_values_and_terraform_masks_are_redacted() {
        let report = parse(EXPLICIT_SENSITIVE_FIXTURE).unwrap();
        let rendered = serde_json::to_string(&artifact(&report)).unwrap();
        assert!(!rendered.contains("k9M2qV7xL4"));
        assert!(rendered.contains(REDACTED));

        let terraform_plan = json!({
            "change": {
                "after": {
                    "password": "terraform-after-sentinel",
                    "nested": { "token": "terraform-nested-sentinel" }
                },
                "after_sensitive": {
                    "password": true,
                    "nested": { "token": true }
                }
            },
            "values": {
                "private": "terraform-values-sentinel",
                "public": "safe"
            },
            "sensitive_values": { "private": true }
        });
        let redacted = redact(&terraform_plan, None).unwrap();
        let rendered = serde_json::to_string(&redacted).unwrap();
        for sentinel in [
            "terraform-after-sentinel",
            "terraform-nested-sentinel",
            "terraform-values-sentinel",
        ] {
            assert!(!rendered.contains(sentinel));
        }
        assert!(rendered.contains("safe"));
    }
    #[test]
    fn malformed_sensitivity_markers_fail_closed() {
        let malformed = r#"{"type":"test_plan","@testfile":"x","@testrun":"a","test_plan":{"outputs":{"session":{"sensitive":"unknown","value":"opaque"}}}}
{"type":"test_run","@testfile":"x","@testrun":"a","test_run":{"status":"pass"}}
{"type":"test_summary","test_summary":{"status":"pass"}}"#;
        let errors = parse(malformed).unwrap_err();
        assert!(
            errors
                .iter()
                .any(|error| error.contains("cannot safely interpret explicit sensitive marker"))
        );
        assert!(
            redact(
                &json!({"after": {"value": "opaque"}, "after_sensitive": "unknown"}),
                None
            )
            .is_err()
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
