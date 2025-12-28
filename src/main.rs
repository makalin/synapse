// Minimal CLI version - UI code commented out for now
mod accessibility;
mod agent;
mod ai_cli;
// mod app;
mod changelog;
mod code_editor;
mod guides;
// mod grid;
// mod lens;
mod logs;
mod report;
// mod roster;
mod scripting;
mod settings;
// mod settings_ui;
// mod status_bar;
// mod terminal;
mod themes;

fn main() {
    println!("SYNAPSE - Runtime-First AI Orchestration Console");
    println!("Version 0.2.0");
    println!();
    println!("CLI mode active. UI components are being developed.");
    println!();
    
    // Initialize core systems
    let settings_manager = settings::SettingsManager::new();
    let settings = settings_manager.get();
    
    println!("Settings loaded from: ~/.synapse/config.json");
    println!("Theme: {}", settings.theme.background);
    println!();
    
    // Initialize agent manager
    let _agent_manager = agent::AgentManager::new();
    println!("Agent Manager initialized");
    
    // Initialize AI CLI manager
    let mut ai_cli_manager = ai_cli::AICLIManager::new();
    let detected = ai_cli_manager.detect_installed_tools();
    println!("AI CLI Tools detected: {}", detected.len());
    for tool_id in &detected {
        println!("  - {}", tool_id);
    }
    println!();
    
    // Initialize theme manager
    let theme_manager = themes::ThemeManager::new();
    let themes = theme_manager.get_all_themes();
    println!("Available themes: {}", themes.len());
    for theme in &themes {
        println!("  - {} ({})", theme.name, theme.id);
    }
    println!();
    
    // Initialize guide manager
    let guide_manager = guides::GuideManager::new();
    println!("How-to guides loaded: {}", guide_manager.get_guides().len());
    println!();
    
    // Initialize changelog
    let changelog_manager = changelog::ChangelogManager::new();
    if let Some(latest) = changelog_manager.get_latest_version() {
        println!("Latest version: {}", latest.version);
    }
    println!();
    
    println!("SYNAPSE CLI is ready!");
    println!("All core systems initialized successfully.");
    println!();
    println!("Note: Full UI with GPUI will be available once API issues are resolved.");
}