---
id: an-all-on-crossing-record-ends-as-an-invariant
kind: issue
title: A pair record read crossing with all four codes On passes recl silently and ends as an invariant at insert
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by the delta review of PR 6693 (step 4, `a-corner-reads-only-its-own-sectors`); its fix pass stopped on it as larger than a nit.

The probe: two plane sectors hinged on a shared bound, θ = 1e-6, sector angle 1e-4, default band. `pair_search` returns `PairRecord { sa: (On, On), sb: (On, On), intersect: true, coincident: None }`, glued or unglued, for θ from 1e-6 to 1e-2.

- `recl_sectors` skips it (`crates/topo/src/boolean/recl.rs`, `let Some(by) = r.coincident else continue`).
- `on_bound` does not resolve it (`recl.rs` ~339).
- It ends at `crates/topo/src/boolean/insert.rs:444` as a `ClassificationInvariant` ("surviving record without one In and one Out code per side").

That is loud but not honest. Before PR 6693 the record went to the carrier ladder; at its first head it hit `require_same(None)`'s invariant.

**The root is a convention.** A `PairRecord`'s `coincident` and its codes must agree ("every code On" iff `Some`), and only the constructor holds that. Fusing them into one enum touches about 120 field uses in recl and insert, and recl rewrites a coincidence's codes after lumping, so the enum needs a lumped state.

**An honest ending needs a carried reading.** `pair_codes` would have to return each bound's tie reading, which the record does not carry today. Then a crossing read all-On refuses typed, with the bounds' margin.

Done when the all-On crossing record cannot be built without its readings, and the probe above refuses typed, pinned.
