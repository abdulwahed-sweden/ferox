use anyhow::Result;
use clap::Parser;
use ferox::cli::app::FeroxCli;
use ferox::cli::theme::Theme;
use ferox::cli::{CommandRouter, RouterCommand, RouterDispatch};
use ferox::core::module::ModuleRegistry;
use ferox::core::theme::MixedPredatorTheme;
use ferox::tools::theme::{CliThemeApplier, ThemeConfig};
use std::io::{stdin, stdout, IsTerminal};

// Scanner modules
use ferox::modules::scanner::http_scanner::HttpScanner;
use ferox::modules::scanner::port::PortScanner;

// Recon modules
use ferox::modules::recon::asn::AsnDiscovery;
use ferox::modules::recon::dns::DnsEnumerator;
use ferox::modules::recon::subdomains::SubdomainEnum;
use ferox::modules::recon::whois::WhoisLookup;

// Exploit modules
use ferox::modules::exploit::{
    BluekeepExploit, DirtyCowExploit, ExampleExploit, Ms08067Exploit, Ms17010Exploit,
    PrintNightmareExploit, PsexecExploit, SqlInjectionExploit, SshBruteforceExploit,
    SudoBaronSameditExploit, WebShellUploadExploit,
};

// Post-exploitation modules
use ferox::modules::post::{
    ClipboardCapture, FileDownloadModule, FileSearchModule, FullSituationalModule, KeylogCapture,
    NetworkEnum, ProcessesEnum, ScreenshotCapture, SystemInfoEnum, UsersEnum,
};

// Credential modules
use ferox::modules::creds::{
    BrowserCredsHarvest, CloudTokensHarvest, LsassDump, SamDump, SshKeysHarvest, WifiCredsHarvest,
};

// Lateral movement modules
use ferox::modules::lateral::{
    LateralPivot, LateralPsexec, LateralSsh, LateralWinrm, LateralWmi,
};

// Persistence modules
use ferox::modules::persist::{
    PersistCron, PersistRegistry, PersistScheduledTask, PersistService, PersistSshKey,
    PersistSystemd,
};

// Exfiltration modules
use ferox::modules::exfil::{
    ExfilCloudStorage, ExfilDns, ExfilEmail, ExfilHttps, ExfilIcmp, ExfilSteganography,
    ExfilWebhook,
};

// Cleanup modules
use ferox::modules::cleanup::{
    CleanupConnections, CleanupFiles, CleanupHistory, CleanupLinuxLogs, CleanupPersistence,
    CleanupWindowsEvents,
};

// Privilege escalation modules
use ferox::modules::privesc::{
    PrivescKernelExploits, PrivescServiceExploit, PrivescSudoExploit, PrivescSuidScan,
    PrivescTokenImpersonation,
};

// OPSEC modules
use ferox::modules::opsec_modules::{
    OpsecAvCheck, OpsecEdrBypass, OpsecEdrCheck, OpsecNetworkProxy,
};

// Mobile modules
use ferox::modules::mobile::{ApkAnalyzer, AppRecon, IpaAnalyzer};

// Phase 3 modules
use ferox::modules::auxiliary::cloud::onedrive_sync_exfil::OneDriveSyncExfil;
use ferox::modules::c2::teams_tunnel::TeamsTunnel;
use ferox::modules::evasion::edr::silent_shadow::SilentShadow;
use ferox::modules::post::browser::deep_session_hijack::DeepSessionHijack;

// Phase 4 modules - Smart Payload System
use ferox::modules::payloads::rev_tcp_fileless::FilelessRevTcp;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Ferox Offensive Security Framework",
    propagate_version = true
)]
struct FeroxArgs {
    #[command(subcommand)]
    command: Option<RouterCommand>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = FeroxArgs::parse();

    let theme = ThemeConfig::load_default()
        .map(|cfg| cfg.into_theme())
        .unwrap_or_else(|_| MixedPredatorTheme::new());
    theme.apply_to_ui();
    let cli_theme = CliThemeApplier::new(theme.clone());
    cli_theme.apply_colors();

    let router = CommandRouter::initialize(cli_theme).await?;

    tracing_subscriber::fmt()
        .with_env_filter("ferox=info")
        .init();

    #[cfg(windows)]
    {
        Theme::enable_ansi_support()?;
    }

    Theme::init();

    match router.dispatch(args.command).await? {
        RouterDispatch::Handled => return Ok(()),
        RouterDispatch::Fallthrough => {}
    }

    // Gracefully handle non-interactive environments (CI/pipes) without dropping into the console.
    if !stdin().is_terminal() || !stdout().is_terminal() {
        println!(
            "Ferox console requires an interactive TTY. \
             Use subcommands like `ferox doctor <cmd>` or `ferox memory <cmd>` in non-interactive modes."
        );
        return Ok(());
    }

    let mut registry = ModuleRegistry::new();

    // === Scanner Modules ===
    registry.register(Box::new(PortScanner::new()));
    registry.register(Box::new(HttpScanner::new()));

    // === Recon Modules ===
    registry.register(Box::new(SubdomainEnum::new()));
    registry.register(Box::new(DnsEnumerator::new()));
    registry.register(Box::new(WhoisLookup::new()));
    registry.register(Box::new(AsnDiscovery::new()));

    // === Exploit Modules (10 modules) ===
    registry.register(Box::new(ExampleExploit::new()));
    registry.register(Box::new(Ms17010Exploit::new()));
    registry.register(Box::new(Ms08067Exploit::new()));
    registry.register(Box::new(PsexecExploit::new()));
    registry.register(Box::new(PrintNightmareExploit::new()));
    registry.register(Box::new(BluekeepExploit::new()));
    registry.register(Box::new(DirtyCowExploit::new()));
    registry.register(Box::new(SudoBaronSameditExploit::new()));
    registry.register(Box::new(WebShellUploadExploit::new()));
    registry.register(Box::new(SqlInjectionExploit::new()));
    registry.register(Box::new(SshBruteforceExploit::new()));

    // === Post-Exploitation Modules ===
    registry.register(Box::new(SystemInfoEnum::new()));
    registry.register(Box::new(UsersEnum::new()));
    registry.register(Box::new(ProcessesEnum::new()));
    registry.register(Box::new(NetworkEnum::new()));
    registry.register(Box::new(FileSearchModule::new()));
    registry.register(Box::new(FileDownloadModule::new()));
    registry.register(Box::new(FullSituationalModule::new()));
    registry.register(Box::new(ScreenshotCapture::new()));
    registry.register(Box::new(KeylogCapture::new()));
    registry.register(Box::new(ClipboardCapture::new()));

    // === Credential Modules ===
    registry.register(Box::new(BrowserCredsHarvest::new()));
    registry.register(Box::new(WifiCredsHarvest::new()));
    registry.register(Box::new(SshKeysHarvest::new()));
    registry.register(Box::new(CloudTokensHarvest::new()));
    registry.register(Box::new(SamDump::new()));
    registry.register(Box::new(LsassDump::new()));

    // === Lateral Movement Modules ===
    registry.register(Box::new(LateralPsexec::new()));
    registry.register(Box::new(LateralWmi::new()));
    registry.register(Box::new(LateralWinrm::new()));
    registry.register(Box::new(LateralSsh::new()));
    registry.register(Box::new(LateralPivot::new()));

    // === Persistence Modules ===
    registry.register(Box::new(PersistRegistry::new()));
    registry.register(Box::new(PersistScheduledTask::new()));
    registry.register(Box::new(PersistService::new()));
    registry.register(Box::new(PersistCron::new()));
    registry.register(Box::new(PersistSystemd::new()));
    registry.register(Box::new(PersistSshKey::new()));

    // === Exfiltration Modules ===
    registry.register(Box::new(ExfilDns::new()));
    registry.register(Box::new(ExfilHttps::new()));
    registry.register(Box::new(ExfilIcmp::new()));
    registry.register(Box::new(ExfilCloudStorage::new()));
    registry.register(Box::new(ExfilWebhook::new()));
    registry.register(Box::new(ExfilEmail::new()));
    registry.register(Box::new(ExfilSteganography::new()));

    // === Cleanup Modules ===
    registry.register(Box::new(CleanupWindowsEvents::new()));
    registry.register(Box::new(CleanupLinuxLogs::new()));
    registry.register(Box::new(CleanupFiles::new()));
    registry.register(Box::new(CleanupHistory::new()));
    registry.register(Box::new(CleanupConnections::new()));
    registry.register(Box::new(CleanupPersistence::new()));

    // === Privilege Escalation Modules ===
    registry.register(Box::new(PrivescTokenImpersonation::new()));
    registry.register(Box::new(PrivescServiceExploit::new()));
    registry.register(Box::new(PrivescSuidScan::new()));
    registry.register(Box::new(PrivescSudoExploit::new()));
    registry.register(Box::new(PrivescKernelExploits::new()));

    // === OPSEC Modules ===
    registry.register(Box::new(OpsecEdrCheck::new()));
    registry.register(Box::new(OpsecEdrBypass::new()));
    registry.register(Box::new(OpsecAvCheck::new()));
    registry.register(Box::new(OpsecNetworkProxy::new()));

    // === Mobile Modules ===
    registry.register(Box::new(ApkAnalyzer::new()));
    registry.register(Box::new(IpaAnalyzer::new()));
    registry.register(Box::new(AppRecon::new()));

    // === Phase 3 Modules ===
    registry.register(Box::new(TeamsTunnel::new()));
    registry.register(Box::new(DeepSessionHijack::new()));
    registry.register(Box::new(OneDriveSyncExfil::new()));
    registry.register(Box::new(SilentShadow::new()));

    // === Phase 4: Smart Payload System ===
    registry.register(Box::new(FilelessRevTcp::new()));

    let mut app = FeroxCli::new(registry)?;
    app.run().await?;

    Ok(())
}
