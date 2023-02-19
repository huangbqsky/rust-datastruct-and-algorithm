


pub mod bubble_sort; // 冒泡排序



#[cfg(test)]
mod test{
   use super::*;

   #[test]
   fn test_empty_vec() { 
    let origin = [];
    let output = [];
    {
        let mut input = origin.clone();
        bubble_sort(&mut input);
        assert_eq!(input, output);
    }
   }

   #[test]
   fn test_one_element_vec() { 
    let origin = [1];
    let output = [1];
    {
        let mut input = origin.clone();
        bubble_sort(&mut input);
        assert_eq!(input, output);
    }
   }

   #[test]
   fn test_sort_vec(){
      let origin = [8, 6, 7, 5, 0, 100, -1, 8, 9, 2, 11];
      let output = [-1, 0, 2, 5, 6, 7, 8, 8, 9, 11, 100];
      {
        let mut input = origin.clone();
        bubble_sort(&mut input);
        assert_eq!(input, output);
      }
   }
}