---
id: main-red-prose-census-misses-two-boolean-error-fields
kind: issue
title: Main red: pncad-py's prose census finds two BooleanError fields (edge, face in topo/src/boolean/mod.rs) it neither decides nor names
status: closed
priority: P1
cost: E
opened: 2026-10-27
closed: 2026-10-27
---


Found by INTENT stage 3 E's slow-set run (`intent/e-operation-frame-of-reads`)
and confirmed on a clean `origin/main` worktree at `9d97abd7`:
`pncad-py` `prose_census::tests::every_site_this_census_cannot_decide_is_named_with_its_reason`
(`crates/pncad-py/src/prose_census.rs:2291`) reds with two sites the
census cannot decide and `UNDECIDED` does not name:

- `("crates/topo/src/boolean/mod.rs", "BooleanError", "edge")`
- `("crates/topo/src/boolean/mod.rs", "BooleanError", "face")`

Both arrived with main's recent `topo/src/boolean/mod.rs` changes
(merged between `29110056e8` and `9d97abd7`; the latest is PR 5772,
`3b0a7a6e`). The fix is the census's own rule: render each field so the
census can decide it, or give each its line in `UNDECIDED` with the
reason it cannot be decided.

## 2026-10-27 — the undecided-site row is green; a Display row is red instead

On main `6597797b` (measured on a clean worktree, and on E over that
merge) `every_site_this_census_cannot_decide_is_named_with_its_reason`
passes, so the two fields above are decided now. A sibling row reds in
their place, with the same file and enum:
`prose_census::tests::no_display_impl_renders_a_brace_shaped_payload_through_debug`
(`crates/pncad-py/src/prose_census.rs:2274`) finds `BooleanError`'s
`relation` field (`crates/topo/src/boolean/mod.rs`) rendered through
`Debug` in a `Display` impl, so the binding's `typed_err` would panic on
a struct-shaped payload instead of raising. The fix is the row's own:
render the payload through its own `Display`, or give it one.

## 2026-10-27 — gone on main `39134695`

Both rows pass on main `39134695` (E's slow set over that merge, the
default ε), and so does every other prose-census row. Closed.
