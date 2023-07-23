use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum IoMessage {
    /// Heating PWM value between 0.0 and 1.0
    SetHeating(f32),
    /// True if camera exposure is active
    SetExposureActive(bool),
    /// Set status led mode
    SetStatus(StatusMode)
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum StatusMode {
    On,
}