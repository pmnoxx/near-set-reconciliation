use crate::blt::BLT;

#[derive(Clone)]
struct Strata {
    ibf: Vec<BLT>,
}

impl Strata {
    fn new() -> Self {
        let mut ibf = Vec::with_capacity(32);
        for _ in 0..32 {
            ibf.push(BLT::new(80, 0))
        }
        Self { ibf }
    }

    fn destructive_strata_decode_estimator(&mut self, rhs: &Strata) -> usize {
        let mut count: usize = 0;
        for i in (0..32).rev() {
            self.ibf[i].merge(&rhs.ibf[i]);
            let result = self.ibf[i].recover();

            match result {
                Err(_) => {
                    return count << (i + 1);
                }
                Ok(result) => {
                    println!("{} {}", i, result.len());
                    count += result.len();
                }
            }
        }
        return count;
    }

    fn add(&mut self, elem: u64) {
        let hash = self.ibf[0].compute_hash(elem);

        for i in 0..32 {
            if (hash & (1 << i)) != 0 {
                self.ibf[i].add(elem);
                return;
            }
        }
    }

    fn remove(&mut self, elem: u64) {
        let hash = self.ibf[0].compute_hash(elem);

        for i in 0..32 {
            if (hash & (1 << i)) != 0 {
                self.ibf[i].add(elem);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::strata::Strata;

    #[test]
    fn create_create_strata() {
        let mut a = Strata::new();
        let mut b = Strata::new();

        for x in 0..512 {
            a.add(x);
        }

        for x in 1000 + 0..1000 + 512 {
            b.add(x);
        }

        let estimate = a.clone().destructive_strata_decode_estimator(&b);
        println!("estimate {}", estimate);
        assert!(estimate >= 500 && estimate <= 2000);

        for x in 0..512 {
            a.remove(x);
        }

        for x in 1000 + 0..1000 + 512 {
            b.remove(x);
        }

        assert_eq!(a.clone().destructive_strata_decode_estimator(&b), 0);
    }
}
