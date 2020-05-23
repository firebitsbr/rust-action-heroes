use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture, SpriteRender},
};

/// TODO: This is marked as dead code...
#[derive(Debug, Copy, Clone)]
pub(crate) enum TransformedInputEvent {
    Up,
    Down,
    Left,
    Right,
    Interact,
}

pub(crate) fn load_sprite_sheet(world: &mut World, sprite_img: &str, sprite_key: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = loader.load(
        sprite_img,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    loader.load(
        sprite_key,
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

pub(crate) fn get_sprite(handle: Handle<SpriteSheet>, index: usize) -> SpriteRender {
    SpriteRender {
        sprite_sheet: handle,
        sprite_number: index,
    }
}
