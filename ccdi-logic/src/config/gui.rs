use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GuiConfig {

}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {

        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ButtonSet<T> {
    buttons: Vec<Vec<Button<T>>>
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Button<T> {
    pub text: String,
    pub value: T,
}