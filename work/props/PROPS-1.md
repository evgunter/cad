---
id: PROPS-1
kind: unit
title: The lost-correlation members of the linalg audit — mirror_across_plane and reject_from respelled, one re-baseline pass
status: closed
parent: certified-lane-non-real-contract-audit
branch: props/1-linalg-lost-correlation
opened: 2026-09-05
closed: 2026-09-05
pr: 1918
refs: [certified-lane-non-real-contract-audit, 1277, 1143]
---

The first unit of the linalg interval-honesty lane, cut ahead of
S-CERT's exit because no live PR touched
`geom-core/src/linalg/{frame,vec,point}.rs` at dispatch —
`work/props/log.md`, the early opening entry.

What: `frame::mirror_across_plane`'s translation respelled through the
Householder identity so the anchor is mentioned once;
`Vec3::reject_from` respelled through the triple cross so `self` is
mentioned once; `Point{2,3}::lerp` decided and left with its `Interval`
cost stated at the site. Both respells move `f64` bits, so the unit
takes the golden / k-lint / render accounting both members owed, once.

Not here: the `rotation_about` diagonal floor
(`rotation-about-diagonal-width-floor`), the `orthonormal_basis` sign
hull (`interval-orthonormal-basis-sign-hull`, the next unit on the
file), `normalize`'s overflow (S-CERT's until the inheritance).

## Closed

Landed on PR #1918. Audit members 1 and 3 are closed, member 4 is
decided-and-left with its cost paragraph at both `lerp` sites; the
dispositions are recorded on
`certified-lane-non-real-contract-audit`'s `## Disposition` section.

Evidence: `crates/geom-core/tests/props1_evidence.rs` (the unit's own
corpora, as literals — four `#[ignore]`d instruments) and
`crates/geom-core/tests/props1_review_rows.rs` (the rows the blinded
dual review's falsification attempts contributed, adopted with a note
each). Twenty-five gating rows in all: 18 at `Interval`, 7 at `f64` in
both lanes.

**Two deviations from the spec, argued rather than met:**

1. Its pin (a) asks for "narrower than the old form on every corpus
   row". That is FALSE per component once the anchor is exact, where
   both widths are pure rounding floor: over the committed corpus the
   shipped floor is wider in some components, up to 1.43×. The pin now
   asserts the tightness bound (the image width of the anchor box, plus
   the floor) and a separate row measures the floor and pins its ratio.
2. `reject_from`'s new form AMPLIFIES `onto`'s width through two cross
   products: with a wide `onto` the shipped rejection is up to 34×
   wider than the retired one on the corpus, 1022× on a randomized
   sweep with a zero-straddling component. The form is KEPT — every
   in-tree consumer passes an exact stored axis as `onto` and a
   computed, often wide, vector as `self` — and the doc now states the
   trade with the measured ratios, with a gating row on the regression.
