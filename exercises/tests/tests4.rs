#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
        
        // 测试其他有效值
        let rect2 = Rectangle::new(1, 1);  // 最小有效值
        assert_eq!(rect2.width, 1);
        assert_eq!(rect2.height, 1);
        
        let rect3 = Rectangle::new(100, 200);  // 更大的值
        assert_eq!(rect3.width, 100);
        assert_eq!(rect3.height, 200);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height cannot be negative!")]
    fn negative_width() {
        // 可以添加 expected 参数来检查 panic 消息
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
        // 0 也会触发 panic，因为条件检查是 width <= 0
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
