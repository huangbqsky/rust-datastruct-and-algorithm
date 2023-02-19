// 冒泡排序, 时间复杂度为O(n^2), 空间复杂度为O(1)
pub fn bubble_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() - 1 {
            if num[j] > num[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}