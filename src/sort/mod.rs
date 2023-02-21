

pub mod bubble_sort; // 冒泡排序
pub mod select_sort; // 选择排序
pub mod insert_sort; // 插入排序
pub mod shell_sort;  // 希尔排序



#[cfg(test)]
mod test{
   use crate::sort::bubble_sort::bubble_sort;
   use crate::sort::select_sort::select_sort;
   use crate::sort::insert_sort::insert_sort;
   use crate::sort::shell_sort::shell_sort;

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
     // 选择排序
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

   }

   #[test]
   fn test_sort_vec(){
      let origin = [8, 6, 7, 5, 0, 100, -1, 8, 9, 2, 11];
      let output = [-1, 0, 2, 5, 6, 7, 8, 8, 9, 11, 100];
       // 冒泡排序
      {
        let mut input = origin.clone();
        bubble_sort(&mut input);
        assert_eq!(input, output);
      }

      {
        let mut input = origin.clone();
        select_sort(&mut input);
        assert_eq!(input, output);
      }
      // 选择排序
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
   }
}