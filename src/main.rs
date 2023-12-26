use bevy::app::Main;
use bevy::math::Vec2;
use bevy::prelude::{App, Camera, Camera2dBundle, Commands, Component, GlobalTransform, Query, ResMut, Resource, Startup, Window, With};
use bevy::window::PrimaryWindow;

#[derive(Resource,Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

fn main() {
   App::new()
      .init_resource::<MyWorldCoords>()
       .add_systems(Startup, setup)
   ;

}

fn setup(mut commands: Commands) {
   commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn my_cursor_system(
   mut my_coords: ResMut<MyWorldCoords>,
   q_window: Query<&Window, With<PrimaryWindow>>,
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
){
   let (camera, camera_transform) = q_camera.single();
   let window = q_window.single();

}