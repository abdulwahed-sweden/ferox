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
    const LINE_HEIGHT: f32 = 6.0;

    fn add_text(
        layer: &PdfLayerReference,
        text: &str,
        x: f32,
        y: f32,
        font: &IndirectFontRef,
        size: f32,
    ) {
        layer.use_text(text, size, Mm(x), Mm(y), font);
    }
}

#[cfg(feature = "pdf-export")]
impl Reporter for PdfReporter {
    fn export(&self, data: &ReportData, output_path: &Path) -> Result<()> {
        let (doc, page1, layer1) = PdfDocument::new(
            "Ferox Framework Report",
            Mm(Self::PAGE_WIDTH),
            Mm(Self::PAGE_HEIGHT),
            "Layer 1",
        );

        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Add fonts
        let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

        let mut y_pos = Self::PAGE_HEIGHT - Self::MARGIN;

        // Title
        Self::add_text(
            &current_layer,
            "FEROX FRAMEWORK REPORT",
            Self::MARGIN,
            y_pos,
            &font_bold,
            20.0,
        );
        y_pos -= Self::LINE_HEIGHT * 2.0;

        // Metadata
        Self::add_text(
            &current_layer,
            &format!(
                "Generated: {}",
                data.generated_at.format("%Y-%m-%d %H:%M:%S UTC")
            ),
            Self::MARGIN,
            y_pos,
            &font,
            10.0,
        );
        y_pos -= Self::LINE_HEIGHT;

        Self::add_text(
            &current_layer,
            &format!("Ferox Version: {}", data.ferox_version),
            Self::MARGIN,
            y_pos,
            &font,
            10.0,
        );
        y_pos -= Self::LINE_HEIGHT * 2.0;

        // Summary
        Self::add_text(
            &current_layer,
            "SUMMARY",
            Self::MARGIN,
            y_pos,
            &font_bold,
            14.0,
        );
        y_pos -= Self::LINE_HEIGHT * 1.5;

        Self::add_text(
            &current_layer,
            &format!("Total Results: {}", data.summary.total_results),
            Self::MARGIN,
            y_pos,
            &font,
            10.0,
        );
        y_pos -= Self::LINE_HEIGHT;

        Self::add_text(
            &current_layer,
            &format!("Successful: {}", data.summary.successful_results),
            Self::MARGIN,
            y_pos,
            &font,
            10.0,
        );
        y_pos -= Self::LINE_HEIGHT;

        Self::add_text(
            &current_layer,
            &format!("Failed: {}", data.summary.failed_results),
            Self::MARGIN,
            y_pos,
            &font,
            10.0,
        );
        y_pos -= Self::LINE_HEIGHT;

        Self::add_text(
            &current_layer,
            &format!(
                "Sessions: {} active / {} total",
                data.summary.active_sessions, data.summary.total_sessions
            ),
            Self::MARGIN,
            y_pos,
            &font,
            10.0,
        );
        y_pos -= Self::LINE_HEIGHT * 2.0;

        // Modules used
        if !data.summary.modules_used.is_empty() {
            Self::add_text(
                &current_layer,
                "MODULES USED",
                Self::MARGIN,
                y_pos,
                &font_bold,
                14.0,
            );
            y_pos -= Self::LINE_HEIGHT * 1.5;

            for module in &data.summary.modules_used {
                Self::add_text(
                    &current_layer,
                    &format!("• {}", module),
                    Self::MARGIN,
                    y_pos,
                    &font,
                    10.0,
                );
                y_pos -= Self::LINE_HEIGHT;
            }
            y_pos -= Self::LINE_HEIGHT;
        }

        // Results summary (limited to first few to fit on page)
        if !data.results.is_empty() {
            Self::add_text(
                &current_layer,
                "RESULTS",
                Self::MARGIN,
                y_pos,
                &font_bold,
                14.0,
            );
            y_pos -= Self::LINE_HEIGHT * 1.5;

            let max_results = 5.min(data.results.len());
            for result in data.results.iter().take(max_results) {
                let status = if result.result.success {
                    "SUCCESS"
                } else {
                    "FAILED"
                };
                let module = format!(
                    "{}/{}",
                    result.module_info.category, result.module_info.name
                );

                Self::add_text(
                    &current_layer,
                    &format!("[{}] {}", status, module),
                    Self::MARGIN,
                    y_pos,
                    &font_bold,
                    10.0,
                );
                y_pos -= Self::LINE_HEIGHT;

                Self::add_text(
                    &current_layer,
                    &format!("  {}", result.result.message),
                    Self::MARGIN,
                    y_pos,
                    &font,
                    9.0,
                );
                y_pos -= Self::LINE_HEIGHT * 1.5;

                if y_pos < Self::MARGIN + 20.0 {
                    break; // Stop if we're running out of space
                }
            }

            if data.results.len() > max_results {
                Self::add_text(
                    &current_layer,
                    &format!("... and {} more results", data.results.len() - max_results),
                    Self::MARGIN,
                    y_pos,
                    &font,
                    9.0,
                );
            }
        }

        // Footer
        Self::add_text(
            &current_layer,
            "Generated by Ferox Framework - Fast. Fierce. Fearless.",
            Self::MARGIN,
            Self::MARGIN,
            &font,
            8.0,
        );

        // Save document
        let file = File::create(output_path)
            .with_context(|| format!("Failed to create PDF file: {}", output_path.display()))?;

        let mut writer = BufWriter::new(file);
        doc.save(&mut writer)
            .with_context(|| "Failed to save PDF document")?;

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
