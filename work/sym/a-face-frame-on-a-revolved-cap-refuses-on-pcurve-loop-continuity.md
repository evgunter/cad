---
id: a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity
kind: issue
title: A boss on a FaceFrame taken from a revolved body's cap refuses on pcurve_loop_continuity at every symbolic dial, and the tier's rule E makes the freeze population 300x worse without moving the refusal
status: open
opened: 2026-09-14
priority: P0
cost: H
---


## What

Found by SYM-5 PR-2's dual review (R2's e2e ladder, reproduced by the
fix pass at `c12d75f5d`, release, ε default). A `Datum::FaceFrame`
taken from the END cap of a REVOLVED body refuses certification on
`Sym<Interval>` under `ProfileLift::Guided`, at every symbolic dial
setting, and the refusal is not the one the rest of the derived-frame
family makes.

The document:

- an authored `Datum::Frame { origin: 0, u: (1,0,0), v: (0,1,t) }`,
  `t = 0.25 ± 1e-3` (a Scalar document parameter);
- a half-size square at `(1.5, 0)` on that frame;
- `Node::Revolve` of it through π about the frame's own `v` axis
  through its origin (`fixture::axis_in_plane(base, (0,0), (0,1))`);
- a `Datum::FaceFrame` on `RoleSeg::RevolveCap(MeridianEnd::End)`;
- a half-size square on that frame, extruded 0.25.

Measured, `Guided`, half `1e-3`:

| dial | refusals | frozen | `symbolic_zero` | wall |
| --- | --- | --- | --- | --- |
| plain `Interval` | 4 | — | — | — |
| `without_rule_e` | 4 | 3 | 107 | 0.0 s |
| `shipped` (rule E on) | 4 | **902** | 477 | 1.4 s |

The refusing node is the REVOLVE itself, one gate upstream of the boss:

```
node 3 — the revolve op refused: revolve pcurve mint pass: pcurve
minting at half-edge HalfEdgeKey(10v1) escalated: predicate
'pcurve_loop_continuity' indeterminate: enclosure [-5.31e…]
```

and nodes 4, 5, 6 are poisoned through it. Under `Pinned` the same
document certifies at both dials (22.3 s off, 19.7 s on), so the
refusal is the `Guided` lift's placement of the boss's profile, not the
revolve alone.

## Two things worth separating

1. **The refusal is not the tier's to remove.** `pcurve_loop_continuity`
   is a margin on the revolve's pcurve mint pass; no dial of the
   symbolic tier moves it, and it is already refused with every rule
   off. Whether it is a real non-continuity or an enclosure too wide to
   classify is not settled by this row.
2. **Rule E makes the freeze population 300× worse here without moving
   the refusal** (3 → 902). That is the rule's own shape — a form it
   brings back under the budget is one whose consumers then build what
   the freeze used to cut off — but on a document where nothing is
   gained it is 1.4 s of work for no decision.

## Who owns it

Not obvious; the candidates, in the order the evidence points:

- **`sweep`'s revolve** (`crates/sweep/src/revolve.rs` and the pcurve
  mint pass): the predicate is its own and the refusal is at its node.
- **PROPS' widening class**
  (`work/sym/real-margin-dependency-widening`): if the enclosure is
  wide rather than the geometry non-continuous, this is that class at a
  new site.
- **SYM, the tier**: only for point 2 — whether rule E should decline a
  document it cannot help. There is no cheap syntactic test for that
  and this row does not propose one.

Filed on `work/sym/` because SYM found it and SYM owns point 2; a
program that claims point 1 should take the row.

## Reproduction

`crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`,
`sym5_the_reach_on_documents_the_unit_did_not_build`, case
`tiltV revolved-cap` (`CAD_SYM5_CASES="tiltV revolved-cap"`).
