---
id: a-picked-half-rim-blends-half-and-refuses
kind: issue
title: a click on a full-revolve rim picks one half-arc, so the blend tool's fillet refuses SeamVertex and its recourse is a kernel-only query
status: closed
opened: 2026-10-02
closed: 2026-10-31
branch: vseam/a-picked-rim-is-the-whole-rim
pr: 6129
priority: P1
cost: M
---


## Finding

On a full revolve of a profile that touches the axis, every latitude
rim is TWO edges, `BandRim` (`[0, π)`) and `BandRimPi` (`[π, 2π)`),
meeting at two seam vertices. The viewer draws one circle. One pick is
one `StableName` (`crates/viewer/src/session/select.rs`,
`EdgeSelection::name`), so clicking that circle selects half of it,
and the blend tool (`crates/viewer/src/blend.rs`) commits a
`Node::Fillet` over that half. The fillet refuses
`UnsupportedCorner { corner: SeamVertex }`, whose recourse
(`sweep::blend::FILLET3_SEAM_VERTEX_RECOURSE`) says to request every
arc, as listed by `rim_of`. `rim_of` is `topo::query::rim_of`, a
kernel query over arena keys that the viewer offers no action for.
The user can recover by shift-picking the other half if they guess
that the circle is two edges. Nothing on screen says it is.

Measured on the tour's teapot lid (`demos/tour/src/teapot.rs`,
`rim_arcs`, show `teapot-lid-unbored`): each of its three rims refuses
at radii 2/256, 1/256 and 1/512 when asked for by the `BandRim` name
alone, and builds when asked for by both names.

## Relation to neighbours

- `work/offer/a-refusal-offers-no-action-in-the-viewer.md` is the
  general row (a refusal's recourse should be a control where the
  viewer can act). This is a specific instance whose control is
  obvious: "add the rest of the rim to the selection". But the better
  fix is upstream of the refusal, at the pick, so it is filed here, on
  the program that owns the pick and the blend tool.
- `work/emit/band-rim-pi-has-no-minting-builder.md` is the authoring
  side of the same gap (no builder mints the `BandRimPi` name).
  2026-10-06: PR 4164 adds `band_rim_pi` (`pncad::select`, and
  `pncad.band_rim_pi` in Python), so a whole rim is authored by
  `band_rim` and `band_rim_pi`; the pick side here is unchanged.

## Fix shape

When the blend tool takes an edge pick whose edge is one arc of a
seam-split rim, extend the held set to the whole rim (`rim_of` on the
evaluated body, mapped back to names). Or offer that as a one-click
action on the refusal. Either way the panel should show a rim as one
pick.

## Re-banded P2 → P1 (on request in chat, 2026-10-25)

Measured again on the lily's lantern (`demos/tour/src/lily.rs`): its
mouth, the sphere zone × pucker cone circle, refuses
`UnsupportedCorner { SeamVertex }` at r = 0.005, 0.01 and 0.02 when
either half-arc is asked for alone, and builds tier-3 valid at all
three when both halves are asked for together. Lily wall 6 had read
that refusal as a frontier ("the kernel cannot fillet a curved body")
for as long as it stood, which is the cost of this row in one
example: rounding a visible circle is an everyday request, and the
one pick a person makes for it refuses. The scene now asks for both
halves (`lily::roll_the_mouth`). The upstream cause, that a full
revolve leaves every circle as two edges, is
`work/profile/lower-profiles-to-carrier-and-interval-not-vertex-and-bulge.md`.

## Closed

2026-10-31, PR 6129, at the pick. The pick index records each drawn
body's closed rims of two or more arcs (`topo::query::rim_of` on the
body it drew, mapped back to names through that body's table) and
answers `PickIndex::whole_edge`. `Tools::feed` takes the index, and
the blend tool takes a click as the whole drawn edge: a rim is one
pick in the panel's count, and un-picking either arc drops the rim.
`load_all_edges` groups the same way. The selection value stays one
name (GQ7). The row is
`blend_authoring::a_click_on_a_seam_split_rim_blends_the_whole_rim`:
one click on a full-revolve cap rim, then a fillet commit, builds a
tier-3 valid body, and the same flow on the merge base refuses
`UnsupportedCorner { SeamVertex }`. The hover and selected lanes still
light half the circle; that is
`work/vgeom/a-hovered-or-selected-rim-marks-half-its-circle.md`.
