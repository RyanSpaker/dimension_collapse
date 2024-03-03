use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{app::AppState, ecs::{assets::{images::ImageStore, shapes::Shapes}, main_camera::MainCamera, player::Player}};

fn spawn_main_menu(mut commands: Commands, shapes: Res<Shapes>, images: Res<ImageStore>, mut next_state: ResMut<NextState<AppState>>){
    // Main Camera
    commands.spawn((Camera2dBundle{
        projection: OrthographicProjection { 
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin { min_width: 100.0, min_height: 100.0 }, 
            near: -10.0,
            ..Default::default()},
        camera: Camera{clear_color: ClearColorConfig::Custom(Color::BLACK), ..Default::default()},
        ..Default::default()
    }, MainCamera));
    // Screen Barriers
    commands.spawn(SpriteBundle{
        sprite: {let mut sprite = images.screen_barrier_sprite.clone(); sprite.custom_size = Some(Vec2::new(10.0, 100.0)); sprite},
        texture: images.screen_barrier_img.clone(), transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0)), ..Default::default()
    });
    commands.spawn(SpriteBundle{
        sprite: {let mut sprite = images.screen_barrier_sprite.clone(); sprite.custom_size = Some(Vec2::new(10.0, 100.0)); sprite},
        texture: images.screen_barrier_img.clone(), 
        transform: Transform::from_matrix(Mat4::from_translation(Vec3::new(50.0, 0.0, 0.0))*Mat4::from_rotation_z(PI)), 
        ..Default::default()
    });
    commands.spawn(SpriteBundle{
        sprite: {let mut sprite = images.screen_barrier_sprite.clone(); sprite.custom_size = Some(Vec2::new(10.0, 100.0)); sprite},
        texture: images.screen_barrier_img.clone(), 
        transform: Transform::from_matrix(Mat4::from_translation(Vec3::new(0.0, -50.0, 0.0))*Mat4::from_rotation_z(PI/2.0)), 
        ..Default::default()
    });
    commands.spawn(SpriteBundle{
        sprite: {let mut sprite = images.screen_barrier_sprite.clone(); sprite.custom_size = Some(Vec2::new(10.0, 100.0)); sprite},
        texture: images.screen_barrier_img.clone(), 
        transform: Transform::from_matrix(Mat4::from_translation(Vec3::new(0.0, 50.0, 0.0))*Mat4::from_rotation_z(-PI/2.0)), 
        ..Default::default()
    });
    //Player
    commands.spawn((SpriteBundle{
        sprite: {
            let mut sprite = shapes.ring_sprite.clone(); sprite.custom_size = Some(Vec2::ONE*50.0); sprite
        }, texture: shapes.ring.clone(), ..Default::default()
    }, Player));
    next_state.set(AppState::Gameplay);
    info!("Main Menu Scene has been spawned!");
}

/// Adds functionaltiy for spawning the main menu scene
pub struct SpawnScenePlugin;
impl Plugin for SpawnScenePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_main_menu.run_if(in_state(AppState::SpawnScene).and_then(run_once())));
    }
}