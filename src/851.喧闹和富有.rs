/*
 * @lc app=leetcode.cn id=851 lang=rust
 *
 * [851] 喧闹和富有
 */

// @lc code=start
use std::collections::HashMap;

#[derive(Default)]
struct DfsSolution {
    // 输入
    richer: Vec<Vec<i32>>,
    quiet: Vec<i32>,

    // 中间辅助变量
    graph: Vec<Vec<i32>>,
    
    // 输出
    res: Vec<i32>,
}

impl DfsSolution {
    /**
     * 非常臃肿的图实现
     */
    pub fn new(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> DfsSolution {
        DfsSolution {
            res: vec![-1i32; quiet.len()],
            graph: vec![vec![]; quiet.len()],
            richer,
            quiet,
            ..Default::default()
        }
    }

    pub fn solve(mut self) -> Vec<i32> {
        // 建图
        let mut in_degree = vec![0i32; self.quiet.len()];
        for pair in self.richer.iter() {
            // pair[0] <- pair[1]
            in_degree[pair[0] as usize] += 1;
            self.graph[pair[1] as usize].push(pair[0]);
        }
        for (i,degree) in in_degree.into_iter().enumerate() {
            if degree == 0 {
                // root
                self.dfs(i);
            }
        }

        self.res
    }

    fn dfs(&mut self, root: usize) -> usize {
        /**
         * 深度优先遍历，返回最安静的索引
         */
        if self.res[root as usize] != -1 {
            return self.res[root] as usize;
        }
        
        let mut quiet_i = root as usize;
        let children:Vec<i32> = self.graph[root].iter().map(|&x|x).collect();
        for child in children.into_iter() {
            let c_quiet_i = self.dfs(child as usize);
            if self.quiet[c_quiet_i] < self.quiet[quiet_i] {
                quiet_i = c_quiet_i;
            }
        }

        self.res[root] = quiet_i as i32;
        quiet_i
    }
}
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        /**
         * 思路：
         * 1. 对所有人按照富有程度构建一张有向图无环图，没钱的人指向有钱的人
         * 2. 要求比i有钱的人中，最安静的人，沿着结点i往下遍历即可
         */
        let mut solution = DfsSolution::new(richer, quiet);
        solution.solve()
    }
}
// @lc code=end

