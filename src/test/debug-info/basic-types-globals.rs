// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Caveats - gdb prints any 8-bit value (meaning rust I8 and u8 values)
// as its numerical value along with its associated ASCII char, there
// doesn't seem to be any way around this. Also, gdb doesn't know
// about UTF-32 character encoding and will print a rust char as only
// its numerical value.

// ignore-android: FIXME(#10381)

// compile-flags:-g
// debugger:rbreak zzz
// debugger:run
// debugger:finish
// debugger:print B
// check:$1 = false
// debugger:print I
// check:$2 = -1
// debugger:print C
// check:$3 = 97
// debugger:print/d I8
// check:$4 = 68
// debugger:print I16
// check:$5 = -16
// debugger:print I32
// check:$6 = -32
// debugger:print I64
// check:$7 = -64
// debugger:print U
// check:$8 = 1
// debugger:print/d U8
// check:$9 = 100
// debugger:print U16
// check:$10 = 16
// debugger:print U32
// check:$11 = 32
// debugger:print U64
// check:$12 = 64
// debugger:print F32
// check:$13 = 2.5
// debugger:print F64
// check:$14 = 3.5
// debugger:detach
// debugger:quit

#[allow(unused_variable)];

pub static B: bool = false;
pub static I: int = -1;
pub static C: char = 'a';
pub static I8: i8 = 68;
pub static I16: i16 = -16;
pub static I32: i32 = -32;
pub static I64: i64 = -64;
pub static U: uint = 1;
pub static U8: u8 = 100;
pub static U16: u16 = 16;
pub static U32: u32 = 32;
pub static U64: u64 = 64;
pub static F32: f32 = 2.5;
pub static F64: f64 = 3.5;

fn main() {
    _zzz();
}

fn _zzz() {()}
