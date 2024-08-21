#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Ir {
    pub x: f32,
    pub y: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Line {
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
    pub pause: bool,
    pub shutdown: bool,
    pub reboot: bool,
    pub vel: Velocity,
    pub ir: Ir,
    pub line: Line,
    pub have_ball: bool,
}
