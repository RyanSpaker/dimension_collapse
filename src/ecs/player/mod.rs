use bevy::prelude::*;

use super::assets::shapes::Shapes;

#[derive(Debug, Default, Clone, Reflect, Component)]
pub struct Player;

// Spawns the player entity at a specified location.
fn spawn_player(commands: &mut Commands, shapes: &Shapes, location: Vec2){
    commands.spawn((SpriteBundle{
        sprite: {
            let mut sprite = shapes.ring_sprite.clone(); sprite.custom_size = Some(Vec2::ONE*50.0); sprite
        }, texture: shapes.ring.clone(), 
        transform: Transform::from_translation(location.extend(0.0)),        
        ..Default::default()
    }, Player));
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
    }
}