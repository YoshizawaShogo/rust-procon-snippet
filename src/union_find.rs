use cargo_snippet::snippet;

#[snippet("union_find")]
struct UnionFind {
    parents: Vec<usize>,
    heights: Vec<usize>, // unionを効率よく行うために保持
    weights: Vec<isize>,
}

#[snippet("union_find")]
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            heights: vec![1; n],
            weights: vec![0; n],
        }
    }
    fn root(&mut self, node: usize) -> (usize, isize) {
        let p = self.parents[node];
        if node == p {
            (node, 0)
        } else {
            let (r, w) = self.root(p);
            self.parents[node] = r;
            self.weights[node] += w;
            (self.parents[node], self.weights[node])
        }
    }
    fn union(&mut self, n1: usize, n2: usize, w: isize) {
        let (r1, w1) = self.root(n1);
        let (r2, w2) = self.root(n2);
        debug_assert!(!self.is_connected(n1, n2));

        let h1 = self.heights[r1];
        let h2 = self.heights[r2];

        if h1 <= h2 {
            self.parents[r1] = r2;
            self.heights[r2] = self.heights[r2].max(self.heights[r1] + 1);
            self.weights[r1] = w2 - w1 - w;
        } else {
            self.parents[r2] = r1;
            self.heights[r1] = self.heights[r1].max(self.heights[r2] + 1);
            self.weights[r2] = w1 - w2 + w;
        }
    }
    fn is_connected(&mut self, n1: usize, n2: usize) -> bool {
        let (r1, _w1) = self.root(n1);
        let (r2, _w2) = self.root(n2);
        r1 == r2
    }
    fn is_valid(&mut self, n1: usize, n2: usize, w: isize) -> bool {
        let (r1, w1) = self.root(n1);
        let (r2, w2) = self.root(n2);
        debug_assert!(r1 == r2);
        w2 - w1 == w
    }
}
