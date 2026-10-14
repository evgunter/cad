---
id: a-concave-fill-and-a-whole-shell-inside-a-bands-region
kind: issue
title: blend: what a band does with the solid's other features in and around its region — the rule (Boolean semantics), what is built, what refuses and with what recourse
status: open
opened: 2026-10-13
priority: P2
cost: H
---


Split from `ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver`
at its step 4 (PR 4957), which built the ruling's convex half for the
planar band and the chamfer and stopped at the concave fill.

## The ruling it applies

Ev, PR 4285: "A feature wholly inside the material a fillet or chamfer
band removes (convex) or fills (concave) goes with that material, as a
feature wholly inside a Boolean's tool goes with the tool; a feature
crossing it still refuses."

On the concave side the band adds material in a region F that is air in
the source. Three shapes meet F, and one shape meets both sides.

## Ruled (Ev, PR 4963, 2026-10-14)

As recommended below: Boolean semantics is the blend's definition
(convex `S − R`, concave `S ∪ F`). A crossing refuses as an unbuilt
case, with the intention to build it once the Boolean serves the
band × feature surface pairs ("keeping the intention makes sense"). A
mouth the fill covers is sealed. An advisory lint, for every operation,
reports a feature an op makes vanish or change type unasked.
`docs/DESIGN.md`'s void-insertion convention gains the blend's seal as
its fourth producer. The build order is the one below.

Erratum (2026-10-14, PR 5099): the table's concave closed-shell cell
said an absorbed island's voids "become the solid's"; under `S ∪ F`
they are filled, since the prism's complement is connected and so the
island's voids lie in `F`.

## The question

What does a fillet or chamfer do with the solid's other features in and
around the material its band removes (convex, region `R`) or adds
(concave, region `F`): features wholly inside the region, a hole whose
mouth a fill covers, a closed void or island inside it, and a feature
that crosses the band surface? Ev asked for one simple rule instead of
case-by-case readings ("probably treating it exactly like a boolean,
and maybe also having a lint"), and tied that choice to what happens
to a crossing feature and to how convenient the refusal's recourse is.

## Recommended

**The rule: Boolean semantics is the blend's definition.** A convex
band's result is `S − R` and a concave band's `S ∪ F`, where the region
is the material between the band surface and its supports' strips over
the band's extent, cut off at its end faces (`blend/reach.rs`'s region),
read from the supports' carriers and never from the features near it.
Every case follows by set algebra:

| Case | Convex band (`S − R`) | Concave band (`S ∪ F`) |
|---|---|---|
| A feature wholly inside, attached by cycles on the faces bounding the region | goes (the through-bore is built: steps 1–4) | a protrusion is absorbed |
| A closed shell wholly inside | a void goes; a free solid inside `R` goes with it | an island is absorbed, its voids with it: they lie in `F`, which fills them |
| A hole whose mouth lies wholly on the face the fill covers | — | sealed: a through-hole turns blind, a blind hole becomes a void |
| A feature crossing the band surface | refuses as unbuilt | refuses as unbuilt |

**A crossing refuses as an unbuilt case, not as an exception to the
rule.** The result is defined; the kernel does not build it yet, as
the Boolean itself refuses the surface pairs it does not serve. The
refusal names the face and, where it exists, the reorder: in the API
`fillet(subtract(body, hole))` becomes `subtract(fillet(body), hole)`.
Two facts bound that recourse:
- A selection written on the feature's output names the feature's node
  (`From { read, of }`, N1 in `crates/editor-core/src/names/README.md`),
  so after the reorder the fillet's edges are re-queried on `body`, or
  unwrapped to their `of`. An edge the feature split has no one-to-one
  counterpart upstream; re-querying picks the whole parent edge.
- A reorder reaches the same body only for a feature of the band's own
  sign (a hole under a convex band, a protrusion under a fill). For the
  opposite sign (a boss under a convex band, a hole under a fill) it
  gives the other body, which is how a user chooses it. Where the
  obstacle and the filleted edge come from one node (a hole in an
  extruded profile) there is no reorder; the profile is rewritten.

**Built by surgery, as today.** The band and its supports stay in-place
surgery. Every non-crossing case is built by the blend's own carve, the
seal and the shell removal sharing `topo::insert_void`'s bookkeeping, so
every cavity keeps one home (`docs/DESIGN.md`'s void-insertion
convention gains the blend as a producer). The blend is not run as a
literal Boolean against a body for its region: that tool is cusp-edged
along both trimlines, whose Boolean arm is unbuilt, and most
band × feature surface pairs are outside the intersection table (C5).
When the crossing case is built, it is the hybrid: the surgery keeps
the band and its tangency to the supports, and the Boolean's
intersection machinery cuts only the band face against the faces that
cross it, once those surface pairs are served.

**A lint, for every operation.** An advisory check that never gates
reports when any node makes a feature vanish or changes its type that
the node was not asked to change: a hole sealed or made blind, a void
removed, a solid absorbed or removed. It reads the naming record and
the body's counts, so a union that seals a hole reports the same way
as a fillet that does.

**Order.** (1) Write the rule into C8 and `crates/sweep/README.md`, and
re-word the crossing refusal's recourse to the reorder (and to what is
true where none exists). (2) The concave bar. (3) The one-cycle members:
blind bore, pin, boss, dimple. (4) Whole shells and the seal, sharing
`insert_void`'s bookkeeping. Step 1: PR 5077. Step 2: PR 5099. Step 3: PR STEP3PR
(the end-wall half). The lint and an editor reorder action
(re-point plus the selection unwrap, one action) run beside any step;
the crossing arm waits on the intersection table.

Two designers (fork-log row 140) recommended this from their first
reports; it replaces row 139's case-by-case recommendation.

## Findings (step 2: the concave bar)

- **Built** for the plane–plane fillet and chamfer and the ruled cut-off:
  a cut-off's cap ring read inside its sliver carries its crease on
  either convexity (`surgery.rs`, the `CapMeter` list), and nothing else
  reads the side — the fill's section is the convex removal's region,
  the prism reading is geometry, the carve topology. Rows:
  `band_concave_bar`, `editor-core` `edit_concave_bar`.
- **Refused, for steps 3 and 4** (pinned in `band_concave_bar`):
  - a bump wholly inside `F` on an otherwise absorbable bar refuses as
    a bound (`FaceClearance { bounded: true }`): the bar's face would
    carry a ring, and the reach's bound reads first;
  - a bar with an enclosed void (a sealed tube) refuses `WhollyInside`
    at the void's shell, which has no cap cycle: step 4's whole shells;
  - a blind hole in an end wall, its mouth inside the sliver, refuses
    as a bound, in one wall or both.
- **Filed:** `an-oblique-caps-inside-reading-charges-a-ring-its-major-semi-axis`
  (a hair inside the fillet's arc at a leaning wall reads a bound, both
  sides alike).

## Findings (step 3: the one-cycle members, end-wall half)

- **Built** on the plane–plane fillet and chamfer: a cup on one
  cut-off cap, wholly inside the band's region — a blind bore under a
  convex band (`S − R`), a pin under a concave one (`S ∪ F`), an
  undercut bore or a headed pin, two cups from the two caps, a square
  pin. Rows: `band_one_cycle_members`, `editor-core`
  `edit_one_cycle_members`. The readings: the cap's existing three-way
  sliver reading for the mouth; each other edge of a cup face read
  inside the band's exact core piece by piece (`reach::curve_in_core`);
  a plane face not parallel to the spine certified by its boundary
  (the section is simply connected); the cell walk's inside reading of
  a face in a straight band's exact core now consumable, keyed by the
  band's crease. `consume` takes a disk (`V − E + F − R = 1`) on one
  cycle beside the annulus, bridges a face's rings to its outer cycle
  before the merge, and requires every face of a feature to be read
  inside the band whose caps carry its cycles.
- **What moved among the step-2 pins** (`band_concave_bar`): the pin
  from one end wall builds (it is this step's member); the bump on the
  bar builds too (`a_bump_on_a_bar_inside_the_fill_is_absorbed_with_it`,
  `ΔV` in closed form): the bump's footprint ring on the bar's wall is
  bridged and the bar and bump go as one annulus. The sealed tube still
  refuses `WhollyInside` (its void shell has no cap cycle: step 4), and
  the blind holes in the walls still refuse as a bound (they run into
  the die, outside `F`; `S ∪ F` seals them: step 4). Elsewhere: the
  convex blind hole under the chamfer in
  `band_planar_cut_off_meters` and the square wall's blind hole in
  `band_planar_oblique_fillet` now build; the planar-consume blind-bore
  refusal row is gone, its case built here.
- **Still refusing, typed, at the meter that reads it**: a cup on the
  ruled band (a bound: its reach has no exact core,
  `a-ruled-bands-cup-has-no-core-to-read-its-far-faces-in`); a cup whose
  axis leans off the spine under the fillet, charged its box diagonal
  times the lean (a bound, conservative); a mouth or head across the band
  surface (a measurement); everything on a support's strip (the dimple
  and the boss: `a-cup-whose-mouth-lies-on-a-supports-strip-is-not-carved`).

