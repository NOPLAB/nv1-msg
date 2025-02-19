use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Movement {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct ToHub {
    pub vel: Movement,
    pub kick: bool,
    pub goal_opp: Option<f32>,
    pub goal_own: Option<f32>,
    pub config: JetsonConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct ToJetson {
    pub sys: System,
    pub vel: Movement,
    pub sensor: Sensor,
    pub opp_goal_color: GoalColor,
    pub config: JetsonConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct System {
    pub pause: bool,
    pub shutdown: bool,
    pub reboot: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Sensor {
    pub ir: Ir,
    pub on_line: bool,
    pub have_ball: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Ir {
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum GoalColor {
    #[default]
    Blue,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum JetsonConfig {
    #[default]
    None,
    OpenCVOpp(OpenCVOpp),
    OpenCVOwn(OpenCVOwn),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenCVOpp {
    pub h_min: u8,
    pub h_max: u8,
    pub s_min: u8,
    pub s_max: u8,
    pub v_min: u8,
    pub v_max: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenCVOwn {
    pub h_min: u8,
    pub h_max: u8,
    pub s_min: u8,
    pub s_max: u8,
    pub v_min: u8,
    pub v_max: u8,
}
