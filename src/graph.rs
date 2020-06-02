pub mod graph {
    use ndarray::Array2;
    use num_traits::Num;

    pub struct Graph<T: Num> {
        size: usize,
        matrix: Array2<T>,
    }

}
