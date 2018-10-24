Rc Writer
====================

[![Build Status](https://travis-ci.org/magiclen/rc-writer.svg?branch=master)](https://travis-ci.org/magiclen/rc-writer)
[![Build status](https://ci.appveyor.com/api/projects/status/4pahg84urfpyls8a/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/rc-writer/branch/master)

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

## Crates.io

https://crates.io/crates/rc-writer

## Documentation

https://docs.rs/rc-writer

## License

[MIT](LICENSE)