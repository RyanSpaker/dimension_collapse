pub mod app;
pub mod ecs;

use bevy::{prelude::*, window::WindowResolution};


pub struct Game;
impl Plugin for Game{
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin{primary_window: Some(Window{
            present_mode: bevy::window::PresentMode::AutoVsync,
            visible: true,
            title: "Dimension Collapse".into(),
            resolution: WindowResolution::new(800.0, 450.0),
            ..Default::default()
        }), ..Default::default()}))
        .add_plugins((app::GameAppPlugin, ecs::EcsPlugin));
    }
}