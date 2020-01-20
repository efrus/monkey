#[cfg(test)]
mod tests {
    use crate::object;
    use crate::object::{Object, ObjectType};
    //use crate::token::Token;

    #[test]
    fn test_string_hash_key() {
        let hello1 = Object::String("Hello World".to_string());
        let hello2 = Object::String("Hello World".to_string());
        let diff1 = Object::String("My name is johnny".to_string());
        let diff2 = Object::String("My name is johnny".to_string());

        if object::create_hash_key(hello1.clone()) != object::create_hash_key(hello2) {
            println!("strings with same content have different hash keys");
            assert!(false);
        }

        if object::create_hash_key(diff1.clone()) != object::create_hash_key(diff2) {
            println!("strings with same content have different hash keys");
            assert!(false);
        }

        if object::create_hash_key(hello1) == object::create_hash_key(diff1) {
            println!("strings with different content have same hash keys");
            assert!(false);
        }
    }

    #[test]
    fn test_bool_hash_key() {
        let t1 = Object::Boolean(true);
        let f1 = Object::Boolean(false);
        let t2 = Object::Boolean(true);

        if object::create_hash_key(t1.clone()) != object::create_hash_key(t2) {
            println!("bools with same val not the same");
            assert!(false);
        }

        if object::create_hash_key(t1) == object::create_hash_key(f1) {
            println!("bools should be different");
            assert!(false);
        }
    }

    #[test]
    fn test_int_hash_key() {
        let i1 = Object::Integer(10);
        let i2 = Object::Integer(12);
        let i3 = Object::Integer(10);

        if object::create_hash_key(i1.clone()) != object::create_hash_key(i3) {
            println!("ints with same content have different hash keys");
            assert!(false);
        }

        if object::create_hash_key(i1) == object::create_hash_key(i2) {
            println!("ints with diff content have same hash keys");
            assert!(false);
        }
    }
}
