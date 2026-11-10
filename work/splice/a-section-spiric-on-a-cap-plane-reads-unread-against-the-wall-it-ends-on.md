---
id: a-section-spiric-on-a-cap-plane-reads-unread-against-the-wall-it-ends-on
kind: issue
title: A section spiric on one operand's cap plane is asked against that operand's cylinder wall, which shares both its ends, and refuses unread: the tilted rod's β 0.7 union stops there
status: closed
opened: 2026-11-03
priority: P1
cost: M
closed: 2026-11-04
pr: 6523
---


Filed by SPLICE's `splice/same-operand-rider` lane, from its Phase 0
reading of the tilted rod's stop (the fourth row of
`an-unread-rider-related-only-by-its-own-vertices-refuses`).

## Finding

`germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
pins the β 0.7 union (`h ∪ c`, spins 0 and π/2) at `"unread rider"`
(`EdgeInFace { CarrierUnread }`, `OTHER_SPINS_STOPS`). Measured on main
at `5f6aaf16` with a probe in `slit::riders` (not committed):

- the edge is a **section chord the join minted** (provenance `Mef` at
  one spin, `Mekr` at the other; its split root is itself, in neither
  operand's input), bounding the half donut's torus wall and the rod's
  plane cap: a spiric, in the cap's plane;
- the face is the **rod's cylinder wall** (the other operand's input
  face, through the graft);
- the two are related only by both ends lying on the wall by structure
  (`Names { ends: true, record: false }`): the chord ends on the rod's
  rim circle, where the cap meets the wall. No vertex-vertex partner,
  no record names the pair, no crossing.

So it is **not** the same-operand case: the edge belongs to no
operand's input, and the lineage exemption leaves it refused, as it
must. What leaves it unread is `slit::carrier_on_surface`'s
`(Spiric | Nurbs, non-spline surface)` arm, which certifies only off a
plane (`clears_plane`); a spiric against a cylinder has no reading.

## What would read it

A spiric lies in its own plane `P` (here one of its own faces' plane).
A cylinder meets `P` along a conic, so the spiric lies along a piece of
the wall only where it runs along that conic. That is a reading of two
curves in one plane (a spiric against a circle or ellipse, the
`geom_brep::curve_pair::meet` table's walk), not of the spiric against
the surface. Where `meet` reads the two as meeting at isolated points
only, the carrier is `Placed::Off` the wall. The same shape answers a
spiric against a cone or sphere (a conic or a circle in `P`), and a
torus wall in `P` (circles).

Rows: the β 0.7 union at spins 0 and π/2 moves on from `"unread rider"`
to its next stop (re-pin `OTHER_SPINS_STOPS` there), and a spiric that
does run along the rim conic still reads as lying on the wall.

## Closed (SPLICE, PR 6523, 2026-11-04)

`slit::conic_or_line_on` takes a section spiric against a sphere, cylinder or cone through a new arm, `slit::spiric_on_quadric`. It reads `Placed::Off` or `Unread`, never `On`, and it escalates an undecided reading (`BooleanError::coincidence(EdgeOnCurvedFace, ..)`), as the conic arms beside it do.

The certificate is a degree bound, not the item's planar-meet plan: `curve_pair::meet` never reports an overlap. A spiric at a decided nonzero offset is an irreducible quartic (argued at the site; the `R − r` bound is kept as a conservative regime). A sphere, cylinder or cone cuts its plane in a curve of degree ≤ 2, so by Bézout the spiric lies along no piece of the surface. A band guard adds one requirement: an interior sample (¼, ½, ¾) must be decided off, so a spiric in band along its whole span stays Unread. A near touch between samples reads Off, as the conic arms already do; the spiric crossing door, `Door::Spiric`, is REACHHOLD's. `(Spiric, Torus)` stays Unread, a choice recorded on ONECOPY.

The tilted rod's β 0.7 union at both spins now stops at `"spiric wall volume"` (SPIRICARM's `a-torus-wall-holding-a-spiric-beside-a-spline-chord-has-no-volume-lane`). It is pinned by identity in `the_steeper_tilts_union_stops_by_identity`, measured with the same keys and payload at 1e-9, 1e-6 and 1e-12.

Single FULL review: APPROVE-WITH-FIXES. The fix pass adopted the reviewer's three rows, and all eight mutants are named red. Residue filed: ONECOPY `the-spiric-two-oval-regime-is-decided-in-three-places` (P3), and TCOST `the-tilted-rods-other-spins-row-runs-past-two-hours-at-1e-12-locally` (P2).
