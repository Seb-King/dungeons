use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    time::FixedTimestep,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Door;

#[derive(Component)]
struct Velocity {
    translation: Vec3,
    rotation: f32,
}

#[derive(Component)]
struct Collider;

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

const TIME_STEP: f32 = 1.0 / 60.0;

fn add_player(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        Velocity {
            translation: Vec3::ZERO,
            rotation: 0.0,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 1.0, 1.0),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0))
                .with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
    ));
}

fn add_door(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.1, 0.1),
                custom_size: Some(Vec2::new(100.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(00.0, 300.0, 0.0)),
            ..default()
        },
        Door,
    ));
}

fn add_dungeon_walls(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(20.0, 620.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            ..default()
        },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(20.0, 620.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..default()
        },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(620.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
            ..default()
        },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(620.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
            ..default()
        },
        Collider,
    ));
}

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_startup_system(add_dungeon_walls)
            .add_startup_system(add_door);
    }
}

fn move_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.translation;
        transform.rotate_z(velocity.rotation * delta);
    }
}

fn player_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity>) {
    let mut vel = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        vel += Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        vel += Vec3::NEG_Y;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        vel += Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        vel += Vec3::NEG_X;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        vel = Vec3::ZERO;
    }

    for mut velocity in &mut query {
        if vel == Vec3::ZERO {
            velocity.translation = Vec3::ZERO;
        } else {
            velocity.translation = Vec3::normalize(vel) * 10.0;
        }
    }
}

fn check_collisions(
    mut player_query: Query<(&mut Velocity, &Transform, &Sprite), With<Player>>,
    collider_query: Query<(&Transform, &Sprite), With<Collider>>,
) {
    let (mut player_velocity, player_transform, player_sprite) = player_query.single_mut();

    for (transform, sprite) in &collider_query {
        let collision = collide(
            transform.translation,
            sprite.custom_size.ok_or(0).unwrap(),
            player_transform.translation,
            player_sprite.custom_size.ok_or(0).unwrap(),
        );

        if let Some(collision) = collision {
            match collision {
                Collision::Left => {
                    if player_velocity.translation.x < 0.0 {
                        player_velocity.translation.x = 0.0;
                    }
                }
                Collision::Right => {
                    if player_velocity.translation.x > 0.0 {
                        player_velocity.translation.x = 0.0;
                    }
                }
                Collision::Top => {
                    if player_velocity.translation.y > 0.0 {
                        player_velocity.translation.y = 0.0;
                    }
                }
                Collision::Bottom => {
                    if player_velocity.translation.y < 0.0 {
                        player_velocity.translation.y = 0.0;
                    }
                }
                Collision::Inside => { /* do nothing */ }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_system(
                    check_collisions
                        .after(player_input_system)
                        .before(move_system),
                )
                .with_system(player_input_system.before(move_system))
                .with_system(move_system)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        )
        .run();
}
