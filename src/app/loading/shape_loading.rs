// Loades the primitive Shapes into the app

use bevy::{prelude::*, utils::hashbrown::HashSet};
// Resource to hold the shape images and materials
#[derive(Default, Debug, Clone, Reflect, Resource)]
pub struct Shapes{
    pub ring: Handle<Image>,
    pub ring_mat: Handle<StandardMaterial>,

    pub rounded_square: Handle<Image>,
    pub randouned_square_mat: Handle<StandardMaterial>,

    pub triangle: Handle<Image>,
    pub triangle_mat: Handle<StandardMaterial>,

    unloaded_images: HashSet<AssetId<Image>>,
}
// Adds the task to the LoadingTasks resource
pub fn add_task(mut tasks: ResMut<super::LoadingTasks>){
    tasks.tasks.insert("ShapeLoading".into());
}
// Initializes the image assets and requests that they be loaded
pub fn load_images(mut shapes: ResMut<Shapes>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>){
    shapes.ring = asset_server.load("textures/ring.png");
    shapes.ring_mat = materials.add(StandardMaterial{base_color_texture: Some(shapes.ring.clone()), base_color: Color::WHITE, ..Default::default()});
    let ring_img = shapes.ring.id();
    shapes.unloaded_images.insert(ring_img);
}
// Listens for asset events, and figures out when all images have been loaded
pub fn handle_events(mut event_reader: EventReader<AssetEvent<Image>>, mut shapes: ResMut<Shapes>, mut event_writer: EventWriter<super::FinishedLoadingTask>, mut finished: Local<bool>){
    event_reader.read().for_each(|event| {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            shapes.unloaded_images.remove(id);
        }
    });
    if shapes.unloaded_images.is_empty() && !finished.clone(){
        event_writer.send(super::FinishedLoadingTask{name: "ShapeLoading".into()});
        *finished = true;
    }
}
// Plugin to add shape texture and material loading
pub struct ShapeLoadingPlugin;
impl Plugin for ShapeLoadingPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<Shapes>()
            .init_resource::<Shapes>()
            .add_systems(Update, (add_task, load_images).in_set(super::InitLoadingTasks))
            .add_systems(Update, load_images.in_set(super::InitLoadingTasks))
            .add_systems(Update, handle_events.in_set(super::super::LoadingUpdateSystems));
    }
}