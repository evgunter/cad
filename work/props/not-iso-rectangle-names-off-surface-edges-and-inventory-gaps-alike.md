---
id: not-iso-rectangle-names-off-surface-edges-and-inventory-gaps-alike
kind: issue
title: NotIsoRectangle carries both an edge off its own surface (a defect) and a valid face outside the inventory, and only the raising site can tell them apart
status: open
opened: 2026-10-01
---


## What

`PropsError::NotIsoRectangle { what }` (`crates/geom-brep/src/props/mod.rs`)
is raised for two different facts:
- a valid face outside the closed form's inventory;
- a boundary edge that does not lie on its own face's surface.

The second is a defect on any body that has passed only the
structural tiers, such as the boolean's result at its volume
backstop. A consumer that wants to report it as one cannot do so from
the variant. The `what` names do not split it either. Every
`require_zero` site (`curved.rs`) raises the same variant, and some of
its residuals a valid face can carry:
- `props_meridian_great`: a lune's meridian that is not a great circle
  (`crates/sweep/tests/spiric_rim.rs` names that door);
- `props_rim_axis_parallel` / `props_rim_center_on_axis` on a sphere: a
  sphere circle cut by an oblique plane passes `props_rim_fit` (every
  circle on a sphere does) and fails these;
- `props_rim_fit` on a torus: a Villarceau circle.

Other residuals no valid edge can carry:
- a line on a cylinder that is not axial (`props_meridian_axial`) or
  not on the surface (`props_meridian_on_surface`);
- a cylinder or cone rim off the surface (`props_rim_fit` there);
- a cone line that is not a generator (`props_meridian_generator`,
  `props_meridian_apex`).

`mixed_levels` adds a third reading. Its doc calls the state
"kernel-bug-only", yet it emits `NotIsoRectangle { props_rim_level }`,
the name the notched-wall inventory gap
(`a-notched-cylinder-wall-has-no-volume-measurement`) also carries.

## Why it is filed here

REACH's volume backstop (branch `reach/volume-backstop`, 2026-10-01)
first split `VolumeCorrupt` from `VolumeUnmeasured` with a hand-kept
census of the thirteen `require_zero` names. The examples above refute
that split. It now reads tier 3's one classifier
(`topo::validate::classify_mass_props`), which counts only structure
that does not resolve (`Corrupt`, `NullScaffoldEdge`) as a defect and
reads every `NotIsoRectangle` as not yet measurable. That is honest
but blunt: an off-surface edge in a boolean result reports as a
capability gap.

## The shape of a fix

The split belongs at the raising sites, structurally. Give
`require_zero` the premise it checks: an incidence of the edge on the
face's own surface, which a valid body cannot violate, versus an
inventory premise, which it can. Mint a distinct variant for the first
(e.g. `PropsError::OffSurface { what }`). Then `classify_mass_props`
reads defect from the variant, and both tier 3 and the backstop follow
it with no list to keep. Decide `mixed_levels` under the same rule.

## 2026-10-02 — the cylinder flux no longer raises `props_rim_level` (TANG, PR 3851)

A cylinder face's flux is now its chart Green form over every loop
(`geom_brep::props::curved_face_loops`), so the notched-wall inventory
gap this row cites no longer arrives as `NotIsoRectangle` from a
cylinder's flux lane; that row is closed. What this row is about does
not move: the cylinder's per-edge `require_zero` incidence checks
(`props_meridian_axial`, `props_meridian_on_surface`, `props_rim_fit`)
still raise the shared variant, and `props_rim_level` (with
`mixed_levels`' reading) still comes from the shape door, the
material-side gate and the cone, sphere and torus flux arms.
