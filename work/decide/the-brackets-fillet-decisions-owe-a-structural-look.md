---
id: the-brackets-fillet-decisions-owe-a-structural-look
kind: issue
title: the bracket's 28 decisions only a certified-sign read reaches (tangent_on_surface, carrier_on_surface_2 and others, fillet_r a parameter) are unexamined: are they a sign the profile decided and dropped?
status: open
opened: 2026-09-26
priority: P3
cost: D
refs: [the-apothems-sign-is-a-value-read, DECIDE-8]
---


## What

DECIDE-8 measured a certified-sign read asked as the ladder's last rung
(`SymRules::signed_root_last`, draft #3282, dial off). Beyond the six the
apothem's item names, it takes 28 decisions on the bracket (R2's filleted
bracket, `fillet_r` a parameter), as totals 1104/7/150/760 →
1104/35/150/732:
- `tangent_on_surface_1` and `tangent_on_surface_2`, 9 each;
- `carrier_on_surface_2`, 4;
- `tangent_hull_sup`, 2;
- `witness_on_surface_1` and `witness_on_surface_2`, 2;
- `line_span`, 1;
- `contact_at_shared_vertex`, 1.

The read costs +39% on the bracket's leaf.

The `line_span` and `contact_at_shared_vertex` may be the adjacent pair's
shared vertex again (`work/paths/an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root.md`,
at the tangent arm). The rest (`crates/geom-brep/src/certify.rs`'s
`tangent_on_surface_*` and `tangent_hull_sup`) have not been read.

## The question

Do they stand on a sign some layer already decides and drops, such as
`fillet_r > 0`, or a fillet's declared tangency? That is DECIDE-5's
pattern, where the tier re-derived what the profile had decided. If so,
the answer is to carry that decision and no read is needed. If not,
these are the residue a certified-sign read would be weighed on.
