// modified version of this: https://github.com/johanhelsing/bevy_sparse_grid_2d/tree/main
// also of possible use: https://github.com/kul-sudo/spatial_hashing/tree/main

use bevy::{
    prelude::{Entity, Vec2},
    reflect::Reflect,
    utils::{HashMap, HashSet},
};
use smallvec::SmallVec;

use super::aabb_simd::AabbSimd;

use std::simd::num::SimdFloat;
use std::simd::{f32x1, f32x2, f32x4, i32x2, i32x4, StdFloat};

type Key = i32x2;

/// A spatial container that allows querying for entities that share one or more grid cell
#[derive(Default, Reflect, Debug, Clone)]
pub struct SpatialHashSimd2<T: Copy + Eq + std::hash::Hash = Entity, const TILE_SIZE: usize = 1> {
    pub map: HashMap<Key, SmallVec<[T; 100]>>,
}

impl<T: Copy + Eq + std::hash::Hash, const TILE_SIZE: usize> SpatialHashSimd2<T, TILE_SIZE> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
/* 
    /// Insert an entity in the given Aabb coordinates
    pub fn insert_aabb(&mut self, aabb: &AabbSimd/*impl Into<AabbSimd>*/, entity: T) {
        for key in KeyIter::new::<TILE_SIZE>(aabb) {
            self.map.entry(key).or_default().push(entity);
        }
    }

    /// Insert an entity at the given point coordinate
    pub fn insert_point(&mut self, point: Vec2, entity: T) {
        let key = Self::key_from_point(point);
        self.map.entry(key).or_default().push(entity);
    }
*/
    /// Get an iterator with the entities in the grid cells covered by the given [`AabbSimd`]
    ///
    /// may contain duplicates if some entities are in more than one grid cell
    #[inline]
    pub fn aabb_iter(&'_ self, aabb: &AabbSimd/*impl Into<AabbSimd>*/) -> impl Iterator<Item = T> + '_ {
        KeyIter::new::<TILE_SIZE>(aabb)
            .filter_map(|key| self.map.get(&key))
            .flatten()
            .copied()
    }
/*
    /// Get an iterator with the entities in the grid cells at the given point
    #[inline]
    pub fn point_iter(&'_ self, point: Vec2) -> impl Iterator<Item = T> + '_ {
        let key = Self::key_from_point(point);

        std::iter::once(key)
            .filter_map(|key| self.map.get(&key))
            .flatten()
            .copied()
    }

    /// Creates a hash set with all the entities in the grid cells covered by the given [`AabbSimd`]
    #[inline]
    pub fn query_aabb(&self, aabb: &AabbSimd/*impl Into<AabbSimd>*/) -> HashSet<T> {
        self.aabb_iter(aabb).collect()
    }
*/
    /// Remove all entities from the map
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Remove all entities from the map, but keep the heap-allocated inner data structures
    pub fn soft_clear(&mut self) {
        for (_, vec) in self.map.iter_mut() {
            vec.clear()
        }
    }

    pub fn prepopulate(&mut self, x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
        for y in y_min..y_max {
            for x in x_min..x_max {
                self.map.insert(i32x2::from_array([x, y]), SmallVec::default());
                //self.map.entry(i32x2::from_array([x, y])).or_default();
            }
        }
    }
    
/* 
    fn key_from_point(point: Vec2) -> Key {
        (
            (point.x / TILE_SIZE as f32).floor() as i32,
            (point.y / TILE_SIZE as f32).floor() as i32,
        )
    }*/
}

pub struct KeyIter {
    pub width: i32,
    pub start: Key,
    pub current: i32,
    pub count: i32,
}


impl KeyIter {
    pub fn new<const TILE_SIZE: usize>(aabb: &AabbSimd /*impl Into<AabbSimd>*/) -> Self {
        //let AabbSimd { min, max } = aabb.into();
        // convert to key space
        let s = TILE_SIZE as f32;

        // I've taken this formula and made it simd friendly below:
        //let min = ((aabb.data[0] / s).floor() as i32, (aabb.data[1] / s).floor() as i32);
        //let max = ((aabb.data[2] / s).ceil() as i32, (aabb.data[3] / s).ceil() as i32);

        let div = aabb.data / f32x4::splat(s);
        let floor = div.floor(); // as i32x4;
        let ceil = div.ceil();// as i32x4;
        
        let min: i32x2 = i32x2::from_array([floor[0] as i32, floor[1] as i32]); //(floor[0] as i32, floor[1] as i32);
        let max = i32x2::from_array([ceil[2] as i32, ceil[3] as i32]); //(ceil[2] as i32, ceil[3] as i32);
        
        let width = max[0] - min[0];
        let height = max[1] - min[1];
        let count = width * height;
        Self {
            start: min,
            current: -1,
            width,
            count,
        }
    }
}

impl Iterator for KeyIter {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current < self.count {
            //let next = self.start + 
            Some(
                i32x2::from_array([self.start[0] + self.current.rem_euclid(self.width), self.start[1] + self.current / self.width])
            )
            /* 
            Some((
                self.start.0 + self.current.rem_euclid(self.width),
                self.start.1 + self.current / self.width,
            ))*/
        } else {
            None
        }
    }
}

/* 
#[cfg(test)]
mod tests {
    use std::simd::f32x2;

    use bevy::math::vec2;
    use bevy::utils::HashSet;

    use super::*;

    const TILE_SIZE: usize = 1;

    #[test]
    fn keys_single() {
        let keys: Vec<Key> = KeyIter::new::<TILE_SIZE>(&AabbSimd::from_min_max(
            f32x2::from_array([0.001, 0.001]),
                f32x2::from_array([0.001, 0.001]),
        ))
        .collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], (0, 0));
    }

    #[test]
    fn keys_four_around_origin() {
        let keys: Vec<Key> = KeyIter::new::<TILE_SIZE>(&AabbSimd::from_min_max(f32x2::from_array([-0.001, -0.001]), f32x2::from_array([0.001, 0.001])))
        .collect();
        assert!(keys.contains(&(0, 0)));
        assert!(keys.contains(&(0, -1)));
        assert!(keys.contains(&(-1, 0)));
        assert!(keys.contains(&(-1, -1)));
        assert_eq!(keys.len(), 4);
    }

    #[test]
    fn matches() {
        let entity = Entity::from_raw(123);
        let mut db = SpatialHashSimd2::<Entity, TILE_SIZE>::new(); //default();
        db.insert_aabb(
            &AabbSimd::from_min_max(
                f32x2::from_array([-0.001, -0.001]),
                f32x2::from_array([0.001, 0.001]),
            ),
            entity,
        );

        let matches: Vec<Entity> = db
            .aabb_iter(&AabbSimd::from_min_max(
                f32x2::from_array([0.001, 0.001]),
                f32x2::from_array([0.001, 0.001]),
            ))
            .collect();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], entity);
    }

    #[test]
    fn key_negative() {
        let h = TILE_SIZE as f32 / 2.0;
        let keys: Vec<Key> = KeyIter::new::<TILE_SIZE>(&AabbSimd::from_min_max(
            f32x2::from_array([-h, -h]),
            f32x2::from_array([-h, -h]),
        ))
        .collect();
        assert!(keys.contains(&(-1, -1)));
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn query_points() {
        let mut db = SpatialHashSimd2::<Entity, TILE_SIZE>::new(); //default();
        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        db.insert_point(vec2(0.5, 0.5), e1);
        db.insert_point(vec2(0.499, 0.501), e2);

        let matches: HashSet<_> = db.point_iter(vec2(0.499, 0.501)).collect();
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn query_points_negative() {
        let mut db = SpatialHashSimd2::<Entity, TILE_SIZE>::new(); //default();
        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        db.insert_point(vec2(0.5, 0.5), e1);
        db.insert_point(vec2(-0.5, -0.5), e2);

        let matches: HashSet<_> = db.point_iter(vec2(-0.5, -0.5)).collect();
        assert!(!matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn matches_complex() {
        let h = TILE_SIZE as f32 / 2.0;
        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        let e3 = Entity::from_raw(3);
        let mut db: SpatialHashSimd2 = SpatialHashSimd2::new(); //default();
        db.insert_aabb(
            &AabbSimd::from_min_max(
                f32x2::from_array([-h, -h]),
                f32x2::from_array([h, h]),
            ),
            e1,
        );
        db.insert_aabb(
            &AabbSimd::from_min_max(
                f32x2::from_array([h, h]),
                f32x2::from_array([h, h]),
            ),
            e2,
        );
        db.insert_aabb(
            &AabbSimd::from_min_max(
                f32x2::from_array([-h, -h]),
                f32x2::from_array([-h, -h]),
            ),
            e3,
        );

        let matches: Vec<Entity> = db
            .aabb_iter(&AabbSimd::from_min_max(
                f32x2::from_array([-h, -h]),
                f32x2::from_array([h, h]),
            ))
            .collect();
        // assert_eq!(matches.len(), 3);
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert!(matches.contains(&e3));

        let matches = db.query_aabb(&AabbSimd::from_min_max(
            f32x2::from_array([-0.001, -0.001]),
            f32x2::from_array([-0.001, -0.001]),
        ));
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e3));

        let matches: Vec<Entity> = db
            .aabb_iter(&AabbSimd::from_min_max(
                f32x2::from_array([-0.001, -0.001]),
                f32x2::from_array([-0.001, -0.001]),
            ))
            .collect();
        assert_eq!(matches[0], e1);
    }

    #[test]
    fn query_points_tilesize_10() {
        let mut db = SpatialHashSimd2::<Entity, 10>::new(); //default();
        let e1 = Entity::from_raw(1);
        let e2 = Entity::from_raw(2);
        let e3 = Entity::from_raw(3);
        db.insert_point(vec2(12f32, 15f32), e1);
        db.insert_point(vec2(15f32, 12f32), e2);
        db.insert_point(vec2(15f32, 20f32), e3);
        let matches: HashSet<_> = db.point_iter(vec2(19.9, 19.9)).collect();
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
        assert!(!matches.contains(&e3));
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn non_entity() {
        let h = TILE_SIZE as f32 / 2.0;
        let e1 = 1;
        let e2 = 2;
        let mut db = SpatialHashSimd2::<usize>::new(); //default();
        db.insert_aabb(
            &AabbSimd::from_min_max(
                f32x2::from_array([-h, -h]),
                f32x2::from_array([h, h]),
            ),
            e1,
        );
        db.insert_aabb(
            &AabbSimd::from_min_max(
                f32x2::from_array([h, h]),
                f32x2::from_array([h, h]),
            ),
            e2,
        );
        let matches: Vec<usize> = db
            .aabb_iter(&AabbSimd::from_min_max(
                f32x2::from_array([-h, -h]),
                f32x2::from_array([h, h]),
            ))
            .collect();
        // assert_eq!(matches.len(), 2);
        assert!(matches.contains(&e1));
        assert!(matches.contains(&e2));
    }
}
*/