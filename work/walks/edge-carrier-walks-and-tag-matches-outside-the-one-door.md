---
id: edge-carrier-walks-and-tag-matches-outside-the-one-door
kind: issue
title: Nine hand-written edge to carrier walks and six tag-only carrier comparisons still read past readback::edge_carrier_ref
status: open
opened: 2026-09-14
refs: [edge-carrier-kind-has-no-readback-door, 2587]
priority: P1
cost: E
---



## What

`edge-carrier-kind-has-no-readback-door` (PR 2587) gave the crate ONE
walk from an edge to its certified carrier —
`topo::readback::edge_carrier_ref`, which names the three ways that
walk comes back empty (`CarrierAbsence`) and lets a caller rename them
in its own vocabulary. Three readers go through it today:
`readback::edge_carrier_kind`, `readback::edge_pose` and
`query::rim_of`. The rest of the crate still writes the walk by hand,
and a handful of sites outside it compare the carrier's TAG after
walking there themselves.

Neither half is a defect at any one site — each refuses in its own
vocabulary, and that is why they were written out — but together they
are the class the unit closed one instance of, and the swept pattern in
PR 2587 (`CurveKind::of`) could not see either half by construction.

## The walks — nine sites, `grep -rn 'certified()' crates/topo/src` minus `readback.rs` and `null.rs`

| site | its refusal vocabulary |
|---|---|
| `attach.rs` (the scaffold carrier read) | `StaleGeometry` / `NullScaffoldCurve` |
| `coherence.rs` (the examinability rung) | `Unexaminable` |
| `props.rs` (the mass-properties edge read) | `corrupt` / `NullScaffoldEdge` |
| `seqgen.rs` (the generator's carrier peek) | `Option`, an honest skip |
| `split.rs` ×4 | the split's own |
| `revert.rs` | the revert's own |
| `euler_kill.rs` | the kill's own |

`DanglingRef` already has a `From` into `EulerOpError`, which is the
shape the operator-layer ones would delegate through — the vertex side
already does, via `readback::vertex_point_ref`. What each site needs
deciding is only which `CarrierAbsence` arm maps onto which of its own
words, and `query::rim_of` (PR 2587) is the worked example: the rename
is exhaustive, so a fourth kind of absence cannot arrive silently at
any of them.

## The tag comparisons — the sites that hold a KEY and could read the seat

`grep -rn 'carrier()' crates/ demos/ tools/ | grep matches!` is 46
lines, 12 of them outside `tests/`. They are not one disposition, and
the PR-2587 body's first draft said they all consume fields, which is
false. Two groups:

- **Holds only a key, walks to the carrier, reads the tag and drops
  it** — these could read `query::edge_carrier_kind` /
  `edge_carrier_matches` instead, which is the literal "is this edge
  straight" the new door's doc offers:
  `crates/topo/src/validate.rs` (`all_lines`),
  `crates/topo/src/merge_faces.rs` (`straight`),
  `crates/topo/src/boolean/surface_group.rs` (the circle skip),
  `crates/mesh/src/trimmed.rs` (the `Ellipse | Nurbs` test — a
  `CurveKindSet` comparand, and `mesh` is not this program's).
- **Already holds the carrier for other reasons**, so reading its tag
  is not a second walk and routing it through a seat would ADD one:
  `crates/topo/src/boolean/ops.rs` (`curved`, which then calls
  `params()` and `eval()` on the same carrier) and
  `crates/sweep/src/blend/surgery.rs` ×2 (`sc` is in hand;
  `sweep` is not this program's either). These are correct as written
  and are listed so the class's boundary is stated rather than implied.

The ~309 `.carrier()` reads overall are mostly the third thing — a
`match` that destructures the arm's fields — and no tag read could
serve them.

## Not urgent, and why it is filed anyway

Nothing here is wrong today. What the row buys is that the next lane to
touch one of these files can see the door exists, and that the class
was measured rather than assumed. A unit closing it takes the walks
first (mechanical, one `From`-shaped rename per site, rows unchanged)
and leaves the tag comparisons to the units already editing those
files.
