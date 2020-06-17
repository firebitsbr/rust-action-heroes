use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle, output::Output, Source},
    ecs::{World, WorldExt},
    assets::AssetStorage,
};

const MOVE_SOUND: &str = "audio/bloop.ogg";

pub(crate) struct Sounds {
    pub(crate) move_sfx: SourceHandle,
}

fn load_audio(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub(crate) fn initialize_audio(world: &mut World) {
    world.insert({
        let loader = world.read_resource::<Loader>();

        Sounds {
            move_sfx: load_audio(&loader, &world, MOVE_SOUND),
        }
    });
}

pub(crate) fn play_move_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.move_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}