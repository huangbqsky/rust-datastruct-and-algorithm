#![allow(dead_code)]

/**
基数排序：时间复杂度是 O(nk), 空间复杂度O(n + k)
它利用正数的进制规律来排序，基本是收集分配这样一思路，

算法步骤：
1.将所有待比较数值（正整数）统一为同样的数位长度，数位较短的数前面补零
2.从最低位开始，依次进行一次排序（比如个、十、百、千分别为在1，2, 3, 4位）
3.从最低位排序一直到最高位排序完成以后, 数列就变成一个有序序列
*/

// 基数排序，非比较排序，数据不小于0，利用正数的进制规律来做收集分配，实现排序
pub fn radix_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {return;}

    // 找到最大的数，取最大值的位数
    let max_sum = match nums.iter().max() {
        Some(&x) => x,
        None => return
    };

    // 找最接近且 >= nums 长度的 2 的次幂值作为桶大小，如 ：
    // 最接近且 >= 10 的 2 的次幂值是 2^4 = 16
    // 最接近且 >= 17 的 2 的次幂值是 2^5 = 32
    let radix = nums.len().next_power_of_two();

    // digit 代表小于某个位对应桶的所有数
    // 个、十、 百、千分别为在 1，2, 3, 4 位
    // 起始从个位开始，所以是 1
    let mut digit = 1;
    while digit <= max_sum {
        // 获取数据在桶中的位置
        let index_of = |x| (x / digit) as usize % radix;

        // 计数器
        let mut counter = vec![0;radix];
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }

        for i in 1..radix {
            counter[i] += counter[i-1];
        }

        // 排序
        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }

        // 跳转至下一个桶
        digit *= radix as i32;
    }

}

// 所有元素必须是非负数
pub fn radix_sort1(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    // setup buckets 10*n
    let mut buckets = Vec::with_capacity(10);
    for _ in 0..10 {
        buckets.push(Vec::new());
    }

    // setup 1 找到最大的数，求出其有多少位
    let max_digits = get_max_digits(nums);

    // setup 2 从最低位开始，依次进行循环
    for i in 1..= max_digits {
        let base = 10i32.pow((i - 1) as u32);
        let nums_size = nums.len();

        // 从个位开始，通过个位的数字找到对应的桶，将原数放入到桶中
        for j in 0..nums_size {
            let idx = (nums[j] / base % 10) as usize;
            buckets[idx].push(nums[j]);
        }

        // 依次从桶中取数，放入原数组中，一轮排序结束
        let mut k = 0usize;
        for i in 0..10 {
            for j in 0..buckets[i].len() {
                let bucket = &buckets[i];
                nums[k] = bucket[j];
                k += 1;
            }
        }

        // 清空桶中的数据，便于下一次迭代
        for i in 0..10 {
            buckets[i].clear();
        }
    }
}

fn get_max_digits(nums: &[i32]) -> usize {
    let mut max_val = i32::MIN;
    for i in 0..nums.len() {
        if nums[i] > max_val {
            max_val = nums[i];
        }
    }
    if max_val < 10 {
        1
    } else {
        let mut digits = 0usize;
        while max_val > 0 {
            max_val /= 10;
            digits += 1;
        }
        digits
    }
}