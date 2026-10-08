---
id: rows-pin-join-desync-without-its-what
kind: issue
title: rows pin JoinDesync, an invariant-break variant, as an expected outcome without its what, so any new desync passes them
status: open
opened: 2026-10-03
priority: P3
cost: E
branch: join/strut-side-one-rule
---

## Finding

`BooleanError::JoinDesync` is the join's invariant break: the two
solids' surgery diverged, or a key the join read stopped resolving.
Several rows accept it as an expected outcome by variant alone, so a new
desync of any cause passes them as the frontier they meant:

- `crates/topo/tests/m3_pr6_saddle.rs` (the saddle frontier) and
  `crates/topo/tests/review_m3_pr6.rs` (the same pose), `matches!` on
  `JoinDesync { .. }`;
- `crates/topo/tests/union_flush_onto_edge_contact.rs`
  (`y ∩ cube`, `cube ∪ y`), the same;
- `crates/sweep/tests/m5_s13_review_probes.rs` and
  `crates/sweep/tests/r1_probes_m9_3.rs` (three sites), which accept it
  among other refusals;
- `crates/topo/tests/graft_disjoint.rs`, which matches `"JoinDesync"` in
  `Debug` text for the graft door's caller errors (an empty source, an
  N-solid source at the single door): not a join at all.

`crates/sweep/tests/join1_mechanisms.rs` pins its desync's `what`, which
is the shape to copy: a different desync turns it red.

**What would close it.** Each row pins its desync's `what` (or the
refusal it is meant to be, where a typed one now exists); the graft
door's caller errors get a variant of their own or are pinned by `what`.

Found by JOIN-2's fix pass 2 (PR 3880), sweeping for the shape of the
delta review's S9 (`contact8_dangling_seam`'s exact-plug row, which now
pins that the join builds the plug).

## Built

- The saddle rows (`m3_pr6_saddle.rs`, `review_m3_pr6.rs`) and
  `union_flush_onto_edge_contact.rs` no longer accept `JoinDesync`
  anywhere; nothing to change.
- `m5_s13_review_probes.rs` (probe 6) and `r1_probes_m9_3.rs` (three
  probes) build at 1e-9, 1e-6 and 1e-12, so their `JoinDesync` arms were
  acceptance with no witness; the arms are gone, and a desync now turns
  them red.
- The graft door's caller errors are pinned by `what`, in
  `graft_disjoint.rs` and in `instance.rs`'s unit test. That they are
  `JoinDesync` at all is filed as
  `work/issues/graft-door-caller-errors-are-join-desync.md`.
- Swept beyond the list: `join.rs`'s radical-plane row and `finish.rs`'s
  stale-section-face row now pin their `what`; `offer_rows.rs`'s
  `coaxial_tiny_sphere` withdrawal meets `Because::Desyncs(what)`.
