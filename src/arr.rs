pub struct Arrangement {
    capacity: usize,
    count: usize,
}

impl Arrangement {
    pub fn new(capacity: usize, count: usize) -> Arrangement {
        // TODO compare capacity & count
        Arrangement {
            capacity: capacity,
            count: count,
        }
    }

}
