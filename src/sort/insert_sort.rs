#![allow(dead_code)]
/**
插入排序: 时间复杂度为O(n^2), 空间复杂度为O(1)
始终在数据集的较低位置处维护一个有序的子序列，然后将新项插入子序列，使得子序列扩大，最终实现集合排序

算法步骤：
1.将第一待排序序列第一个元素看做一个有序序列，把第二个元素到最后一个元素当成是未排序序列。
2.从头到尾依次扫描未排序序列，将扫描到的每个元素插入有序序列的适当位置。
（如果待插入的元素与有序序列中的某个元素相等，则将待插入元素插入到相等元素的后面。）
 */

pub fn insert_sort(nums: &mut [i32]){
    if nums.len() <= 1  { return; }
    // 从下标为1的元素开始选择合适的位置插入，因为下标为0的只有一个元素，默认是有序的
    for i in 1..nums.len() {
        // 需要插入的记录
        let cur = nums[i];
        // 从已经排序的序列最右边的开始比较，找到比其小的值
        let mut pos = i;
        while pos > 0 && cur < nums[pos-1] {
            // nums.swap(pos, pos -1); // 向后移动数据
            nums[pos] = nums[pos - 1]; // 向后移动数据
            pos -= 1;
        }

        // 存在比其小的数，插入
        if pos != i {
            nums[pos] = cur;
        }
    }

}