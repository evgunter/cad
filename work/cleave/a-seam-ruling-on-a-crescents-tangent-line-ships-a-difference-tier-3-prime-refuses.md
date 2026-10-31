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
not recorded, so that gate never reads it. At rest; turned is
unmeasured. The PR 5808-era tree (`1953c3a21f`) built the same cell
`SOUND` with tier 3′ passing, so this arrived with the tier 3′ reading
that refuses the wedge.

## Repro

`sweep`
`a_closed_conic_in_the_partners_face::a_rim_flush_in_its_partners_top_tangent_inside_refuses_typed_or_builds`,
whose `B ∖ A` cells with `A`'s vertex at 1 or π and `B`'s at 0 pin the
current `OK BAD … t3p=false` line, citing this row.

## Shape of a fix

The reduction records `B`'s seam ruling as lying in `A`'s wall, as it
does `A`'s in `B`'s, and the parting then refuses it typed with
`CurvedFace` until
`work/splitdoor/the-boolean-builds-the-slit-on-a-curved-face-once-it-is-meshed-and-measured.md`
builds it. Whether the result gate should also read tier 3′ on such a
body is
`work/heldrecords/the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime.md`'s
question.

## Tangent outside too (probe `rev5808r`, S7)

The same probe's S7 rods, tangent outside along a ruling, show the same
shape on `main` (`45f6be756b`) and on PR 5852 alike, at every ε row:
`A` `r = 0.3` about the origin with its vertex at azimuth 0, on the
tangent line through `(0.3, 0)`, and `B` `r = 0.1` about `(0.4, 0)`,
`z ∈ [0, 1]`, `[0.5, 1.5]` or `[−0.5, 1.5]`, its vertex at 0 or π.
`A ∪ B` and `B ∪ A` return `Ok`, two shells, two solids, the
closed-form volume, tiers 1 and 3 and the certificate passing, and tier
3′ refuses (44 cells at the default ε, the one- and two-arc forms, at
rest and turned). With `A`'s vertex off the line (`baz = 1`) every cell
refuses typed, `CurvedPierceUnsupported`. Which reading tier 3′ refuses there is not read yet; the
probe at `1953c3a21f` showed the same cells.
