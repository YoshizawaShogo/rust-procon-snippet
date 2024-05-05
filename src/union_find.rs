use cargo_snippet::snippet;

#[snippet("union_find")]
/// parents: Vec<usize>, // 親を記録。自分を指し示している場合はroot。
/// heights: Vec<usize>, // 木の高さ。root以外の高さは意味を持たない。unionを効率よく行うために保持
/// sizes: Vec<usize>, // 木のサイズ。root以外のサイズは意味を持たない。
/// distances: Vec<isize>, // rootとの距離(重み)
struct UnionFind {
    parents: Vec<usize>, // 親を記録。自分を指し示している場合はroot。
    heights: Vec<usize>, // 木の高さ。root以外の高さは意味を持たない。unionを効率よく行うために保持
    sizes: Vec<usize>, // 木のサイズ。root以外のサイズは意味を持たない。
    distances: Vec<isize>, // rootとの距離(重み)
}

#[snippet("union_find")]
#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            heights: vec![1; n],
            sizes: vec![1; n],
            distances: vec![0; n],
        }
    }
    fn root_distance(&mut self, node: usize) -> (usize, isize) {
        let p = self.parents[node];
        if node == p {
            (node, 0)
        } else {
            let (r, d) = self.root_distance(p);
            self.parents[node] = r;
            self.distances[node] += d;
            (self.parents[node], self.distances[node])
        }
    }
    fn union(&mut self, n1: usize, n2: usize, w: isize) {
        let (r1, w1) = self.root_distance(n1);
        let (r2, w2) = self.root_distance(n2);
        debug_assert!(!self.is_connected(n1, n2));
        let h1 = self.heights[r1];
        let h2 = self.heights[r2];
        if h1 <= h2 {
            self.parents[r1] = r2;
            self.heights[r2] = self.heights[r2].max(self.heights[r1] + 1);
            self.sizes[r2] += self.sizes[r1];
            self.distances[r1] = w2 - w1 - w;
        } else {
            self.parents[r2] = r1;
            self.heights[r1] = self.heights[r1].max(self.heights[r2] + 1);
            self.sizes[r1] += self.sizes[r2];
            self.distances[r2] = w1 - w2 + w;
        }
    }
    fn size(&mut self, node: usize) -> usize {
        let r = self.root_distance(node).0;
        self.sizes[r]
    }
    fn is_connected(&mut self, n1: usize, n2: usize) -> bool {
        let (r1, _w1) = self.root_distance(n1);
        let (r2, _w2) = self.root_distance(n2);
        r1 == r2
    }
    fn is_valid(&mut self, n1: usize, n2: usize, w: isize) -> bool {
        let (r1, w1) = self.root_distance(n1);
        let (r2, w2) = self.root_distance(n2);
        debug_assert!(r1 == r2);
        w2 - w1 == w
    }
}