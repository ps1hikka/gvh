use super::assets::{Assets, SfxId};
use macroquad::audio::play_sound_once;

pub fn play(assets: &Assets, id: SfxId) {
    play_sound_once(assets.sound(id));
}
