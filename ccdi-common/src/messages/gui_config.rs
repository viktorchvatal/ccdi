use serde_derive::{Serialize, Deserialize};
use serde::{Serializer};

// ============================================ PUBLIC =============================================

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GuiConfig {
    pub temperature: ButtonSet<f64>,
    pub exposure: ButtonSet<f64>,
    pub gain: ButtonSet<u16>,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            temperature: default_temperature_buttons(),
            exposure: ButtonSet { buttons: vec![] },
            gain: ButtonSet { buttons: vec![] },
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ButtonSet<T> {
    pub buttons: Vec<Vec<Button<T>>>
}

#[derive(Clone, PartialEq, Debug)]
pub struct Button<T> {
    pub text: String,
    pub value: T,
}

impl<T: serde::Serialize> serde::Serialize for Button<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let tuple = (&self.text, &self.value);
        tuple.serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Button<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let tuple: (String, T) = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self { text: tuple.0, value: tuple.1 })
    }
}

// =========================================== PRIVATE =============================================

fn bt<T>(name: &str, value: T) -> Button<T> {
    Button {
        text: String::from(name),
        value
    }
}

fn default_temperature_buttons() -> ButtonSet<f64> {
    ButtonSet {
        buttons: vec![
            vec![
                bt("-20", -20.0), bt("-15", -15.0),
                bt("-10", -10.0), bt("-5", -5.0),
            ],
            vec![
                bt("0", 0.0), bt("5", 5.0),
                bt("10", 10.0), bt("15", 15.0),
            ],
            vec![
                bt("20", 20.0), bt("25", 25.0),
            ]
        ]
    }
}
