pub mod bubble_sort; // 冒泡排序
pub mod select_sort; // 选择排序
pub mod insert_sort; // 插入排序
pub mod shell_sort; // 希尔排序 
pub mod merge_sort; // 归并排序
pub mod quick_sort; // 快速排序
pub mod heap_sort; // 堆排序
pub mod bucket_sort; // 桶排序
pub mod count_sort; // 计数排序
pub mod radix_sort; // 基数排序

#[cfg(test)]
mod test {
    use crate::sort::bubble_sort::bubble_sort;

    use crate::sort::select_sort::select_sort;
    use crate::sort::insert_sort::insert_sort;
    use crate::sort::shell_sort::shell_sort;
    use crate::sort::merge_sort::merge_sort;
    use crate::sort::quick_sort::quick_sort;
    use crate::sort::heap_sort::heap_sort;
    use crate::sort::bucket_sort::bucket_sort;
    use crate::sort::count_sort::count_sort;
    use crate::sort::radix_sort::radix_sort;

    #[test]
    fn test_empty_vec() {
        let origin = [];
        let output = [];
        // 冒泡排序
        {
            let mut input = origin.clone();
            bubble_sort(&mut input);
            assert_eq!(input, output);
        }
        // 选择排序
        {
            let mut input = origin.clone();
            select_sort(&mut input);
            assert_eq!(input, output);
        }
        // 插入排序
        {
            let mut input = origin.clone();
            insert_sort(&mut input);
            assert_eq!(input, output);
        }
        // 希尔排序
        {
            let mut input = origin.clone();
            shell_sort(&mut input);
            assert_eq!(input, output);
        }
        // 归并排序
        {
            let mut input = origin.clone();
            merge_sort(&mut input);
            assert_eq!(input, output);
        }
        // 快速排序
        {
            let mut input = origin.clone();
            quick_sort(&mut input);
            assert_eq!(input, output);
        }
        // 堆排序
        {
            let mut input = origin.clone();
            heap_sort(&mut input);
            assert_eq!(input, output);
        }
        // 桶排序
        {
            let mut input = origin.clone();
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }
        // 计数排序
        {
            let mut input = origin.clone();
            count_sort(&mut input);
            assert_eq!(input, output);
        }
        
        // 基数排序
        {
            let mut input = origin.clone();
            radix_sort(&mut input);
            assert_eq!(input, output);
        }
    }

    #[test]
    fn test_one_element_vec() {
        let origin = [1];
        let output = [1];
        // 冒泡排序
        {
            let mut input = origin.clone();
            bubble_sort(&mut input);
            assert_eq!(input, output);
        }
        // 选择排序
        {
            let mut input = origin.clone();
            select_sort(&mut input);
            assert_eq!(input, output);
        }
        // 插入排序
        {
            let mut input = origin.clone();
            insert_sort(&mut input);
            assert_eq!(input, output);
        }
        // 希尔排序
        {
            let mut input = origin.clone();
            shell_sort(&mut input);
            assert_eq!(input, output);
        }
        // 归并排序
        {
            let mut input = origin.clone();
            merge_sort(&mut input);
            assert_eq!(input, output);
        }
        // 快速排序
        {
            let mut input = origin.clone();
            quick_sort(&mut input);
            assert_eq!(input, output);
        }
        // 堆排序
        {
            let mut input = origin.clone();
            heap_sort(&mut input);
            assert_eq!(input, output);
        }
        // 桶排序
        {
            let mut input = origin.clone();
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }
        // 计数排序
        {
            let mut input = origin.clone();
            count_sort(&mut input);
            assert_eq!(input, output);
        }
        // 基数排序
        {
            let mut input = origin.clone();
            radix_sort(&mut input);
            assert_eq!(input, output);
        }
    }

    #[test]
    fn test_sort_vec() {
        let origin = [8, 6, 7, 5, 0, 100, -1, 8, 9, 2, 11];
        let output = [-1, 0, 2, 5, 6, 7, 8, 8, 9, 11, 100];
        // 冒泡排序
        {
            let mut input = origin.clone();
            bubble_sort(&mut input);
            assert_eq!(input, output);
        }
        // 选择排序
        {
            let mut input = origin.clone();
            select_sort(&mut input);
            assert_eq!(input, output);
        }
        // 插入排序
        {
            let mut input = origin.clone();
            insert_sort(&mut input);
            assert_eq!(input, output);
        }
        // 希尔排序
        {
            let mut input = origin.clone();
            shell_sort(&mut input);
            assert_eq!(input, output);
        }
        // 归并排序
        {
            let mut input = origin.clone();
            merge_sort(&mut input);
            assert_eq!(input, output);
        }
        // 快速排序
        {
            let mut input = origin.clone();
            quick_sort(&mut input);
            assert_eq!(input, output);
        }
        // 堆排序
        {
            let mut input = origin.clone();
            heap_sort(&mut input);
            assert_eq!(input, output);
        }
        // 桶排序
        {
            let mut input = origin.clone();
            bucket_sort(&mut input);
            assert_eq!(input, output);
        }
        // 计数排序
        {
            let mut input = origin.clone();
            count_sort(&mut input);
            assert_eq!(input, output);
        }
        // 基数排序
        {
            // let mut input = origin.clone();
            let mut input = [54,32,99,18,75,31,43,56,21,22];
            let output = [18,21,22,31,32,43,54,56,75,99];
            radix_sort(&mut input);
            assert_eq!(input, output);
        }
    }
}
