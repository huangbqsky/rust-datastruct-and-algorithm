#![allow(dead_code)]

/**
 * 计数排序：时间复杂度O(n+k), 空间复杂度O(n+k)， 属于选择非比较类排序
 * 原理是利用数组的下标来确定元素的位置
 * 计数排序是桶排序的特殊情况，它的桶就只处理同种数据，所以比较费空间
 * 
 * 算法步骤：
 * 第一步，初始化长度为maxV-minV+1的计数器集合，值全为其中maxV为待排序集合的最大值，minV为最小值。
 * 第二步，扫描待排序集合，以当前值减minV作下标，并对计数器中此下标的计数加1。
 * 第三步，扫描一遍计数器集合，按顺序把值写回原集合，完成排序。
 */

pub fn count_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }
   // step 1：取得最大最小值
   let mut min_val = i32::MAX;
   let mut max_val = i32::MIN;
   for i in 0..nums.len() {
       if nums[i] > max_val {
           max_val = nums[i];
       } else if nums[i] < min_val {
           min_val = nums[i];
       }
   }
    // step 2: 将数据标记到桶
    let bucket_nums = max_val - min_val + 1;
    let mut buckets = vec![0; bucket_nums as usize];
    for i in 0..nums.len() {
        buckets[(nums[i] - min_val) as usize] += 1;
    }

    // step 3: 数据写回原nums
    let mut j = 0;
    for i in 0..bucket_nums as usize {
        while buckets[i] > 0 {
            nums[j] = i as i32 + min_val;
            j += 1;
            buckets[i] -= 1;
        }
    }
}