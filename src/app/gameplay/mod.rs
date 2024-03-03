// Module containing gameplay structure code
use bevy::{prelude::*, winit::WinitWindows};
use winit::window::Icon;
use crate::ecs::assets::shapes::Shapes;

#[derive(Debug, Clone, Default, Reflect, PartialEq, Eq, Hash, SystemSet)]
pub struct GameplayUpdate;

/// State struct which represents the level of gameplay 
#[derive(Default, Debug, Clone, Reflect, PartialEq, Eq, Hash, States)]
pub enum GameplayState{
    #[default] MainMenu
}

// Runs when we enter gameplay state, makes the game window visible
pub fn make_window_visible(mut windows: Query<(Entity, &mut Window)>, winit: NonSend<WinitWindows>, images: Res<Assets<Image>>, shapes: Res<Shapes>){
    let image = images.get(shapes.ring.id()).expect("make_window_visible: Couldn't get ring image from assets");
    let image = image.convert(bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb).expect("Couldn't convert image");
    let icon = Icon::from_rgba(image.data.clone(), image.width(), image.height()).expect("Couldn't create Icon");
    for (entity, window) in windows.iter_mut(){
        window.into_inner().visible = true;
        let Some(window_id) = winit.entity_to_winit.get(&entity) else {continue;};
        let Some(winit_window) = winit.windows.get(window_id) else {continue;};
        winit_window.set_window_icon(Some(icon.clone()))        
    }
    info!("Window has been made visible!");
}

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin{
    fn build(&self, app: &mut App) {
        app
            .register_type::<GameplayState>()
            .register_type::<GameplayUpdate>()
            .configure_sets(Update, GameplayUpdate.run_if(in_state(super::AppState::Gameplay)))
            .add_systems(OnEnter(super::AppState::Gameplay), make_window_visible);
    }
}