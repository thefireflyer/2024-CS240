///////////////////////////////////////////////////////////////////////////////

use std::{time::Duration, vec};

use bevy::prelude::*;
use hanoi_tower_solver::hanoi_general_rec;

use crate::{
    camera::PanOrbitCamera,
    components::{Disk, Disks, Tower},
    events::ConfigEvent,
    resources::TowerConfig,
};

///////////////////////////////////////////////////////////////////////////////

const TOWER_INC: &str = "TowerInc";
const TOWER_DEC: &str = "TowerDec";
const TOWER_DISPLAY: &str = "TowerDisplay";
const DISK_INC: &str = "DiskInc";
const DISK_DEC: &str = "DiskDec";
const DISK_DISPLAY: &str = "DiskDisplay";
const SPEED_INC: &str = "SpeedInc";
const SPEED_DEC: &str = "SpeedDec";
const SPEED_DISPLAY: &str = "SpeedDisplay";

const START_STOP: &str = "StartStop";

///////////////////////////////////////////////////////////////////////////////

pub fn setup(mut commands: Commands, mut ev_config: EventWriter<ConfigEvent>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius: 5.0,
            ..Default::default()
        },
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..default()
        },
        ..default()
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_content: bevy::ui::AlignContent::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(100.),
                        width: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.)),
                        align_self: bevy::ui::AlignSelf::End,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Px(100.),
                                align_self: bevy::ui::AlignSelf::End,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                display: Display::Grid,
                                grid_auto_flow: GridAutoFlow::Column,
                                column_gap: Val::Px(10.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "3 towers",
                                    TextStyle {
                                        font: default(),
                                        font_size: 30.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                Name::new(TOWER_DISPLAY),
                            ));
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: bevy::prelude::BackgroundColor(
                                            Color::rgb(0.1, 0.1, 0.1),
                                        ),
                                        ..default()
                                    },
                                    Name::new(TOWER_INC),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new(""),
                                        TextBundle::from_section(
                                            "+",
                                            TextStyle {
                                                font: default(),
                                                font_size: 50.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: bevy::prelude::BackgroundColor(
                                            Color::rgb(0.1, 0.1, 0.1),
                                        ),
                                        ..default()
                                    },
                                    Name::new(TOWER_DEC),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new(""),
                                        TextBundle::from_section(
                                            "-",
                                            TextStyle {
                                                font: default(),
                                                font_size: 50.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                    ));
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Px(100.),
                                align_self: bevy::ui::AlignSelf::End,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                display: Display::Grid,
                                grid_auto_flow: GridAutoFlow::Column,
                                column_gap: Val::Px(10.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "3 disks",
                                    TextStyle {
                                        font: default(),
                                        font_size: 30.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                Name::new(DISK_DISPLAY),
                            ));
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: bevy::prelude::BackgroundColor(
                                            Color::rgb(0.1, 0.1, 0.1),
                                        ),
                                        ..default()
                                    },
                                    Name::new(DISK_INC),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new(""),
                                        TextBundle::from_section(
                                            "+",
                                            TextStyle {
                                                font: default(),
                                                font_size: 50.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: bevy::prelude::BackgroundColor(
                                            Color::rgb(0.1, 0.1, 0.1),
                                        ),
                                        ..default()
                                    },
                                    Name::new(DISK_DEC),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new(""),
                                        TextBundle::from_section(
                                            "-",
                                            TextStyle {
                                                font: default(),
                                                font_size: 50.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                    ));
                                });
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Px(100.),
                                align_self: bevy::ui::AlignSelf::End,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                display: Display::Grid,
                                grid_auto_flow: GridAutoFlow::Column,
                                column_gap: Val::Px(10.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "2.0s",
                                    TextStyle {
                                        font: default(),
                                        font_size: 30.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                Name::new(SPEED_DISPLAY),
                            ));
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: bevy::prelude::BackgroundColor(
                                            Color::rgb(0.1, 0.1, 0.1),
                                        ),
                                        ..default()
                                    },
                                    Name::new(SPEED_INC),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new(""),
                                        TextBundle::from_section(
                                            "+",
                                            TextStyle {
                                                font: default(),
                                                font_size: 50.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(60.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: bevy::prelude::BackgroundColor(
                                            Color::rgb(0.1, 0.1, 0.1),
                                        ),
                                        ..default()
                                    },
                                    Name::new(SPEED_DEC),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Name::new(""),
                                        TextBundle::from_section(
                                            "-",
                                            TextStyle {
                                                font: default(),
                                                font_size: 50.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                        ),
                                    ));
                                });
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(120.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: bevy::prelude::BackgroundColor(Color::rgb(
                                    0.1, 0.1, 0.1,
                                )),
                                ..default()
                            },
                            Name::new(START_STOP),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new(""),
                                TextBundle::from_section(
                                    "start",
                                    TextStyle {
                                        font: default(),
                                        font_size: 30.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                            ));
                        });
                });
        });

    commands.spawn((Disks, PbrBundle { ..default() }));

    ev_config.send(ConfigEvent {
        pegs: true,
        disks: true,
        speed: true,
        state: true,
    });
}

///////////////////////////////////////////////////////////////////////////////

pub fn handle_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &Name),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<(&mut Text, &Name)>,
    mut tower_state: ResMut<TowerConfig>,
    mut config_ev: EventWriter<ConfigEvent>,
) {
    for (interaction, mut color, children, name) in &mut interaction_query {
        let (mut text, _) = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                *color = Color::rgb(0.0, 0.0, 0.0).into();

                ///////////////////////////////////////////////////////////////

                let mut event = ConfigEvent::default();

                match name.as_str() {
                    START_STOP => {
                        if text.sections[0].value == "start" {
                            text.sections[0].value = "stop".to_owned();
                            tower_state.running = true;
                            event.state = true;
                            tower_state.timer.unpause();
                        } else if text.sections[0].value == "stop" {
                            text.sections[0].value = "start".to_owned();
                            tower_state.running = false;
                            tower_state.timer.pause();
                            event.state = false;
                        }
                    }
                    TOWER_INC => {
                        tower_state.number_of_towers += 1;
                        event.pegs = true;
                    }
                    TOWER_DEC if tower_state.number_of_towers > 3 => {
                        tower_state.number_of_towers -= 1;
                        event.pegs = true;
                    }
                    DISK_INC => {
                        tower_state.number_of_disks += 1;
                        event.disks = true;
                    }
                    DISK_DEC if tower_state.number_of_disks > 1 => {
                        tower_state.number_of_disks -= 1;
                        event.disks = true;
                    }
                    SPEED_INC => {
                        tower_state.speed += 0.5;
                        let speed = tower_state.speed;
                        tower_state
                            .timer
                            .set_duration(Duration::from_secs_f32(speed));
                        update_labels(&mut tower_state, &mut text_query);
                        // return;
                        event.speed = true;
                    }
                    SPEED_DEC if tower_state.speed > 0.0 => {
                        tower_state.speed -= 0.5;
                        let speed = tower_state.speed;
                        tower_state
                            .timer
                            .set_duration(Duration::from_secs_f32(speed));
                        update_labels(&mut tower_state, &mut text_query);
                        // return;
                        event.speed = true;
                    }
                    _ => {}
                }

                config_ev.send(event);
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.3, 0.3, 0.3).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.2, 0.2, 0.2).into();
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn on_config_event(
    mut ev_config: EventReader<ConfigEvent>,
    mut commands: Commands,
    mut tower_state: ResMut<TowerConfig>,
    mut tower_query: Query<(Entity, &Tower)>,
    mut disks_query: Query<(Entity, &Disks)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<(&mut PanOrbitCamera, &mut Transform)>,
    mut text_query: Query<(&mut Text, &Name)>,
) {
    for ev in ev_config.read() {
        if ev.disks || ev.pegs || tower_state.moves.is_empty() {
            build_tower_config(
                &mut commands,
                &mut tower_state,
                &mut tower_query,
                &mut disks_query,
                &mut meshes,
                &mut materials,
                &mut camera_query,
            );

            tower_state.moves = run_solver(
                tower_state.number_of_towers.try_into().unwrap(),
                tower_state.number_of_disks.into(),
            );
        }
        update_labels(&mut tower_state, &mut text_query);
    }
}

///////////////////////////////////////////////////////////////////////////////

fn update_labels(
    tower_state: &mut ResMut<TowerConfig>,
    text_query: &mut Query<(&mut Text, &Name)>,
) {
    for (mut text, name) in text_query {
        if name == &Name::new(TOWER_DISPLAY) {
            text.sections[0] = TextSection::new(
                tower_state.number_of_towers.to_string() + " towers",
                TextStyle {
                    font: default(),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            );
        }
        if name == &Name::new(DISK_DISPLAY) {
            text.sections[0] = TextSection::new(
                tower_state.number_of_disks.to_string() + " disks",
                TextStyle {
                    font: default(),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            );
        }
        if name == &Name::new(SPEED_DISPLAY) {
            text.sections[0] = TextSection::new(
                tower_state.speed.to_string() + "s",
                TextStyle {
                    font: default(),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            );
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

fn build_tower_config(
    commands: &mut Commands,
    tower_state: &mut ResMut<TowerConfig>,
    tower_query: &mut Query<(Entity, &Tower)>,
    disks_query: &mut Query<(Entity, &Disks)>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    camera_query: &mut Query<(&mut PanOrbitCamera, &mut Transform)>,
) {
    let (entity, _) = disks_query.iter_mut().next().unwrap();
    for (entity, _) in tower_query.iter() {
        commands.entity(entity).despawn();
    }
    for i in 0..tower_state.number_of_towers {
        commands.spawn((
            Tower,
            PbrBundle {
                mesh: meshes.add(bevy::math::primitives::Cylinder {
                    radius: 0.3,
                    half_height: f32::from(tower_state.number_of_disks) * 0.3,
                    ..default()
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::GRAY,
                    ..default()
                }),
                transform: Transform::from_translation(Vec3 {
                    x: f32::from(i) * 3.0,
                    y: f32::from(tower_state.number_of_disks) * 0.6 * 0.5,
                    z: 0.0,
                }),
                ..default()
            },
        ));
    }

    ///////////////////////////////////////////////////////////////

    commands.entity(entity).despawn_recursive();

    println!();

    commands
        .spawn((Disks, PbrBundle { ..default() }))
        .with_children(|parent| {
            for i in 0..tower_state.number_of_disks {
                parent.spawn((
                    Disk,
                    Name::new("disk".to_string() + &i.to_string()),
                    PbrBundle {
                        mesh: meshes.add(bevy::math::primitives::Torus {
                            minor_radius: 0.3,
                            major_radius: 0.6,
                            ..default()
                        }),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(
                                1.0,
                                lerp(
                                    0.0,
                                    1.0,
                                    f32::from(tower_state.number_of_disks - i)
                                        / f32::from(tower_state.number_of_disks),
                                ),
                                lerp(
                                    0.0,
                                    1.0,
                                    f32::from(tower_state.number_of_disks - i)
                                        / f32::from(tower_state.number_of_disks),
                                ),
                            ),
                            ..default()
                        }),
                        transform: Transform::from_translation(Vec3 {
                            x: 0.0,
                            y: f32::from(tower_state.number_of_disks - i - 1) * 0.6,
                            z: 0.0,
                        }),
                        ..default()
                    },
                ));
            }
        });

    let center = Vec3::new(
        (f32::from(tower_state.number_of_towers - 1) * 3.0) * 0.5,
        (f32::from(tower_state.number_of_disks - 1) * 0.6) * 0.5,
        0.0,
    );

    let (mut cam, mut transform) = camera_query.iter_mut().next().unwrap();
    cam.focus = center;
    cam.radius = (f32::from(tower_state.number_of_towers - 1) * 3.0)
        + (f32::from(tower_state.number_of_disks) * 1.0);

    let rot_matrix = Mat3::from_quat(transform.rotation);
    transform.translation = cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, cam.radius));

    if tower_state.running {
        tower_state.moves = run_solver(
            tower_state.number_of_towers.try_into().unwrap(),
            tower_state.number_of_disks.into(),
        );
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn animate_disks(
    mut tower_state: ResMut<TowerConfig>,
    mut disk_query: Query<(Entity, &mut Disk, &Name, &mut Transform)>,
    time: Res<Time>,
) {
    if tower_state.running && !tower_state.moves.is_empty() {
        println!("animating");

        let (from, to, disk) = tower_state.moves[0];

        let (_, _, _, mut transform) = disk_query
            .iter_mut()
            .filter(|e| e.2 == &Name::new("disk".to_owned() + &disk.to_string()))
            .next()
            .unwrap();

        println!("moving disk {} from {} to {}", disk, from, to);

        let progress = tower_state.timer.fraction();

        println!("{}% through step", progress);

        let to_x_pos = f32::from(u16::try_from(to).unwrap()) * 3.0;
        let from_x_pos = f32::from(u16::try_from(from).unwrap()) * 3.0;
        let disk_height =
            f32::from(tower_state.number_of_disks - u16::try_from(disk).unwrap() - 1) * 0.6;
        let tower_height = f32::from(tower_state.number_of_disks) * 0.6 + 0.45;

        let height_diff = tower_height - disk_height;
        let tower_diff = f32::abs(to_x_pos - from_x_pos);

        let total_diff = tower_diff + 2.0 * height_diff;

        let height_time_frac = height_diff / total_diff;
        let tower_time_frac = (height_diff + tower_diff) / total_diff;

        println!(
            "height {}% | tower {}% | total distance {}",
            height_time_frac, tower_time_frac, total_diff
        );

        if progress <= height_time_frac {
            let _ = transform.set(Box::new(Transform::from_translation(Vec3 {
                x: from_x_pos,
                y: cubic_interp(disk_height, tower_height, progress / height_time_frac),
                z: 0.0,
            })));
        } else if progress <= tower_time_frac {
            let _ = transform.set(Box::new(Transform::from_translation(Vec3 {
                x: cubic_interp(from_x_pos, to_x_pos, progress / tower_time_frac),
                y: tower_height,
                z: 0.0,
            })));
        } else {
            let _ = transform.set(Box::new(Transform::from_translation(Vec3 {
                x: to_x_pos,
                y: cubic_interp(tower_height, disk_height, progress / 1.0),
                z: 0.0,
            })));
        }

        println!();
    }

    if tower_state.timer.tick(time.delta()).just_finished()
        && tower_state.running
        && !tower_state.moves.is_empty()
    {
        println!("next");
        tower_state.moves.remove(0);
    }
}

///////////////////////////////////////////////////////////////////////////////

fn run_solver(i: usize, j: u32) -> Vec<(usize, usize, u32)> {
    println!("{} pegs with {} disks", i, j);
    let f: Vec<u32> = (0..j).rev().collect();
    let mut rods = vec![];
    for _ in 0..i {
        rods.push(vec![]);
    }
    rods[0] = f.clone();
    let mut moves = vec![];
    hanoi_general_rec(
        j.try_into().unwrap(),
        0,
        &mut rods,
        0,
        i - 1,
        (1..i - 1).collect(),
        &mut moves,
    );
    println!("{:?}", moves);
    println!();
    moves
}

///////////////////////////////////////////////////////////////////////////////

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    // [source] wikipedia
    let t = f32::min(f32::max(t, 0.0), f32::MAX);
    (1.0 - t) * v0 + t * v1
}

//---------------------------------------------------------------------------//

fn cubic_interp(v0: f32, v1: f32, t: f32) -> f32 {
    // [source] https://www.youtube.com/watch?v=aVwxzDHniEw
    let p0 = v0;
    let p1 = lerp(v0, v1, 0.10);
    let p2 = lerp(v0, v1, 0.90);
    let p3 = v1;

    let a = lerp(p0, p1, t);
    let b = lerp(p1, p2, t);
    let c = lerp(p2, p3, t);

    let d = lerp(a, b, t);
    let e = lerp(b, c, t);

    lerp(d, e, t)
}

///////////////////////////////////////////////////////////////////////////////
