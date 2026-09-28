# CONTACT-5 — the backstop clears a meeting pair only through the touch analysis

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-5.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/5-gate-and-beam`. It carries two rows; read both in full:

- `work/contact/partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate` (P0)
- `work/contact/a-beam-across-two-supports-edges-refuses-on-coplanar-edge-crosses` (P0)

Also read `work/contact/CONTACT-1.md`'s Closed section and the touch
analysis it landed in `crates/topo/src/census.rs`: `TouchSite`, `Cone`,
`touch_verdict`, `touch_lever`, and the lever rule's stated `sin α` gap.

## What is wrong

Arm 2 of `sweep_cross_solid_backstop` has two defects.

**The gate clears on a question it cannot answer.** The box gate asks
whether either solid's box could CONTAIN the other. When both
orderings separate on some axis, the pair clears before any touch
finding is read. That is correct only when the two boundaries do not
meet. Two half-overlapping cubes `[0,2]³` and `[1,3]×[0,2]×[0,2]` meet
only in touches. Their coplanar same-normal faces read `Crossing` under
CONTACT-1's analysis (measured by CONTACT-1's reviewers). The gate
still clears them over a unit cube of shared material. This is pinned
today as a wrong clear:
`bool4r2_probes::two_half_overlapping_cubes_are_cleared_at_the_gate`.

**A coplanar edge cross has no touch site.** When two boundary edges
cross in a common plane (a beam's bottom edge over a support's top
edge), `TouchSite::of` returns `None` for `EdgeEdgeCross`, so `blocks`
reads the finding as a crossing. An ordinary resting assembly, a beam
across two supports, refuses.

## The invariant

**Arm 2 clears a pair whose boundaries meet — any touch or cross finding
between the two solids — only when the touch analysis reads every such
finding as a rest.** The box gate stays a fast path for pairs with NO
finding between them. For a pair with findings, it is at most a
containment pre-check. It never clears such a pair on its own.

A coplanar `EdgeEdgeCross` is a touch. Its site is the crossing point,
and its cones are the two edges' dihedral wedges there: the same
`Cone::wedge` CONTACT-1 builds for an edge interior point. A
NON-coplanar edge cross is a pierce-like transverse crossing and stays
a crossing. Deciding "coplanar" is a `decide` under the run band, and
in band escalates.

## Settled design

- **One home.** The coplanar cross becomes a `TouchSite` variant or
  case, mapped through the same `TouchSite::{of, entities, verdict}`
  path as every other kind. Do not add a side path.
- **Gate order.**
  - First, collect the pair's findings (the exact sweeps already push
    them).
  - With no findings, the gate may clear as today.
  - With findings, run the touch analysis. Any finding that is not a
    `Rest` blocks, with its reason. A pair whose findings all read
    `Rest` then goes to the containment question as today: the probe
    material test when a box could contain the other, and a clear
    otherwise.
  - Work out whether the probe test is still needed for all-rest pairs
    whose boxes separate. State the argument at the site.
- **Honesty about the lever gap.** CONTACT-1 states that levered
  readings can read a dipping face as `Rest` at an obtuse sector, which
  the pair-level wedge masks. Routing more pairs through the analysis
  gives that gap more ways to matter. Re-run CONTACT-1's reviewers'
  obtuse-sector probe (`an_obtuse_sector_is_read_through_its_rays`) in
  a gate-separated pose. If a pair now CLEARS wrongly through it, stop
  and report: that would make the redesign row
  (`touch-cone-readings-are-levered-directions-not-face-distances`) a
  precondition.

## Rows

- The half-overlapping cubes flip from the pinned wrong clear to a
  refusal (`MixedTouch`). Rename the row.
- Two cubes sharing a face with OPPOSITE normals (side by side, in
  contact) still clear.
- The beam across two supports (the row's pose) clears. The same beam
  sunk 1 mm into a support refuses.
- A coplanar cross at an in-band tilt escalates.
- A sweep: several hundred axis-aligned brick pairs on a grid chosen
  to produce coplanar, collinear and flush coincidences. Take ground
  truth from grid sampling of the shared interior (CONTACT-1's
  reviewers did this). Report wrong clears (must be 0) and refusals of
  true rests, head against base.

## Review

**Dual.** The feared failure is a wrong clear, on the door every
consumer reads as proof.
