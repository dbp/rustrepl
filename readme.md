## About

This is a really simple implementation of what I'd like a real interactive mode for rust to look like. You can enter statements (right now, use, import, fn declarations, and any expression) and it will evaluate the output. The way it is actually implemented is a hack - it stores view items (use and import), declarations (let and fn) and a statement (which is updated as you go). It then writes them to a file, compiles that, and runs it. If it encounters an error compiling, it shows just that error. It actually works surprisingly well.

## Example session

    rust> 1+1
    2
    rust> 1+'a'
    error: internal compiler error: char type passed to convert_integral_ty_to_int_ty_set()
    rust> 1+~"a"
    error: mismatched types: expected `<VI0>` but found `~str` (integral variable vs ~str)
    rust> 1+a
    error: unresolved name: a
    rust> fn hello() { ~"hello world" }
    error: mismatched types: expected `()` but found `~str` (() vs ~str)
    rust> fn hello() -> ~str { ~"hello world" }
    rust> hello()
    ~"hello world"
    rust> hello() + ~"...goodbye"
    ~"hello world...goodbye"
    rust> let a = 23
    rust> a/2
    11
    rust> (a as float)/2.0
    11.5

## What it's missing

Right now the major thing is readline support. Then doing this properly (see, Future), and finally, tab completion, etc.

## Future

Ideally this can be rewritten to use LLVM's ExecutionEngine (interpreter / JIT). I tried that first, but don't know enough about the compiler to not end up in LLVM segfaults. So I instead just took the interface that I had sketched out for a proper interpreter, and hacked it together. And it works surprisingly well!

## Platform support
Right now it is *nix only, just because of finding a temporary directory. I'm in the process of adding a os::tmpdir() function to the core lib, so once that lands, I can redo this to be cross platform.
