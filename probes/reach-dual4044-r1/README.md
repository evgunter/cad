Probes for the reach-dual4044-r1 review. `reach_dual4044_r1_probes.rs` is a
module of `crates/sweep/tests/all.rs`: copy it into `crates/sweep/tests/` and add
`mod reach_dual4044_r1_probes;`. Then run
`cargo nextest run -p sweep --test all reach_dual4044 --no-capture`. It prints
OK/REFUSED/WRONG/PANIC per op and asserts only that nothing is WRONG or panics.
The two `.py` files are the independent slice-integral oracles that
`slab_cap_with` quotes.
Instrumentation used (not committed): an `eprintln!` of `span` at `ops.rs:3924`
and of the cut ends after `ops.rs:3920`.
