use ahash::AHasher;
use std::cmp::{max, min};
use std::hash::Hasher;

const NUM_HASHES: usize = 3;

#[derive(Default, Clone, Debug)]
struct Elem {
    xor_elem: u64,
    xor_hash: u64,
}

impl Elem {
    fn adjust(&mut self, elem: u64, elem_hash: u64) {
        self.xor_elem ^= elem;
        self.xor_hash ^= elem_hash;
    }

    fn merge(&mut self, rhs: &Elem) {
        self.xor_elem ^= rhs.xor_elem;
        self.xor_hash ^= rhs.xor_hash;
    }
}

#[derive(Clone)]
pub struct BLT<H: Hasher + Default + Clone = AHasher> {
    pub capacity: usize,
    k: i32,
    data: Vec<Elem>,
    seed: u64,
    hasher: H,
}

impl<H: Hasher + Default + Clone> BLT<H> {
    pub fn new(capacity: usize, seed: u64) -> Self {
        let mut k = 0;
        while (1 << k) + 2 < capacity {
            k += 1;
        }

        let mut hasher = H::default();
        hasher.write_u64(seed);
        let new_capacity = (1 << k) + 2;
        Self {
            capacity: new_capacity,
            data: vec![Elem::default(); new_capacity],
            seed,
            hasher,
            k,
        }
    }

    pub fn add(&mut self, elem: u64) {
        self.adjust(elem)
    }

    pub fn remove(&mut self, elem: u64) {
        self.adjust(elem)
    }

    pub fn compute_hash(&self, elem: u64) -> u64 {
        let mut h = self.hasher.clone();
        h.write_u64(elem);
        h.finish()
    }

    fn adjust(&mut self, elem: u64) {
        self.adjust_value(elem);
    }

    pub fn merge(&mut self, rhs: &BLT) {
        assert_eq!(self.capacity, rhs.capacity);
        assert_eq!(self.seed, rhs.seed);
        for i in 0..self.capacity {
            self.data[i].merge(&rhs.data[i]);
        }
    }

    pub fn recover(&mut self) -> Result<Vec<u64>, &'static str> {
        let (result, ok) = self.try_recover();

        if !ok {
            for i in 0..self.capacity {
                if self.data[i].xor_elem != 0 {
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
        return Ok(result);
    }

    pub fn try_recover(&mut self) -> (Vec<u64>, bool) {
        let mut result = Vec::with_capacity(self.capacity);
        let mut to_check = Vec::default();
        for i in 0..self.capacity {
            to_check.push(i);

            while let Some(i) = to_check.pop() {
                let elem = self.data[i].xor_elem;
                if elem == 0 && self.data[i].xor_hash == 0 {
                    continue;
                }
                let elem_hash = self.compute_hash(elem);
                if elem_hash != self.data[i].xor_hash {
                    continue;
                }

                result.push(elem);
                self.adjust_value2(elem, elem_hash, &mut to_check);
            }
        }
        for i in 0..self.capacity {
            if self.data[i].xor_elem != 0 {
                return (result, false);
            }
        }
        (result, true)
    }

    fn generate_idx(&mut self, elem_hash: u64) -> [usize; NUM_HASHES] {
        let pos0 = elem_hash & ((1 << self.k) - 1);
        let mut pos1 = (elem_hash >> self.k) & ((1 << self.k) - 1);
        let mut pos2 = (elem_hash >> 2 * self.k) & ((1 << self.k) - 1);
        if pos1 >= pos0 {
            pos1 += 1;
        }
        if pos2 >= min(pos0, pos1) {
            pos2 += 1;
        }
        if pos2 >= max(pos0, pos1) {
            pos2 += 1;
        }
        [pos0 as usize, pos1 as usize, pos2 as usize]
    }

    fn adjust_value2(&mut self, elem: u64, elem_hash: u64, queue: &mut Vec<usize>) {
        let pos_list = self.generate_idx(elem_hash);

        for &pos in &pos_list {
            self.data[pos].adjust(elem, elem_hash);
            queue.push(pos);
        }
    }

    fn adjust_value(&mut self, elem: u64) {
        let elem_hash = self.compute_hash(elem);
        let pos_list = self.generate_idx(elem_hash);

        for &pos in &pos_list {
            self.data[pos].adjust(elem, elem_hash);
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
