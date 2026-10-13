---
id: end-face-vertex-window-picks-its-first-half-turn-by-floor
kind: issue
title: the open blend's end-face window picks its first half-turn by an open-coded floor
status: open
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
