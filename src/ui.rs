use std::collections::HashMap;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::simulation::PhysicalProperties;

#[derive(Component)]
pub struct RenderInUI(pub String);

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PositionText;

#[derive(Component)]
struct VelocityText;

#[derive(Component)]
struct RootNode;

/// Plugin used to create and update UI components.
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(setup_ui);
        //app.add_system(update_fps);
        app.add_system(update_positions_of_simulated_components);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                margin: Rect {
                    left: Val::Px(20.),
                    right: Val::Px(20.),
                    top: Val::Px(20.),
                    bottom: Val::Px(20.),
                },
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexStart,
                        ..default()
                    },
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: "FPS: ".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::GOLD,
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                })
                .insert(FpsText);
        });
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}

fn update_positions_of_simulated_components(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gravity_query: Query<(&PhysicalProperties, &RenderInUI)>,
    mut ptext_query: Query<&mut Text, (With<PositionText>, Without<VelocityText>)>,
    mut vtext_query: Query<&mut Text, (With<VelocityText>, Without<PositionText>)>,
    mut root_query: Query<Entity, With<RootNode>>,
) {
    let mut entities: HashMap<String, &PhysicalProperties> = HashMap::new();

    for (p_props, simulated) in gravity_query.iter() {
        entities.insert(simulated.0.clone(), &p_props);
    }

    for mut text in ptext_query.iter_mut() {
        if entities.contains_key(&text.sections[0].value) {
            let p_props = entities[&text.sections[0].value];
            text.sections[1].value = format!("{:}", p_props.translation.x);
            text.sections[2].value = format!("{:}", p_props.translation.y);
            text.sections[3].value = format!("{:}", p_props.translation.z);
        }
    }

    for mut text in vtext_query.iter_mut() {
        if entities.contains_key(&text.sections[0].value) {
            let p_props = entities[&text.sections[0].value];
            text.sections[1].value = format!("{:}", p_props.acceleration.x);
            text.sections[2].value = format!("{:}", p_props.acceleration.y);
            text.sections[3].value = format!("{:}", p_props.acceleration.z);

            entities.remove(&text.sections[0].value);
        }
    }

    let root_node = match root_query.get_single_mut() {
        Ok(root_node) => root_node,
        Err(_) => return,
    };

    for (simulated, _transform) in entities.iter() {
        commands.entity(root_node).with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexStart,
                        ..default()
                    },
                    // Use `Text` directly
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: format!("{}", simulated),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "-".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::RED,
                                },
                            },
                            TextSection {
                                value: "-".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::GREEN,
                                },
                            },
                            TextSection {
                                value: "-".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::BLUE,
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                })
                .insert(PositionText);
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexStart,
                        ..default()
                    },
                    // Use `Text` directly
                    text: Text {
                        // Construct a `Vec` of `TextSection`s
                        sections: vec![
                            TextSection {
                                value: format!("{}", simulated),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            },
                            TextSection {
                                value: "-".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::RED,
                                },
                            },
                            TextSection {
                                value: "-".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::GREEN,
                                },
                            },
                            TextSection {
                                value: "-".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/AstroSpace.ttf"),
                                    font_size: 24.0,
                                    color: Color::BLUE,
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                })
                .insert(VelocityText);
        });
    }
}
