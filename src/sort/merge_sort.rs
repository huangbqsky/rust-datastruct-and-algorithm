#![allow(dead_code)]

/**
 * 归并排序: 时间复杂度O(nlogn)，空间复杂度为 O(n)
 * 使用分而治之策略作为提高排序算法性能的另外一种方法，通过不断将列表折半来进行排序
 *
算法步骤
1. 申请空间，使其大小为两个已经排序序列之和，该空间用来存放合并后的序列；
2. 设定两个指针，最初位置分别为两个已经排序序列的起始位置；
3. 比较两个指针所指向的元素，选择相对小的元素放入到合并空间，并移动指针到下一位置；
4. 重复步骤 3 直到某一指针达到序列尾；
5. 将另一序列剩下的所有元素直接复制到合并序列尾。
 */
pub fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1; // 中间位置
        merge_sort(&mut nums[..mid]); // 排序前半部分
        merge_sort(&mut nums[mid..]); // 排序后半部分
        merge(nums, mid); // 合并排序结果
    }
}

// 合并排序结果
pub fn merge(nums: &mut [i32], mid: usize) {
    let mut temp_vec = Vec::with_capacity(nums.len());
    let mut i = 0; // 标记前半部分数据
    let mut j = mid; // 标记后半部分数据
    for _ in 0..nums.len() {
        if i == mid || j == nums.len() {
            break;
        }
        // 数据放到临时集合temp
        if nums[i] <= nums[j] {
            temp_vec.push(nums[i]);
            i += 1;
        } else {
            temp_vec.push(nums[j]);
            j += 1;
        }
    }

    // 合并的两部分数据长度大概率不一样长
    // 所以要将未处理完集合的数据全部加入

    // i这个序列还有剩余元素
    while i < mid {
        temp_vec.push(nums[i]);
        i += 1;
    }
    // j这个序列还有剩余元素
    while j < nums.len() {
        temp_vec.push(nums[j]);
        j += 1;
    }

    // temp数据放回 nums， 完成排序
    for i in 0..temp_vec.len() {
        nums[i] = temp_vec[i];
    }
}
