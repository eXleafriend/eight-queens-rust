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
            base: self.arrangement.clone(),
        }
    }

}

impl Index<usize> for Arrangement {
    type Output = bool;

    fn index<'a>(&'a self, index: usize) -> &'a bool {
        &self.arrangement[index]
    }
}

pub struct Iter {
    base: Vec<bool>,
}

impl Iterator for Iter {
    type Item = Arrangement;
    fn next(&mut self) -> Option<Self::Item> {
        let mut passed_count = 0;
        let capacity = (&self.base).len();
        for i in 0..(capacity - 1) {
            if is_occupied((&self.base)[i]) {
                if is_vacant((&self.base)[i + 1]) {
                    let mut arrangement = vec![];
                    for _ in 0..passed_count {
                        arrangement.push(true);
                    }
                    for _ in passed_count..i {
                        arrangement.push(false);
                    }
                    arrangement.push(false);
                    arrangement.push(true);
                    for n in (i + 2)..capacity {
                        arrangement.push((&self.base)[n]);
                    }
                    //let new = Arrangement::from(arrangement);
                    self.base = arrangement.clone();
                    return Some(Arrangement::from(arrangement));
                } else {
                    passed_count += 1;
                }
            } else /* if is_vacant(arr[i]) */ {

            }
        }
        None
    }
}

fn is_vacant(b: bool) -> bool {
    !b
}

fn is_occupied(b: bool) -> bool {
    b
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
