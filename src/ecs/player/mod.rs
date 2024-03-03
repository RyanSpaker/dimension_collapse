use bevy::prelude::*;

#[derive(Debug, Default, Clone, Reflect, Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
    }
}