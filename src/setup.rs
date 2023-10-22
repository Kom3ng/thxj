use std::f32::consts::PI;
use std::time::Duration;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use crate::common::*;

pub struct BootPlugin;

impl Plugin for BootPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<FadeTimer>()
            .add_state::<LoadingAnimeState>()
            .add_systems(Startup,setup)
            .add_systems(OnEnter(LoadingAnimeState::AliceLogo),(play_start_audio,reset_fade_timer.before(play_start_audio)))
            // .add_systems(OnEnter(LoadingAnimeState::AliceLogo),reset_fade_timer)
            .add_systems(OnEnter(LoadingAnimeState::AbsLogo),reset_fade_timer)
            .add_systems(OnEnter(LoadingAnimeState::Title),reset_fade_timer)
            .add_systems(OnEnter(LoadingAnimeState::Title),title_logo_anime_setup)
            // .add_systems(Update,text_resize)
            .add_systems(Update,alice_logo_anime.run_if(in_state(LoadingAnimeState::AliceLogo)))
            .add_systems(Update,abs_logo_anime.run_if(in_state(LoadingAnimeState::AbsLogo)))
            .add_systems(Update,title_anime.run_if(in_state(LoadingAnimeState::Title)));
    }
}

#[derive(Component)]
struct AliceLogo;
#[derive(Component)]
struct AbsLogo;
#[derive(Component)]
struct Title;
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum LoadingAnimeState{
    AliceLogo,
    #[default]
    None,
    AbsLogo,
    Title,
}

#[derive(Resource)]
struct FadeTimer(Timer);
impl Default for FadeTimer{
    fn default() -> Self {
        FadeTimer(Timer::from_seconds(3.,TimerMode::Once))
    }
}

fn reset_fade_timer(
    mut timer: ResMut<FadeTimer>,
){
    timer.0.reset();
}

fn play_start_audio(
    player: Res<Audio>,
    asset_server: Res<AssetServer>,
){
    player.play(asset_server.load(res_path::START_AUDIO));
}
fn alice_logo_anime(
    mut query: Query<&mut Text,With<AliceLogo>>,
    mut timer: ResMut<FadeTimer>,
    time: Res<Time>,
    mut state: ResMut<NextState<LoadingAnimeState>>
){
    for mut q in &mut query {
        q.sections[0].style.color.set_a((PI*timer.0.percent()).sin());
    }

    if timer.0.just_finished() {
        state.set(LoadingAnimeState::AbsLogo);
        return;
    }

    timer.0.tick(time.delta());
}
fn abs_logo_anime(
    mut query: Query<&mut Text,With<AbsLogo>>,
    mut timer: ResMut<FadeTimer>,
    time: Res<Time>,
    mut state: ResMut<NextState<LoadingAnimeState>>
){
    for mut q in &mut query {
        q.sections[0].style.color.set_a((PI*timer.0.percent()).sin());
    }

    if timer.0.just_finished() {
        state.set(LoadingAnimeState::Title);
        return;
    }

    timer.0.tick(time.delta());
}
fn title_anime(
    mut query: Query<&mut Text,With<Title>>,
    mut timer: ResMut<FadeTimer>,
    time: Res<Time>,
    mut state: ResMut<NextState<LoadingAnimeState>>
){
    for mut q in &mut query {
        q.sections[0].style.color.set_a((PI*timer.0.percent()).sin());
    }

    if timer.0.just_finished() {
        state.set(LoadingAnimeState::None);
        return;
    }

    timer.0.tick(time.delta());
}
fn title_logo_anime_setup(
    base_node: Query<Entity,With<BaseNode>>,
    alice_logo: Query<Entity,With<AliceLogo>>,
    abs_logo: Query<Entity,With<AbsLogo>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<FadeTimer>,
){
    for node in &base_node {
        for alice in &alice_logo {
            for abs in &abs_logo {
                commands.entity(alice).despawn_recursive();
                commands.entity(abs).despawn_recursive();
                commands.entity(node)
                    .with_children(|p| {
                        p.spawn((
                            TextBundle::from_section(
                                strings::APP_NAME,
                                TextStyle{
                                    font: asset_server.load(res_path::FONT),
                                    font_size: 100.0,
                                    color: Color::rgba(1.,1.,1.,0.),
                                }
                            ),
                            Title
                        ));
                    });
            }
        }
    }

    timer.0.set_duration(Duration::from_secs(4));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut anime_state: ResMut<NextState<LoadingAnimeState>>,
){
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        NodeBundle{
            style: Style{
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        },
        BaseNode
    )).with_children(|p| {
        p.spawn((
            TextBundle::from_section(
                strings::ALICE_TITLE,
                TextStyle{
                    font: asset_server.load(res_path::FONT),
                    font_size: 80.0,
                    color: Color::rgba(1.,1.,1.,0.),
                }
            ),
            AliceLogo
        ));

        p.spawn(
            (
                TextBundle::from_section(
                    strings::ABSTRUCK_TITLE,
                    TextStyle{
                        font: asset_server.load(res_path::FONT),
                        font_size: 80.0,
                        color: Color::rgba(1.,1.,1.,0.),
                    }
                ),
                AbsLogo
            )
        );
    });

    anime_state.set(LoadingAnimeState::AliceLogo);
}



















// use bevy::asset::{Asset, AssetServer, LoadState};
// use bevy::prelude::*;
// use bevy::window::WindowResized;
// use crate::prelude::*;
//
// pub struct BootPlugin;
//
// impl Plugin for BootPlugin{
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup,setup)
//             .add_state::<AppState>()
//             .init_resource::<FlickerTimer>()
//             .init_resource::<FlickerCounter>()
//             .add_systems(OnExit(AppState::Loading), finish_load)
//             .add_systems(Update, load_resource.run_if(in_state(AppState::Loading)))
//             .add_systems(Update,text_fit_window_size_sys.run_if(in_state(AppState::Loading)))
//             .add_systems(Update,text_flicker_system.run_if(in_state(AppState::Loading)));
//     }
// }
// fn setup(
//     mut commands: Commands,
//     asset_service: Res<AssetServer>,
// ) {
//     let texture_handle = asset_service.load("texture/menu.png");
//     let font_handle = asset_service.load("fonts/SanJiLuRongTi-ZhongCu-2.ttf");
//
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn(
//         NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.),
//                 height: Val::Percent(100.),
//                 flex_direction: FlexDirection::Column,
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..default()
//             },
//             background_color: Color::BLACK.into(),
//             ..default()
//         }
//     ).with_children(|parent| {
//         parent.spawn((
//             ImageBundle{
//                 style: Style{
//                     width: Val::VMin(100.),
//                     height: Val::VMin(100.),
//                     justify_items: JustifyItems::Center,
//                     align_items: AlignItems::Center,
//                     align_self: AlignSelf::Center,
//                     ..default()
//                 },
//                 image: UiImage::new(texture_handle),
//                 ..default()
//             },
//             Ui
//             ));
//     }).with_children(|parent| {
//         parent.spawn((
//             TextBundle::from_section(
//                 "少女祈祷中",
//                 TextStyle{
//                     font: font_handle,
//                     font_size: 80.0,
//                     color: Color::BLACK,
//                 }
//             ).with_style(Style {
//                 bottom: Val::Px(100.),
//                 right: Val::VMin(-30.),
//                 ..default()
//             }),
//             LoadingText
//         ));
//     });
// }
//
// #[derive(Component)]
// struct LoadingText;
//
// #[derive(Resource)]
// struct FlickerTimer(Timer);
//
// #[derive(Resource)]
// struct FlickerCounter(usize);
//
// impl Default for FlickerTimer{
//     fn default() -> Self {
//         FlickerTimer(Timer::from_seconds(1. , TimerMode::Repeating))
//     }
// }
//
// impl Default for FlickerCounter{
//     fn default() -> Self {
//         FlickerCounter(0)
//     }
// }
//
// fn text_flicker_system(
//     time: Res<Time>,
//     mut timer: ResMut<FlickerTimer>,
//     mut counter: ResMut<FlickerCounter>,
//     mut query: Query<&mut Text,With<LoadingText>>
// ) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for mut text in &mut query {
//             text.sections[0].value = format!("{}{}","少女祈祷中",".".repeat(counter.0));
//         }
//         counter.0 += 1;
//         if counter.0 > 3 { counter.0 = 0 }
//     }
// }
//
// fn text_fit_window_size_sys(
//     mut on_window_size_changed: EventReader<WindowResized>,
//     mut query: Query<&mut Text,With<LoadingText>>
// ){
//     for e in &mut on_window_size_changed{
//         let w = e.width;
//         let h = e.height;
//         let new_size = {
//             let min = if w > h{
//                 h
//             } else {
//                 w
//             };
//             min/720.*80.
//         };
//
//         for mut text in &mut query{
//             text.sections[0].style.font_size=new_size;
//         }
//     }
// }
//
// fn load_resource(
//     asset_server: Res<AssetServer>,
//     mut next_state: ResMut<NextState<AppState>>
// ){
//     let intro_gbm_handle:Handle<AudioSource> = asset_server.load("audio/intro.ogg");
//     if is_loaded(&asset_server, &intro_gbm_handle) {
//         next_state.set(AppState::Menu);
//     }
// }
//
// fn is_loaded<T: Asset>(
//     asset_server: &Res<AssetServer>,
//     handle: &Handle<T>
// ) -> bool{
//     asset_server.get_load_state(handle) == LoadState::Loaded
// }
//
// fn finish_load(
//     mut query: Query<Entity,With<LoadingText>>,
//     mut commands: Commands
// ){
//     for e in &mut query {
//         commands.entity(e).despawn_recursive();
//     }
//
//     commands.remove_resource::<FlickerCounter>();
//     commands.remove_resource::<FlickerTimer>();
// }