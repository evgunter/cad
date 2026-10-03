---
id: fillet-meridian-radius-term-is-registered-only
kind: issue
title: the profile fillet lowers through chord and bulge, so its extruded cylinder's Frame, Radius and FidelityU envelope terms do not close and M10-9's bracket and pad refuse at pcurve_envelope
status: parked
opened: 2026-10-02
priority: P0
cost: M
blocked_on: [store-constructed-carriers]
---


Found by `pcert/certificate-incidence-fidelity`, by measurement, once the
loop walk's branch became a literal (which closed
`loop-walk-branch-is-an-opaque-floor-atom`).

With the walk's branch literal, every one of check 4's terms on M10-7's
plate is a theorem over its box. The r2 filleted bracket and the pad
still refuse whole at `pcurve_envelope`. On the bracket at `3.87e2·ε`
the over-band term is `EnvelopeTerm::Radius`, numeric on four rows. All
four are `Derivation::CylinderMeridian` lines: the straight struts on a
fillet cylinder. A typical one is the carrier
`Line { origin: (0.00245, 0.0005, 0), dir: (0, 0, 1) }` on
`Cylinder { origin: (0.0022, 0.0005, 0), radius: 0.00025 }`.

The term is `norm_gap(r, r², radial(c − origin))` (`norm_gap` at
`crates/geom-brep/src/pcurve_cache.rs:4144`, the
`CylinderMeridian` arm of `incidence` at `:4191`). That is
`|r² − ‖q‖²| / (r + ‖q‖)`, with `q` the strut's offset from the axis. The identity `‖q‖ = r` holds on the fillet because
the fillet's construction registers it: the rim identity is a
registered door identity, as `carrier_matches_mapped_source` is on the
plate. But `r² − q·q` is a polynomial that is not zero in the
document's parameters. The strut's offset and the fillet's radius are
two spellings that agree only through the registered relation, and
`norm_gap`'s quotient never asks the door the question in the form the
registrant stated. So the term falls to the value channel and widens.

Spelling the gap as `|r − ‖q‖|` instead (the norm itself, which the
door could match) was measured in scratch. It made every radius term
numeric, on the plate as well, so it is not the fix.

The pad's ceiling refuses at the envelope too, and probably for the same
cause. Its per-term replay was too slow to finish in the measuring
lane, so this is not confirmed.

The question is design: where does the radius incidence of a strut on a
registered fillet get its identity? Two candidates:

- check 4 asks the door for `‖q − c‖ = r` in the registrant's spelling,
  before the quotient; or
- the fillet mints its struts' offsets from the radius, so that
  `r² − q·q` is the zero polynomial.

## A regression, disclosed (PR 3812's review)

This is a regression from main, not a pre-existing gap (R2, claim 9):
main's `m10_9_pins_interval::measured_studies` certify the bracket and
the pad whole at their brackets, and 3759 + 3812 together leave both
refusing there at `pcurve_envelope`. The unit's sequencing clause says
3759 "does not merge with the widening regression", so this row is
raised to P0 and carries that question to the orchestrator: block the
merge, or rule that it does not.

`m10_9_no_registrant_lies_on_any_measured_document` now ASSERTS both
refusals (`pcurve_envelope`), so the regression is a pinned row rather
than a truncated count.

Not fixed in 3812's fix pass, because it is a design question. The two
spellings measured so far fail in opposite directions:

- the quotient `|r² − q·q| / (r + ‖q‖)` is a theorem wherever `r² = q·q`
  is a polynomial identity (the plate's rulings), and numeric where it
  holds only through the fillet's registration;
- the plain `|r − ‖q‖|` was measured (scratch, `CIF_NORM_GAP`) to make
  every radius term numeric, the plate's included.

So neither spelling reaches the door in the registrant's form. The
candidates are still the two above: ask the door for `‖q − c‖ = r` in
the registrant's spelling before the quotient, or have the fillet mint
its struts' offsets from the radius so `r² − q·q` is the zero form.

**After 3812's fix pass** (check 4 meters the frame on the chart's
Gram–Schmidt twin, `EnvelopeTerm::Frame`, bounded in the frame's
invariants), the bracket still refuses at `3.87e2·ε` on the envelope's
sum (`[0, 2.9e-9]` at ε = 1e-9), no single term over the band on its
own; the pad still refuses at `pcurve_envelope`.

## Weighed by a designer pair (2026-10-03): not a fork, parked on PATHS 5b

Two designers, briefed on the problem only, from main at 63aabc993. They converged on their first reports, and both measured on the bracket. **Premise corrections:**

- **The fillet is a sketch fillet** (`ProgramStep::Fillet`, lowered in `crates/profile`), extruded. `sweep::blend` is not involved. The sample carrier above (radius 0.25 mm at x = 2.2 mm) is `bore_b`'s seam, not a fillet strut.
- **Nothing registers the identity.** `m10_9_per_predicate_split_at_the_nominal` reads 16 theorems and 4 numeric for `pcurve_envelope`, the same with the registry shut or open.
- **Three terms stand, not one.** The four rows are the fillet cylinder's two rims and two struts. On each, three terms are numeric:
  - Frame, about 7.9e-10;
  - Radius, about 3.4e-10;
  - FidelityU, about 7.7e-10.

  Each is under the band alone; their sum is over it. Every standing atom is the fillet's chord-and-bulge spelling: `|L(1+b²)/4b|`, `tan(π/8)` through `sqrt 2`, `copysign`/`abs` for the turn. The boss's literal-bulge arcs, through the same extrude and certificate, are theorems 12 of 12.
- **Not a regression from main.** Before 3759, extrude minted no rows, so main never asked the question.

**The answer is already ratified.** D1, as answered on PR 3453: each arc mode lowers in its own algebra, the radius as authored, registrations only for what the algebra cannot close. That is PATHS unit 5b, `work/paths/store-constructed-carriers`. Both designers rejected two other homes:
- asking the registry in the registrant's spelling: the wrong node and the wrong quantity, and a discharge rather than C4's theorem;
- minting strut origins from the radius in the sweep.

Both add one line to 5b's fillet arm: spell the tangent points from the centre and the authored radius (`t = centre ± r·n̂`), with the turn as a decided literal sign. Then `r² − q·q` and the rim frame's `u_ref·u_ref − 1` are zero forms. Whether FidelityU also closes is likely but not measured; if it still bounds the ceiling, it becomes PCERT's next row. The pad is unmeasured (same construction, claimed by analogy). `work/paths/copysign-stands-in-for-the-turn-side-and-hulls-at-a-decidable-tie` is the same defect class.

Parked on `store-constructed-carriers`, with a seam note in `work/paths/log.md` asking PATHS to rank 5b for this P0 loss. Check after 5b lands: re-run the per-term probe (`m10_9_evidence_interval`'s split row plus `explain_depth`) on the bracket and the pad. This row closes when `m10_9_no_registrant_lies_on_any_measured_document` no longer asserts their `pcurve_envelope` refusals.
