---
id: a-last-leg-walked-back-row-fails-off-the-default-eps
kind: issue
title: viewer: sketch::tests::a_last_leg_no_close_can_follow_is_walked_back fails at eps 1e-6 and 1e-12
status: closed
opened: 2026-09-30
priority: P2
cost: E
closed: 2026-10-02
refs: [a-last-leg-no-close-can-follow-is-dropped]
---


Found by the PATHS 5a fix pass (PR 3527) after merging main at 94ad2a6e.
`crates/viewer/src/sketch.rs`'s row
`a_last_leg_no_close_can_follow_is_walked_back` is green at the default
ε and red at `CAD_TOLERANCE_EPS=1e-6` and `1e-12`, on the case
`At(0,0), LineTo(0.01,0.01), LineTo(0.02,0), LineTo(0.01,1e-9)`:
the preview draws 4 vertices where the row expects 3. The last leg ends
1e-9 off the entry's first side — a literal equal to the default ε, so
at 1e-6 the close's geometry reads differently and at 1e-12 it is
resolvable; which walk-back the case exercises depends on the ε row.
The path has no arc, so 5a's consistency checks are not on it.

**What would close it.** Scale the offset by the running ε (or pin the
case's ε-sensitivity per row), so the row asserts the walk-back at every
ε the nightly takes.

## Closed 2026-10-02 — fixed on TOPO's ground (PR 3590)

c52d1e7976 ("the banded close legs sit inside the band at every ε the
CI runs") scaled the case's in-band offsets by `Tol::witness().eps()`
(1ε and 0.4ε, the same points at the default ε), which is this row's
"what would close it" exactly; TOPO's seam note on this log recorded
it. Re-verified at AUTHOR's exit:
`CAD_TOLERANCE_EPS=1e-6` and `=1e-12 cargo test -p viewer --lib
a_last_leg_no_close_can_follow_is_walked_back` both pass.
