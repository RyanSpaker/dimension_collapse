// Loads Miscellaneous Images

use bevy::{prelude::*, utils::hashbrown::HashSet};
use crate::app::loading::{FinishedLoadingTask, InitLoading, LoadingTasks, LoadingUpdate};

/// A Resource to hold various assorted images used in the game
#[derive(Debug, Default, Clone, Reflect, Resource)]
pub struct ImageStore{
    pub screen_barrier_img: Handle<Image>,
    pub screen_barrier_mat: Handle<StandardMaterial>,
    pub screen_barrier_sprite: Sprite,

    unloaded_images: HashSet<AssetId<Image>>,
    message_sent: bool
}
impl ImageStore{
    /// Adds a loading task to the LoadingTasks Resource
    pub fn add_loading_task(mut tasks: ResMut<LoadingTasks>){
        tasks.tasks.insert("MiscImageLoading".into());
    }
    /// Loads the images from assets folder, adding them all to the unloaded images set
    pub fn load_images(mut image_store: ResMut<Self>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>){
        image_store.unloaded_images = HashSet::default(); image_store.message_sent = false;
        image_store.screen_barrier_img = asset_server.load("textures/edge_barrier.png");
        image_store.screen_barrier_mat = materials.add(StandardMaterial{base_color_texture: Some(image_store.screen_barrier_img.clone()), base_color: Color::WHITE, ..Default::default()});
        image_store.screen_barrier_sprite = Sprite{color: Color::WHITE, flip_x: false, flip_y: false, custom_size: Some(Vec2::ONE), rect: None, anchor: bevy::sprite::Anchor::CenterRight};
        let handle = image_store.screen_barrier_img.id();
        image_store.unloaded_images.insert(handle);
    }
    /// Listens for asset load events, keeping track of which images are yet to be finished. Sends a FinishedLoadingEvent when all images are loaded
    pub fn update_load_state(mut event_reader: EventReader<AssetEvent<Image>>, mut images: ResMut<Self>, mut event_writer: EventWriter<FinishedLoadingTask>){
        event_reader.read().for_each(|event| {
            if let AssetEvent::LoadedWithDependencies { id } = event {
                images.unloaded_images.remove(id);
            }
        });
        if images.unloaded_images.is_empty() && !images.message_sent{
            event_writer.send(FinishedLoadingTask{name: "MiscImageLoading".into()});
            images.message_sent = true;
        }
    }
}

pub struct MiscImagesPlugin;
impl Plugin for MiscImagesPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<ImageStore>()
            .init_resource::<ImageStore>()
            .add_systems(Update, (ImageStore::add_loading_task, ImageStore::load_images).in_set(InitLoading))
            .add_systems(Update, ImageStore::update_load_state.in_set(LoadingUpdate));
    }
}
