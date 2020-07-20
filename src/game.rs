use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        world.register::<Player>();
        world.register::<Ball>();
        let assets = load_assets(world);
        initialise_players(world, assets.clone());
        initialise_ball(world, assets);
        initialise_camera(world);
    }
}

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

#[derive(PartialEq, Eq)]
pub enum PlayerSide {
    Left,
    Right,
}

pub struct Player {
    pub side: PlayerSide,
    pub width: f32,
    pub height: f32,
}
const PLAYER_START_WIDTH: f32 = 4.0;
const PLAYER_START_HEIGHT: f32 = 16.0;

impl Player {
    fn new(side: PlayerSide) -> Self {
        Self {
            side,
            width: PLAYER_START_WIDTH,
            height: PLAYER_START_HEIGHT,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}


fn initialise_players(world: &mut World, sprite_handle: Handle<SpriteSheet>) {

    let arena_half_height = ARENA_HEIGHT / 2.0;
    let left_player = Player::new(PlayerSide::Left);
    let right_player = Player::new(PlayerSide::Right);

    let mut left_transform = Transform::default();
    left_transform.set_translation_xyz(left_player.width * 0.5, arena_half_height, 0.0);
    let mut right_transform = Transform::default();
    right_transform.set_translation_xyz(ARENA_WIDTH - right_player.width * 0.5, arena_half_height, 0.0);

    // Create a left plank entity.
    world
        .create_entity()
        .with(left_player)
        .with(left_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_handle.clone(),
            sprite_number: 0,
        })
        .build();

    // Create right plank entity.
    world
        .create_entity()
        .with(right_player)
        .with(right_transform)
        .with(SpriteRender {
            sprite_sheet: sprite_handle,
            sprite_number: 0,
        })
        .build();
}

fn load_assets(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

const BALL_VELOCITY_X: f32 = 75.0;
const BALL_VELOCITY_Y: f32 = 50.0;
const BALL_RADIUS: f32 = 2.0;

pub struct Ball {

    pub velocity_x: f32,
    pub velocity_y: f32,
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1, // ball is the second sprite on the sprite sheet
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity_x: BALL_VELOCITY_X,
            velocity_y: BALL_VELOCITY_Y,
        })
        .with(local_transform)
        .build();
}