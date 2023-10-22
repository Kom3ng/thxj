use bevy::prelude::*;
use bevy::window::WindowResized;

pub const BASE_WINDOW_HEIGHT:f32 = 720.0;
pub const BASE_WINDOW_WIDTH:f32 = 1280.0;

pub struct CommonPlugin;
impl Plugin for CommonPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PreviousWindowSizeReferenceValue>()
            .add_systems(Update,text_resize);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    Loading,
    Menu,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuAnimeState{
    EnterMenu,
    #[default]
    None,
}

#[derive(Component)]
pub struct Ui;

#[derive(Component)]
pub struct BaseNode;

#[derive(Resource)]
pub struct PreviousWindowSizeReferenceValue(f32);
impl Default for PreviousWindowSizeReferenceValue{
    fn default() -> Self {
        PreviousWindowSizeReferenceValue(BASE_WINDOW_HEIGHT)
    }
}

fn text_resize(
    mut window_resized_event: EventReader<WindowResized>,
    mut texts: Query<&mut Text>,
    mut previous_window_size_reference_value: ResMut<PreviousWindowSizeReferenceValue>,
){
    for e in &mut window_resized_event {
        let w = e.width;
        let h = e.height;

        let min = if w > h{
            h
        } else {
            w
        };

        for mut text in &mut texts {
            for section in &mut text.sections {
                section.style.font_size = min / previous_window_size_reference_value.0 * section.style.font_size;
            }
        }

        previous_window_size_reference_value.0 = min;
    }
}

pub mod res_path{
    pub const FONT:&'static str = "fonts/SanJiLuRongTi-ZhongCu-2.ttf";
    pub const START_AUDIO:&'static str = "audio/start.ogg";
    pub const INTRO_AUDIO:&'static str = "audio/intro.ogg";
}

pub mod strings{
    pub const APP_NAME:&'static str = "東方隙間傳";
    pub const ALICE_TITLE:&'static str = "上海アリス幻楽団";
    pub const ABSTRUCK_TITLE:&'static str = "Abstruck Studio";
}