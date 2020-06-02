pub mod point {
    use num_traits::Num;

    pub struct Point<T: Num> {
        id: i32,
        coordinates: (T,T),
    }

    impl<T: Num> Point<T> {
        fn new(id: i32, coordinates: (T,T)) -> Point<T> {
            Point {
                id,
                coordinates,
            }
        }

        fn id(&self) -> i32 {
            self.id
        }

        fn x(&self) -> T {
            self.coordinates.0
        }

        fn y(&self) -> T {
            self.coordinates.1
        }
        
        fn dist_to(&self, point: &Point<T>){
            let x = point.x();
            let y = point.y();

            ((x - self.x()).powi(2) + (y - self.y()).powi(2)).sqrt();
        }
    }

}
