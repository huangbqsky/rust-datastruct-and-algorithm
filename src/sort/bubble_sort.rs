#![allow(dead_code)]
/** 
冒泡排序： 时间复杂度为O(n^2), 空间复杂度为O(1)
算法步骤:
1. 比较相邻的元素。如果第一个比第二个大，就交换他们两个。
2. 对每一对相邻元素作同样的工作，从开始第一对到结尾的最后一对。这步做完后，最后的元素会是最大的数。
3. 针对所有的元素重复以上的步骤，除了最后一个。
4. 持续每次对越来越少的元素重复上面的步骤，直到没有任何一对数字需要比较。
*/

// 方案1：从第一个数开始，依次往后比较，相邻的元素两两比较，根据大小来交换元 素的位置
pub fn bubble_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    for i in 1..nums.len() {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}
// 方案2：从后往前递减，相邻的元素两两比较，根据大小来交换元素的位置
fn bubble_sort2(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    let mut len = nums.len() - 1;  // 最大索引下标
    while len > 0 { 
        for i in 0..len { 
            if nums[i] > nums[i+1] { 
                nums.swap(i, i+1); 
            } 
        }
        len -= 1;
    }
}

// 方案3 ：添加一个 compare 变量来控制是否继续比较，在遇到已排序集合时直接退出
fn bubble_sort3(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    let mut compare = true; // 否继续比较
    let mut len = nums.len() - 1; // 最大索引下标
    while len > 0 && compare {
         compare = false;
         for i in 0..len { 
            if nums[i] > nums[i+1] {
                nums.swap(i, i+1);
                compare = true; // 数据无序，还需继续比较 
            } 
        }
        len -= 1;
    }
}
