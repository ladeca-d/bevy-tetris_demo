use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use rand::{thread_rng, Rng};

// Static variables.
// screen matrix.
const ROWS: usize = 20 + 4 + 4;
const COLS: usize = 10 + 4 + 4;
static ONEPIXEL: f32 = 30.0;
static BOUNS: [usize; 4] = [0, 4, 9, 16];
static BOUND: [f32; 2] = [(COLS - 8) as f32 * ONEPIXEL, (ROWS - 8) as f32 * ONEPIXEL];
static L1: [[u8; 4]; 4] = [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static L2: [[u8; 4]; 4] = [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]];
static S1: [[u8; 4]; 4] = [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static Z11: [[u8; 4]; 4] = [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static Z12: [[u8; 4]; 4] = [[1, 0, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
static Z21: [[u8; 4]; 4] = [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static Z22: [[u8; 4]; 4] = [[0, 1, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
static W1: [[u8; 4]; 4] = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static W2: [[u8; 4]; 4] = [[1, 0, 0, 0], [1, 1, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
static W3: [[u8; 4]; 4] = [[1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static W4: [[u8; 4]; 4] = [[0, 1, 0, 0], [1, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
static L11: [[u8; 4]; 4] = [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
static L12: [[u8; 4]; 4] = [[1, 0, 0, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static L13: [[u8; 4]; 4] = [[1, 1, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [0, 0, 0, 0]];
static L14: [[u8; 4]; 4] = [[1, 1, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static L21: [[u8; 4]; 4] = [[1, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]];
static L22: [[u8; 4]; 4] = [[1, 1, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static L23: [[u8; 4]; 4] = [[1, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 0, 0, 0]];
static L24: [[u8; 4]; 4] = [[0, 0, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]];
static BLOCKSLIB: [[[[u8; 4]; 4]; 4]; 7] = [
    [L1, L2, L1, L2],
    [S1, S1, S1, S1],
    [Z11, Z12, Z11, Z12],
    [Z21, Z22, Z21, Z22],
    [W1, W2, W3, W4],
    [L11, L12, L13, L14],
    [L21, L22, L23, L24],
];

fn main() {
    // init screen resource
    let mut screen = [[1; COLS]; ROWS];
    for i in 0..ROWS - 4 {
        for j in 4..COLS - 4 {
            screen[i][j] = 0;
        }
    }

    // prepare next block
    let mut rng = thread_rng();
    let type_id = rng.gen_range(0..7);
    let shape_id = rng.gen_range(0..4);
    let next_block = NextBlock { type_id, shape_id };
    // build app
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .init_resource::<ButtonMaterials>()
        .init_resource::<BackGroundMusicResources>()
        .add_state(AppState::Menu)
        .insert_resource(Screen(screen))
        .insert_resource(Block {
            exists: false,
            i: 6,
            j: 6,
            velocity_x: 1.0,
            velocity_y: 1.0,
            real_x: 6.0,
            real_y: 6.0,
            type_id: 1,
            shape_id: 1,
        })
        .insert_resource(next_block)
        .insert_resource(Speed(2.0))
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_startup_system(bgm_loop_system.system())
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu.system()))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::GameOver)
                .with_system(setup_over_menu.system()),
        )
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(over_menu.system()))
        .add_system_set(
            SystemSet::on_exit(AppState::GameOver)
                .with_system(cleanup_over_menu.system())
                .with_system(clear_game.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(block_spawn_system.system())
                .with_system(block_drop_speed_adjustment_system.system())
                .with_system(block_drop_system.system())
                .with_system(judge_system.system())
                .with_system(block_movement_system.system())
                .with_system(block_transformation_system.system())
                .with_system(render_system.system())
                .with_system(scoreboard_system.system())
                .with_system(speed_adjustment_system.system())
                .with_system(pause_system.system()),
        )
        .run();
}

struct Scoreboard {
    score: usize,
}

struct Screen([[u8; COLS]; ROWS]);

// the index of left top element
struct Block {
    exists: bool,
    i: usize,
    j: usize,
    velocity_x: f32,
    velocity_y: f32,
    real_x: f32,
    real_y: f32,
    type_id: usize,
    shape_id: usize,
}

struct NextBlock {
    type_id: usize,
    shape_id: usize,
}

struct Speed(f32);

struct ScreenSprite(usize, usize);

enum Pixel {
    Screen,
    SubScreen,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Menu,
    GameOver,
}

struct MenuData {
    button_entity: Entity,
}

fn bgm_loop_system(
    audio: Res<Audio>,
    bgm_audio: Res<BackGroundMusicResources> 
) {
   audio.play_looped_in_channel(bgm_audio.bgm_handle.clone(), &bgm_audio.channel);
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cemeras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // Add walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = ONEPIXEL;
    let bounds = Vec2::new(BOUND[0], BOUND[1]);

    // left
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(-bounds.x / 2.0 - wall_thickness / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness * 2.0)),
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(bounds.x / 2.0 + wall_thickness / 2.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness * 2.0)),
        ..Default::default()
    });
    // bottom
    commands.spawn_bundle(SpriteBundle {
        material: wall_material.clone(),
        transform: Transform::from_xyz(0.0, -bounds.y / 2.0 - wall_thickness / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness * 2.0, wall_thickness)),
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        material: wall_material,
        transform: Transform::from_xyz(0.0, bounds.y / 2.0 + wall_thickness / 2.0, 0.0),
        sprite: Sprite::new(Vec2::new(bounds.x + wall_thickness * 2.0, wall_thickness)),
        ..Default::default()
    });

    // Add subwindow
    let subwin_rows = 8;
    let subwin_cols = 6;
    let subwindow_material = materials.add(Color::rgb(1.0, 0.5, 0.5).into());
    let win_bounds = Vec2::new(
        ONEPIXEL * subwin_cols as f32 / 2.0,
        ONEPIXEL * subwin_rows as f32 / 2.0,
    );
    let win_thickness = ONEPIXEL / 2.0;
    let win_init_x = bounds.x / 2.0 + wall_thickness + 2.0 * ONEPIXEL / 2.0 + win_thickness / 2.0;
    let win_init_y = bounds.y / 2.0 + wall_thickness / 2.0 + (wall_thickness - win_thickness)
        - win_thickness / 2.0;

    // left
    commands.spawn_bundle(SpriteBundle {
        material: subwindow_material.clone(),
        transform: Transform::from_xyz(
            win_init_x,
            win_init_y - (win_bounds.y + win_thickness) / 2.0,
            0.0,
        ),
        sprite: Sprite::new(Vec2::new(win_thickness, win_bounds.y + 2.0 * win_thickness)),
        ..Default::default()
    });
    // right
    commands.spawn_bundle(SpriteBundle {
        material: subwindow_material.clone(),
        transform: Transform::from_xyz(
            win_init_x + win_bounds.x + win_thickness,
            win_init_y - (win_bounds.y + win_thickness) / 2.0,
            0.0,
        ),
        sprite: Sprite::new(Vec2::new(win_thickness, win_bounds.y + 2.0 * win_thickness)),
        ..Default::default()
    });
    // bottom
    commands.spawn_bundle(SpriteBundle {
        material: subwindow_material.clone(),
        transform: Transform::from_xyz(
            win_init_x + (win_bounds.x + win_thickness) / 2.0,
            win_init_y - win_bounds.y - win_thickness,
            0.0,
        ),
        sprite: Sprite::new(Vec2::new(win_bounds.x + win_thickness * 2.0, win_thickness)),
        ..Default::default()
    });
    // top
    commands.spawn_bundle(SpriteBundle {
        material: subwindow_material.clone(),
        transform: Transform::from_xyz(
            win_init_x + (win_bounds.x + win_thickness) / 2.0,
            win_init_y,
            0.0,
        ),
        sprite: Sprite::new(Vec2::new(win_bounds.x + win_thickness * 2.0, win_thickness)),
        ..Default::default()
    });

    // screen sprite
    for i in 4..ROWS - 4 {
        for j in 4..COLS - 4 {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    transform: Transform::from_xyz(
                        -bounds.x / 2.0 - wall_thickness / 2.0
                            + ONEPIXEL
                            + (j - 4) as f32 * ONEPIXEL,
                        bounds.y / 2.0 + wall_thickness / 2.0
                            - ONEPIXEL
                            - (i - 4) as f32 * ONEPIXEL,
                        1.0,
                    ),
                    sprite: Sprite::new(Vec2::new(ONEPIXEL, ONEPIXEL)),
                    ..Default::default()
                })
                .insert(Pixel::Screen)
                .insert(ScreenSprite(i, j));
        }
    }

    // subscreen sprite
    for i in 0..4 {
        for j in 0..4 {
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    transform: Transform::from_xyz(
                        win_init_x + ONEPIXEL / 2.0 * 2.0 + j as f32 * ONEPIXEL / 2.0,
                        win_init_y - ONEPIXEL / 2.0 * 3.0 - i as f32 * ONEPIXEL / 2.0,
                        2.0,
                    ),
                    sprite: Sprite::new(Vec2::new(ONEPIXEL / 2.0, ONEPIXEL / 2.0)),
                    ..Default::default()
                })
                .insert(Pixel::SubScreen)
                .insert(ScreenSprite(i, j));
        }
    }

    // fill subscreen
    for i in 0..subwin_rows {
        for j in 0..subwin_cols {
            commands.spawn_bundle(SpriteBundle {
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                transform: Transform::from_xyz(
                    win_init_x + ONEPIXEL / 2.0 + j as f32 * ONEPIXEL / 2.0,
                    win_init_y - ONEPIXEL / 2.0 - i as f32 * ONEPIXEL / 2.0,
                    1.0,
                ),
                sprite: Sprite::new(Vec2::new(ONEPIXEL, ONEPIXEL)),
                ..Default::default()
            });
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Play",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

fn block_spawn_system(mut block: ResMut<Block>, mut next_block: ResMut<NextBlock>) {
    if !block.exists {
        // change block id
        block.type_id = next_block.type_id;
        block.shape_id = next_block.shape_id;

        let mut rng = thread_rng();
        let type_id = rng.gen_range(0..7);
        let shape_id = rng.gen_range(0..4);
        next_block.type_id = type_id;
        next_block.shape_id = shape_id;

        // TODO: generate block.j according to the width of screen
        let block_matrix = BLOCKSLIB[block.type_id][block.shape_id];
        let mut first = 5;
        let mut second = 5;
        for i in (0..4).rev() {
            for j in 0..4 {
                if block_matrix[i][j] == 1 {
                    if second == 5 {
                        second = i;
                    } else {
                        first = i;
                    }
                    break;
                }
            }
        }
        if first == 5 {
            first = second;
        }
        let height = second + 1 - first;

        block.i = 3 + 1 - height;
        block.j = 3 + 4;
        block.velocity_x = 1.0;
        block.velocity_y = 1.0;
        block.real_x = block.j as f32;
        block.real_y = block.i as f32;

        block.exists = true;
    }
}

fn block_drop_speed_adjustment_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut block: ResMut<Block>,
) {
    if keyboard_input.pressed(KeyCode::Down) {
        let delta_seconds = time.delta_seconds();
        block.velocity_y *= 1.0 + 3.0 * delta_seconds;
    } else {
        block.velocity_y = 1.0;
    }
}

fn block_drop_system(
    time: Res<Time>,
    speed: Res<Speed>,
    mut screen: ResMut<Screen>,
    mut block: ResMut<Block>,
    mut state: ResMut<State<AppState>>,
) {
    let delta_seconds = time.delta_seconds();

    block.real_y += speed.0 * block.velocity_y * delta_seconds;
    let new_i = (block.real_y * 1.01) as usize;

    // check if the move successes
    let block_matrix = BLOCKSLIB[block.type_id][block.shape_id];
    let mut if_drop = true;
    for i in 0..4 {
        for j in 0..4 {
            if block_matrix[i][j] == 1 && (screen.0)[new_i + i][block.j + j] == 1 {
                if_drop = false;
                break;
            }
        }
        if !if_drop {
            break;
        }
    }

    // drop if it is possible
    if if_drop {
        block.i = new_i;
    } else {
        for i in 0..4 {
            for j in 0..4 {
                (screen.0)[block.i + i][block.j + j] += block_matrix[i][j];
            }
        }
        block.exists = false;

        // check if game is over
        for &content in (screen.0)[3][4..COLS - 4].iter() {
            if content != 0 {
                state.set(AppState::GameOver).unwrap();
                return;
            }
        }
    }
}

// It should be the first one after the drop system,
// since it needs to know if the block has been a part of the block mountain.
fn judge_system(mut scoreboard: ResMut<Scoreboard>, mut screen: ResMut<Screen>) {
    let mut eliminate_rows = vec![];
    let nrows = screen.0.len();
    let ncols = (screen.0)[0].len();
    for i in 4..nrows - 4 {
        let mut filled_flag = true;
        for j in 4..ncols - 4 {
            if (screen.0)[i][j] == 0 {
                filled_flag = false;
                break;
            }
        }
        if filled_flag {
            eliminate_rows.push(i);
        }
    }

    if !eliminate_rows.is_empty() {
        // eliminate filled rows
        let all_eliminate_rows_amount = eliminate_rows.len();
        let mut eliminate_rows_amount = eliminate_rows.len();
        let mut eliminated_amount = 0;
        for i in (4..nrows - 4).rev() {
            if (eliminate_rows_amount == 0)
                || (eliminate_rows_amount > 0 && i != eliminate_rows[eliminate_rows_amount - 1])
            {
                for j in 4..ncols - 4 {
                    (screen.0)[i + eliminated_amount][j] = (screen.0)[i][j];
                }
            } else {
                eliminate_rows_amount -= 1;
                eliminated_amount += 1;
            }
        }

        for i in 4..4 + all_eliminate_rows_amount {
            for j in 4..ncols - 4 {
                (screen.0)[i][j] = 0;
            }
        }

        scoreboard.score += all_eliminate_rows_amount * 5 + BOUNS[all_eliminate_rows_amount - 1];
    }
}

fn block_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    screen: Res<Screen>,
    mut block: ResMut<Block>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction = -1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction = 1.0;
    }

    let delta_seconds = time.delta_seconds();
    block.real_x += 12.0 * block.velocity_x * delta_seconds * direction;

    if block.real_x < 0.0 {
        block.real_x = 0.0;
    }

    if block.real_x >= COLS as f32 - 4.0 - 1.0 {
        block.real_x = COLS as f32 - 4.0 - 1.0 + 0.01;
    }

    let new_j = block.real_x as usize;

    // check if the move successes
    let block_matrix = BLOCKSLIB[block.type_id][block.shape_id];
    let mut if_move = true;
    for i in 0..4 {
        for j in 0..4 {
            if block_matrix[i][j] == 1 && (screen.0)[block.i + i][new_j + j] == 1 {
                if_move = false;
                break;
            }
        }
        if !if_move {
            break;
        }
    }

    // move if it is possible
    if if_move {
        block.j = new_j;
    }
}

fn block_transformation_system(
    keyboard_input: Res<Input<KeyCode>>,
    screen: Res<Screen>,
    mut block: ResMut<Block>,
) {
    if keyboard_input.just_released(KeyCode::Up) {
        // check if the transformation successes
        let block_matrix = BLOCKSLIB[block.type_id][(block.shape_id + 1) % 4];

        let mut if_transform = true;
        for i in 0..4 {
            for j in 0..4 {
                if block_matrix[i][j] == 1 && (screen.0)[block.i + i][block.j + j] == 1 {
                    if_transform = false;
                    break;
                }
            }
            if !if_transform {
                break;
            }
        }

        // move if it is possible
        if if_transform {
            block.shape_id = (block.shape_id + 1) % 4;
        }
    }
}

fn render_system(
    screen: Res<Screen>,
    block: Res<Block>,
    next_block: Res<NextBlock>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&ScreenSprite, &Pixel, &mut Handle<ColorMaterial>)>,
) {
    let have_material = materials.add(asset_server.load("block/block.png").into());
    let no_material = materials.add(Color::rgb(1.0, 1.0, 1.0).into());
    let block_matrix = BLOCKSLIB[block.type_id][block.shape_id];
    // render screen and subscreen
    for (screen_sprite, pixel_type, mut material) in query.iter_mut() {
        match pixel_type {
            Pixel::SubScreen => {
                let i = screen_sprite.0;
                let j = screen_sprite.1;
                let next_block_matrix = BLOCKSLIB[next_block.type_id][next_block.shape_id];
                if next_block_matrix[i][j] == 0 {
                    *material = no_material.clone();
                } else {
                    *material = have_material.clone();
                }
            }
            Pixel::Screen => {
                let i = screen_sprite.0;
                let j = screen_sprite.1;
                if (screen.0)[i][j] == 0 {
                    *material = no_material.clone();
                } else {
                    *material = have_material.clone();
                }

                if i >= block.i && i <= block.i + 3 && j >= block.j && j <= block.j + 3 {
                    let b_i = i - block.i;
                    let b_j = j - block.j;
                    if block_matrix[b_i][b_j] == 1 {
                        *material = have_material.clone();
                    }
                }
            }
        }
    }
}

fn pause_system(mut state: ResMut<State<AppState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();
    }
}

fn scoreboard_system(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut().unwrap();
    text.sections[1].value = scoreboard.score.to_string();
}

fn speed_adjustment_system(scoreboard: Res<Scoreboard>, mut speed: ResMut<Speed>) {
    speed.0 += (scoreboard.score / 200) as f32;
}

struct OverMenuData {
    button_entity: Entity,
}

fn setup_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Reply",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(OverMenuData { button_entity });
}

fn over_menu(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn cleanup_over_menu(mut commands: Commands, over_menu_data: Res<OverMenuData>) {
    commands
        .entity(over_menu_data.button_entity)
        .despawn_recursive();
}

fn clear_game(
    mut scoreboard: ResMut<Scoreboard>,
    mut screen: ResMut<Screen>,
    mut block: ResMut<Block>,
    mut next_block: ResMut<NextBlock>,
) {
    // clear scoreboard
    scoreboard.score = 0;

    // clear screen
    for i in 0..ROWS - 4 {
        for j in 4..COLS - 4 {
            (screen.0)[i][j] = 0;
        }
    }

    // clear block
    block.exists = false;

    // prepare next block
    let mut rng = thread_rng();
    let type_id = rng.gen_range(0..7);
    let shape_id = rng.gen_range(0..4);
    next_block.type_id = type_id;
    next_block.shape_id = shape_id;
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

struct BackGroundMusicResources {
    bgm_handle: Handle<AudioSource>,
    channel: AudioChannel,
}

impl FromWorld for BackGroundMusicResources {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        let audio = asset_server.load("bgm/default.mp3");
        Self { bgm_handle: audio, channel: AudioChannel::new("bgm".to_owned()) }
    }
}