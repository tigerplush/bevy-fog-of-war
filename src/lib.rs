use bevy::prelude::*;

mod types;
pub use types::*;

pub struct FogOfWarPlugin;
pub struct FogOfWarPlugin2D;

impl Plugin for FogOfWarPlugin {
    fn build(&self, _app: &mut App) {}
}

impl Plugin for FogOfWarPlugin2D {
    fn build(&self, _app: &mut App) {}
}
