#![allow(dead_code)]

/**
 * 堆排序：时间复杂度O(nlog(n))，空间复杂度O(1)，属于选择比较类排序
 * 堆排序的基本思想是：
 * 1.将待排序序列构造成一个小顶堆，此时，整个序列的最小值就是 堆顶根节点。
 * 2.将其与末尾元素进行交换，此时末尾就为最小值。这个最小值不再计算到堆内，
 * 3.再将剩余的 n - 1个元素重新构造成一个堆，这样会得到一个新的最小值。此时将该最小值再次交换到新堆的末尾，这样就有了两个排序的值。
 * 4.重复这个过程，直到得到一个有序序列
 * 
算法步骤
1.创建一个堆 H[0……n-1]；
2.把堆首（最大值）和堆尾互换；
3.把堆的尺寸缩小 1，并调用 shift_down(0)，目的是把新的数组顶端数据调整到相应位置；
4.重复步骤 2，直到堆的尺寸为 1。
 */
pub fn heap_sort(nums: &mut [i32]){
    if nums.len() <= 1 { return; }

    let len = nums.len();
    // 建堆，从尾到首
    for index in (0..len/2).rev() {
        heapify(nums, index, len);
    }
    // 收缩堆，从尾到首
    for index in (1..len).rev(){
        // 1. 最大的移动到末尾
        // 2. 重建堆（收缩）
        nums.swap(0, index);
        heapify(nums, 0, index)
    }
}

// 建堆小顶堆
fn heapify(nums: &mut [i32], root: usize, end: usize) {
    let mut largest = root;
    // 找到最大值： 左右孩子
    let left = 2 * root + 1; // 左孩子
    if left < end && nums[left] > nums[largest] {
        largest = left;
    }
    let right = left + 1; // 右孩子
    if right < end && nums[right] > nums[largest]{
        largest = right;
    }
    // 交换最大值
    if largest != root {
        nums.swap(largest, root);
        heapify(nums, largest, end);
    }

}