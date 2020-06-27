#libazdice C/C++ ffi examples.

libazdice has a C/C++ ffi which contains several functions. Firstly it includes three test functions.

-The `test` function, which is used to test whether the library is being loaded and that strings can be returned to C/C++.

-The `test2` function, which is used to test whether the library is loaded and that strings can be sent from C/C++ without causing 

-The `parse_and_roll2` function which is used to test whether the parser "works". It can be used for normal "work", but is not recommended as it does not return an error if a nonsense dice string is encountered, but instead crashes the thread.

It also includes four actual functions.

- `parse_and_roll`, which parses a dice string and rolls it once, returning either a total result or an error string. NB: If a more detailed result breakdown is wanted, use `parse_and_roll_n_times`

-`parse_and_roll_n_times` will parse a dice string and roll it N times, returning either a detailed report of the rolls or an error string. Can be used to roll the dice once.

For the meanwhile the ffi does not include an exported version of a `DiceBag` and the dice string will have to be parsed each time a set of rolls is desired. This is not seen as a problem because

a) Although there are scenarios where `DiceBag`s can be cached and reused, it is not the expected use case.

b) Computationally speaking, parsing a dice string to a dice string is not too expensive (especially since they are generally quite short).
