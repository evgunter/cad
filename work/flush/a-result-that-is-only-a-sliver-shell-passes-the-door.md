---
id: a-result-that-is-only-a-sliver-shell-passes-the-door
kind: issue
title: A result that is only a sliver shell passes the door: check 7 exempts its in-band volume sign and check 10 never reads a one-shell solid
status: open
opened: 2026-10-09
priority: P0
cost: M
refs: [4415]
---


## What

Found by the door-typing unit (branch `join/door-types-in-band-results`).

D10 (Booleans): a result holds no in-band pair or shell. The gate reads a
shell's role in two places. Check 10 reads it only in a solid of two or
more shells. Check 7 reads a solid's volume sign, and it exempts a sign
in band (`validate.rs` `plus_v_at_target`, the `Certified::Open` arm).
So a result whose only lump is a sliver ships with no reading refusing
it.

**Repro.** Take the census witness pose (`notch307 nt e0 a3 d1e-8`), but
replace the notch prism with its convex piece beyond the notch,
`prism([(2,0), (4,0), (4,2), (2,1)], 1)`. Then `intersect_with(piece,
cube)` at ε = 1e-9 builds one shell. `classify_shells` refuses that
shell `Escalated`, and its certified `V/A` is [3.28872e-9, 3.28872e-9],
wholly in band: the verdict check 10's `ValidationError::ShellRoleUndecided`
carries as `sliver: Some(..)`, but only in a solid of two or more shells.
The intersection still returns `Ok`. At d = 3e-8 the same holds with `V/A` 9.866e-9. At d = −1e-8 the
intersection is empty.

## The shape to give

The door reads check 7's in-band arm (`Certified::Open` with a certified
reading wholly in band) and types it `Escalated { ShellRole }`, the way
`finding_arm` types check 10's. Check 7's at-rest exemption is not
touched. A body at rest that is wholly a sliver is a separate question
from what a boolean may return.

**From PR 4415's first review (NOTE-6).** Keep the in-band judgement in
one home. `RoleUnread::certified_by` is the only reader of the
certified reading's `terminal_sliver`, and `finding_arm` only types
what it found. Typing check 7's `Certified::Open` arm must go through
that same `certified_by` (a `CertifiedSliver`), not through a second
reading of the record.

**From PR 4415's second review (NOTE-2).** The class is "an in-band
shell that is the only shell of its solid", not only a one-shell result.
Check 10 reads roles only within a solid of two or more shells.

The near-tangent witness is refused because its sliver shares the main
lump's solid (`1v1` at d = 1e-8). At d = 1e-7 the result is two solids,
`[1, 1]`, but there the lump's role reads definite, so nothing is in
band.

The pieces sort cannot give an in-band shell a solid of its own: an
unread role blocks the move (`PieceSortError::RoleUnread`). So the class
is reached where the build itself leaves the sliver alone in a solid,
as in the repro above.
