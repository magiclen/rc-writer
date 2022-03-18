Rc Writer
====================

[![CI](https://github.com/magiclen/rc-writer/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/rc-writer/actions/workflows/ci.yml)

A tiny implement for writing data to a reference counted instance.

## Examples

### RcWriter

```rust
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

## Crates.io

https://crates.io/crates/rc-writer

## Documentation

https://docs.rs/rc-writer

## License

[MIT](LICENSE)