use bevy::asset::AssetMetaCheck;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: asset_server.load("ducky.png"),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn update(
    time: Res<Time>,
    mut input: EventReader<KeyboardInput>,
    mut query: Query<(&Sprite, &mut Transform)>,
) {
    let (_, mut transform) = query.single_mut().unwrap();
    let mut velocity: Vec3 = Vec3::ZERO;
    for event in input.read() {
        match event.key_code {
            KeyCode::KeyA => {
                velocity.x -= 100.0;
            }
            KeyCode::KeyD => {
                velocity.x += 100.0;
            }
            _ => {}
        }
    }
    if velocity != Vec3::ZERO {
        transform.translation += velocity * time.delta().as_secs_f32();
    }
}
