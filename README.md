# na_print: debugging without allocation

it does whats in the name, I dont know what else to say here.

## MSRV

rust stable `1.56.1`

## Platform compatability

currently only `#[cfg(unix)]` platforms (so no windows). PRs adding support for other platforms are welcome!

## Saftey / Stability

It should be perfectly fine to use, however for now it should only be used in debugging code.

note: *This warning will be removed once it is reviewed by someone who knows more about this kind of thing, which should happen soon. after that, expect a 1.0 release*
