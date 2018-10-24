Rc Writer
====================

[![Build Status](https://travis-ci.org/magiclen/rc-writer.svg?branch=master)](https://travis-ci.org/magiclen/rc-writer)

A tiny implement for writing data to a reference counted instance.

## Example

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

## Crates.io

https://crates.io/crates/rc-writer

## Documentation

https://docs.rs/rc-writer

## License

[MIT](LICENSE)