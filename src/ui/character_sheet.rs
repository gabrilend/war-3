use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(150.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(20.0),
                    top: Val::Px(15.0),
                    ..Default::default()
                },
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        });
    }

