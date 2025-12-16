/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    // 实现冒泡排序
    let len = array.len();
    
    // 如果数组长度小于等于1，已经是有序的
    if len <= 1 {
        return;
    }
    
    // 外层循环控制排序轮数
    for i in 0..len {
        // 内层循环进行相邻元素比较和交换
        // 每轮排序后，最大的元素会"冒泡"到末尾
        // 所以下一轮可以少比较一个元素（len - i - 1）
        let mut swapped = false;
        
        for j in 0..len - i - 1 {
            // 如果前一个元素比后一个元素大，交换它们
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // 如果在某轮循环中没有发生任何交换，说明数组已经有序
        // 可以提前结束排序
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
