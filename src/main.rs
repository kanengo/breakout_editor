use bevy::app::Update;
use bevy::diagnostic::LogDiagnosticsPlugin;

use bevy::math::Vec2;
use bevy::prelude::{*};

use bevy::ui::RelativeCursorPosition;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use serde::{Serialize, Deserialize};

const BACKGROUND_COLOR: Color = Color::rgb(35.0/255.0, 35.0/255.0, 105.0/255.0);
const EDGE_COLOR: Color = Color::rgb(25.0/255.0, 25.0/255.0, 72.0/255.0);

const BRICK_DEFAULT_COLOR: Color = Color::rgb(52.0/255.0, 216.0/255.0, 0.0/255.0);
const BRICK_SIZE: Vec2 = Vec2::new(10., 10.);

const GAP_BETWEEN_BRICK: Vec2 = Vec2::new(2.,2.);
// const BRICK_COLOR: Color = Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(253.0/255.0, 240.0/255.0, 0.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(250.0/255.0, 163.0/255.0, 1.0/255.0);
// const BRICK_COLOR: Color = Color::rgb(248.0/255.0, 38.0/255.0, 2.0/255.0);

const BRICK_COLORS: &[Color]= &[
   Color::rgb(52.0/255.0, 216.0/255.0, 0.0/255.0),
   Color::rgb(64.0/255.0, 230.0/255.0, 255.0/255.0),
   Color::rgb(253.0/255.0, 240.0/255.0, 0.0/255.0),
   Color::rgb(250.0/255.0, 163.0/255.0, 1.0/255.0),
   Color::rgb(248.0/255.0, 38.0/255.0, 2.0/255.0),
];

const WALL_COLOR: Color = Color::rgb(117.0/255.0, 117.0/255.0, 119.0/255.0);

const SCREEN_SIZE:(f32, f32) = (720.0, 960.0);
const EDGE_SIZE:(f32, f32) = (680.0, 900.0);

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

#[derive(Component, Copy, Clone, PartialEq)]
enum Brick {
   Normal(Color),
   Wall(Color),
}

impl Brick {
    fn color(&self) -> Color {
        match self {
            Brick::Normal(color) => {
                *color
            }
            Brick::Wall(color) => {
                *color
            }
        }
    }
}

#[derive(Component)]
struct PlacedBrick(Brick);

#[derive(Resource)]
struct PlacedBricks {
    m: HashMap<i32, Entity>,
}

impl PlacedBricks {
    fn new() -> Self {
        Self {
            m: HashMap::new(),
        }
    }
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
        .insert_resource(PlacedBricks::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, (
            setup,
            setup_brick,
            setup_gizmos,
        ))
        .add_systems(Update, (
            my_cursor_system,
            update_buttons,
            move_brick_system,
            check_relative_cursor_position,
            place_brick_system,
            export_placed_bricks,
            touch_system,
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
          transform: Transform::from_translation(Vec3::new(0.,0., 0.)).with_scale(BRICK_SIZE.extend(0.)),
          ..default()
      },
      brick_component,
    ));
}

fn setup_gizmos(
    mut gizmos: Gizmos
) {
    for x in 0..=(EDGE_SIZE.0 as i32) / 2 {
        gizmos.line_2d((x as f32, -EDGE_SIZE.1 / 2.0).into(), (x as f32, EDGE_SIZE.1 / 2.0).into(), Color::WHITE);
        gizmos.line_2d((-x as f32, -EDGE_SIZE.1 / 2.0).into(), (-x as f32, EDGE_SIZE.1 / 2.0).into(), Color::WHITE);
    }
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
    mut commands: Commands,
    button_query: Query<
      (&Interaction, &BrickButton),
      Changed<Interaction>
    >,
    mut selected_button_res: ResMut<SelectedButton>,
    brick_query: Query<(Entity,  &Brick)>,
){
    let (brick_entity, brick) = brick_query.single();

    for (&interaction, &brick_button) in &button_query {
        match interaction {
        Interaction::Pressed => {
            selected_button_res.0 = brick_button;
            info!("selected_button_res:{:?}", selected_button_res.0);

            let pressed_brick_component;
            match selected_button_res.0 {
                 BrickButton::Brick(color) => {
                     pressed_brick_component = Brick::Normal(color);
                 },
                 BrickButton::Wall(color) => {
                     pressed_brick_component = Brick::Wall(color);
                 },
            };
            if pressed_brick_component.eq(brick) {
                return;
            }

            commands.entity(brick_entity).insert((
               pressed_brick_component,
               Sprite {
                  color: pressed_brick_component.color(),
                  ..default()
              },
            ));
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

fn move_brick_system(
    cursor_world_coords: Res<CursorWorldCoords>,
    mut brick_query: Query<&mut Transform, With<Brick>>
) {
    for mut transform in &mut brick_query {
        let mut x = cursor_world_coords.x.clamp(-EDGE_SIZE.0/2. + BRICK_SIZE.x / 2., EDGE_SIZE.0/2. - BRICK_SIZE.x / 2.);
        let mut y = cursor_world_coords.y.clamp(-EDGE_SIZE.1/2. + BRICK_SIZE.y / 2., EDGE_SIZE.1/2. - BRICK_SIZE.y / 2.);

        x = (x / (BRICK_SIZE.x + GAP_BETWEEN_BRICK.x)).floor() * (BRICK_SIZE.x + GAP_BETWEEN_BRICK.x)
            + (BRICK_SIZE.x + GAP_BETWEEN_BRICK.x) / 2.0;

        y = (y / (BRICK_SIZE.x + GAP_BETWEEN_BRICK.x)).floor() * (BRICK_SIZE.y + GAP_BETWEEN_BRICK.y)
            + (BRICK_SIZE.y + GAP_BETWEEN_BRICK.y) / 2.0 ;

        transform.translation = Vec2::new(x,y).extend(1.0);

        // info!("pos:{}", transform.translation);
    }
}

fn check_relative_cursor_position(
    relative_cursor_position_query: Query<&RelativeCursorPosition>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let relative_cursor_position = relative_cursor_position_query.single();
    let mut window = q_window.single_mut();

    // if relative_cursor_position.mouse_over() {
    //    window.cursor.visible = true;
    // } else {
    //     window.cursor.visible = false;
    // }
}

fn place_brick_system(
    mut commands: Commands,
    brick_query: Query<(&Transform, &Brick), With<Brick>>,
    mut placed_bricks_res: ResMut<PlacedBricks>,
    placed_brick_query: Query<&PlacedBrick>,
    mouse: Res<Input<MouseButton>>,
    relative_cursor_position_query: Query<&RelativeCursorPosition>,

) {
    let relative_cursor_position = relative_cursor_position_query.single();

    if relative_cursor_position.mouse_over() {
        return;
    }

    let (&brick_transform, brick) = brick_query.single();

    let zone;


    if brick_transform.translation.x > 0.0 {
        if brick_transform.translation.y > 0.0 {
            zone = 1;
        } else {
            zone = 4;
        }
    } else {
        if brick_transform.translation.y > 0.0 {
            zone = 2;
        } else {
            zone = 3;
        }
    }

    let key = zone * 10_000_000 + brick_transform.translation.x.abs() as i32 * 1_000_000
        + brick_transform.translation.y.abs() as i32 * 1000;

    if mouse.pressed(MouseButton::Left) || mouse.pressed(MouseButton::Right) {
        if let Some(&old) = placed_bricks_res.m.get(&key) {
            let old_placed_brick = placed_brick_query.get(old).unwrap();
            if  mouse.pressed(MouseButton::Left) && old_placed_brick.0.eq(brick) {
                info!("same place, jump out!");
                return;
            }
            commands.entity(old).despawn();
            placed_bricks_res.m.remove(&key);
            info!("move old");
        }
    }

    if mouse.pressed(MouseButton::Left) {
        let id = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: brick.color(),
                    ..default()
                },
                // global_transform: GlobalTransform::from(Transform::IDENTITY),
                transform: Transform::from_translation(brick_transform.translation).with_scale(BRICK_SIZE.extend(0.)),
                ..default()
            },
            PlacedBrick(*brick),
        )).id();

        placed_bricks_res.m.insert(key, id);
    }
}
#[derive(Default, Serialize, Deserialize)]
struct BrickData {
   brick_type: u8,
   color: Color,
   pos: Vec2,
}

#[derive(Default, Serialize, Deserialize)]
struct ExportData {
   bricks: Vec<BrickData>,
}

fn export_placed_bricks(
   placed_brick_query: Query<(&Transform, &PlacedBrick)>,
   mouse: Res<Input<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Middle) {
        return;
    }
    let mut bricks = Vec::new();
    for (transform, placed_brick) in &placed_brick_query {
        let brick_type;
        let color;
        match placed_brick.0 {
            Brick::Normal(c) =>  {
            brick_type = 0;
            color = c;
            },
            Brick::Wall(c) => {
            brick_type = 1;
            color = c;
            },
        }
        let data = BrickData {
            brick_type,
            color,
            pos: transform.translation.truncate(),
        };
        bricks.push(data);
    }

    let export_data = ExportData {
        bricks: bricks,
    };

    let j = serde_json::to_string_pretty(&export_data);
    println!("{:?}", j);
    if let Ok(json_str) = j {
        let _ = std::fs::write("./output.json", json_str);
    };
   
}


fn touch_system(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_released() {
        info!(
            "just released touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_canceled() {
        info!("canceled touch with id: {:?}", touch.id());
    }

    // you can also iterate all current touches and retrieve their state like this:
    for touch in touches.iter() {
        info!("active touch: {:?}", touch);
        info!("  just_pressed: {}", touches.just_pressed(touch.id()));
    }
}