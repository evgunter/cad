---
id: certify-zero-arms-quote-no-margin-without-a-seam
kind: issue
title: geom-brep: certify's zero arms render the tighten offer without its value, because a margin derived at T: Decide is no refusal payload outside a ratified Bounds seam
status: dispatched
opened: 2026-09-28
priority: P3
cost: M
design: true
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
- `CertifyError::NotSecondOrderSeparated` (`tangent_second_order` per
  sample) and `CertifyError::TubeNotSeparated { verdict: Definite::Zero }`
  (`tangent_tube_margin`);
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

## Not a new channel

A `Decide`-side read of a DEFINITE margin extends a channel that is
already ratified rather than minting a new seam: `Indeterminate`
carries `MarginDiag::Value(f64)` / `MarginDiag::Enclosure { lo, hi }`
out of generic `T: Decide` code on every `Escalated` payload
(`geom_core::predicate`, `Decide::sign_within` and `Indeterminate`),
and `recourse::SizedDecision::recourse` already branches on it to
quote `m/K` on the undecided arm. D4 ¶1 (ii), as Ev ruled on PR 3352:
"`Indeterminate` carries data, not a recourse: its Display renders the
payload". The fork is whether that data may also ride a definite
verdict.

## The zero span: two texts disagree

D4 ¶1 (i) calls a Zero-where-Zero-fails arm band-decided (lever and
conditional tolerance). `CertifyError::IntervalNotForward`'s own docs,
from Ev's `e1600790f9` (M2 PR 3 fix pass, N1/N2), say "no M2
construction mints zero-length edges … so a zero span is always a
defect, not data". The branch keeps Ev's ruling: the span's Zero arm
ends in the defect ending at every reading (the kernel's at a build,
the file's too at rest and at adoption). Which text governs is part of
this fork.

The span and the tangent tube's margins are SIGNED, unlike the
magnitude margins (`dihedral_wedge`, `tangent_second_order`,
`plane_nurbs_transversality`): a Zero there may hide a sub-ε reversal or
an exact zero (`fixed_zero_length_edge_refused`) that no smaller ε
passes, so an unvalued tighten is worse on a signed arm than on a
magnitude one.

## A consumer: the [ev] import-readings fork (PR 3380)

PR 3380's door-side "m ≤ ε_in" decision needs the span's margin, and
`IntervalNotForward` carries none today; `poleband_eps12.step`
(`crates/step-import/tests/tier_gate.rs`'s POLEBAND12 cells) is its
witness. A ruling here decides what that door can read.

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
verdict alone, where a site may not carry the margin, and goes once the
margins are readable.
