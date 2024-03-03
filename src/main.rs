use bevy::prelude::*;
use dimension_collapse::Game;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(Game);
    app.add_plugins(WorldInspectorPlugin::new());
    app.run();
}