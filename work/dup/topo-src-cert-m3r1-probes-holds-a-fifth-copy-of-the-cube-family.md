---
id: topo-src-cert-m3r1-probes-holds-a-fifth-copy-of-the-cube-family
kind: issue
title: cert_m3r1_probes.rs carries a verbatim in-src copy of the whole geometric-cube fixture family
status: open
opened: 2026-09-16
---


## Finding

- **Where**: `crates/topo/src/cert_m3r1_probes.rs` — `GeoCube`
  (~`:34`), `line` (~`:41`), `plane` (~`:45`), `geometric_cube`
  (~`:50`) and `describe_as_intersections` (~`:160`), against
  `crates/topo/tests/common/mod.rs`'s members of the same names.
- **Importance**: medium
- **Confidence**: sure they are copies — **the file says so twice**
- **Raised by**: the `dup-cube-seq` lane (S-DUP), 2026-09-16
- **Refs**: `brick-has-two-constructions-and-two-homes` (this is a
  further consumer of that row's third link)

Two doc comments in `cert_m3r1_probes.rs` state it outright:
*"`topo/tests/common::geometric_cube`, copied verbatim (in-crate)"* and
*"`topo/tests/common::describe_as_intersections`, copied verbatim"*.
`GeoCube`, `line` and `plane` come with them unlabelled.

The copy's Euler-op call-site counts are identical to the shared
sequence's — `{mvfs: 1, mev: 2, mef: 5, MefSite::Chords: 5,
MevSite::Fan: 1, set_face_surface: 1}` — which is how a shape sweep
found it; a name sweep found it only because the doc comments name the
original.

## Why it cannot be fixed today, and what unblocks it

The module is in `crates/topo/src/`, and its header explains why it has
to be: the corruption route it exists to drive needs `Body::surfaces`,
which is `pub(crate)`. Nothing in `src/` can name
`crates/topo/tests/common/mod.rs`, which compiles into the test binary
only. So the copy is not a lazy one — it is what the current home
layout forces.

**It is unblocked by exactly the move
`brick-has-two-constructions-and-two-homes` adjudicates as its third
link**: once the fixture family lives in
`crates/topo/src/test_support_impl.rs`, an in-crate module can name it
and these five items delete. The mover should count this file in the
consumer set, which the row's *"set that would have to move"* does not
yet list.

One thing does NOT delete with them: `cert_m3r1_probes.rs` also carries
`face_surface_of_he`, which `tests/common`'s
`describe_as_intersections` spells as a local closure. Whichever home
wins, that helper is one function, not two.
