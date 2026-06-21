use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowLevel},
};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_MARGIN: f32 = 20.;

const RECT_WIDTH: f32 = 40.;
const RECT_HEIGHT: f32 = 20.;
const RECT_GAP: f32 = 10.;

const BAR_WIDTH: f32 = 120.;
const BAR_HEIGHT: f32 = 10.;

const RECTS_COLUMNS: usize = 14;
const RECTS_ROWS: usize = 9;

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
        .add_systems(Startup, (setup, setup_rects, setup_bar))
        .add_systems(Update, bar_position)
        .run();
}

#[derive(Component)]
struct Rect;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_rects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut matriels: ResMut<Assets<ColorMaterial>>,
) {
    let rect_mesh = meshes.add(Rectangle::new(RECT_WIDTH, RECT_HEIGHT));
    let rect_color = Color::srgba(1., 1., 1., 1.);
    let rect_y = WINDOW_HEIGHT as f32 / 2. - RECT_HEIGHT / 2. - WINDOW_MARGIN;

    for i in 1..=(RECTS_COLUMNS / 2) {
        for j in 0..RECTS_ROWS {
            commands.spawn((
                Mesh2d(rect_mesh.clone()),
                MeshMaterial2d(matriels.add(rect_color)),
                Transform::from_xyz(
                    (RECT_WIDTH + RECT_GAP) * i as f32,
                    rect_y - (RECT_HEIGHT + RECT_GAP) * j as f32,
                    0.,
                ),
                Rect,
            ));
            commands.spawn((
                Mesh2d(rect_mesh.clone()),
                MeshMaterial2d(matriels.add(rect_color)),
                Transform::from_xyz(
                    -(RECT_WIDTH + RECT_GAP) * i as f32,
                    rect_y - (RECT_HEIGHT + RECT_GAP) * j as f32,
                    0.,
                ),
                Rect,
            ));
        }
    }
}

#[derive(Component)]
struct Bar;

fn setup_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut matriels: ResMut<Assets<ColorMaterial>>,
) {
    let rect_mesh = meshes.add(Rectangle::new(BAR_WIDTH, BAR_HEIGHT));
    let rect_color = Color::srgba(1., 1., 1., 1.);

    commands.spawn((
        Mesh2d(rect_mesh.clone()),
        MeshMaterial2d(matriels.add(rect_color)),
        Transform::from_xyz(
            0.,
            -(WINDOW_HEIGHT as f32) / 2. + WINDOW_MARGIN + RECT_GAP,
            0.,
        ),
        Bar,
    ));
}

fn bar_position(
    window: Single<&Window, With<PrimaryWindow>>,
    mut bar: Single<&mut Transform, With<Bar>>,
) {
    if let Some(position) = window.cursor_position() {
        const HWW: f32 = (WINDOW_WIDTH / 2) as f32;
        let x = position.x - HWW as f32;
        bar.translation.x = x.clamp(-HWW, HWW);
    }
}
