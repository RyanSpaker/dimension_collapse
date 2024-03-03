// Loading functionality Module
pub mod shape_loading;

use bevy::{prelude::*, utils::hashbrown::HashSet};

// System set for adding loading tasks to the LoadingTasks resource
#[derive(Debug, Clone, Reflect, PartialEq, Eq, Hash, SystemSet)]
pub struct InitLoadingTasks;
// Resource used to keep track of all loading tasks that need to be finished before gameplay
#[derive(Default, Debug, Clone, Reflect, Resource)]
pub struct LoadingTasks{
    tasks: HashSet<String>,
    start_time: f64
}

// event for when a task has finished
#[derive(Default, Debug, Clone, Reflect, Event)]
pub struct FinishedLoadingTask{pub name: String}

// Initializes the LoadinTasks resource for the loading of the app.
pub fn init_loading_tasks(mut tasks: ResMut<LoadingTasks>, time: Res<Time>){
    info!("Started Loading!");
    tasks.tasks = HashSet::default();
    tasks.start_time = time.elapsed_seconds_f64();
}
// Finishes the loading of the app by updating the app state and printing ellapsed time
pub fn finish_loading(mut tasks: ResMut<LoadingTasks>, time: Res<Time>, mut next_state: ResMut<NextState<super::AppState>>, mut events: EventReader<FinishedLoadingTask>){
    let delay = time.elapsed_seconds_f64() - tasks.start_time;
    for event in events.read(){
        if tasks.tasks.remove(&event.name){
            info!("Finished {}! Took {} seconds.", event.name, delay);
        }
    }
    if tasks.tasks.is_empty(){
        info!("Loading Has Finished! It took {} seconds.", delay);
        next_state.set(super::AppState::MainMenu);
    }
}
// Adds all Loading functionality to the app.
pub struct LoadingPlugin;
impl Plugin for LoadingPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<LoadingTasks>()
            .register_type::<InitLoadingTasks>()
            .register_type::<FinishedLoadingTask>()
            .add_event::<FinishedLoadingTask>()
            .init_resource::<LoadingTasks>()
            .add_systems(Update, init_loading_tasks.run_if(run_once()).in_set(super::LoadingUpdateSystems))
            .configure_sets(Update, InitLoadingTasks.run_if(run_once()).in_set(super::LoadingUpdateSystems).after(init_loading_tasks))
            .add_systems(Update, finish_loading.in_set(super::LoadingUpdateSystems).after(InitLoadingTasks))
            .add_plugins(shape_loading::ShapeLoadingPlugin);
    }
}