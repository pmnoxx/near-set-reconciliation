use crate::sketch::Sketch;

pub mod sketch;
mod strata;

struct Strata {
    ibf: [Sketch; 32],
}

impl Strata {
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
                    count += result.len();
                }
            }
        }
        return count;
    }
}
