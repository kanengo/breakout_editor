
use bevy::app::Update;
use bevy::diagnostic::{LogDiagnosticsPlugin};
use bevy::math::Vec2;
use bevy::prelude::{*};
use bevy::ui::RelativeCursorPosition;
use bevy::window::PrimaryWindow;


const BACKGROUND_COLOR: Color = Color::rgb(35.0/255.0, 35.0/255.0, 105.0/255.0);
const EDGE_COLOR: Color = Color::rgb(25.0/255.0, 25.0/255.0, 72.0/255.0);

const BRICK_DEFAULT_COLOR: Color = Color::rgb(52.0/255.0, 216.0/255.0, 0.0/255.0);
const BRICK_SIZE: Vec2 = Vec2::new(10., 10.);
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

#[derive(Resource,Default, Deref, DerefMut)]
struct CursorWorldCoords(Vec2);

#[derive(Resource, Deref, DerefMut, Debug)]
struct SelectedButton(BrickButton);

impl Default for SelectedButton {
    fn default() -> Self {
        Self(BrickButton::Brick(BRICK_DEFAULT_COLOR))
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component,Debug, Clone, Copy)]
enum BrickButton {
   Brick(Color),
   Wall(Color),
}

#[derive(Component)]
enum Brick {
   Normal(Color),
   Wall(Color),
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
         // FrameTimeDiagnosticsPlugin,
      ))
      .init_resource::<CursorWorldCoords>()
      .init_resource::<SelectedButton>()
      .insert_resource(ClearColor(BACKGROUND_COLOR))
      .add_systems(Startup, (
         setup,
      ))
      .add_systems(Update, (
         my_cursor_system,
         update_buttons,
      ))
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

   commands.spawn(NodeBundle {
      //base ui node
      style: Style {
         width: Val::Px(UI_NODE_SIZE.x),
         height: Val::Px(UI_NODE_SIZE.y),
         top: Val::Px(SCREEN_SIZE.1 - UI_NODE_SIZE.y),
         align_items: AlignItems::Center,
         justify_content: JustifyContent::Center,
         ..default()
      },
      background_color: Color::rgb(35.0/255.0, 35.0/255.0, 38.0/255.0).into(),
      ..default()
   }).insert(RelativeCursorPosition::default()).with_children(|parent| {
       //main node 
      parent.spawn(NodeBundle {
         style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            // margin: UiRect::horizontal(Val::Px(20.)),
            ..default()
         },
         ..default()
      }).with_children(|parent|{
         //brick node
         parent.spawn(NodeBundle {          
            style: Style {
               flex_direction: FlexDirection::Column,
               align_items: AlignItems::Center,
               justify_content: JustifyContent::Center,
               margin: UiRect::horizontal(Val::Px(20.)),
               // padding: UiRect::all(Val::Px(5.0)),
               ..default()
            },
            // background_color: Color::BLUE.into(),
            ..default()
         }).with_children(|parent|{ 
            //brick text
            parent.spawn(
               TextBundle::from_section("Bricks", text_style.clone())
            );

            //brick container
            parent.spawn(NodeBundle { 
                style: Style {
                  flex_direction: FlexDirection::Row,
                  align_items: AlignItems::Stretch,
                  padding: UiRect::all(Val::Px(2.)),
                  margin: UiRect::top(Val::Px(10.)),
                  ..Default::default()
                },
                background_color: Color::YELLOW.into(),
                ..Default::default()
            }).with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
               spawn_ui_brick_button(parent);
            });
         });

         //wall node
         parent.spawn(NodeBundle {          
            style: Style {
               flex_direction: FlexDirection::Column,
               align_items: AlignItems::Center,
               justify_content: JustifyContent::Center,
               margin: UiRect::horizontal(Val::Px(20.)),
               // padding: UiRect::all(Val::Px(5.0)),
               ..default()
            },
            // background_color: Color::GREEN.into(),
            ..default()
         }).with_children(|parent: &mut ChildBuilder<'_, '_, '_>|{ 
            //brick text
            parent.spawn(
               TextBundle::from_section("Wall", text_style.clone())
            );

            spawn_ui_wall_button(parent);
         });
      });
   });

}

fn setup_brick(
   mut commands: Commands,
   selected_button_res: Res<SelectedButton>
) {
   let brick_component;
   let brick_color;
   match selected_button_res.0 {
      BrickButton::Brick(color) => {
         brick_component = Brick::Normal(color);
         brick_color = color;
      },
      BrickButton::Wall(color) => {
         brick_component = Brick::Wall(color);
         brick_color = color;
      },
   };
   commands.spawn((
      SpriteBundle {
          sprite: Sprite {
              color: brick_color,
              ..default()
          },
          // global_transform: GlobalTransform::from(Transform::IDENTITY),
          transform: Transform::from_translation(0.,0., 0.).with_scale(BRICK_SIZE),
          ..default()
      },
      brick_component,
  ));
}

fn spawn_ui_brick_button(parent: &mut ChildBuilder) {
   parent.spawn(NodeBundle {
      style: Style {
         flex_direction: FlexDirection::Row,
         ..default()
      },
      background_color: Color::BLACK.into(),
      ..default()
   }).with_children(|parent| {
      for &brick_color in BRICK_COLORS {
         parent.spawn((
            ButtonBundle {
                  style: Style {
                  width: Val::Px(20.),
                  height: Val::Px(20.),
                  border: UiRect::all(Val::Px(2.)),
                  margin: UiRect::all(Val::Px(2.)),
                  ..default()
                },
                background_color: brick_color.into(),
                ..default()
            },
            BrickButton::Brick(brick_color),
          ));
      }
   });
   
}

fn spawn_ui_wall_button(parent: &mut ChildBuilder) {
   parent.spawn(NodeBundle {
      style: Style {
         // flex_direction: FlexDirection::Row,
         margin: UiRect::all(Val::Px(10.)),
         ..default()
      },
      background_color: Color::BLACK.into(),
      ..default()
   }).with_children(|parent| {
      parent.spawn((
         ButtonBundle {
               style: Style {
               width: Val::Px(20.),
               height: Val::Px(20.),
               border: UiRect::all(Val::Px(2.)),
               margin: UiRect::all(Val::Px(2.)),
               ..default()
               },
               background_color: WALL_COLOR.into(),
               ..default()
         },
         BrickButton::Wall(WALL_COLOR),
         ));
     
   });
   
}

fn update_buttons(
   button_query: Query<
      (&Interaction, &BrickButton),
      Changed<Interaction>
   >,
   mut selected_button_res: ResMut<SelectedButton>
){
   for (&interaction, &brick_buttion) in &button_query {
      match interaction {
         Interaction::Pressed => {
            selected_button_res.0 = brick_buttion;
            info!("selected_button_res:{:?}", selected_button_res.0);
         },
         _ => {}
      }
   }
}

fn my_cursor_system(
   mut cursor_world_coords: ResMut<CursorWorldCoords>,
   q_window: Query<&Window, With<PrimaryWindow>>,
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
){
   let (camera, camera_transform) = q_camera.single();
   let window = q_window.single();

   let Some(cursor_position) = window.cursor_position()
   else {
      return;
   };

   let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position)
   else {
      return;
   };

   cursor_world_coords.0 = point;
   // window.cursor.visible = true;
   
   // info!("cursor:{:?}, point:{:?}", cursor_position, point)
}