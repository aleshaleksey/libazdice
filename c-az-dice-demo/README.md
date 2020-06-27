# libazdice C/C++ ffi examples.

In the example in this directory, there is a C toy program which dynamically loads `liblibazdice.so` using `dlfcn.h` and parses the dice string "5d6dl2mn2", demonstrating each of the functions in the library.

## In brief.

libazdice has a C/C++ ffi which contains several functions. Firstly it includes three test functions.

- The `test` function, which is used to test whether the library is being loaded and that strings can be returned to C/C++.

- The `test2` function, which is used to test whether the library is loaded and that strings can be sent from C/C++ without causing 

- The `parse_and_roll2` function which is used to test whether the parser "works". It can be used for normal "work", but is not recommended as it does not return an error if a nonsense dice string is encountered, but instead crashes the thread.

It also includes three actual functions.

- `parse_and_roll`, which parses a dice string and rolls it once, returning either a total result or an error string. NB: If a more detailed result breakdown is wanted, use `parse_and_roll_n_times`.

- `parse_and_roll_n_times` will parse a dice string and roll it N times, returning either a detailed report of the rolls or an error string. Can be used to roll the dice once.

- `parse_and_generate_distribution` will parse a dice string and roll the dice N times, using the results to generate a probability distribution of rolls (count vs roll totals).

For the meanwhile the ffi does not include an exported version of a `DiceBag` and the dice string will have to be parsed each time a set of rolls is desired. This is not seen as a problem because

a) Although there are scenarios where `DiceBag`s can be cached and reused, it is not the expected use case.

b) Computationally speaking, parsing a dice string to a dice string is not too expensive (especially since they are generally quite short).

## Using libazdice in a ffi.

1) Compile the libazdice libraries. Currently it is compiled as a dylib (`.so`/`.dll`) or a rust lib (`.rlib`). It can be compiled as static lib, but this is currently not standard.

2) [Optional] If libazdice is compiled as a standard lib, statically like it to your C program or include in your make file.

3) Include the `libazdice.h` ("src/include/libazdice.h") in the C/C++ progam with `#include "path/to/libazdice.h"`. **NB: Currently the functions may need to be included seperately.**

4) Compile and run. Make sure tha the dylib is in the correct location if using a dylib.
