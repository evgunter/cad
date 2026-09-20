---
id: door-table-reason-strings-are-unguarded-prose
kind: issue
title: review_m1_pr5_internal's allowlist checks membership, never an entry's stated reason against its door
status: open
opened: 2026-09-19
---


## Finding

`crates/topo/src/review_m1_pr5_internal.rs` carries `ALLOWED`, a table
of `(door, why tier 1 survives it)` read by two guards —
`every_public_mutation_path_preserves_tier1` and
`the_two_door_tables_cover_the_same_surface`. Both read the **first**
element. **Neither reads the second.** The guards check that a door
naming no tier-1 postcondition is on the list, and that the list has
no entry without a door; the reason string beside it is never compared
with anything in the door's body.

So every entry's reason is unguarded prose, and the guards go green
whether it is right, stale or wrong. The section comment above the
test-support builders makes a claim over a whole group of entries
(*"each writes only through … the asserting operators above"*) and is
equally unread.

## Evidence that the class is real, not hypothetical

Two entries have been read against their doors, both on 2026-09-19, on
PR #2887:

- **`cyl_wall_sheet` — was wrong.** Its entry said "every mutation is
  one of those, each asserting" while the door also calls
  `set_surface_source` and `mint_pcurves`. Tier 1 does survive
  (both are separately allowlisted for writing fields tier 1 does not
  constrain), so the guard was not silenced — the REASON was false.
  Fixed on that PR, and the section comment with it.
- **`prism_ops` — was right**, and is the precedent the fix was written
  to.

**No count of how many of the rest are wrong is published here, because
nobody has measured one.** What is measured is that ~25 entries carry a
reason string and two of them have been checked.

## What a unit here owes

Read every entry against its door and say which are wrong — that is the
measurement, and it is the deliverable even if the answer is none.
Then decide whether the reason can be guarded at all. A cheap arm
exists and would have caught the one defect above: each reason names
doors in backticks, and `mutation_doors()` already parses door bodies,
so a check that every backticked name the reason cites is a door the
body actually calls, and that every non-asserting door the body calls
is cited, is mechanical. It would not check that the reason is a good
argument; it would check that it is not about a different function.

## Why this row is not on a territory GUARD claims

GUARD's `paths` is `scripts/gates/*`, and this guard is an in-crate
`#[test]` under `crates/topo/src`. The charter is what routes it —
*"every finding since about what the gates cannot see … the selftests
that cannot observe what a gate names"* — and this is a guard that
cannot observe what its own table claims. Filed here; whoever owns the
file is free to take it.

Found by the S-DUP unit
`the-cylindrical-patch-rim-builder-is-written-nine-times`, whose own
entry was the wrong one.
