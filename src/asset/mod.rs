use crate::coords::Coords;
use std::collections::HashMap;
use std::path::PathBuf;

pub enum Sprite {
    Fixed(PathBuf),
    Dynamic(fn(Asset) -> Option<PathBuf>),
}

pub struct Asset {
    coords: Coords,
    sprite: Sprite,
}

