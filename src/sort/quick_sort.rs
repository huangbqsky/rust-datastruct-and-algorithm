#![allow(dead_code)]
/**
 快速排序：时间复杂度O(nlog(n))，空间复杂度为O(nlog(n))
 快速排序和冒泡排序有相似之处，应该说快速排序是冒泡排序的升级版。快速排序使用 分治策略来加快排序速度
 快速排序只有两个步骤，一是选择中枢值，二是分区排序。

算法步骤：
1.从数列中挑出一个元素，称为 “基准”（pivot）;
2.重新排序数列，所有元素比基准值小的摆放在基准前面，所有元素比基准值大的摆在基准的后面（相同的数可以到任一边），
在这个分区退出之后，该基准就处于数列的中间位置。这个称为分区（partition）操作；
3.递归地（recursive）把小于基准值元素的子数列和大于基准值元素的子数列排序；
 */
pub fn quick_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let p = partition(nums); // 分区点位置
        quick_sort(&mut nums[0..p]); //排序左分区
        quick_sort(&mut nums[p + 1..]); // 排序右分区
    }
}
// 得到分区位置点，分区排序 （最常用的挖空法）
pub(self) fn partition(nums: &mut [i32]) -> usize {
    let pivot = nums[0]; // 中枢值
    let mut left = 0; // 左标记
    let mut right = nums.len() - 1; // 左标记
    while left < right {
        while left < right && nums[right] >= pivot {
            // 左移递减右标记
            right -= 1;
        }
        // 移动数据
        nums[left] = nums[right];

        while left < right && nums[left] < pivot {
            // 右移递增左标记
            left += 1;
        }
        // 移动数据
        nums[right] = nums[left];
    }
    // 填空
    nums[left] = pivot;
    // 左标记值作为分裂点
    return left;
}

// 得到分区位置点，分区排序
pub(self) fn partition1(nums: &mut [i32]) -> usize {
    let pivot = 0; // 中枢值位置
    let mut left = 0; // 左标记
    let mut right = nums.len() - 1; // 左标记
    while left < right {
        while left < right && nums[right] >= nums[pivot] {
            // 左移递减右标记
            right -= 1;
        }
        while left < right && nums[left] < nums[pivot] {
            // 右移递增左标记
            left += 1;
        }
        // 交换两端的差异值
        if left != right {
            nums.swap(left, right);
        }
    }
    // 选取的值放在最中间，保证分割线
    nums.swap(pivot, left);
    // 左标记值作为分裂点
    return left;
}


