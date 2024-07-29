use crate::math::min;
use cargo_snippet::snippet;

#[snippet("max-flow")]
#[derive(Clone)]
struct Edge {
    // 終点
    dst: usize,
    // 容量
    cap: usize,
    // 終点の隣接リストにおける始点のインデックス
    rev: usize,
}

#[snippet("max-flow")]
impl Edge {
    fn new(dst: usize, cap: usize, rev: usize) -> Self {
        Self { dst, cap, rev }
    }
}

#[snippet("max-flow")]
// 最大フロー問題を解く
// 頂点の番号は1-originとする
pub struct MaxFlow {
    // 頂点数（0を除く）
    size: usize,
    // 流量を表現する残余グラフ（隣接リスト）
    graph: Vec<Vec<Edge>>,
}

#[snippet("max-flow")]
#[snippet(include = "min")]
impl MaxFlow {
    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            graph: vec![vec![]; size + 1],
        }
    }

    // 辺を追加して残余グラフを構成する
    pub fn add_edge(&mut self, src: usize, dst: usize, cap: usize) {
        let ls = self.graph[src].len();
        let ld = self.graph[dst].len();
        self.graph[src].push(Edge::new(dst, cap, ld));
        self.graph[dst].push(Edge::new(src, 0, ls));
    }

    // in_flow: 頂点uに入ってくる流量
    // Returns: 頂点vertexから流れる総流量
    fn dfs(
        &mut self,
        is_visited: &mut Vec<bool>,
        in_flow: usize,
        vertex: usize,
        goal: usize,
    ) -> usize {
        is_visited[vertex] = true;
        if vertex == goal {
            return in_flow;
        }
        let mut total_flow = 0;
        for i in 0..self.graph[vertex].len() {
            let edge = self.graph[vertex][i].clone();
            if is_visited[edge.dst] || edge.cap == 0 {
                continue;
            }
            let out_flow = min(in_flow, edge.cap);
            let flow = self.dfs(is_visited, out_flow, edge.dst, goal);
            if flow == 0 {
                continue;
            }
            // 順方向　流した分だけ減らす
            self.graph[vertex][i].cap -= flow;
            // 逆方向　流した分だけ増やす
            self.graph[edge.dst][edge.rev].cap += flow;
            total_flow += flow;
        }
        total_flow
    }

    // 最大流量を求める
    pub fn solve(&mut self, start: usize, goal: usize) -> usize {
        let mut total_flow = 0;
        let mut is_visited = vec![false; self.size + 1];
        // Ford-Fulkerson法
        loop {
            let flow = self.dfs(&mut is_visited, usize::MAX, start, goal);
            if flow == 0 {
                break;
            }
            total_flow += flow;
            // 初期化
            for i in 0..is_visited.len() {
                is_visited[i] = false;
            }
        }
        total_flow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_flow() {
        let mut mf = MaxFlow::new(6);
        mf.add_edge(1, 2, 5);
        mf.add_edge(1, 4, 4);
        mf.add_edge(2, 3, 4);
        mf.add_edge(2, 5, 7);
        mf.add_edge(3, 6, 3);
        mf.add_edge(4, 5, 3);
        mf.add_edge(5, 6, 5);
        assert_eq!(mf.solve(1, 6), 8);
    }
}
