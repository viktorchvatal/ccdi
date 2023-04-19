use argh::FromArgs;

// ============================================ PUBLIC =============================================

#[derive(FromArgs)]
/// CCD imaging service
pub struct ServiceConfig {
    /// run with a demo driver
    #[argh(switch)]
    pub demo: bool,

    /// enagle debug logging
    #[argh(switch)]
    pub debug: bool,
}