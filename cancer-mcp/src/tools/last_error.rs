//! Last error tool - get the most recent error from logs

use crate::error::{McpError, Result};
use serde::Serialize;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct LastErrorInfo {
    pub found: bool,
    pub error: Option<ErrorDetails>,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub timestamp: Option<String>,
    pub level: String,
    pub message: String,
    pub stacktrace: Option<String>,
}

pub fn execute(project_root: &Path) -> Result<LastErrorInfo> {
    // Common log file locations
    let log_paths = [
        project_root.join("storage/logs/app.log"),
        project_root.join("logs/app.log"),
        project_root.join("app.log"),
        project_root.join("storage/logs/laravel.log"),
    ];

    // Find the first existing log file
    let log_file = log_paths.iter().find(|p| p.exists());

    let Some(log_file) = log_file else {
        return Ok(LastErrorInfo {
            found: false,
            error: None,
        });
    };

    let file = fs::File::open(log_file).map_err(|e| McpError::IoError(e))?;
    let reader = BufReader::new(file);

    // Read all lines and find the last error
    let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    // Search from end to find last error
    let mut last_error_idx: Option<usize> = None;
    for (idx, line) in all_lines.iter().enumerate().rev() {
        let line_upper = line.to_uppercase();
        if line_upper.contains("ERROR") || line_upper.contains("PANIC") || line_upper.contains("FATAL") {
            last_error_idx = Some(idx);
            break;
        }
    }

    let Some(error_idx) = last_error_idx else {
        return Ok(LastErrorInfo {
            found: false,
            error: None,
        });
    };

    let error_line = &all_lines[error_idx];

    // Try to collect stacktrace (lines after the error that look like stack frames)
    let mut stacktrace_lines = Vec::new();
    for line in all_lines.iter().skip(error_idx + 1) {
        // Stop if we hit another log entry or empty line
        if line.starts_with('[') || line.is_empty() {
            break;
        }
        // Stack trace lines often start with whitespace or "at"
        if line.starts_with(' ') || line.starts_with('\t') || line.trim().starts_with("at ") {
            stacktrace_lines.push(line.clone());
        }
    }

    let stacktrace = if stacktrace_lines.is_empty() {
        None
    } else {
        Some(stacktrace_lines.join("\n"))
    };

    // Parse the error line
    let (timestamp, message) = parse_error_line(error_line);

    Ok(LastErrorInfo {
        found: true,
        error: Some(ErrorDetails {
            timestamp,
            level: "ERROR".to_string(),
            message,
            stacktrace,
        }),
    })
}

fn parse_error_line(line: &str) -> (Option<String>, String) {
    let line = line.trim();

    // Try to extract timestamp from [timestamp] format
    if line.starts_with('[') {
        if let Some(bracket_end) = line.find(']') {
            let timestamp = line[1..bracket_end].to_string();
            let message = line[bracket_end + 1..].trim().to_string();
            return (Some(timestamp), message);
        }
    }

    (None, line.to_string())
}
