///////////////////////////////////////////////////////////////////////////////

use std::vec;

use bevy::{
    app::{App, FixedUpdate, Startup, Update},
    time::{Timer, TimerMode},
    DefaultPlugins,
};
use hanoi_tower_solver::{debug, hanoi_simple_iter, hanoi_simple_rec};

use crate::{
    camera::pan_orbit_camera,
    events::ConfigEvent,
    resources::TowerConfig,
    systems::{animate_disks, handle_buttons, on_config_event},
};

///////////////////////////////////////////////////////////////////////////////

mod camera;
mod components;
mod events;
mod resources;
mod systems;

///////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut o = vec![4, 3, 2, 1];
    let mut a = vec![];
    let mut t = vec![];
    hanoi_simple_rec(4, &mut o, &mut a, &mut t, 0, &|o, a, t, d| {
        debug(o, a, t, d)
    });

    println!();

    let mut o = vec![4, 3, 2, 1];
    let mut a = vec![];
    let mut t = vec![];
    hanoi_simple_iter(&mut o, &mut a, &mut t);

    println!();
    App::new()
        .insert_resource(TowerConfig {
            number_of_disks: 3,
            number_of_towers: 3,
            running: false,
            speed: 2.0,
            moves: vec![],
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        })
        .add_event::<ConfigEvent>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (pan_orbit_camera, handle_buttons, on_config_event))
        .add_systems(FixedUpdate, animate_disks)
        .run();
}

///////////////////////////////////////////////////////////////////////////////
