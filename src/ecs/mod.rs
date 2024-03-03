// This module contains all ecs elements
pub mod player;
pub mod main_camera;
pub mod assets;

use bevy::prelude::*;

pub struct EcsPlugin;
impl Plugin for EcsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin, assets::EcsAssetsPlugin, main_camera::MainCameraPlugin));
    }
}