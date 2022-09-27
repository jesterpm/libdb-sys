**Status**

![CI](https://github.com/fishi0x01/libdb-sys/workflows/CI/badge.svg)
[![crates.io badge](https://img.shields.io/crates/v/libdb-sys.svg)](https://crates.io/crates/libdb-sys)
[![docs.rs badge](https://docs.rs/libdb-sys/badge.svg)](https://docs.rs/libdb-sys)

# libdb-sys

Statically linked bindings for Berkeley DB 4.8.x and 5.3.x.

This is a humble fork from [jesterpm's](https://github.com/jesterpm/libdb-sys) libdb-sys.

## Features

`v4_8` uses bindings for Berkeley DB 4.8.x.

`v5_3` uses bindings for Berkeley DB 5.3.x.

By default, Berkeley DB 5.3.x is used. 

## crev

This crate has its author's [crev review](https://github.com/fishi0x01/crev-proofs).

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)
to verify the trustworthiness of each of your dependencies, including this one.

## Berkeley DB licensing notice

Website: http://www.oracle.com/database/berkeley-db/

License: Sleepycat

Description:
```
The Berkeley Database (Berkeley DB) is a programmatic toolkit that
provides embedded database support for both traditional and
client/server applications. The Berkeley DB includes B+tree, Extended
Linear Hashing, Fixed and Variable-length record access methods,
transactions, locking, logging, shared memory caching, and database
recovery. The Berkeley DB supports C, C++, Java, and Perl APIs. It is
used by many applications, including Python and Perl, so this should
be installed on all systems.
```
