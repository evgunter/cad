---
id: not-iso-rectangle-names-off-surface-edges-and-inventory-gaps-alike
kind: issue
title: NotIsoRectangle carries both an edge off its own surface (a defect) and a valid face outside the inventory, and only the raising site can tell them apart
status: review
opened: 2026-10-01
branch: props/recourse-grammar
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

## Resolved (props/recourse-grammar) — decided as a typing question

**Two arms, not better prose.** `PropsError::OffSurface { what }` is
minted for the incidence reading, and the split is taken at the raising
site: `require_zero` takes a `Premise` (`OnSurface` | `Inventory`) and
the definite arm is `OffSurface` or `NotIsoRectangle` accordingly.

**The premise is the one a residual CHECKS, and it is not a function of
the predicate name** — which is why a census of the thirteen names could
not have worked:

| residual | surface | premise |
|---|---|---|
| `props_meridian_axial`, `props_meridian_on_surface` | cylinder | OnSurface |
| `props_meridian_generator`, `props_meridian_apex` | cone | OnSurface |
| `props_rim_fit` | cylinder, cone, sphere | OnSurface |
| `props_rim_axis_parallel`, `props_rim_center_on_axis` | cylinder, cone | OnSurface |
| `props_rim_axis_parallel`, `props_rim_center_on_axis` | sphere, torus | Inventory |
| `props_rim_fit` | torus | Inventory |
| `props_meridian_great` | sphere | Inventory |
| `props_meridian_fit`, `props_meridian_plane` | torus | Inventory |
| `props_du_consistent`, `props_rim_only_closed`, `props_rim_only_join` | all | Inventory |

The test: is the residual NECESSARY for the edge to lie on the surface
at all, or does it additionally demand iso-ness? On a cylinder or a cone
the only circles on the surface are its cross-sections, so the radius
fit and the two incidences are all necessary. On a sphere the radius fit
alone is necessary (`‖w‖² + r_c² = R²` holds for every circle on the
sphere) while the incidence pair asks for the iso-v rim, which an
oblique-plane circle fails while lying on the surface. On a torus even
the radius fit is an iso-v demand, which a Villarceau circle fails. This
reproduces the finding's own three counterexamples exactly.

`topo::validate::classify_mass_props` now reads defect from the variant:
`P::OffSurface` is `defect: true` with the file-or-kernel ending, and no
list is kept anywhere. REACH's backstop follows tier 3's classifier, so
it follows too.

**`mixed_levels` is decided and left alone, with its argument.** Under
the same rule it checks no residual at all — its own doc says it is "not
routed through the funnel ... the pair has no comparand, so there is
nothing to decide" — so the premise split does not reach it. It is a
kernel-bug-only state, and its own doc parks the CHOICE of how to
express one (`unreachable!`, poison, or a typed refusal) on
`work/verdict/props-curved-carries-two-readings-of-d9-unreachable-vs-poison`,
noting that "a decision taken at one site would pre-empt it". Both
alternatives that census may pick remove the refusal entirely, so
routing it to a refusal variant now would be that pre-emption. Evidence
appended to that row instead.
