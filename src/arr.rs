use std::ops::Index;

#[derive(PartialEq, Debug)]
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

    pub fn from(arrangement: Vec<bool>) -> Arrangement {
        let count = arrangement.iter().filter(|&b| *b).collect::<Vec<&bool>>().len();
        Arrangement {
            capacity: arrangement.len(),
            count: count,
            arrangement: arrangement,
        }
    }

    pub fn capacity(&self) -> usize{
        self.capacity
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn iter(&self) -> Iter {
        Iter {
            base: self,
        }
    }

}

impl Index<usize> for Arrangement {
    type Output = bool;

    fn index<'a>(&'a self, index: usize) -> &'a bool {
        &self.arrangement[index]
    }
}

pub struct Iter<'a> {
    base: &'a Arrangement,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Arrangement;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/*
impl IntoIterator for Arrangement {
    type Item = Arrangement;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter
    }
}
*/
