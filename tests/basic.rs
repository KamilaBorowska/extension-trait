#[macro_use]
extern crate extension_trait;

extension_trait! { Between for str {
    fn between(&self, front: &str, end: &str) -> Option<&str> {
        self.after(front).and_then(|t| t.before(end))
    }

    fn before(&self, end: &str) -> Option<&str> {
        self.split(end).next()
    }

    fn after(&self, front: &str) -> Option<&str> {
        self.splitn(2, front).nth(1)
    }
} }

#[test]
fn between_works() {
    assert_eq!("<a>".between("<", ">"), Some("a"));
}

#[test]
fn before_works() {
    assert_eq!("<a>".before(">"), Some("<a"));
}

#[test]
fn after_works() {
    assert_eq!("<a>".after("a"), Some(">"));
}

mod x {
    extension_trait! { pub Public for i32 {
        fn method(self) -> i32 {
            42
        }
    } }
}

#[test]
fn pub_extension_trait() {
    use x::Public;
    assert_eq!(24i32.method(), 42);
}

extension_trait! { <T> pub Length for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
} }

#[test]
fn generic_extension_traits() {
    assert_eq!(vec!["q"].size(), 1);
}

extension_trait! { pub ReturnArgument for () {
    fn return_argument<T>(&self, arg: T) -> String
    where
        T: ToString,
    {
        arg.to_string()
    }
} }

#[test]
fn generic_function_extension_traits() {
    assert_eq!(().return_argument(42), "42");
}

extension_trait! { <T: Copy + Into<String>> pub DoubleBracketConversion for Vec<T> {
    fn first_into_string(&self) -> String {
        self[0].into()
    }
} }

#[test]
fn double_bracket_conversion() {
    assert_eq!(vec!["asdf"].first_into_string(), String::from("asdf"));
}

extension_trait! { <T> pub DoubleBracketConversionUsingWhere for Vec<T>
where
    T: Copy + Into<String>
{
    fn first_into_string_using_where(&self) -> String {
        self[0].into()
    }
} }

#[test]
fn double_bracket_conversion_using_where() {
    assert_eq!(
        vec!["asdf"].first_into_string_using_where(),
        String::from("asdf")
    );
}

extension_trait! { <'a, T> pub Unsafe for (&'a mut T, &'a mut T) {
    unsafe fn swap(&mut self) {
        std::ptr::swap(self.0, self.1)
    }
} }

#[test]
fn unsafe_method() {
    let mut a = 1;
    let mut b = 2;
    unsafe {
        (&mut a, &mut b).swap();
    }
    assert_eq!(a, 2);
    assert_eq!(b, 1);
}

extension_trait! { <T: Copy> pub SliceMapExt<T> for [T] {
    fn map_in_place<F: FnMut(T) -> T>(&mut self, mut f: F) {
        for v in self {
            *v = f(*v);
        }
    }
}}

#[test]
fn slice_map_ext() {
    let mut values = [1, 2, 3];
    values.map_in_place(|x| x + 1);
    assert_eq!(values, [2, 3, 4]);
}

extension_trait! {
    /// This extension trait is documented
    Documented for () {
        /// A function is documented too.
        fn documented(self) {}
    }
}

#[test]
fn documented() {
    assert_eq!(().documented(), ());
}
