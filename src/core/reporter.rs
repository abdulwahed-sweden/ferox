use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
#[cfg(feature = "pdf-export")]
use printpdf::*;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tera::{Context as TeraContext, Tera};

use crate::core::module::Session;
use crate::core::result_store::StoredResult;

/// Report data structure containing all information for export
#[derive(Debug, Clone, Serialize)]
pub struct ReportData {
    pub results: Vec<StoredResult>,
    pub sessions: Vec<Session>,
    pub generated_at: DateTime<Utc>,
    pub ferox_version: String,
    pub summary: ReportSummary,
}

/// Summary statistics for the report
#[derive(Debug, Clone, Serialize)]
pub struct ReportSummary {
    pub total_results: usize,
    pub successful_results: usize,
    pub failed_results: usize,
    pub total_sessions: usize,
    pub active_sessions: usize,
    pub modules_used: Vec<String>,
    pub time_range: Option<TimeRange>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl ReportData {
    /// Create a new report from results and sessions
    pub fn new(results: Vec<StoredResult>, sessions: Vec<Session>) -> Self {
        let summary = Self::generate_summary(&results, &sessions);

        Self {
            results,
            sessions,
            generated_at: Utc::now(),
            ferox_version: env!("CARGO_PKG_VERSION").to_string(),
            summary,
        }
    }

    /// Generate summary statistics
    fn generate_summary(results: &[StoredResult], sessions: &[Session]) -> ReportSummary {
        let total_results = results.len();
        let successful_results = results.iter().filter(|r| r.result.success).count();
        let failed_results = total_results - successful_results;
        let total_sessions = sessions.len();
        let active_sessions = sessions.iter().filter(|s| s.active).count();

        let mut modules_used: Vec<String> = results
            .iter()
            .map(|r| format!("{}/{}", r.module_info.category, r.module_info.name))
            .collect();
        modules_used.sort();
        modules_used.dedup();

        let time_range = if !results.is_empty() {
            let timestamps: Vec<_> = results.iter().map(|r| r.result.timestamp).collect();
            Some(TimeRange {
                start: *timestamps.iter().min().unwrap(),
                end: *timestamps.iter().max().unwrap(),
            })
        } else {
            None
        };

        ReportSummary {
            total_results,
            successful_results,
            failed_results,
            total_sessions,
            active_sessions,
            modules_used,
            time_range,
        }
    }
}

/// Reporter trait for different export formats
pub trait Reporter {
    fn export(&self, data: &ReportData, output_path: &Path) -> Result<()>;
}

/// JSON Reporter - exports data as JSON
pub struct JsonReporter;

impl Reporter for JsonReporter {
    fn export(&self, data: &ReportData, output_path: &Path) -> Result<()> {
        let file = File::create(output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, data)
            .with_context(|| "Failed to serialize data to JSON")?;

        Ok(())
    }
}

/// HTML Reporter - exports data as professional HTML report
pub struct HtmlReporter;

impl HtmlReporter {
    /// Get the HTML template
    fn get_template() -> &'static str {
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ferox Framework Report - {{ generated_at }}</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
            padding: 20px;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            background: white;
            padding: 40px;
            box-shadow: 0 0 20px rgba(0,0,0,0.1);
            border-radius: 8px;
        }

        .header {
            text-align: center;
            margin-bottom: 40px;
            padding-bottom: 20px;
            border-bottom: 3px solid #e74c3c;
        }

        .header h1 {
            color: #e74c3c;
            font-size: 2.5em;
            margin-bottom: 10px;
        }

        .header .subtitle {
            color: #7f8c8d;
            font-size: 1.1em;
        }

        .summary {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }

        .summary-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
        }

        .summary-card.success {
            background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
        }

        .summary-card.failed {
            background: linear-gradient(135deg, #eb3349 0%, #f45c43 100%);
        }

        .summary-card.sessions {
            background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
        }

        .summary-card h3 {
            font-size: 2em;
            margin-bottom: 5px;
        }

        .summary-card p {
            font-size: 0.9em;
            opacity: 0.9;
        }

        .section {
            margin-bottom: 40px;
        }

        .section-title {
            font-size: 1.8em;
            color: #2c3e50;
            margin-bottom: 20px;
            padding-bottom: 10px;
            border-bottom: 2px solid #ecf0f1;
        }

        .result-card {
            background: #f8f9fa;
            border-left: 4px solid #3498db;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 4px;
        }

        .result-card.success {
            border-left-color: #27ae60;
        }

        .result-card.failed {
            border-left-color: #e74c3c;
        }

        .result-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }

        .result-title {
            font-size: 1.3em;
            font-weight: bold;
            color: #2c3e50;
        }

        .result-status {
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 0.85em;
            font-weight: bold;
        }

        .result-status.success {
            background: #d4edda;
            color: #155724;
        }

        .result-status.failed {
            background: #f8d7da;
            color: #721c24;
        }

        .result-meta {
            color: #7f8c8d;
            font-size: 0.9em;
            margin-bottom: 10px;
        }

        .result-message {
            background: white;
            padding: 15px;
            border-radius: 4px;
            margin-bottom: 10px;
        }

        .result-data {
            background: #2c3e50;
            color: #ecf0f1;
            padding: 15px;
            border-radius: 4px;
            overflow-x: auto;
            font-family: 'Courier New', monospace;
            font-size: 0.85em;
        }

        .session-card {
            background: #e8f4f8;
            border-left: 4px solid #3498db;
            padding: 15px;
            margin-bottom: 15px;
            border-radius: 4px;
        }

        .session-card.inactive {
            opacity: 0.6;
            border-left-color: #95a5a6;
        }

        .badge {
            display: inline-block;
            padding: 3px 10px;
            border-radius: 12px;
            font-size: 0.8em;
            margin-left: 10px;
        }

        .badge.active {
            background: #d4edda;
            color: #155724;
        }

        .badge.inactive {
            background: #f8d7da;
            color: #721c24;
        }

        .footer {
            text-align: center;
            margin-top: 40px;
            padding-top: 20px;
            border-top: 2px solid #ecf0f1;
            color: #7f8c8d;
        }

        .modules-list {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
            margin-bottom: 20px;
        }

        .module-tag {
            background: #3498db;
            color: white;
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 0.9em;
        }

        @media print {
            body {
                background: white;
                padding: 0;
            }

            .container {
                box-shadow: none;
                padding: 20px;
            }

            .result-card {
                page-break-inside: avoid;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🦊 Ferox Framework Report</h1>
            <p class="subtitle">Security Assessment Results</p>
            <p class="subtitle">Generated: {{ generated_at | date(format="%Y-%m-%d %H:%M:%S UTC") }}</p>
            <p class="subtitle">Version: {{ ferox_version }}</p>
        </div>

        <div class="summary">
            <div class="summary-card">
                <h3>{{ summary.total_results }}</h3>
                <p>Total Results</p>
            </div>
            <div class="summary-card success">
                <h3>{{ summary.successful_results }}</h3>
                <p>Successful</p>
            </div>
            <div class="summary-card failed">
                <h3>{{ summary.failed_results }}</h3>
                <p>Failed</p>
            </div>
            <div class="summary-card sessions">
                <h3>{{ summary.active_sessions }}/{{ summary.total_sessions }}</h3>
                <p>Active Sessions</p>
            </div>
        </div>

        {% if summary.modules_used %}
        <div class="section">
            <h2 class="section-title">Modules Used</h2>
            <div class="modules-list">
                {% for module in summary.modules_used %}
                <span class="module-tag">{{ module }}</span>
                {% endfor %}
            </div>
        </div>
        {% endif %}

        {% if results %}
        <div class="section">
            <h2 class="section-title">Execution Results ({{ results | length }})</h2>
            {% for result in results %}
            <div class="result-card {% if result.result.success %}success{% else %}failed{% endif %}">
                <div class="result-header">
                    <span class="result-title">{{ result.module_info.category }}/{{ result.module_info.name }}</span>
                    <span class="result-status {% if result.result.success %}success{% else %}failed{% endif %}">
                        {% if result.result.success %}✓ SUCCESS{% else %}✗ FAILED{% endif %}
                    </span>
                </div>
                <div class="result-meta">
                    <strong>Timestamp:</strong> {{ result.result.timestamp | date(format="%Y-%m-%d %H:%M:%S UTC") }} |
                    <strong>Module:</strong> {{ result.module_info.name }} v{{ result.module_info.version }} |
                    <strong>Author:</strong> {{ result.module_info.author }}
                </div>
                <div class="result-meta">
                    <strong>Description:</strong> {{ result.module_info.description }}
                </div>
                <div class="result-message">
                    <strong>Message:</strong> {{ result.result.message }}
                </div>
                {% if result.result.data %}
                <div class="result-data">
                    <pre>{{ result.result.data | json_encode(pretty=true) }}</pre>
                </div>
                {% endif %}
            </div>
            {% endfor %}
        </div>
        {% endif %}

        {% if sessions %}
        <div class="section">
            <h2 class="section-title">Sessions ({{ sessions | length }})</h2>
            {% for session in sessions %}
            <div class="session-card {% if not session.active %}inactive{% endif %}">
                <strong>Session ID:</strong> {{ session.id }}
                <span class="badge {% if session.active %}active{% else %}inactive{% endif %}">
                    {% if session.active %}ACTIVE{% else %}INACTIVE{% endif %}
                </span>
                <br>
                <strong>Module:</strong> {{ session.module }} |
                <strong>Target:</strong> {{ session.target }} |
                <strong>Platform:</strong> {{ session.platform }} |
                <strong>Established:</strong> {{ session.established_at | date(format="%Y-%m-%d %H:%M:%S") }}
                {% if session.user %}
                <br><strong>User:</strong> {{ session.user }}
                {% endif %}
            </div>
            {% endfor %}
        </div>
        {% endif %}

        <div class="footer">
            <p><strong>🦊 Ferox Framework v{{ ferox_version }}</strong></p>
            <p>Fast. Fierce. Fearless.</p>
            <p>This report was automatically generated by Ferox Framework</p>
        </div>
    </div>
</body>
</html>"#
    }
}

impl Reporter for HtmlReporter {
    fn export(&self, data: &ReportData, output_path: &Path) -> Result<()> {
        let mut tera = Tera::default();
        tera.add_raw_template("report", Self::get_template())
            .with_context(|| "Failed to add HTML template")?;

        let mut context = TeraContext::new();
        context.insert("results", &data.results);
        context.insert("sessions", &data.sessions);
        context.insert("generated_at", &data.generated_at.to_rfc3339());
        context.insert("ferox_version", &data.ferox_version);
        context.insert("summary", &data.summary);

        let html = tera
            .render("report", &context)
            .with_context(|| "Failed to render HTML template")?;

        std::fs::write(output_path, html)
            .with_context(|| format!("Failed to write HTML to {}", output_path.display()))?;

        Ok(())
    }
}

/// PDF Reporter - exports data as PDF document
#[cfg(feature = "pdf-export")]
pub struct PdfReporter;

#[cfg(feature = "pdf-export")]
impl PdfReporter {
    const PAGE_WIDTH: f32 = 210.0; // A4 width in mm
    const PAGE_HEIGHT: f32 = 297.0; // A4 height in mm
    const MARGIN: f32 = 20.0;
    const CONTENT_WIDTH: f32 = 170.0; // PAGE_WIDTH - 2*MARGIN

    /// Helper to write text with builtin font
    fn write_text(text: &str, font: BuiltinFont) -> Op {
        Op::WriteTextBuiltinFont {
            items: vec![TextItem::Text(text.to_string())],
            font,
        }
    }

    /// Helper to set font size for builtin font
    fn set_font(font: BuiltinFont, size: f32) -> Op {
        Op::SetFontSizeBuiltinFont {
            font,
            size: Pt(size),
        }
    }

    /// Helper to set RGB fill color
    fn set_color(r: f32, g: f32, b: f32) -> Op {
        Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r,
                g,
                b,
                icc_profile: None,
            }),
        }
    }

    /// Build PDF operations for the report content
    fn build_operations(data: &ReportData) -> Vec<Op> {
        let mut ops = Vec::new();

        // Initialize text section
        ops.push(Op::SaveGraphicsState);
        ops.push(Op::StartTextSection);

        // Position cursor at top of page (from bottom-left origin)
        ops.push(Op::SetTextCursor {
            pos: Point::new(Mm(Self::MARGIN), Mm(Self::PAGE_HEIGHT - Self::MARGIN)),
        });

        // ========== TITLE ==========
        ops.push(Self::set_font(BuiltinFont::HelveticaBold, 24.0));
        ops.push(Op::SetLineHeight { lh: Pt(28.0) });
        ops.push(Self::set_color(0.9, 0.3, 0.2)); // Ferox red
        ops.push(Self::write_text("FEROX FRAMEWORK REPORT", BuiltinFont::HelveticaBold));
        ops.push(Op::AddLineBreak);

        // Subtitle
        ops.push(Self::set_font(BuiltinFont::Helvetica, 12.0));
        ops.push(Op::SetLineHeight { lh: Pt(16.0) });
        ops.push(Self::set_color(0.4, 0.4, 0.4));
        ops.push(Self::write_text("Security Assessment Results", BuiltinFont::Helvetica));
        ops.push(Op::AddLineBreak);
        ops.push(Op::AddLineBreak);

        // ========== METADATA ==========
        ops.push(Self::set_font(BuiltinFont::Helvetica, 10.0));
        ops.push(Op::SetLineHeight { lh: Pt(14.0) });
        ops.push(Self::set_color(0.3, 0.3, 0.3));
        ops.push(Self::write_text(
            &format!(
                "Generated: {}",
                data.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
            ),
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);
        ops.push(Self::write_text(
            &format!("Ferox Version: {}", data.ferox_version),
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);
        ops.push(Op::AddLineBreak);

        // ========== EXECUTIVE SUMMARY ==========
        ops.push(Self::set_font(BuiltinFont::HelveticaBold, 14.0));
        ops.push(Op::SetLineHeight { lh: Pt(18.0) });
        ops.push(Self::set_color(0.2, 0.2, 0.2));
        ops.push(Self::write_text("EXECUTIVE SUMMARY", BuiltinFont::HelveticaBold));
        ops.push(Op::AddLineBreak);

        // Draw a separator line effect with dashes
        ops.push(Self::set_font(BuiltinFont::Helvetica, 8.0));
        ops.push(Op::SetLineHeight { lh: Pt(10.0) });
        ops.push(Self::set_color(0.8, 0.8, 0.8));
        ops.push(Self::write_text(
            "--------------------------------------------------------------------------------",
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);

        // Summary stats
        ops.push(Self::set_font(BuiltinFont::Helvetica, 11.0));
        ops.push(Op::SetLineHeight { lh: Pt(16.0) });
        ops.push(Self::set_color(0.2, 0.2, 0.2));

        // Total Results
        ops.push(Self::write_text(
            &format!("Total Results:     {}", data.summary.total_results),
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);

        // Successful (green)
        ops.push(Self::set_color(0.1, 0.6, 0.3));
        ops.push(Self::write_text(
            &format!("Successful:        {}", data.summary.successful_results),
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);

        // Failed (red)
        ops.push(Self::set_color(0.8, 0.2, 0.2));
        ops.push(Self::write_text(
            &format!("Failed:            {}", data.summary.failed_results),
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);

        // Sessions (blue)
        ops.push(Self::set_color(0.2, 0.4, 0.8));
        ops.push(Self::write_text(
            &format!(
                "Active Sessions:   {} / {}",
                data.summary.active_sessions, data.summary.total_sessions
            ),
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);
        ops.push(Op::AddLineBreak);

        // ========== MODULES USED ==========
        if !data.summary.modules_used.is_empty() {
            ops.push(Self::set_font(BuiltinFont::HelveticaBold, 14.0));
            ops.push(Op::SetLineHeight { lh: Pt(18.0) });
            ops.push(Self::set_color(0.2, 0.2, 0.2));
            ops.push(Self::write_text("MODULES USED", BuiltinFont::HelveticaBold));
            ops.push(Op::AddLineBreak);

            ops.push(Self::set_font(BuiltinFont::Helvetica, 8.0));
            ops.push(Op::SetLineHeight { lh: Pt(10.0) });
            ops.push(Self::set_color(0.8, 0.8, 0.8));
            ops.push(Self::write_text(
                "--------------------------------------------------------------------------------",
                BuiltinFont::Helvetica,
            ));
            ops.push(Op::AddLineBreak);

            ops.push(Self::set_font(BuiltinFont::Helvetica, 10.0));
            ops.push(Op::SetLineHeight { lh: Pt(14.0) });
            ops.push(Self::set_color(0.3, 0.5, 0.7));

            for module in &data.summary.modules_used {
                ops.push(Self::write_text(&format!("  * {}", module), BuiltinFont::Helvetica));
                ops.push(Op::AddLineBreak);
            }
            ops.push(Op::AddLineBreak);
        }

        // ========== TIME RANGE ==========
        if let Some(ref time_range) = data.summary.time_range {
            ops.push(Self::set_font(BuiltinFont::HelveticaBold, 14.0));
            ops.push(Op::SetLineHeight { lh: Pt(18.0) });
            ops.push(Self::set_color(0.2, 0.2, 0.2));
            ops.push(Self::write_text("TIME RANGE", BuiltinFont::HelveticaBold));
            ops.push(Op::AddLineBreak);

            ops.push(Self::set_font(BuiltinFont::Helvetica, 8.0));
            ops.push(Op::SetLineHeight { lh: Pt(10.0) });
            ops.push(Self::set_color(0.8, 0.8, 0.8));
            ops.push(Self::write_text(
                "--------------------------------------------------------------------------------",
                BuiltinFont::Helvetica,
            ));
            ops.push(Op::AddLineBreak);

            ops.push(Self::set_font(BuiltinFont::Helvetica, 10.0));
            ops.push(Op::SetLineHeight { lh: Pt(14.0) });
            ops.push(Self::set_color(0.3, 0.3, 0.3));
            ops.push(Self::write_text(
                &format!(
                    "Start: {}",
                    time_range.start.format("%Y-%m-%d %H:%M:%S UTC")
                ),
                BuiltinFont::Helvetica,
            ));
            ops.push(Op::AddLineBreak);
            ops.push(Self::write_text(
                &format!("End:   {}", time_range.end.format("%Y-%m-%d %H:%M:%S UTC")),
                BuiltinFont::Helvetica,
            ));
            ops.push(Op::AddLineBreak);
            ops.push(Op::AddLineBreak);
        }

        // ========== DETAILED RESULTS ==========
        if !data.results.is_empty() {
            ops.push(Self::set_font(BuiltinFont::HelveticaBold, 14.0));
            ops.push(Op::SetLineHeight { lh: Pt(18.0) });
            ops.push(Self::set_color(0.2, 0.2, 0.2));
            ops.push(Self::write_text(
                &format!("EXECUTION RESULTS ({})", data.results.len()),
                BuiltinFont::HelveticaBold,
            ));
            ops.push(Op::AddLineBreak);

            ops.push(Self::set_font(BuiltinFont::Helvetica, 8.0));
            ops.push(Op::SetLineHeight { lh: Pt(10.0) });
            ops.push(Self::set_color(0.8, 0.8, 0.8));
            ops.push(Self::write_text(
                "--------------------------------------------------------------------------------",
                BuiltinFont::Helvetica,
            ));
            ops.push(Op::AddLineBreak);

            // Show up to 10 results with details
            let max_results = 10.min(data.results.len());
            for (idx, result) in data.results.iter().take(max_results).enumerate() {
                let status_text = if result.result.success {
                    "SUCCESS"
                } else {
                    "FAILED"
                };
                let module_path =
                    format!("{}/{}", result.module_info.category, result.module_info.name);

                // Result header with status
                ops.push(Self::set_font(BuiltinFont::HelveticaBold, 11.0));
                ops.push(Op::SetLineHeight { lh: Pt(15.0) });

                if result.result.success {
                    ops.push(Self::set_color(0.1, 0.6, 0.3)); // Green
                } else {
                    ops.push(Self::set_color(0.8, 0.2, 0.2)); // Red
                }

                ops.push(Self::write_text(
                    &format!("{}. [{}] {}", idx + 1, status_text, module_path),
                    BuiltinFont::HelveticaBold,
                ));
                ops.push(Op::AddLineBreak);

                // Module info
                ops.push(Self::set_font(BuiltinFont::Helvetica, 9.0));
                ops.push(Op::SetLineHeight { lh: Pt(12.0) });
                ops.push(Self::set_color(0.4, 0.4, 0.4));
                ops.push(Self::write_text(
                    &format!(
                        "   Module: {} v{} by {}",
                        result.module_info.name, result.module_info.version, result.module_info.author
                    ),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);

                // Timestamp
                ops.push(Self::write_text(
                    &format!(
                        "   Time: {}",
                        result.result.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                    ),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);

                // Message
                ops.push(Self::set_color(0.2, 0.2, 0.2));
                ops.push(Self::write_text(
                    &format!("   Message: {}", result.result.message),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);

                // Description (truncated)
                let desc = if result.module_info.description.len() > 80 {
                    format!("{}...", &result.module_info.description[..77])
                } else {
                    result.module_info.description.clone()
                };
                ops.push(Self::set_color(0.5, 0.5, 0.5));
                ops.push(Self::write_text(
                    &format!("   Desc: {}", desc),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);
                ops.push(Op::AddLineBreak);
            }

            // Show remaining count
            if data.results.len() > max_results {
                ops.push(Self::set_font(BuiltinFont::Helvetica, 10.0));
                ops.push(Op::SetLineHeight { lh: Pt(14.0) });
                ops.push(Self::set_color(0.5, 0.5, 0.5));
                ops.push(Self::write_text(
                    &format!(
                        "... and {} more results (see JSON/HTML export for full details)",
                        data.results.len() - max_results
                    ),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);
            }
        }

        // ========== SESSIONS ==========
        if !data.sessions.is_empty() {
            ops.push(Op::AddLineBreak);
            ops.push(Self::set_font(BuiltinFont::HelveticaBold, 14.0));
            ops.push(Op::SetLineHeight { lh: Pt(18.0) });
            ops.push(Self::set_color(0.2, 0.2, 0.2));
            ops.push(Self::write_text(
                &format!("SESSIONS ({})", data.sessions.len()),
                BuiltinFont::HelveticaBold,
            ));
            ops.push(Op::AddLineBreak);

            ops.push(Self::set_font(BuiltinFont::Helvetica, 8.0));
            ops.push(Op::SetLineHeight { lh: Pt(10.0) });
            ops.push(Self::set_color(0.8, 0.8, 0.8));
            ops.push(Self::write_text(
                "--------------------------------------------------------------------------------",
                BuiltinFont::Helvetica,
            ));
            ops.push(Op::AddLineBreak);

            let max_sessions = 5.min(data.sessions.len());
            for session in data.sessions.iter().take(max_sessions) {
                ops.push(Self::set_font(BuiltinFont::Helvetica, 10.0));
                ops.push(Op::SetLineHeight { lh: Pt(14.0) });

                if session.active {
                    ops.push(Self::set_color(0.1, 0.6, 0.3)); // Green for active
                } else {
                    ops.push(Self::set_color(0.5, 0.5, 0.5)); // Gray for inactive
                }

                let status = if session.active { "ACTIVE" } else { "INACTIVE" };
                ops.push(Self::write_text(
                    &format!(
                        "[{}] {} - {} @ {}",
                        status, session.id, session.module, session.target
                    ),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);

                ops.push(Self::set_color(0.4, 0.4, 0.4));
                ops.push(Self::set_font(BuiltinFont::Helvetica, 9.0));
                ops.push(Op::SetLineHeight { lh: Pt(12.0) });

                let user_info = session
                    .user
                    .as_ref()
                    .map(|u| format!(" | User: {}", u))
                    .unwrap_or_default();
                ops.push(Self::write_text(
                    &format!(
                        "   Platform: {:?} | Established: {}{}",
                        session.platform,
                        session.established_at.format("%Y-%m-%d %H:%M:%S"),
                        user_info
                    ),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);
            }

            if data.sessions.len() > max_sessions {
                ops.push(Self::set_color(0.5, 0.5, 0.5));
                ops.push(Self::write_text(
                    &format!("... and {} more sessions", data.sessions.len() - max_sessions),
                    BuiltinFont::Helvetica,
                ));
                ops.push(Op::AddLineBreak);
            }
        }

        // End text section for main content
        ops.push(Op::EndTextSection);
        ops.push(Op::RestoreGraphicsState);

        // ========== FOOTER (separate text section at fixed position) ==========
        ops.push(Op::SaveGraphicsState);
        ops.push(Op::StartTextSection);
        ops.push(Op::SetTextCursor {
            pos: Point::new(Mm(Self::MARGIN), Mm(15.0)),
        });

        // Footer separator
        ops.push(Self::set_font(BuiltinFont::Helvetica, 8.0));
        ops.push(Op::SetLineHeight { lh: Pt(10.0) });
        ops.push(Self::set_color(0.7, 0.7, 0.7));
        ops.push(Self::write_text(
            "________________________________________________________________________________",
            BuiltinFont::Helvetica,
        ));
        ops.push(Op::AddLineBreak);

        ops.push(Self::set_font(BuiltinFont::HelveticaBold, 9.0));
        ops.push(Op::SetLineHeight { lh: Pt(12.0) });
        ops.push(Self::set_color(0.9, 0.3, 0.2));
        ops.push(Self::write_text("Ferox Framework", BuiltinFont::HelveticaBold));

        ops.push(Self::set_font(BuiltinFont::Helvetica, 9.0));
        ops.push(Self::set_color(0.5, 0.5, 0.5));
        ops.push(Self::write_text(
            &format!(" v{} - Fast. Fierce. Fearless.", data.ferox_version),
            BuiltinFont::Helvetica,
        ));

        ops.push(Op::EndTextSection);
        ops.push(Op::RestoreGraphicsState);

        ops
    }
}

#[cfg(feature = "pdf-export")]
impl Reporter for PdfReporter {
    fn export(&self, data: &ReportData, output_path: &Path) -> Result<()> {
        // Create a new PDF document with the printpdf 0.8.x API
        let mut doc = PdfDocument::new("Ferox Framework Report");

        // Build operations for the page content
        let ops = Self::build_operations(data);

        // Create a page with A4 dimensions and our operations
        let page = PdfPage::new(Mm(Self::PAGE_WIDTH), Mm(Self::PAGE_HEIGHT), ops);

        // Save the document
        let bytes = doc
            .with_pages(vec![page])
            .save(&PdfSaveOptions::default(), &mut Vec::new());

        std::fs::write(output_path, bytes)
            .with_context(|| format!("Failed to write PDF to {}", output_path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::module::{ModuleInfo, ModuleResult, ModuleType};
    use uuid::Uuid;

    fn create_test_report_data() -> ReportData {
        let module_info = ModuleInfo {
            name: "test".to_string(),
            version: "1.0".to_string(),
            author: "Test".to_string(),
            description: "Test module".to_string(),
            module_type: ModuleType::Scanner,
            category: "scanner".to_string(),
        };

        let result = ModuleResult::success("Test successful");
        let stored = StoredResult {
            id: Uuid::new_v4(),
            module_info,
            result,
        };

        ReportData::new(vec![stored], vec![])
    }

    #[test]
    fn test_report_data_creation() {
        let data = create_test_report_data();
        assert_eq!(data.summary.total_results, 1);
        assert_eq!(data.summary.successful_results, 1);
        assert_eq!(data.summary.failed_results, 0);
    }
}
