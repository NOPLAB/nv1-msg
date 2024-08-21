#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Velocity {
    pub linear_x: f32,
    pub linear_y: f32,
    pub angular_z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Ir {
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct HubMsgPackRx {
    pub vel: Velocity,
    pub kick: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct HubMsgPackTx {
    pub stop: bool,
    pub ir: Ir,
    pub have_ball: bool,
}
