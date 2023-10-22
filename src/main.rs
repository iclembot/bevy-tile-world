#[macro_use] extern crate worldgen;

mod rand_seed;

use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{NoiseMapGenerator, NoiseMap, Seed, Step, Size};
use worldgen::world::{World, Tile};
use worldgen::world::tile::{Constraint, ConstraintType};
use rand_seed::rand_seed;
use bevy_tilemap::prelude::*;
use bevy::asset::HandleId;
use bevy::prelude::*;

fn main() {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of(rand_seed(6)))
        .set(Step::of(0.005, 0.005));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of(rand_seed(6)))
        .set(Step::of(0.05, 0.05));

    let nm = Box::new(nm1 + nm2 * 3);

    let world = World::new()
        .set(Size::of(128, 128))

        // Water
        .add(Tile::new('~')
            .when(constraint!(nm.clone(), < -0.1)))

        // Grass
        .add(Tile::new(',')
            .when(constraint!(nm.clone(), < 0.45)))

        // Mountains
        .add(Tile::new('^')
            .when(constraint!(nm.clone(), > 0.8)))

        // Hills
        .add(Tile::new('n'));

    for row in world.generate(0, 0).iter() {
        for val in row.iter() {
            for c in val.iter() {
                print!("{}", c);
            }

            println!("");
        }

        println!("");
    }
}

fn tilemap_now() {
    
    // Build a default Tilemap with 32x32 pixel tiles.
    let mut tilemap = Tilemap::default();
    
    // We need a Asset<TextureAtlas>. For this example we get a random one as a placeholder.
    let texture_atlas_handle = Handle::weak(HandleId::random::<TextureAtlas>());
    
    // Set the texture atlas for the Tilemap
    tilemap.set_texture_atlas(texture_atlas_handle);
    
    // Create tile data
    let tile = Tile {
        // 2D location x,y (units are in tiles)
        point: (16,16),
        
        // Which tile from the TextureAtlas
        sprite_index: 0,
        
        // Which z-layer in the Tilemap (0-up)
        sprite_order: 0,
        
        // Give the tile an optional green tint
        tint: bevy::render::color::Color::GREEN,
        };
    
    // Insert a single tile
    tilemap.insert_tile( tile);
}