#![allow(dead_code)]
/**
 希尔排序：
 也称递减增量排序算法，是插入排序的一种更高效的改进版本
 希尔排序的基本思想是：先将整个待排序的记录序列分割成为若干子序列分别进行直接插入排序，
 待整个序列中的记录“基本有序”时，再对全体记录进行依次直接插入排序。

算法步骤:
1.选择一个增量序列 t1，t2，……，tk，其中 ti > tj, tk = 1；
2.按增量序列个数 k，对序列进行 k 趟排序；
3.每趟排序，根据对应的增量 ti，将待排序列分割成若干长度为 m 的子序列，分别对各子表进行直接插入排序。
仅增量因子为 1 时，整个序列作为一个表来处理，表长度即为整个序列的长度。
 */

 pub fn shell_sort(nums: &mut [i32]) {
    fn insertion_with_gap(nums: &mut [i32], start: usize, gap: usize) {
        // 带间隙的插入排序
        for index in ((start + gap)..nums.len()).step_by(gap) {
            let current_value = nums[index];
            let mut current_index = index;
            // 存储当前的数值，顺序挪动数字，减少swap交换的重复移位
            while current_index >= gap && nums[current_index - gap] > current_value {
                nums[current_index] = nums[current_index - gap]; // 向后移动数据
                current_index -= gap;
            } 
            // 找到属于自己的位置，回填值
            nums[current_index] = current_value;
        }
    } 
    let mut gap = nums.len() / 2;
    // 修改间隙
    while gap > 0 {
        for start in 0..gap {
            insertion_with_gap(nums, start, gap);
        }
        gap /= 2;
    }
}


pub fn shell_sort1(nums: &mut [i32]){
    fn insertion_with_gap(nums: &mut [i32], gap: usize) {
        for i in gap..nums.len() {
            let cur_num = nums[i];
            let mut j = i - gap;
            loop {
                if nums[j] > cur_num {
                    nums[j + gap] = nums[j];
                    // 当首元素也移动之后，loop应该要结束
                    // 将当前要插入的元素移动到首元素的位置
                    if j < gap {
                        nums[j] = cur_num;
                        break;
                    }
                } else {
                    nums[j + gap] = cur_num;
                    break;
                }

                j -= gap;
            }
        }
    }
    let mut gap = nums.len() >> 1;
    while gap > 0 {
        insertion_with_gap(nums, gap);
        gap >>= 1;
    }

}