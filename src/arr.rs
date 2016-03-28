use std::ops::Index;

pub struct Arrangement {
    capacity: usize,
    count: usize,
    arrangement: Vec<bool>,
}

impl Arrangement {

    pub fn new(capacity: usize, count: usize) -> Arrangement {
        // TODO compare capacity & count
        if count > capacity {
            panic!("Capacity shou");
        }
        let mut arr = Arrangement {
            capacity: capacity,
            count: count,
            arrangement: vec![]
        };
        for _ in 0..count {
            arr.arrangement.push(true);
        }
        for _ in count..capacity {
            arr.arrangement.push(false);
        }
        arr
    }

    pub fn capacity(&self) -> usize{
        self.capacity
    }

    pub fn count(&self) -> usize {
        self.count
    }

}

impl Index<usize> for Arrangement {
    type Output = bool;

    fn index<'a>(&'a self, index: usize) -> &'a bool {
        &self.arrangement[index]
    }
}
