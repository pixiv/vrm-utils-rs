use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpressionPresetName {
    Aa,
    Angry,
    Blink,
    BlinkLeft,
    BlinkRight,
    Ee,
    Happy,
    Ih,
    LookDown,
    LookLeft,
    LookRight,
    LookUp,
    Neutral,
    Oh,
    Ou,
    Relaxed,
    Sad,
    Surprised,
}
