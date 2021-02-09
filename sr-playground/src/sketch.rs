use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const NUM_HASHERS: usize = 4;

#[derive(Default, Clone, Debug)]
struct Elem {
    count: i32,
    xor_elem: u64,
    xor_hash: u64,
}

impl Elem {
    fn adjust(&mut self, elem: u64, elem_hash: u64, count: i32) {
        self.count += count;
        self.xor_elem ^= elem;
        self.xor_hash ^= elem_hash;
    }

    fn merge(&mut self, rhs: &Elem) {
        self.count -= rhs.count;
        self.xor_elem ^= rhs.xor_elem;
        self.xor_hash ^= rhs.xor_hash;
    }

    fn is_one(&self) -> bool {
        self.count == -1 || self.count == 1
    }
}

#[derive(Clone)]
pub struct Sketch {
    // maybe make this into bits?
    capacity: usize,
    data: Vec<Elem>,
}

impl Sketch {
    pub fn new(capacity: usize) -> Self{
        Self {
            capacity,
            data: vec![Elem::default(); capacity],
        }
    }

    pub fn add(&mut self, elem: u64) {
        self.adjust(elem, 1)
    }

    pub fn remove(&mut self, elem: u64) {
        self.adjust(elem, -1)
    }

    fn compute_hash(elem: u64) -> u64 {
        let mut h =  DefaultHasher::new();
        elem.hash(&mut h);
        h.finish()
    }

    fn adjust(&mut self, elem: u64, count: i32) {
        self.adjust_value(elem, count)
    }

    pub fn merge(&mut self, rhs: &Sketch) {
        assert_eq!(self.capacity, rhs.capacity);
        for i in 0..self.capacity {
            self.data[i].merge(&rhs.data[i]);
        }
    }

    pub fn recover(&mut self) -> Result<Vec<u64>, &'static str> {
        let mut result = Vec::with_capacity(self.capacity);
        let mut to_check = Vec::new();
        for i in 0..self.capacity {
            if !self.data[i].is_one() {
                continue;
            }
            to_check.push(i);

            while let Some(i) = to_check.pop() {

                if !self.data[i].is_one() {
                    continue;
                }
                let elem = self.data[i].xor_elem;
                let elem_hash = Self::compute_hash(elem);
                if elem_hash != self.data[i].xor_hash {
                    continue;
                }

                result.push(elem);
                self.adjust_value2(elem, elem_hash, -self.data[i].count, &mut to_check);
            }
        }
        for i in 0..self.capacity {
            if self.data[i].count != 0 {
                return Err("unable to recover result")
            }
        }
        Ok(result)
    }

    // TODO: how do we remove code duplication?
    fn adjust_value2(&mut self, elem: u64, elem_hash: u64, count: i32, queue: &mut Vec<usize>) {
        let mut cur_hash = elem_hash;

        for i in 0..NUM_HASHERS {
            let pos = (cur_hash as usize % self.capacity) as usize;
            self.data[pos].adjust(elem, elem_hash, count);

            if i != 0 {
                queue.push(pos);
            }

            if i == NUM_HASHERS - 1 {
                break;
            }
            cur_hash = Self::compute_hash(cur_hash);
        }
    }

    fn adjust_value(&mut self, elem: u64, count: i32) {
        let elem_hash = Self::compute_hash(elem);
        let mut cur_hash = elem_hash;

        for i in 0..NUM_HASHERS {
            let pos = (cur_hash as usize % self.capacity) as usize;
            self.data[pos].adjust(elem, elem_hash, count);

            if i == NUM_HASHERS - 1 {
                break;
            }
            cur_hash = Self::compute_hash(cur_hash);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sketch::Sketch;

    fn create_sketch(elements: impl IntoIterator<Item = u64>) -> Sketch {
        let mut sketch = Sketch::new( 400);
        for item in elements.into_iter() {
            sketch.add(item);
        }
        sketch
    }

    #[test]
    fn create_sketch_alice() {
        let set = 1000000_3_00000u64..1000000_301000u64;

        create_sketch(set);
    }
}
