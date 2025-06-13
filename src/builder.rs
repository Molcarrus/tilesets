use bevy::prelude::*;

use rand::Rng;

#[derive(Debug, Resource)]
pub struct Environment {
    pub size: u8,
    pub tileset: Vec<Vec<u8>>
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            size: 3,
            tileset: generate_tileset(3)
        }
    }
}

pub fn generate_tileset(size: u8) -> Vec<Vec<u8>> {
    let mut tileset = vec![vec![0; size as usize]; size as usize];

    let mut rng = rand::rng();

    let mut generate_random_height = | | -> u8 {
        rng.random_range(0..=size)
    };

    for i in 0..size as usize {
        for j in 0..size as usize {
            tileset[i][j] = generate_random_height();
        }
    }

    tileset
}