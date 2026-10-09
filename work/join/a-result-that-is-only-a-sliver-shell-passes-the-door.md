---
id: a-result-that-is-only-a-sliver-shell-passes-the-door
kind: issue
title: A result that is only a sliver shell passes the door: check 7 exempts its in-band volume sign and check 10 never reads a one-shell solid
status: open
opened: 2026-10-09
priority: P0
cost: M
refs: [a-near-tangent-intersections-sliver-lump-reads-its-role-in-band-and-refuses]
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
shell `Escalated`: its certified `V/A` is [3.28872e-9, 3.28872e-9],
wholly in band (`sliver: Some(..)`). The intersection still returns
`Ok`. At d = 3e-8 the same holds with `V/A` 9.866e-9. At d = −1e-8 the
intersection is empty.

## The shape to give

The door reads check 7's in-band arm (`Certified::Open` with a certified
reading wholly in band) and types it `Escalated { ShellRole }`, the way
`finding_arm` types check 10's. Check 7's at-rest exemption is not
touched. A body at rest that is wholly a sliver is a separate question
from what a boolean may return.
