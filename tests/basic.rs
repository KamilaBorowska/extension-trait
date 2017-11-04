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
