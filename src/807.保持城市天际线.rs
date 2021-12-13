/*
 * @lc app=leetcode.cn id=807 lang=rust
 *
 * [807] 保持城市天际线
 */

// @lc code=start
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_max = vec![0i32; m];
        let mut col_max = vec![0i32; n];

        for r in 0..m {
            for c in 0..n {
                row_max[r] = row_max[r].max(grid[r][c]);
                col_max[c] = col_max[c].max(grid[r][c]);
            }
        }

        let mut res = 0i32;
        for r in 0..m {
            for c in 0..n {
                res += row_max[r].min(col_max[c]) - grid[r][c];
            }
        }

        res
    }
}
// @lc code=end
