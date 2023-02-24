extern crate unwrap_fields;

#[cfg(test)]
mod tests {
    use unwrap_fields::UnwrapFields;

    #[derive(UnwrapFields)]
    struct MyStruct {
        foo: Option<String>,
        // bar: i32,
        baz: Option<Vec<i32>>,
    }

    #[test]
    fn test_derive() {
        let my_struct = MyStruct {
            foo: Some("hello".to_string()),
            // bar: 42,
            baz: Some(vec![1, 2, 3]),
        };

        assert_eq!(my_struct.foo(), "hello");
        // assert_eq!(my_struct.bar(), &42);
        assert_eq!(my_struct.baz(), &[1, 2, 3]);
    }
}
