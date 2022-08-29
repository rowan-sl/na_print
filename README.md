# na_print: debugging without allocation

it does whats in the name, I dont know what else to say here.

This is mostly intended for debugging global alloc implementations, when allocating to print debug info would cause recursion.

## Thread-saftey

This is thread safe, however it does not use any locking, so there may be some issues with interleaved output when using multithreading.

## MSRV

rust stable `1.56.1`

## Platform compatability

currently only `#[cfg(unix)]` platforms (so no windows). PRs adding support for other platforms are welcome!
