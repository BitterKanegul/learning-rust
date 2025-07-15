Lifetimes, Traits and the whole type system is beautiful,
I need to understand the rust math that goes on under the hood.

I wonder how this correlates with Parallel Programming math.
===========================================================================================
2025-07-13:
A study of the Canonical Solver written in a mix of Rust and Lean.
URL: https://github.com/chasenorman/Canonical
- Consists of 3 crates: compat, core and lean
  - looking at core:
    - uses rayon, mimalloc, arc-swap, once_cell, thread_local_collect and hashbrown
      - mimalloc is a general purpose allocator from MS, developed for the lean and koka langs.  https://microsoft.github.io/mimalloc/
      - Arcswap: Making Arc atomic https://docs.rs/arc-swap/latest/arc_swap/
      Optimized for read-mostly scenarios.
      Will have to understand why and how this is used.
      - Once_Cell:
      https://docs.rs/once_cell/latest/once_cell/
      Can store non copy types, assigned to once and allows direct access to the stored contents. Useful for safe global inits, lazy inits and other things.
      -ParkingLot is another one to figure out.
      - Thread Local Collect: This library supports the collection and aggregation of thread-local data across threads. It provides several modules, each of which accomplishes the aforementioned task in a different way. https://docs.rs/thread_local_collect/
      - Hashbrown: Rust port of Google's SwissTable hash map.
      Drop in replacement of Rust's standard Hashmap and Hashset types.
     https://docs.rs/hashbrown/latest/hashbrown/
  - Divided into the following modules:
    - core:
      Uses:
       - memory
       - stats


    - heuristic
      Uses:
        - Search::Next
        - stats
      Creates:
        - div
          (a,b,prior, breakpoint) -> f64
          smoothly transitions between prior and a/b as b goes to breakpoint
        - next
           (first:Metainfo, second: Metainfo)-> Metainfo
           Selects which metavariable to refine between two options.


    - lib
    - memory
    - print
    - prover
      Uses:
        - search
        - core
        - memory
        - stats
        - rayon's prelude
        - atomic ordering and usize
        - arc
        - hashbrown hashmap
        -
    - search
    - stats
===============================
Lessons to pick from this:
  Some important standard traits to use:
    Debug
    Copy
    Clone
    Eq
    PartialEq
  Macros:
    #[derive]
    #[global_allocator]
    thread_local!







A study of the CvxLean repository that also uses a mix of Rust and Lean.
URL: https://github.com/verified-optimization/CvxLean
