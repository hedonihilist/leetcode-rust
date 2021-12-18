/*
 * @lc app=leetcode.cn id=419 lang=rust
 *
 * [419] 甲板上的战舰
 */

// @lc code=start
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        //Self::easy_solution(board)
        Self::advanced_solution(board)
    }

    fn advanced_solution(board: Vec<Vec<char>>) -> i32 {
        /**
         * 进阶解法，只扫描一遍，O(1)空间，不修改原数据
         * 和easy_solution一样，从上往下扫描
         * 观察到如果当前位置的上方是X或者左方是X, 那么当前位置的战舰其实已经被计数了
         * 所以，通过判断当前为是上方和左方是否为X, 可以避免重复计数
         */
        let (m, n) = (board.len(), board[0].len());
        let mut cnt = 0i32;

        for r in 0..m {
            for c in 0..n {
                if board[r][c] == 'X' {
                    if r > 0 && board[r - 1][c] == 'X' || c > 0 && board[r][c - 1] == 'X' {
                        continue; // 已经计数过了
                    }
                    cnt += 1;
                }
            }
        }

        cnt
    }

    fn easy_solution(mut board: Vec<Vec<char>>) -> i32 {
        /**
         * 非常直接的解法，扫描一遍，每次遇到X将当前战舰标记为.
         */
        let (m, n) = (board.len(), board[0].len());
        let mut cnt = 0i32;

        for r in 0..m {
            for c in 0..n {
                if board[r][c] == 'X' {
                    board[r][c] = '.';
                    cnt += 1;
                    let mut rr = r + 1;
                    let mut cc = c + 1;
                    while rr < m && board[rr][c] == 'X' {
                        board[rr][c] = '.';
                        rr += 1;
                    }
                    while cc < n && board[r][cc] == 'X' {
                        board[r][cc] = '.';
                        cc += 1;
                    }
                }
            }
        }

        cnt
    }
}
// @lc code=end
