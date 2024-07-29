use cargo_snippet::snippet;

#[snippet("union-find")]
/// Union-Find木. 頂点番号は1-origin
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

#[snippet("union-find")]
impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: vec![0; size + 1], // 0: 親なし
            sizes: vec![1; size + 1],
        }
    }

    // uが属するグループの根を求める
    fn root(&self, u: usize) -> usize {
        let par = self.parents[u];
        if par == 0 {
            return u;
        }
        self.root(par)
    }

    // u, v が同じグループに属しているか
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    // u, v の属するグループを統合する
    pub fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u == root_v {
            return;
        }
        // Union by size
        if self.sizes[root_u] > self.sizes[root_v] {
            self.parents[root_v] = root_u;
            self.sizes[root_u] += self.sizes[root_v];
        } else {
            self.parents[root_u] = root_v;
            self.sizes[root_v] += self.sizes[root_u];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::new(3);
        assert_eq!(uf.is_same(1, 2), false);
        assert_eq!(uf.is_same(2, 3), false);
        assert_eq!(uf.is_same(3, 1), false);

        uf.unite(1, 2);
        assert_eq!(uf.is_same(1, 2), true);
        assert_eq!(uf.is_same(2, 3), false);
        assert_eq!(uf.is_same(3, 1), false);

        uf.unite(2, 3);
        assert_eq!(uf.is_same(1, 2), true);
        assert_eq!(uf.is_same(2, 3), true);
        assert_eq!(uf.is_same(3, 1), true);
    }
}
