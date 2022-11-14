// Unit Tests
use super::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        struct TestData {
            param1: String,
            expected: bool,
        }

        let tests = [TestData{
            param1: String::from("thisunqe"),
            expected: true,
        },
        TestData{
            param1: String::from("thisisnotunique"),
            expected: false,
        },
        TestData{
            param1: String::from("true"),
            expected: true,
        },
        TestData{
            param1: String::from(r#" "!#$%&'()*+,-/0123456789:;<=>?@\^_AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz{|}~"#),
            expected: true,
        },
        TestData{
            param1: String::from(r#" "!#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_abcdefghijklmnopqrstuvwxyz{|}~REAPEATED"#),
            expected: false,
        }
        ];
        for test in tests{
            // println!("With input: {}", test.param1);
            let actual = is_unique(test.param1);
            // println!("I got the value {}", actual);
            // println!("What I expected: {}", test.expected);
            assert_eq!(actual, test.expected);
        }
    }

    #[test]
    fn test_string_compression() {
        struct TestData {
            param1: String,
            expected: String,
        }

        let tests = [TestData{
            param1: String::from("aabbbcc"),
            expected: String::from("a2b3c2"),
        },
        TestData{
            param1: String::from("abcdefg"),
            expected: String::from("abcdefg"),
        },
        TestData{
            param1: String::from("aabcdefg"),
            expected: String::from("aabcdefg"),
        },
        TestData{
            param1: String::from("aaabbbcdeffg"),
            expected: String::from("a3b3cdef2g"),
        },
        TestData{
            param1: String::from(""),
            expected: String::from(""),
        },
        TestData{
            param1: String::from("a"),
            expected: String::from("a"),
        },
        TestData{
            param1: String::from("aaaAAaa"),
            expected: String::from("a3A2a2"),
        },
        ];
        for test in tests{
            // println!("With input: {}", test.param1);
            let actual = string_compression(test.param1);
            // println!("I got the value {}", actual);
            // println!("What I expected: {}", test.expected);
            assert_eq!(actual, test.expected);
        }
    }

    #[test]
    fn test_urlify() {
        struct TestData<'a> {
            param1: &'a mut String,
            expected: &'a mut String,
        }

        let tests = [TestData{
            param1: &mut String::from("This Test"),
            expected: &mut String::from("This%20Test"),
        },
        TestData{
            param1: &mut String::from("This is a Test. Please try this... "),
            expected: &mut String::from("This%20is%20a%20Test.%20Please%20try%20this...%20"),
        },
        ];
        for test in tests{
            // println!("With input: {}", test.param1);
            print!("Before: {}", test.param1);
            urlify2(test.param1);
            print!("After: {}", test.param1);
            // println!("I got the value {}", actual);
            // println!("What I expected: {}", test.expected);
            assert_eq!(test.param1, test.expected);
        }
    }

    #[test]
    fn test_string_rotation() {
        struct TestData {
            param1: String,
            param2: String,
            expected: bool,
        }

        let tests = [TestData{
            param1: String::from("waterbottle"),
            param2: String::from("tlewaterbot"),
            expected: true,
        },
        TestData{
            param1: String::from("tlewaterbot"),
            param2: String::from("waterbottle"),
            expected: true,
        },
        TestData{
            param1: String::from("waterbottle"),
            param2: String::from("waterbottle"),
            expected: true,
        },
        TestData{
            param1: String::from(""),
            param2: String::from(""),
            expected: true,
        },
        TestData{
            param1: String::from("a"),
            param2: String::from("b"),
            expected: false,
        },
        TestData{
            param1: String::from("aa"),
            param2: String::from("a"),
            expected: false,
        },
        TestData{
            param1: String::from(r#" "!#$%&'()*+,-/0123456789:;<=>?@\^_AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz{|}~"#),
            param2: String::from(r#"eFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz{|}~ "!#$%&'()*+,-/0123456789:;<=>?@\^_AaBbCcDdE"#),
            expected: true,
        }
        ];
        for test in tests{
            // println!("With input: {}", test.param1);
            let actual = string_rotation(test.param1, test.param2);
            // println!("I got the value {}", actual);
            // println!("What I expected: {}", test.expected);
            assert_eq!(actual, test.expected);
        }
    }
}