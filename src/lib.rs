pub mod app;
use bevy::{prelude::*, window::WindowResolution};


pub struct Game;
impl Plugin for Game{
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin{primary_window: Some(Window{
            present_mode: bevy::window::PresentMode::AutoVsync,
            visible: false,
            mode: bevy::window::WindowMode::Windowed,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            title: "Dimension Collapse".into(),
            resolution: WindowResolution::new(800.0, 450.0),
            ..Default::default()
        }), ..Default::default()}))
        .add_plugins(app::GameAppPlugin);
    }
}