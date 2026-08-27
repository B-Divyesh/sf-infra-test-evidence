//! Validate a small, portable JSON record of an infrastructure test run.
//!
//! ```text
//! infra-test-evidence --json examples/passing-evidence.json
//! ```

use std::{env, fs, process};

const USAGE: &str = "infra-test-evidence 0.1.0\n\nUsage:\n  infra-test-evidence [--json] <evidence.json>\n\nChecks that an evidence record names its run, environment, timestamp, and at\nleast one check. `--json` emits a machine-readable result. Exit 0 is valid,\n2 is invalid input, and 64 is incorrect usage. No data leaves this machine.";

#[derive(Debug, PartialEq)]
struct Report {
    valid: bool,
    errors: Vec<&'static str>,
    checks: usize,
}

fn has_string_field(input: &str, field: &str) -> bool {
    let needle = format!("\"{field}\"");
    input.find(&needle)
        .and_then(|index| input[index + needle.len()..].find(':').map(|colon| index + needle.len() + colon))
        .is_some_and(|colon| input[colon + 1..].trim_start().starts_with('"'))
}

fn count_checks(input: &str) -> usize {
    let Some(checks) = input.find("\"checks\"") else { return 0 };
    let remainder = &input[checks..];
    let Some(open) = remainder.find('[') else { return 0 };
    let Some(close) = remainder[open + 1..].find(']') else { return 0 };
    remainder[open + 1..open + 1 + close].matches('{').count()
}

fn validate(input: &str) -> Report {
    let mut errors = Vec::new();
    for (field, message) in [
        ("run", "missing a string `run`"),
        ("environment", "missing a string `environment`"),
        ("recordedAt", "missing a string `recordedAt`"),
    ] {
        if !has_string_field(input, field) { errors.push(message); }
    }
    let checks = count_checks(input);
    if checks == 0 { errors.push("missing a non-empty `checks` array"); }
    Report { valid: errors.is_empty(), errors, checks }
}

fn json_escape(value: &str) -> String { value.replace('\\', "\\\\").replace('"', "\\\"") }

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        println!("{USAGE}");
        process::exit(if args.is_empty() { 64 } else { 0 });
    }
    let json = args.first().is_some_and(|arg| arg == "--json");
    let path = if json { args.get(1) } else { args.first() };
    if path.is_none() || args.len() != if json { 2 } else { 1 } {
        eprintln!("{USAGE}"); process::exit(64);
    }
    let path = path.expect("checked above");
    let input = match fs::read_to_string(path) {
        Ok(value) => value,
        Err(error) => { eprintln!("Cannot read {path}: {error}"); process::exit(2); }
    };
    let report = validate(&input);
    if json {
        let errors = report.errors.iter().map(|error| format!("\"{}\"", json_escape(error))).collect::<Vec<_>>().join(",");
        println!("{{\"valid\":{},\"checks\":{},\"errors\":[{}]}}", report.valid, report.checks, errors);
    } else if report.valid {
        println!("Valid evidence: {} check(s) recorded in {path}", report.checks);
    } else {
        eprintln!("Invalid evidence in {path}: {}.", report.errors.join("; "));
    }
    process::exit(if report.valid { 0 } else { 2 });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_documented_record() {
        let report = validate(r#"{"run":"deploy-42","environment":"staging","recordedAt":"2026-08-27T10:00:00Z","checks":[{"name":"health","status":"pass"}]}"#);
        assert_eq!(report, Report { valid: true, errors: vec![], checks: 1 });
    }
    #[test]
    fn describes_missing_fields() {
        let report = validate("{}");
        assert!(!report.valid);
        assert_eq!(report.checks, 0);
        assert_eq!(report.errors.len(), 4);
    }
}
