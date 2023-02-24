#![allow(dead_code)]

/**
桶排序：时间复杂度O(n+k), 空间复杂度O(n+k)， 属于选择非比较类排序
工作的原理：
将数组分到有限数量的桶里。每个桶再个别排序，最后依次把各个桶中的记录列出来记得到有序序列。
桶和哈希表中槽的概念是类似的，只是槽只能装一个元素， 而桶可以装若干元素。槽用于保存元素，桶用于排序元素

桶排序基本思路是：
第一步，将待排序元素划分到不同的桶，先遍历求出maxV和minV，设桶个数为k，
       则把区间[minV,maxV]均匀划分成k个区间，每个区间就是一个桶，
       将序列中的元素分配到各自的桶（求余法）。
第二步，对每个桶内的元素进行排序，排序算法可用任意排序算法。
第三步，将各个桶中的有序元素合并成一个大的有序集合。
 */
pub fn bucket_sort(nums: &mut [i32]){
     if nums.len() <= 1 {
        return;
     }

     // 第一步：取得最大最小值
     let mut min_val = i32::MAX;
     let mut max_val = i32::MIN;
     for i in 0..nums.len() {
         if nums[i] > max_val {
             max_val = nums[i];
         } else if nums[i] < min_val {
             min_val = nums[i];
         }
     }

    // 第二步：利用映射函数将数据分配到各个桶中
     let bucket_size = 5; // 桶数量
     let bucket_len = ((max_val - min_val) / bucket_size) + 1; // 每个桶区间范围（桶长度）

     let mut buckets = vec![vec![]; bucket_len as usize];
     for i in 0..nums.len() {
        let value = nums[i];
        // 分桶规则一定要定好，对于每个桶一定保证
        // x < y && max(x) < min(y)
        let idx = (nums[i] - min_val) / bucket_size;  // （求余法）
        buckets[idx as usize].push(value);
     }
     // 第三步：桶内排序
     for bucket in buckets.iter_mut() {
        super::insert_sort::insert_sort(bucket);
     }

    // 第四步：拆桶 ，将所有排序数据融合到一个Vec
    let mut ret = vec![];
     for bucket in buckets.iter() {
        for vp in bucket {
            ret.push(*vp);
        }
    }
    nums.clone_from_slice(&ret)
}