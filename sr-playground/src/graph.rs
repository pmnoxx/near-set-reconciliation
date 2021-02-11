use rand::rngs::ThreadRng;
use rand::RngCore;
use std::collections::HashSet;
use std::mem::swap;

#[derive(Clone, Hash)]
struct Edge {
    u: usize,
    v: usize,
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: usize,
    pub edges: HashSet<(usize, usize)>,
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes,
            edges: Default::default(),
        }
    }

    pub fn add_random_edges(&mut self, e: usize, rng: &mut ThreadRng) {
        for _ in 0..e {
            loop {
                let mut a = rng.next_u32() as usize % self.nodes;
                let mut b = rng.next_u32() as usize % self.nodes;
                if a > b {
                    swap(&mut a, &mut b);
                }

                if self.edges.insert((a, b)) {
                    break;
                }
            }
        }
    }

    pub fn add_edges(&mut self, items: &Vec<(usize, usize)>) {
        for e in items.iter() {
            self.edges.insert(e.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::graph::Graph;
    #[test]
    fn generate_some_edges() {
        let mut g = Graph::new(1000);
        let mut rng = rand::thread_rng();
        g.add_random_edges(7777, &mut rng);
        assert_eq!(7777, g.edges.len());
    }
}
