
/**
选择排序: 时间复杂度为O(n^2), 空间复杂度为O(1)
选择排序是对冒泡排序的改进，每次遍历集合只做一次交换，比冒泡排序更快。

算法步骤:
 1.首先在未排序序列中找到最小（大）元素，存放到排序序列的起始位置
 2.再从剩余未排序元素中继续寻找最小（大）元素，然后放到已排序序列的末尾。
 3.重复第二步，直到所有元素均排序完毕。
*/ 

// 方案1 ：从首位开始递增，找到最小值，放前面
pub fn select_sort(nums: &mut [i32]){
   let len = nums.len();
   if len < 1 { return; }
   
   // 总共要经过 N-1 轮比较
   for i in 0..len - 1  {
      let mut min = i; // 最小元素下标
      // 每轮需要比较的次数 N-i
      for j in i + 1..len {
        if nums[j] < nums[min] {
            min = j;
        }
      }
      // 将找到的最小值和i位置所在的值进行交换
      if i != min {
         nums.swap(i, min)
      }
   }
}

// 方案2，从最左侧（最大下标处）递减，找出最大值，放左侧
pub fn bubble_sort1(nums: &mut [i32]) {
    let mut left = nums.len() - 1; // 待排序数据下标
    while left > 0 {
        let mut max_index = 0; 
        for i in 1..left {
            if nums[i] > nums[max_index]{
                max_index = i; //选择当前轮次最大值的下标
            }
        }
        // 数据交换，完成一个数据的排序，待排序的数据量减1
        nums.swap(left, max_index);
        left -= 1;
    }
}