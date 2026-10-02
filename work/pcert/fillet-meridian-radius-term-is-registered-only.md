---
id: fillet-meridian-radius-term-is-registered-only
kind: issue
title: check 4's radius incidence on a fillet cylinder's meridian strut holds only by registration, which the quotient spelling cannot reach, so the bracket and the pad refuse at the envelope
status: open
opened: 2026-10-02
priority: P0
cost: M
design: true
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
`crates/geom-brep/src/pcurve_cache.rs:@NORM_GAP@`, the
`CylinderMeridian` arm of `incidence` at `:@MERIDIAN@`). That is `|r² − ‖q‖²| / (r + ‖q‖)`, with `q` the strut's
offset from the axis. The identity `‖q‖ = r` holds on the fillet because
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
