use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct MenuNode;

#[derive(Clone, Component, Debug)]
struct NewGameButton;

pub fn cleanup_main_menu(mut commands: Commands, entity_query: Query<Entity, With<MenuNode>>) {
    commands.entity(entity_query.single()).despawn_recursive();
}

pub fn setup_main_menu(
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
