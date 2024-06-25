use bevy::prelude::*;

use crate::app_state::AppState;

#[derive(Clone, Component, Debug)]
struct MenuNode;

#[derive(Clone, Component, Debug)]
struct NewGameButton;

fn button_animation(
    mut button_query: Query<
        (&mut Interaction, &mut TextureAtlas),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut texture_atlas) in button_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                texture_atlas.index = 2;
            }
            Interaction::None => {
                texture_atlas.index = 0;
            }
            Interaction::Pressed => {
                texture_atlas.index = 1;
            }
        }
    }
}

fn new_game_clicked(
    mut next_app_state: ResMut<NextState<AppState>>,
    new_game_button_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
) {
    for interaction in new_game_button_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                next_app_state.set(AppState::Game);
            }
            _ => (),
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, entity_query: Query<Entity, With<MenuNode>>) {
    commands.entity(entity_query.single()).despawn_recursive();
}

fn setup_main_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let font_handle = asset_server.load("fonts/kenvector_future.ttf");
    let button_image_handle = asset_server.load("sprites/button.png");
    let button_texture_atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::new(190.0, 49.0), 1, 3, None, None);
    let button_texture_atlas_handle = texture_atlases.add(button_texture_atlas_layout);

    let slicer = TextureSlicer {
        border: BorderRect::square(10.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands
        .spawn((
            MenuNode,
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    height: Val::Vh(100.0),
                    justify_content: JustifyContent::Center,
                    width: Val::Vw(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Bevy Chess",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 40.0,
                            ..default()
                        },
                    ));
                });

            // Menu
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // New game
                    parent
                        .spawn((
                            ButtonBundle {
                                image: button_image_handle.clone().into(),
                                style: Style {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            // ImageScaleMode::Sliced(slicer.clone()),
                            NewGameButton,
                            TextureAtlas {
                                index: 0usize,
                                layout: button_texture_atlas_handle,
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                TextStyle {
                                    font: font_handle.clone(),
                                    font_size: 32.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ));
                        });

                    // TODO Credits
                });
        });
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu);
        app.add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
        app.add_systems(Update, (button_animation, new_game_clicked));
    }
}
