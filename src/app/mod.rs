// App structure module. Loading Gameplay, Scene transition stuff is placed here.
pub mod loading;
pub mod spawn_scene;
pub mod gameplay;

use bevy::prelude::*;

/// State of the application
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, States, Reflect)]
pub enum AppState{
    #[default] Startup,
    Loading,
    SpawnScene,
    Gameplay,
    Paused
}

/// Starts loading the app. Run first thing. Used so that OnEnter(Loading) actually exists
pub fn begin_loading(mut next_state: ResMut<NextState<AppState>>){
    next_state.set(AppState::Loading);
}

pub struct GameAppPlugin;
impl Plugin for GameAppPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<AppState>()
            .init_state::<AppState>()
            .add_plugins((
                loading::LoadingPlugin, 
                spawn_scene::SpawnScenePlugin,
                gameplay::GameplayPlugin
            ))
            .add_systems(Startup, begin_loading);
    }
}
