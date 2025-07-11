Working through the Rust Programming Language Book
https://doc.rust-lang.org/book/





Macros are also pretty important to learn:
  https://lukaswirth.dev/tlborm/decl-macros/macros-methodical.html
Learning about the different std datastructures...
```

Since our DFS is a collection, we will have to figure out how to dynamically assign memory.
  - Arena? - Finite amount of huge memory we allocate at once. Not scalable
  - RC Cells? - arc based GC is a lil sus



The default hashmap collection should already handle dynamically sizing things.
Some ideas from a competitive prog website: https://rustp.org/data-structures/
- vec is the bog standard:
  - vec![a;b;c;f;d;,.....]
    - len , capacity
    - .push(element)
    - Vec::new(), Vec::with_capacity()
    - vec![1,2,3,4,5], vec![0;5]
     range slice, vec1[1..3].to_vec()
    - inserts can be done at an inbetween index but ofc shitty perf
        - .insert(idx, element)
        - pop(), remove(idx), swap_remove(idx), get(idx)-> option type.

clone trait allows things to be copied deeply.
copy trait is for the stack-only data.

Borrow rules:
-- can't move out if borrowed
-- can't mut/ immut borrow if already mutably borrowed
-- can't mut borrow if already immut borrowed
-- can imut borrow many times.

-- Lifetimes are a measure/ logical clock determining how long a reference should last.
Stuff on heap is borrowed into another reference via assignment
Old stuff is dropped as well so s= new(1); s= new(2)

Collections:
https://doc.rust-lang.org/book/ch08-00-common-collections.html
The standard collections are found at : https://doc.rust-lang.org/std/collections/index.html

Heap shenanigans.
Iter + Enumerate is used to go over each element in the collection,

Slices are references to parts of a collection.
slice datastructure:
  ptr to start, len

deref coersions??

structs, enums, generics, traits

Generics and Traits generalize datatypes:

One can restrict generic types to those that implement certain traits like PartialOrd for example
```

===========================================================================
Some interesting projects I could try:
- Implement Elements of Differentiable Programming
- Gonzalo Navarro's book on succinct data structures.
- Porting parts of Ghidra's analysis and decompilation engine into Rust.

==========================================================================
Performance and Debugging in Rust:
Nethercote's book is seminal: https://nnethercote.github.io/perf-book/introduction.html
- coz is great, performs causal profiling

if you are using standard libraries, will have to build the stdlib with symbols: https://github.com/rust-lang/rust
  - frame pointers maybe optimized out.
  - mangling format maybe cool
- gdb/lldb for the debugger
- need to check out other debuggers

=========================================================================
