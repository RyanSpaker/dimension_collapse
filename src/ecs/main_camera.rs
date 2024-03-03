use bevy::prelude::*;

#[derive(Default, Debug, Clone, Reflect, Component)]
pub struct MainCamera;

pub struct MainCameraPlugin;
impl Plugin for MainCameraPlugin{
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>();
    }
}