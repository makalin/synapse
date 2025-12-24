// use gpui::*; // Commented out for CLI version
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub title: String,
    pub agent_name: String,
    pub timestamp: u64,
    pub status: ReportStatus,
    pub output: String,
    pub metrics: ReportMetrics,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReportStatus {
    Running,
    Success,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetrics {
    pub tokens_used: Option<u64>,
    pub api_calls: u64,
    pub errors: u64,
    pub warnings: u64,
}

pub struct ReportWindow {
    reports: VecDeque<Report>,
    selected_report: Option<usize>,
    filter: ReportFilter,
    max_reports: usize,
}

#[derive(Debug, Clone)]
pub struct ReportFilter {
    pub status: Option<ReportStatus>,
    pub agent_name: Option<String>,
    pub search_query: String,
}

impl ReportWindow {
    pub fn new(_cx: &mut ()) -> Self {
        Self {
            reports: VecDeque::new(),
            selected_report: None,
            filter: ReportFilter {
                status: None,
                agent_name: None,
                search_query: String::new(),
            },
            max_reports: 100,
        }
    }

    pub fn add_report(&mut self, report: Report) {
        self.reports.push_front(report);
        if self.reports.len() > self.max_reports {
            self.reports.pop_back();
        }
    }

    pub fn update_report(&mut self, id: &str, status: ReportStatus, output: String) {
        if let Some(report) = self.reports.iter_mut().find(|r| r.id == id) {
            report.status = status.clone();
            report.output = output;
            if status == ReportStatus::Success || status == ReportStatus::Failed {
                // Calculate duration
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                if now > report.timestamp {
                    report.duration_ms = (now - report.timestamp) * 1000;
                }
            }
        }
    }

    pub fn filtered_reports(&self) -> Vec<&Report> {
        self.reports
            .iter()
            .filter(|report| {
                if let Some(ref status) = self.filter.status {
                    if report.status != *status {
                        return false;
                    }
                }
                if let Some(ref agent) = self.filter.agent_name {
                    if !report.agent_name.contains(agent) {
                        return false;
                    }
                }
                if !self.filter.search_query.is_empty() {
                    let query = self.filter.search_query.to_lowercase();
                    if !report.title.to_lowercase().contains(&query)
                        && !report.output.to_lowercase().contains(&query)
                    {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn get_reports(&self) -> &VecDeque<Report> {
        &self.reports
    }
}

// UI rendering code commented out for CLI version
