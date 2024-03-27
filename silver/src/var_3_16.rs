// 1. 기본적인 테스트 함수 작성
#[cfg(test)]
pub mod tests {
    use super::*;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

/* 
// 2. 여러 개의 테스트 함수 작성
#[cfg(test)]
pub mod tests {
    use super::*;

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 2), 3);
    }
}

// 3. 테스트 모듈화 및 그룹화
#[cfg(test)]
pub mod tests {
    use super::*;

    pub mod math_operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_add() {
                assert_eq!(add(2, 3), 5);
            }

            #[test]
            fn test_subtract() {
                assert_eq!(subtract(5, 2), 3);
            }
        }
    }
}
*/
// 4. 여러 개의 테스트 파일 작성
// src/math_operations.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[test]
pub fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
pub fn test_subtract() {
    assert_eq!(subtract(5, 2), 3);
}
