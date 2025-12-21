struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Rectangle {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!");
        }
        Rectangle { width, height }
    }
}

// 你的测试代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
        
        let rect2 = Rectangle::new(1, 1);
        assert_eq!(rect2.width, 1);
        assert_eq!(rect2.height, 1);
        
        let rect3 = Rectangle::new(100, 200);
        assert_eq!(rect3.width, 100);
        assert_eq!(rect3.height, 200);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
    
    #[test]
    #[should_panic]
    fn zero_width() {
        let _rect = Rectangle::new(0, 10);
    }
    
    #[test]
    #[should_panic]
    fn zero_height() {
        let _rect = Rectangle::new(10, 0);
    }
    
    #[test]
    #[should_panic]
    fn both_negative() {
        let _rect = Rectangle::new(-10, -20);
    }
}
