// Loading functionality Module
use bevy::{prelude::*, utils::hashbrown::HashSet};

use super::AppState;

/// System set containgin all Update systems to run when loading.
#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingUpdate;

/// System set containgin methods to call when we initialize loading
#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash, SystemSet)]
pub struct InitLoading;

/// Resource used to keep track of all loading tasks that need to be finished before gameplay
#[derive(Default, Debug, Clone, Reflect, Resource)]
pub struct LoadingTasks{
    pub tasks: HashSet<String>,
    start_time: f64
}

/// event for when a loading task has finished
#[derive(Default, Debug, Clone, Reflect, Event)]
pub struct FinishedLoadingTask{pub name: String}

/// Initializes the LoadinTasks resource for the loading of the app.
pub fn init_loading_tasks(mut tasks: ResMut<LoadingTasks>, time: Res<Time>){
    info!("Started Loading!");
    tasks.tasks = HashSet::default();
    tasks.start_time = time.elapsed_seconds_f64();
}

/// Updates the state of loading by either printing fnish messages, or setting the app state to main menu
pub fn update_load_state(
    mut tasks: ResMut<LoadingTasks>, 
    time: Res<Time>,
    mut next_state: ResMut<NextState<super::AppState>>, 
    mut events: EventReader<FinishedLoadingTask>
){
    let delay = time.elapsed_seconds_f64() - tasks.start_time;
    for event in events.read(){
        if tasks.tasks.remove(&event.name){
            info!("Finished {}! Took {} seconds.", event.name, delay);
        }
    }
    if tasks.tasks.is_empty(){
        info!("Loading Has Finished! It took {} seconds.", delay);
        next_state.set(super::AppState::SpawnScene);
    }
}

/// Adds all Loading functionality to the app.
pub struct LoadingPlugin;
impl Plugin for LoadingPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<LoadingUpdate>()
            .register_type::<InitLoading>()
            .register_type::<LoadingTasks>()
            .register_type::<FinishedLoadingTask>()
            .add_event::<FinishedLoadingTask>()
            .init_resource::<LoadingTasks>()
            .configure_sets(Update, LoadingUpdate.run_if(in_state(AppState::Loading)))
            .configure_sets(Update, InitLoading.run_if(in_state(AppState::Loading).and_then(run_once())).before_ignore_deferred(LoadingUpdate))
            .add_systems(Update, update_load_state.after_ignore_deferred(InitLoading).in_set(LoadingUpdate))
            .add_systems(Update, init_loading_tasks.run_if(in_state(AppState::Loading).and_then(run_once())).before_ignore_deferred(InitLoading));
    }
}