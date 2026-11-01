---
id: end-face-vertex-window-picks-its-first-half-turn-by-floor
kind: issue
title: the open blend's end-face window picks its first half-turn by an open-coded floor
status: closed
closed: 2026-10-26
opened: 2026-10-13
priority: P3
cost: E
refs: [periodic-branch-floor-remains-at-the-raw-picks]
---


Found by PCERT's `periodic-branch-floor-remains-at-the-raw-picks` (PR
4941's review, MINOR-F), by reading. Not measured, and not changed:
BAND's code.

`crates/sweep/src/blend/open/end_face.rs`, in the end-face section's
window: `first = −⌊−ta/π⌋` (`⌈ta/π⌉`, the first major vertex at or past
the window's start). It is read only through `(first·π − tb).lo() ≤ 0`,
and a wrong pick errs to `far = major`, the larger reach. It looks sound
at every scalar, conservatively. It is the one production `floor`
branch pick PCERT's census missed. At `Sym` the `floor` is an atom unless
its argument's form is zero, so the vertex test is read numerically.
Worth a sentence at the site saying so, or a structural pick if an
identity ever reads `first`.

## Closed (2026-10-26, tracker sweep)

The pick is gone. `12ee8c73f0` ("band: drop the cap sliver's dead
end-curve terms; pin the whole-circle arm on a cap edge", PR 5110,
2026-10-14) deleted the end curve's own `far` term from
`CapSliver::of`, and with it the two lines this row names
(`let first = T::zero() - (T::zero() - ta / T::pi()).floor();` and the
`vertex_inside` test). `crates/sweep/src/blend/open/end_face.rs` holds
no `floor`/`ceil` branch pick now; the round end reads the section's
major and minor semi-axes through `SectionFrame` (`end_face.rs:858-893`).
