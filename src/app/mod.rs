// App structure module. Loading Gameplay, Scene transition stuff is placed here.
pub mod loading;

use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

// State of the application
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash, States, Reflect)]
pub enum AppState{
    #[default] Loading,
    MainMenu,
    Gameplay,
    Paused
}

// System group for Loading functionaltiy
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet, Reflect)]
pub struct LoadingUpdateSystems;

// System group for Gameplay functionality
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet, Reflect, ScheduleLabel)]
pub struct GameplayUpdateSystems;

// Runs when we enter gameplay state, makes the game window visible
pub fn make_window_visible(mut windows: Query<&mut Window>){
    for window in windows.iter_mut(){
        window.into_inner().visible = true;
    }
}

pub struct GameAppPlugin;
impl Plugin for GameAppPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<AppState>()
            .register_type::<LoadingUpdateSystems>()
            .register_type::<GameplayUpdateSystems>()
            .init_state::<AppState>()
            .configure_sets(Update, LoadingUpdateSystems.run_if(in_state(AppState::Loading)))
            .configure_sets(Update, GameplayUpdateSystems.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Loading), make_window_visible)
            .add_plugins(loading::LoadingPlugin);
    }
}
