use bevy::prelude::*;
use bevy_fog_of_war::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FogOfWarPlugin2D))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), FogOfWarRevealer2D::default()));

    commands.spawn((Camera2dBundle { ..default() }, MainCamera));
}
