/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */

// @lc code=start
use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct UnionFind {
    fa: HashMap<i32, (i32, i32)>,
}

impl UnionFind {
    pub fn add(&mut self, a: i32) -> i32 {
        /**
         * 添加单个元素, 返回添加之后所在树的大小
         */
        if self.fa.contains_key(&a) {
            let rt = self.root(a);
            return self.fa.get(&rt).unwrap().1;
        }
        let (left, right) = (
            self.fa.contains_key(&(a - 1)),
            self.fa.contains_key(&(a + 1)),
        );
        self.fa.insert(a, (a, 1));

        let mut root = a;
        if left {
            root = self.union(a - 1, a);
        }
        if right {
            root = self.union(a, a + 1);
        }

        self.size(root)
    }

    fn union(&mut self, a: i32, b: i32) -> i32 {
        let rta = self.root(a);
        let rtb = self.root(b);
        let sza = self.size(rta);
        let szb = self.size(rtb);
        self.fa.insert(rta, (rtb, sza + szb));
        self.fa.insert(rtb, (rtb, sza + szb));
        rtb
    }

    fn root(&mut self, a: i32) -> i32 {
        let entry = self.fa.get(&a).unwrap();
        let sz = entry.1;
        let mut rt = entry.0;
        if rt != a {
            rt = self.root(rt);
            self.fa.insert(a, (rt, sz)); // 路径压缩
        }
        rt
    }

    fn size(&mut self, a: i32) -> i32 {
        let rt = self.root(a);
        self.fa.get(&rt).unwrap().1
    }

    pub fn contains(&mut self, a: i32) -> bool {
        self.fa.contains_key(&a)
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        Self::sort_solution(nums)
        //Self::union_find_solution(nums)
    }

    fn sort_solution(nums: Vec<i32>) -> i32 {
        /**
         * 排序解法, 简单高效，用这个
         */
        let uniq_nums = nums
            .into_iter()
            .collect::<BTreeSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        let mut res = 0i32;
        let mut i = 0usize;
        while i < uniq_nums.len() {
            let mut j = i + 1;
            while j < uniq_nums.len() && uniq_nums[j - 1] + 1 == uniq_nums[j] {
                j += 1;
            }
            res = res.max((j - i) as i32);
            i = j;
        }
        res
    }

    fn union_find_solution(nums: Vec<i32>) -> i32 {
        let mut uf = UnionFind::default();
        let mut res = 0i32;

        for num in nums.into_iter() {
            if uf.contains(num) {
                continue;
            }
            res = res.max(uf.add(num));
        }

        res
    }
}
// @lc code=end
