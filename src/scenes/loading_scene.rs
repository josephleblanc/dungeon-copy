use bevy::prelude::*;

use crate::config::*;
use crate::materials::ingame::InGameMaterials;
use crate::scenes::SceneState;

use crate::materials::font::FontMaterials;
use crate::materials::heroes::HeroesMaterials;
use crate::materials::icon::IconMaterials;
use crate::materials::menu_box::MenuBoxMaterials;
use crate::materials::scenes::ScenesMaterials;
use crate::resources::dictionary::Dictionary;
use crate::resources::language::Language;

const LOADING_TEXT_FONT_SIZE: f32 = 30.0;
const TEXT_FONT_SIZE: f32 = 40.0;

const LOADING_BORDER_WIDTH: f32 = 600.0;
const LOADING_BORDER_HEIGHT: f32 = 60.0;

#[derive(Component)]
struct LoaderComponent {
    max_width: f32,
    current_width: f32,
}

#[derive(Resource)]
struct LoadingSceneData {
    user_interface_root: Entity,
}

pub struct LoadingScenePlugin;

impl Plugin for LoadingScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::LoadingScene), setup)
            .add_systems(
                Update,
                (load_materials, update_loader).run_if(in_state(SceneState::LoadingScene)),
            )
            .add_systems(OnExit(SceneState::LoadingScene), cleanup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, dictionary: Res<Dictionary>) {
    let user_interface_root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|parent| {
            loading_text(parent, &asset_server, &dictionary);
            loader_bundle(parent, &asset_server, &dictionary);
        })
        .id();

    commands.insert_resource(LoadingSceneData {
        user_interface_root,
    })
}

fn cleanup(mut commands: Commands, loading_scene_data: Res<LoadingSceneData>) {
    commands
        .entity(loading_scene_data.user_interface_root)
        .despawn_recursive();
}

fn loader_bundle(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn(
        // Border
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                width: Val::Px(LOADING_BORDER_WIDTH),
                height: Val::Px(LOADING_BORDER_HEIGHT),
                top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0)),
                bottom: Val::Auto,
                right: Val::Auto,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..Default::default()
        },
    )
    .with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    width: Val::Px(0.0),
                    height: Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
                    left: Val::Px(5.0),
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::rgb(
                    247.0 / 255.0,
                    104.0 / 255.0,
                    12.0 / 255.0,
                )),
                ..Default::default()
            })
            .with_children(|parent| {
                let font_str = match dictionary.get_current_language() {
                    Language::EN => ROBOTO_FONT,
                    // more languages here
                };

                parent.spawn(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load(font_str),
                            font_size: TEXT_FONT_SIZE,
                            color: Color::WHITE,
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..Default::default()
                });
            })
            .insert(LoaderComponent {
                max_width: LOADING_BORDER_WIDTH - 10.0,
                current_width: 0.0,
            });
    });
}

fn loading_text(
    root: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    dictionary: &Res<Dictionary>,
) {
    root.spawn(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            width: Val::Px(LOADING_BORDER_WIDTH),
            height: Val::Px(35.0),
            left: Val::Px((WINDOW_HEIGHT * RESOLUTION - LOADING_BORDER_WIDTH) / 2.0),
            top: Val::Px((WINDOW_HEIGHT - LOADING_BORDER_HEIGHT) / 2.0 - 37.0),
            bottom: Val::Auto,
            right: Val::Auto,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        let glossery = dictionary.get_glossary();

        let font_str = match dictionary.get_current_language() {
            Language::EN => ROBOTO_FONT,
            // more languages here
        };

        parent.spawn(TextBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },

            text: Text::from_section(
                glossery.loading_scene_text.loading,
                TextStyle {
                    font: asset_server.load(font_str),
                    font_size: LOADING_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..Default::default()
        });
    });
}

fn update_loader(
    mut query: Query<(&mut LoaderComponent, &mut Style, &Children)>,
    mut state: ResMut<NextState<SceneState>>,
    mut text_query: Query<&mut Text>,
) {
    for (mut loader, mut style, children) in query.iter_mut() {
        if loader.current_width < loader.max_width {
            loader.current_width += 2.5;
            style.width = Val::Px(loader.current_width);

            let value = (loader.current_width / loader.max_width * 100.0) as usize;
            if value >= 6 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = value.to_string() + "%";
            }
        } else {
            state.set(SceneState::MainMenuScene);
        }
    }
}

fn load_materials(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_materials: FontMaterials = FontMaterials {
        roboto_font: asset_server.load(ROBOTO_FONT),
    };

    let scenes_materials = ScenesMaterials {
        main_background_image: asset_server.load(MAIN_MENU_BACKGROUND_IMAGE),
        sub_background_image: asset_server.load(SUB_MENU_BACKGROUND_IMAGE),
        menu_box_materials: MenuBoxMaterials {
            top_right: asset_server.load("scenes/gui/menu_box_parchment/top_right.png"),
            top_center: asset_server.load("scenes/gui/menu_box_parchment/top_center.png"),
            top_left: asset_server.load("scenes/gui/menu_box_parchment/top_left.png"),
            mid_right: asset_server.load("scenes/gui/menu_box_parchment/mid_right.png"),
            mid_center: asset_server.load("scenes/gui/menu_box_parchment/mid_center.png"),
            mid_left: asset_server.load("scenes/gui/menu_box_parchment/mid_left.png"),
            bottom_right: asset_server.load("scenes/gui/menu_box_parchment/bottom_right.png"),
            bottom_center: asset_server.load("scenes/gui/menu_box_parchment/bottom_center.png"),
            bottom_left: asset_server.load("scenes/gui/menu_box_parchment/bottom_left.png"),
        },
        icon_materials: IconMaterials {
            home_icon_normal: asset_server.load("icons/home_icon_normal.png"),
            home_icon_hovered: asset_server.load("icons/home_icon_hovered.png"),
            home_icon_clicked: asset_server.load("icons/home_icon_clicked.png"),
        },
        heroes_materials: HeroesMaterials {
            male_fighter: asset_server.load("scenes/heroes/male_fighter.png"),
            male_wizard: asset_server.load("scenes/heroes/male_wizard.png"),
            female_fighter: asset_server.load("scenes/heroes/female_fighter.png"),
            female_wizard: asset_server.load("scenes/heroes/female_wizard.png"),
        },
    };

    let in_game_materials = InGameMaterials {
        heroes_materials: HeroesMaterials {
            male_fighter: asset_server.load("ingame/heroes/male_fighter.png"),
            male_wizard: asset_server.load("ingame/heroes/male_wizard.png"),
            female_fighter: asset_server.load("ingame/heroes/female_fighter.png"),
            female_wizard: asset_server.load("ingame/heroes/female_wizard.png"),
        },
    };

    commands.insert_resource(font_materials);
    commands.insert_resource(scenes_materials);
    commands.insert_resource(in_game_materials);
}
