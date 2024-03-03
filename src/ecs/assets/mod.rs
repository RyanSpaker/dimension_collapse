// This module contains all of the asset resources and functionality
pub mod shapes;
pub mod images;

use bevy::prelude::*;

pub struct EcsAssetsPlugin;
impl Plugin for EcsAssetsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((shapes::ShapesPlugin, images::MiscImagesPlugin));
    }
}