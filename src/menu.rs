use bevy::app::App;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;


use crate::common::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_state::<MenuAnimeState>()
            .init_resource::<FadeTimer>()
            .add_systems(OnEnter(AppState::Menu), intro)
            .add_systems(Update,enter_anime.run_if(in_state(MenuAnimeState::EnterMenu)));
    }
}

#[derive(Component)]
struct FadeBundle;

fn intro(
    mut query: Query<&mut BackgroundColor>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut anime_state: ResMut<NextState<MenuAnimeState>>,
){
    audio.play(asset_server.load("audio/intro.ogg"));

    for mut q in &mut query {
        q.0.set_a(0.);
    }
    anime_state.set(MenuAnimeState::EnterMenu);
}

#[derive(Resource)]
struct FadeTimer(Timer);
impl Default for FadeTimer{
    fn default() -> Self {
        FadeTimer(Timer::from_seconds(1.,TimerMode::Once))
    }
}

fn enter_anime(
    mut query: Query<&mut BackgroundColor,With<Ui>>,
    time: Res<Time>,
    mut anime_state: ResMut<NextState<MenuAnimeState>>,
    mut timer: ResMut<FadeTimer>,
){
    for mut q in &mut query {
        let a = q.0.a();
        q.0.set_a(a+time.delta_seconds());
    }

    if timer.0.tick(time.delta()).just_finished() {
        anime_state.set(MenuAnimeState::None);
    }
}