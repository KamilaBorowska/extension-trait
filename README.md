# extension-trait

A macro to declare extension traits - a trait that is created to add
methods to an external type.

# Example

```rust
#[macro_use]
extern crate extension_trait;

extension_trait! { pub DoubleExt for str {
   fn double(&self) -> String {
       self.repeat(2)
   }
} }

fn main() {
    assert_eq!("Hello".double(), "HelloHello");
}
```

It's also possible to use generic types. The declaration matches how
`impl` looks, requiring specifying type parameters for a trait itself.

```rust
#[macro_use]
extern crate extension_trait;

extension_trait! { <T: Copy> pub SliceMapExt<T> for [T] {
    fn map_in_place<F: FnMut(T) -> T>(&mut self, mut f: F) {
        for v in self {
            *v = f(*v);
        }
    }
} }

fn main() {
    let mut values = [1, 2, 3];
    values.map_in_place(|x| x + 1);
    assert_eq!(values, [2, 3, 4]);
}
```
