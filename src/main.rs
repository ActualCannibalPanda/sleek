use bevy::asset::AssetMetaCheck;
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
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Sprite, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut velocity: Vec3 = Vec3::ZERO;
        if input.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
        }
        transform.translation.x += velocity.x * 210.0 * time.delta_secs();
        transform.translation.y += velocity.y * 210.0 * time.delta_secs();
    }
}
