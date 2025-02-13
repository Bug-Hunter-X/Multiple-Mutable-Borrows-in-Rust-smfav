Several solutions exist depending on the intended behavior:

1. **Cloning:** Create a copy of `x` before making mutable changes

```rust
fn main() {
    let mut x = 5;
    let mut y = x; 
    let mut z = x;
    y += 1; 
    z += 2;
    x = y+z; 
    println!("x = {}", x);
}
```

2. **Interior Mutability (e.g., `RefCell` or `Mutex`):** If you need mutable access from multiple threads or contexts use interior mutability

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    {
        let mut y = x.borrow_mut();
        *y += 1;
    }
    {
        let mut z = x.borrow_mut();
        *z += 2;
    }
    println!("x = {}", *x.borrow());
}
```

3. **Refactoring:** Restructure your code to avoid the need for multiple mutable borrows.  Often, this involves breaking down your operations into smaller, more independent parts.