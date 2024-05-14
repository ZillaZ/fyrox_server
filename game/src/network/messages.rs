use deku::prelude::*;
use fyrox::core::algebra::{Vector3, Vector4, Vector2};

#[derive(DekuRead, DekuWrite)]
pub struct Transform {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 2]
}

impl Transform {
    pub fn from_vectors(translation: Vector3<f32>, rotation: Vector4<f32>, scale: Vector2<f32>) -> Self {
        Self {
            translation: [translation.x, translation.y, translation.z],
            rotation: [rotation.x, rotation.y, rotation.z, rotation.w],
            scale: [scale.x, scale.y]
        }
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct PlayerInput {
    pub transform: Transform
}

#[derive(DekuRead, DekuWrite)]
pub struct PlayerOutput {
    transform: Transform,
    hp: f32
}

impl PlayerOutput {
    pub fn new(transform: Transform, hp: f32) -> Self {
        Self {
            transform,
            hp
        }
    }
}
