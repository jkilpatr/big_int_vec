# Big int vec

A Rust library that provides arbitrary width integer operations
based off of the BitVec crate.

The intent is exactly emulate the behavior of two's compliment
fixed with addition for integers of arbitrary size. Currently
tested up to 256 bits.

Provides two structs uvec and ivec which are signed and unsigned
arbitrary width integers.

## Member functions are

Creates a new unsigned integer initialized with val of width size
    uvec::new(val: u64, size: usize)

Creates a new signed integer initialized with val of width size
    ivec::new(val: i64, size: usize)

The following operators are defined for pairs of uvec or ivec
    +, -, <, >, <=, >=

To retrieve the stored value you can use getval be warned this truncates
bits above 64, more useful for debugging than anything else.
    ivec::get_val() -> i64
    uvec::get_val() -> u64

## To use this crate add the following to the dependencies section of your Cargo file
    big_int_vec = { git = "https://github.com/jkilpatr/big_int_vec.git" }
