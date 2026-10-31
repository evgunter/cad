---
id: a-seam-ruling-on-a-crescents-tangent-line-ships-a-difference-tier-3-prime-refuses
kind: issue
title: A difference whose minuend's seam ruling lies on a crescent's tangent line ships one shell that tier 3′ refuses (FaceThroughEdgeWedgeUndecided): the edge in the subtrahend's wall is not recorded, so EdgeInFaceGap::CurvedFace never reads it
status: open
opened: 2026-10-31
priority: P1
cost: M
---


Found by PR 5852 on `main` at `45f6be756b`, with the PR's fate-cut
bend reading switched off, so on `main`'s code alike.

## What

`B` the one-segment cylinder `r = 0.5` about `(−0.2, 0)`,
`z ∈ [−0.5, 1]`, its vertex at azimuth 0, so its seam ruling runs
through `(0.3, 0)`; `A` the one-segment cylinder `r = 0.3`,
`z ∈ [0, 1]`, its vertex at azimuth 1 or π. The walls are tangent
inside along the ruling through `(0.3, 0)`. `B ∖ A` returns `Ok`, one
shell, the closed-form volume (0.895353906), tier 2 and the certificate
passing, and `validate_pseudomanifold` refuses it:
`FaceThroughEdgeWedgeUndecided { edge: B's seam, face: A's wall,
witness: (0.3, 0, 0.5), why: InBand }`. The result's contacts hold one
row, a vertex and an edge (`Contact { cells: [Vertex, Edge] }`): `B`'s
seam ruling lying in `A`'s wall along the tangent line is not recorded.

With `A`'s vertex at azimuth 0 instead, `A`'s seam on the tangent line,
the same difference refuses typed, `EdgeInFace { gap: CurvedFace }`
(`crates/topo/src/boolean/slit.rs`, `EdgeInFaceGap::CurvedFace`, the
refusals at `slit.rs:392` and `slit.rs:1024`): the edge lying in the
curved face is recorded there, and the parting declines to build the
crescent's two cusps on a curved face. With `B`'s seam on the line it is
not recorded, so that gate never reads it. Turned rigidly off the axes
(`torax_pose`) the cell reads the same, `BAD` with the one wedge finding,
its witness the turned `(0.3, 0, 0.5)`. The PR 5808-era tree
(`1953c3a21f`) built the same cell `SOUND` with tier 3′ passing, so this
arrived with the tier 3′ reading that refuses the wedge.

## Footprint

The PR 5852 probe (`rev5808r`'s `S10 inner`, PR 5852's head and `main`
alike) finds it in 24 cells at the default ε, every one `B ∖ A` with
`B`'s vertex at azimuth 0, at rest and turned, each with the one
finding above: the one-segment form with `A`'s vertex at 1 or π, and
the two-arc form with `A`'s vertex at 1, each at `B`'s radius 0.5 and
at the near ties 0.300001, 0.300005 and 0.3001. At ε 1e-6 the same
probe finds 12 such cells, at 1e-12 30.

## Repro

`sweep`
`a_closed_conic_in_the_partners_face::a_rim_flush_in_its_partners_top_tangent_inside_builds_or_meets_a_pinned_limit`,
whose `B ∖ A` cells with `A`'s vertex at 1 or π and `B`'s at 0, at rest
and turned, pin the current `OK BAD … t3p=false` line and that tier
3′'s findings are this wedge alone, citing this row.

## Shape of a fix

The reduction records `B`'s seam ruling as lying in `A`'s wall, as it
does `A`'s in `B`'s, and the parting then refuses it typed with
`CurvedFace` until
`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`
builds it. Whether the result gate should also read tier 3′ on such a
body is
`work/heldrecords/the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime.md`'s
question.

## Not this row: the S7 rods tangent outside

The same probe's S7 rods, tangent outside along a ruling, also ship
results tier 3′ refuses (44 cells at the default ε), but on a different
reading: `CensusUndecidable`, the census declining a curved face of one
solid within reach of another on a two-solid union. That is
`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`'s
class, where those cells are recorded.
