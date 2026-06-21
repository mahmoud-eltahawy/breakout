use bevy::{prelude::*, window::WindowLevel};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_MARGIN: f32 = 20.;

const RECT_WIDTH: f32 = 40.;
const RECT_HEIGHT: f32 = 20.;
const RECT_GAP: f32 = 10.;

const RECTS_ROWS: usize = 8;
const RECTS_COLUMNS: usize = 12;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("breakout".to_string()),
                title: "breakout".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                window_level: WindowLevel::AlwaysOnTop,
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Rect;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut matriels: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let rect_mesh = meshes.add(Rectangle::new(RECT_WIDTH, RECT_HEIGHT));
    let rect_color = Color::srgba(1., 1., 1., 1.);
    let rect_x = 0.; //WINDOW_WIDTH as f32 / 2. - RECT_WIDTH / 2.;
    let rect_y = WINDOW_HEIGHT as f32 / 2. - RECT_HEIGHT / 2. - WINDOW_MARGIN;

    for row in 0..RECTS_ROWS {
        for column in 0..RECTS_COLUMNS {
            commands.spawn((
                Mesh2d(rect_mesh.clone()),
                MeshMaterial2d(matriels.add(rect_color)),
                Transform::from_xyz(
                    rect_x - (RECT_WIDTH + RECT_GAP) * row as f32,
                    rect_y - (RECT_HEIGHT + RECT_GAP) * column as f32,
                    0.,
                ),
                Rect,
            ));
            commands.spawn((
                Mesh2d(rect_mesh.clone()),
                MeshMaterial2d(matriels.add(rect_color)),
                Transform::from_xyz(
                    rect_x + (RECT_WIDTH + RECT_GAP) * row as f32,
                    rect_y - (RECT_HEIGHT + RECT_GAP) * column as f32,
                    0.,
                ),
                Rect,
            ));
        }
    }
}
