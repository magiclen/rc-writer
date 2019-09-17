/*!
# Rc Writer

A tiny implement for writing data to a reference counted instance.

## Examples

### RcWriter

```rust
extern crate rc_writer;

use rc_writer::RcWriter;

use std::rc::Rc;

use std::cell::RefCell;

use std::io::Write;

let data = RefCell::new(Vec::new());

let data_rc = Rc::new(data);

let mut writer = RcWriter::new(data_rc.clone());

writer.write(b"Hello world!").unwrap();

writer.flush().unwrap();

assert_eq!(b"Hello world!".to_vec(), *data_rc.borrow());
```

### RcOptionWriter

```rust
extern crate rc_writer;

use rc_writer::RcOptionWriter;

use std::rc::Rc;

use std::cell::RefCell;

use std::io::Write;

let data = RefCell::new(Some(Vec::new()));

let data_rc = Rc::new(data);

let mut writer = RcOptionWriter::new(data_rc.clone());

writer.write(b"Hello world!").unwrap();

writer.flush().unwrap();

let data = data_rc.borrow_mut().take().unwrap(); // remove out the vec from rc

assert_eq!(b"Hello world!".to_vec(), data);
```
*/

mod rc_option_writer;
mod rc_writer;

pub use self::rc_writer::RcWriter;
pub use rc_option_writer::RcOptionWriter;
