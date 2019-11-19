# extension-trait

A macro to declare extension traits - a trait that is created to add
methods to an external type.

# Example

```rust
#[macro_use]
extern crate extension_trait;

#[extension_trait(pub)]
impl DoubleExt for str {
   fn double(&self) -> String {
       self.repeat(2)
   }
}

fn main() {
    assert_eq!("Hello".double(), "HelloHello");
}
```
