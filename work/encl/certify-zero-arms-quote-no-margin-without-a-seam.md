---
id: certify-zero-arms-quote-no-margin-without-a-seam
kind: issue
title: geom-brep: certify's zero arms render the tighten offer without its value, because a margin derived at T: Decide is no refusal payload outside a ratified Bounds seam
status: open
opened: 2026-09-28
---



(ENCL implementer, residue of
`certify-span-and-zero-arms-cannot-carry-their-decisions-full-ending`.)

## What

D4 ¶1 (i) phrases the tolerance on a band-decided arm "with the value
the margin gives". Four Zero arms still render it without one, through
`geom_brep::recourse::RefusedArm::Zero(None)` and
`SizedDecision::recourse`'s `tighten(None)`:

- `CertifyError::NotTransverse` (`certify.rs`, `run_checks`'s
  `classify_dihedral` arm);
- `CertifyError::NotSecondOrderSeparated { verdict: Definite::Zero }`
  (`tangent_second_order` per sample, and `tangent_tube_margin`);
- `CertifyError::IntervalNotForward { verdict: Definite::Zero }` (the
  four `interval_span_forward` decisions);
- `PlaneNurbsRefusal::NotTransverse` (`edge_nurbs.rs`,
  `plane_nurbs_transversality`), which `certify_via` re-speaks as
  `CertifyError::NotTransverse` so the door has one vocabulary.

## Why it is not an implementer's repair

Each margin is a `Margin<T>` at a generic `T: Decide`, and no `f64`
leaves a `T: Decide` for a refusal payload outside a ratified seam:
`geom_core::real::Bounds`'s scope rule, "The two shapes that are not
decisions (#990, ratified 2026-08-27)", clause 2 — "a door that wants
to echo a derived margin is asking to be a seam, ratified
individually", and "no general projection helper exists, by this
ruling". Ev wrote that text (`16650cf9b8`, `8aac555d66`). `certify.rs`'s
own allowlist entry (`scripts/gates/bounds-allowlist.sh`, "M7-8
2026-09-02, the lane split as a BOUND") covers the two lane doors, not
`run_checks`.

The valued arms that did land are the ones already inside a seam: the
plane × NURBS uniqueness tube (`ssi/certify.rs`, M6-2's entry) carries
`recourse::Refused` and quotes `m/K`, as the offset meters (f64
substrate) do. The lane's per-sample `NotTransverse` sits in a seam file
too (`edge_nurbs.rs`, M7-8), but its value would be dropped at
`certify_via`'s one-vocabulary mapping unless `CertifyError::NotTransverse`
carried an optional margin, which is this same question.

## The fork

A design question for designers, then Ev: either

1. ratify a payload seam for certification's Zero verdicts (a
   `Decide`-side read of the classified margin, or `T: Bounds` on
   `run_checks`), so these variants carry `recourse::Classified`; then
   delete `RefusedArm::Zero(None)`, the `Option`, and `tighten(None)`,
   which nothing else produces; or
2. rule that an unvalued conditional is the honest ending where no
   seam exists, and say so in D4 ¶1 (i).

## Convergence pointer

A decided margin is spelled three ways: `recourse::Classified`
(margin, band), `recourse::Refused` (the verdict carrying a
`Classified`, now shared by the offset meters and the SSI tube), and
`sweep::blend::ClassifiedMargin` (`crates/sweep/src/blend/mod.rs`
~:241: predicate, reading, band, sign). `recourse::Classified` is the
payload the other two should converge on; `recourse::Definite` is the
verdict alone, where a site may not carry the margin.
