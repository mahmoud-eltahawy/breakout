use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    window::{PrimaryWindow, WindowLevel},
};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_MARGIN: f32 = 20.;

const RECT_SIZE: Vec2 = Vec2::new(40., 20.);
const RECT_GAP: f32 = 10.;

const BAR_SIZE: Vec2 = Vec2::new(140., 10.);

const RECTS_COLUMNS: usize = 14;
const RECTS_ROWS: usize = 9;

const BALL_SIZE: f32 = 9.;
const BALL_INIT_POSITION: Vec2 = Vec2::new(0., 0.);

const BOTTOM_LEFT: Vec2 = Vec2 {
    x: -((WINDOW_WIDTH / 2) as f32),
    y: -(WINDOW_HEIGHT as f32) / 2.,
};
const TOP_LEFT: Vec2 = Vec2 {
    x: -((WINDOW_WIDTH / 2) as f32),
    y: (WINDOW_HEIGHT as f32) / 2.,
};
const TOP_RIGHT: Vec2 = Vec2 {
    x: (WINDOW_WIDTH / 2) as f32,
    y: (WINDOW_HEIGHT as f32) / 2.,
};
const BOTTOM_RIGHT: Vec2 = Vec2 {
    x: ((WINDOW_WIDTH / 2) as f32),
    y: -(WINDOW_HEIGHT as f32) / 2.,
};
const CORNERS: [Vec2; 4] = [BOTTOM_LEFT, TOP_LEFT, TOP_RIGHT, BOTTOM_RIGHT];

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
        .insert_resource(BallVelocity(Vec3 {
            x: 4.,
            y: 7.,
            z: 0.,
        }))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup, setup_rects, setup_bar, setup_ball))
        .add_systems(Update, (bar_controller, setup_walls, move_ball))
        .run();
}
#[derive(Resource)]
struct BallVelocity(Vec3);

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
    let rect_mesh = meshes.add(Rectangle::new(RECT_SIZE.x, RECT_SIZE.y));
    let rect_color = Color::srgba(1., 1., 1., 1.);
    let rect_y = WINDOW_HEIGHT as f32 / 2. - RECT_SIZE.y / 2. - WINDOW_MARGIN;

    for i in 1..=(RECTS_COLUMNS / 2) {
        for j in 0..RECTS_ROWS {
            commands.spawn((
                Mesh2d(rect_mesh.clone()),
                MeshMaterial2d(matriels.add(rect_color)),
                Transform::from_xyz(
                    (RECT_SIZE.x + RECT_GAP) * i as f32,
                    rect_y - (RECT_SIZE.y + RECT_GAP) * j as f32,
                    0.,
                ),
                Rect,
            ));
            commands.spawn((
                Mesh2d(rect_mesh.clone()),
                MeshMaterial2d(matriels.add(rect_color)),
                Transform::from_xyz(
                    -(RECT_SIZE.x + RECT_GAP) * i as f32,
                    rect_y - (RECT_SIZE.y + RECT_GAP) * j as f32,
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
    let rect_mesh = meshes.add(Rectangle::new(BAR_SIZE.x, BAR_SIZE.y));
    let rect_color = Color::srgba(1., 1., 1., 1.);

    commands.spawn((
        Mesh2d(rect_mesh.clone()),
        MeshMaterial2d(matriels.add(rect_color)),
        Transform::from_xyz(0., -(WINDOW_HEIGHT as f32) / 2. + WINDOW_MARGIN, 0.),
        Bar,
    ));
}

fn bar_controller(
    window: Single<&Window, With<PrimaryWindow>>,
    mut bar: Single<&mut Transform, With<Bar>>,
) {
    if let Some(position) = window.cursor_position() {
        const HWW: f32 = (WINDOW_WIDTH / 2) as f32;
        const HBW: f32 = BAR_SIZE.x / 2.;
        let x = position.x - HWW;
        bar.translation.x = x.clamp(-HWW + HBW, HWW - HBW);
    }
}

#[derive(Component)]
struct Ball;

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut matriels: ResMut<Assets<ColorMaterial>>,
) {
    let ball_mesh = meshes.add(Circle::new(BALL_SIZE));
    let ball_color = Color::srgba(0., 1., 1., 1.);

    commands.spawn((
        Mesh2d(ball_mesh.clone()),
        MeshMaterial2d(matriels.add(ball_color)),
        Transform::from_xyz(BALL_INIT_POSITION.x, BALL_INIT_POSITION.y, 0.),
        Ball,
    ));
}

fn setup_walls(mut gizmos: Gizmos) {
    for (begin, end) in CORNERS.windows(2).map(|xs| (xs[0], xs[1])) {
        gizmos.line_2d(begin, end, GREEN);
    }
}

fn move_ball(
    bar: Single<&Transform, (With<Bar>, Without<Ball>)>,
    mut ball: Single<&mut Transform, With<Ball>>,
    mut velocity: ResMut<BallVelocity>,
) {
    let mut new_position = ball.translation + velocity.0;
    if new_position.y >= (WINDOW_HEIGHT / 2) as f32 {
        velocity.0.y *= -1.;
        new_position = ball.translation + velocity.0;
    }
    if new_position.x >= (WINDOW_WIDTH / 2) as f32 || new_position.x <= -((WINDOW_WIDTH / 2) as f32)
    {
        velocity.0.x *= -1.;
        new_position = ball.translation + velocity.0;
    }

    if new_position.x >= bar.translation.x - BAR_SIZE.x
        && new_position.x <= bar.translation.x + BAR_SIZE.x
        && new_position.y <= bar.translation.y + BAR_SIZE.y
    {
        velocity.0.y *= -1.;
        new_position = ball.translation + velocity.0;
    }

    ball.translation = new_position;
}
