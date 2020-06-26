#![allow(dead_code)]

use num_traits::{pow, Float, Num, NumCast, cast};

pub struct Point<T: Num> {
    id: usize,
    coordinates: (T, T),
}

impl<T: Num + Copy> Point<T> {
    pub fn new(id: usize, coordinates: (T, T)) -> Point<T> {
        Point { id, coordinates }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn x(&self) -> T {
        self.coordinates.0
    }

    pub fn y(&self) -> T {
        self.coordinates.1
    }

}

impl<T: Float> Point<T> {
    pub fn dist_from(&self, point: &Point<T>) -> T {
        let x = point.x();
        let y = point.y();
        let r = pow(x - self.x(), 2) + pow(y - self.y(), 2);
        r.sqrt()
    }
}

impl<T: Num + Copy + NumCast> Point<T> {
    pub fn dist_to<V: Float>(&self, point: &Point<T>) -> V {
        let x = point.x();
        let y = point.y();
        let r = pow(x - self.x(), 2) + pow(y - self.y(), 2);

        let x: V = cast(r).unwrap();
        x.sqrt()
    }
}