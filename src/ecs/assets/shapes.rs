// Module to hold code related to the basic shapes used in the app

use bevy::{prelude::*, utils::hashbrown::HashSet};

use crate::app::loading::{FinishedLoadingTask, InitLoading, LoadingTasks, LoadingUpdate};

/// Resource to hold the shape images and materials
#[derive(Default, Debug, Clone, Reflect, Resource)]
pub struct Shapes{
    pub ring: Handle<Image>,
    pub ring_mat: Handle<StandardMaterial>,
    pub ring_sprite: Sprite,

    pub rounded_square: Handle<Image>,
    pub rounded_square_mat: Handle<StandardMaterial>,
    pub rounded_square_sprite: Sprite,

    pub triangle: Handle<Image>,
    pub triangle_mat: Handle<StandardMaterial>,
    pub triangle_sprite: Sprite,

    unloaded_images: HashSet<AssetId<Image>>,
    message_sent: bool
}
impl Shapes{
    /// System which adds a loading task to the Loading Tasks Resource
    pub fn add_loading_task(mut tasks: ResMut<LoadingTasks>){
        tasks.tasks.insert("ShapeLoading".into());
    }
    /// System which loads the shape images from disk, and sets the unloaded images set
    pub fn load_images(mut shapes: ResMut<Self>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>){
        shapes.unloaded_images = HashSet::default();
        shapes.ring = asset_server.load("textures/ring.png");
        shapes.ring_mat = materials.add(StandardMaterial{base_color_texture: Some(shapes.ring.clone()), base_color: Color::WHITE, ..Default::default()});
        shapes.ring_sprite = Sprite{color: Color::WHITE, flip_x: false, flip_y: false, custom_size: Some(Vec2::ONE), rect: None, anchor: bevy::sprite::Anchor::Center};
        let handle = shapes.ring.id();
        shapes.unloaded_images.insert(handle);

        shapes.rounded_square = asset_server.load("textures/rounded_square.png");
        shapes.rounded_square_mat = materials.add(StandardMaterial{base_color_texture: Some(shapes.rounded_square.clone()), base_color: Color::WHITE, ..Default::default()});
        shapes.rounded_square_sprite = Sprite{color: Color::WHITE, flip_x: false, flip_y: false, custom_size: Some(Vec2::ONE), rect: None, anchor: bevy::sprite::Anchor::Center};
        let handle = shapes.rounded_square.id();
        shapes.unloaded_images.insert(handle);

        shapes.triangle = asset_server.load("textures/triangle.png");
        shapes.triangle_mat = materials.add(StandardMaterial{base_color_texture: Some(shapes.triangle.clone()), base_color: Color::WHITE, ..Default::default()});
        shapes.triangle_sprite = Sprite{color: Color::WHITE, flip_x: false, flip_y: false, custom_size: Some(Vec2::ONE), rect: None, anchor: bevy::sprite::Anchor::Center};
        let handle = shapes.triangle.id();
        shapes.unloaded_images.insert(handle);
        shapes.message_sent = false;
    }
    /// Listens to asset events and records when an image has been loaded. If all images have been loaded, it sends a FinishedLoadingTask event.
    pub fn update_load_state(mut event_reader: EventReader<AssetEvent<Image>>, mut shapes: ResMut<Self>, mut event_writer: EventWriter<FinishedLoadingTask>){
        event_reader.read().for_each(|event| {
            if let AssetEvent::LoadedWithDependencies { id } = event {
                shapes.unloaded_images.remove(id);
            }
        });
        if shapes.unloaded_images.is_empty() && !shapes.message_sent{
            event_writer.send(FinishedLoadingTask{name: "ShapeLoading".into()});
            shapes.message_sent = true;
        }
    }
}


pub struct ShapesPlugin;
impl Plugin for ShapesPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<Shapes>()
            .init_resource::<Shapes>()
            .add_systems(Update, (Shapes::add_loading_task, Shapes::load_images).in_set(InitLoading))
            .add_systems(Update, Shapes::update_load_state.in_set(LoadingUpdate));
    }
}