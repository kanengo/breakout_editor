
use bevy::app::Update;
use bevy::diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::math::Vec2;
use bevy::prelude::{*};
use bevy::window::PrimaryWindow;


const BACKGROUND_COLOR: Color = Color::rgb(35.0/255.0, 35.0/255.0, 105.0/255.0);
const EDGE_COLOR: Color = Color::rgb(25.0/255.0, 25.0/255.0, 72.0/255.0);

const BRICK_COLOR: Color = Color::rgb(52.0/255.0, 216.0/255.0, 0.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(253.0/255.0, 240.0/255.0, 0.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(250.0/255.0, 163.0/255.0, 1.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(248.0/255.0, 38.0/255.0, 2.0/255.0);

const BRICK_COLORS: &[Color]= &[
   Color::rgb(52.0/255.0, 216.0/255.0, 0.0/255.0),
   Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0),
   Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0),
   Color::rgb(253.0/255.0, 240.0/255.0, 0.0/255.0),
   Color::rgb(250.0/255.0, 163.0/255.0, 1.0/255.0),
   Color::rgb(248.0/255.0, 38.0/255.0, 2.0/255.0),
];

const WALL_COLOR: Color = Color::rgb(117.0/255.0, 117.0/255.0, 119.0/255.0);

const SCREEN_SIZE:(f32, f32) = (900.0, 860.0);
const EDGE_SIZE:(f32, f32) = (840.0, 840.0);

const UI_NODE_SIZE: Vec2 = Vec2::new(300.0, 150.0);

#[derive(Resource,Default)]
struct MyWorldCoords(Vec2);

#[derive(Component)]
struct MainCamera;

enum SelectedType {
   Brick,
   Wall,
}



fn main() {
   App::new()
      .add_plugins((
         DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
               title: "breakout editor".into(),
               resolution: SCREEN_SIZE.into(),
               ..default()
            }),
            ..default()
         }),
         LogDiagnosticsPlugin::default(),
         FrameTimeDiagnosticsPlugin,
      ))
      .init_resource::<MyWorldCoords>()
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .add_systems(Startup, setup)
      .add_systems(Update, my_cursor_system)
      .run();
}

fn setup(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
) {
   commands.spawn((Camera2dBundle::default(), MainCamera));

   //edge background
   commands.spawn(SpriteBundle {
      transform: Transform::from_scale(Vec3::new(EDGE_SIZE.0, EDGE_SIZE.1, -1.0)),
      sprite: Sprite {
            color: EDGE_COLOR,
            ..default()
      },
      ..default()
   });

   let text_style = TextStyle {
      font: asset_server.load("fonts/FiraSans-Bold.ttf"),
      font_size: 16.0,
      color: Color::rgb(0.9, 0.9, 0.9),
   };

   //base ui node
   commands.spawn(NodeBundle {
      transform: Transform::from_translation(Vec2::new(0.0, SCREEN_SIZE.1 - UI_NODE_SIZE.y / 2.0).extend(0.0)),
      style: Style {
         width: Val::Px(UI_NODE_SIZE.x),
         height: Val::Px(UI_NODE_SIZE.y),
         top: Val::Px(SCREEN_SIZE.1 - UI_NODE_SIZE.y),
         ..default()
      },
      background_color: Color::rgb(35.0/255.0, 35.0/255.0, 38.0/255.0).into(),
      ..default()
   }).with_children(|parent| { //node 2
      parent.spawn(NodeBundle {
         style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
         },
         ..default()
      }).with_children(|parent|{ //node 3
         // parent.spawn(
         //    TextBundle::from_section("Bricks", text_style.clone())
         // );
         // parent.spawn(
         //    TextBundle::from_section("Bricks", text_style.clone())
         // );
         parent.spawn(NodeBundle {
            style: Style {
               flex_direction: FlexDirection::Column,
               align_items: AlignItems::Center,
               justify_content: JustifyContent::Center,
               ..default()
            },
            ..default()
         });
      });
   });

}

fn my_cursor_system(
   mut my_coords: ResMut<MyWorldCoords>,
   mut q_window: Query<&mut Window, With<PrimaryWindow>>,
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
){
   let (camera, camera_transform) = q_camera.single();
   let mut window = q_window.single_mut();

   let Some(cursor_position) = window.cursor_position()
   else {
      return;
   };

   let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position)
   else {
      return;
   };

   // window.cursor.visible = true;
   
   // info!("cursor:{:?}, point:{:?}", cursor_position, point)
}