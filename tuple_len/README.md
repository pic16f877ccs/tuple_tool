### Library for determining the length of a tuple.

Return the length of a tuple.
### Examples                                                                                                                                                   
Usage:
```
use tuple_length::TupLen;

assert_eq!(().len(), 0);
assert_eq!((1i8,).len(), 1);
assert_eq!((1u16, 2u64).len(), 2);
assert_eq!((1i8, 2usize, 3i64).len(), 3);
```
### Supported tuple lengths
Possible length 8, 16, 32, 64.
By default the selected operations are implemented to tuples upto a length of **8 elements**.
 
### Features
You can specify the length: features = ["16"] or features = ["32"] or features = ["64"].

