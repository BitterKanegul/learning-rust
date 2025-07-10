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
```
