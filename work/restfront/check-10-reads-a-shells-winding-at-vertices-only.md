---
id: check-10-reads-a-shells-winding-at-vertices-only
kind: issue
title: Check 10 reads a shell's winding at its vertices only, so a shell with every vertex on another shell's boundary goes unchecked
status: open
opened: 2026-10-01
priority: P3
cost: M
---

## What

`crates/topo/src/validate.rs`, check 10's shell-winding loop (the
`'witness` walk over `shell_vertices`). It reads the winding that the
other shells put on a shell at the first vertex of that shell that
touches no other shell. A vertex that reads `OnBoundary` against any
other shell moves the walk to the next vertex. A refusal silences the
check for that shell. Once the vertices run out, `winding` stays
`None` and the shell is skipped without a word.

It asks the same question as the boolean's cell-dimension witness
ladder (`crates/topo/src/boolean/shell_witness.rs` `complex_side`):
which side of another boundary an uncut shell lies on. The ladder goes
on to read edge midpoints and certified planar-face interior points
when every vertex lies on the other boundary. Check 10 stops at
vertices. So a void whose every vertex lies on the outer shell, or on
another void, is never checked.

## Measured

Unmeasured. The CLEAVE `cleave/ladders` sweep found this by reading
the code, and no fixture reaching it has been built.

## What a fix would be

Read the witness from the ladder's later tiers as well. That means
sharing `complex_side`'s witness generation, since the winding sums
over several shells rather than asking about one. Alternatively, say
in the check's doc that such a shell goes unread.
