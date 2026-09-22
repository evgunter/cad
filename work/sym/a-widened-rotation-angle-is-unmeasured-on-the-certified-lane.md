---
id: a-widened-rotation-angle-is-unmeasured-on-the-certified-lane
kind: issue
title: the chain's certified picture: a widened rotation angle through Node::Transform has never been measured on Interval or Sym<Interval>, and the derived-frame walls stand between the chain and an enclosure per link
status: closed
opened: 2026-09-22
closed: 2026-09-22
priority: P1
cost: D
---


## What

**Specifically requested by Ev, 2026-09-22 (P1):** the certified
version of the chain demo (`SYM-14`) — an enclosure per link, growing
down the chain, beside the advisory cloud. What stands between the
chain and that picture, as surveyed at the unit's cut:

- **A widened rotation angle has never been measured anywhere in the
  tree.** The only widened `Node::Transform` is a widened TRANSLATION
  with `rotation_angle: ang(0.0)`
  (`crates/editor-core/tests/m10_derived_frame_interval.rs`, the
  `Transform` row). An interval angle puts interval `sin`/`cos` into
  the rotation matrix — unmeasured on `Interval` and on `Sym<Interval>`.
- **The derived-frame walls are live**: a frame whose axes carry a
  widened parameter froze on degree (`derived-frame-placement-freezes-on-the-symbolic-lane`,
  answered per family by rules E, F and G — the tilt-`u` Newell wall
  closed with DECIDE-3 on `props/sign-hull`, `tiltUV` still moves not
  one count), a `FaceFrame` on a revolved cap refuses
  (`a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity`),
  and at half-width `5e-2` clause 1 refuses
  (`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`).
- **Cost**: two stacked derived frames were 219 s per replay with rule
  E off and 1.1 s on; four links over four box axes is a large drive
  against the plate's 512-leaf cap.
- **Not a wall, noted**: the assembly placement lane
  (`placement::Frame`, `DocEdit::SetPlacement`) stores numbers and
  cannot carry a parameter at all; the chain is expressed through
  `Node::Transform` or a `Datum::Frame`, which can.

## What answers it

SYM-14's Phase 2 table: one leaf at `Interval` and `Sym<Interval>` for
chains of 1–4 links at the nominal box (certifies / refuses / the first
refusing predicate / cost), the drive at a small budget, the widest
box that certifies whole. This row CLOSES if the certified lane reaches
the four-link tip's assertion at any box (the enclosure drawn on the
sheet); otherwise it carries the walls' names, each filed as its own
row at P1 with this request, and stays open.

## Result — CLOSED, it reaches the tip (SYM-14, 2026-09-22)

Measured in `demos/tour/src/chaintol.rs`, one leaf over the whole
declared box, in the driver's own lane, on the chain of
`demos/tour/src/chain.rs` (σ = 0.01 rad at every joint, the tip
asserted within 1 mm of a target pin):

| links | lane | | first refusal | cost |
|---|---|---|---|---|
| 1–4 | `Interval` | refuses | `transform_rigid_col0_unit` | <0.01 s |
| 1 | `Sym<Interval>` | **CERTIFIES** | — | 0.16 s |
| 2 | `Sym<Interval>` | refuses | `dihedral_wedge`, margin poisoned | 0.31 s |
| 3 | `Sym<Interval>` | refuses | `dihedral_arm`, `[0, 7.34e-3]` | 0.47 s |
| 4 | `Sym<Interval>` | refuses | `dihedral_arm`, `[0, 7.34e-3]` | 0.73 s |

A widened rotation angle is now measured. The plain interval lane does
not carry one AT ALL — `cos² + sin²` is a bracket around 1, and the
rigid map's own column-unit check is what notices — and the symbolic
tier discharges exactly that, which is the whole difference between
the two lanes on this document.

The widest box that certifies whole, per link count, is `1.000`,
`0.370`, `0.185` and `0.111` of the study — one number in four
spellings: `3σ · f · Σ(k−j)`, the accumulated angular swing at the tip,
is `0.0333` rad at every one of them (the one-link row is capped by the
study itself at `0.030`). **The certified lane carries about 1.9° of
accumulated swing, however many joints it is spread over.**

**The four-link tip's assertion IS certified, at 0.111 of the study**
(`chain::CERTIFIABLE_FRACTION`, bisected): the drive over that box
certifies and the tip assertion HOLDS on every certified leaf. The
enclosure per joint is on the sheet beside the cloud, in teal —
`0.0400, 0.1199, 0.2398, 0.3996` mm across the chain, growing
`1 : 3 : 6 : 10`, the worst-case lever sum, against the advisory σ's
quadrature `1 : 2.24 : 3.74 : 5.48`. That is the certified picture
this row asked for, so it closes.

The cost objection did not materialise: four links in one leaf is
0.73 s, not the 219 s per replay the derived-frame family costs, and
the derived-frame walls never came into it — the chain is placed by
`Node::Transform`, not by a `FaceFrame` stack.

What bounds the box to 0.111 rather than 1 is filed, each at P1 with
Ev's request:

- `work/sym/a-widened-rotation-angle-refuses-on-the-plain-interval-lane`
- `work/sym/a-two-joint-chain-poisons-its-transversality-margin`
- `work/sym/a-chain-of-three-joints-straddles-dihedral-arm`

## Home

SYM — the tier's reach on a document Ev asked for; the walls it names
are the tier's (SYM/DECIDE) or PROPS' (the frame family) by their
subject.
