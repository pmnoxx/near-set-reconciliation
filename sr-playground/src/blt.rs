use ahash::AHasher;
use std::collections::hash_map::DefaultHasher;
use std::hash::{BuildHasher, Hash, Hasher};

const NUM_HASHERS: usize = 3;

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
pub struct BLT {
    capacity: usize,
    data: Vec<Elem>,
    seed: u64,
    hasher: DefaultHasher,
    // hasher: AHasher,
}

impl BLT {
    pub fn new(capacity: usize, seed: u64) -> Self {
        let mut hasher = DefaultHasher::new();
        // let mut hasher = ahash::RandomState::new().build_hasher(); // #hasher,
        hasher.write_u64(seed);
        Self {
            capacity,
            data: vec![Elem::default(); capacity],
            seed,
            hasher,
        }
    }

    pub fn add(&mut self, elem: u64) {
        self.adjust(elem, 1)
    }

    pub fn remove(&mut self, elem: u64) {
        self.adjust(elem, -1)
    }

    pub fn compute_hash(&self, elem: u64) -> u64 {
        let mut h = self.hasher.clone();
        h.write_u64(elem);
        h.finish()
    }

    fn adjust(&mut self, elem: u64, count: i32) {
        self.adjust_value(elem, count)
    }

    pub fn merge(&mut self, rhs: &BLT) {
        assert_eq!(self.capacity, rhs.capacity);
        assert_eq!(self.seed, rhs.seed);
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
                let elem_hash = self.compute_hash(elem);
                if elem_hash != self.data[i].xor_hash {
                    continue;
                }

                result.push(elem);
                self.adjust_value2(elem, elem_hash, -self.data[i].count, &mut to_check);
            }
        }
        for i in 0..self.capacity {
            if self.data[i].count != 0 {
                for i in 0..self.capacity {
                    if self.data[i].count != 0 {
                        println!(
                            "{} {:?} {}",
                            i,
                            self.data[i],
                            self.compute_hash(self.data[i].xor_elem)
                        );
                    }
                }
                return Err("unable to recover result");
            }
        }
        Ok(result)
    }

    fn generate_idx(&mut self, elem: u64, elem_hash: u64) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::with_capacity(NUM_HASHERS);
        result.push((elem as usize % self.capacity) as usize);
        let mut cur_hash = elem_hash;

        for _ in 1..NUM_HASHERS {
            let mut pos = (cur_hash as usize % self.capacity) as usize;
            while result.contains(&pos) {
                cur_hash = self.compute_hash(cur_hash);
                pos = (cur_hash as usize % self.capacity) as usize;
            }
            result.push(pos);
        }
        result
    }

    // TODO: how do we remove code duplication?
    fn adjust_value2(&mut self, elem: u64, elem_hash: u64, count: i32, queue: &mut Vec<usize>) {
        let pos_list = self.generate_idx(elem, elem_hash);

        for &pos in pos_list.iter() {
            self.data[pos].adjust(elem, elem_hash, count);
            queue.push(pos);
        }
    }

    fn adjust_value(&mut self, elem: u64, count: i32) {
        let elem_hash = self.compute_hash(elem);

        let pos_list = self.generate_idx(elem, elem_hash);

        for pos in pos_list {
            self.data[pos].adjust(elem, elem_hash, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::blt::BLT;

    fn create_blt(elements: impl IntoIterator<Item = u64>, capacity: usize) -> BLT {
        let mut sketch = BLT::new(capacity, 0);
        for item in elements.into_iter() {
            sketch.add(item);
        }
        sketch
    }

    #[test]
    fn create_blt_test() {
        let set = 1000000_3_00000u64..1000000_301000u64;

        assert_eq!(1000, create_blt(set, 2048).recover().unwrap().len())
    }
}
