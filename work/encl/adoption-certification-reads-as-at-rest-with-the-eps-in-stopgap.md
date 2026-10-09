---
id: adoption-certification-reads-as-at-rest-with-the-eps-in-stopgap
kind: issue
title: step-import certification refusals read as at rest, with the door's ε_in size decision and the set-ε-to-ε_in stopgap (D4 ¶1, [ev] PR 3380)
status: closed
closed: 2026-10-08
priority: P2
cost: M
opened: 2026-09-29
pr: 4331
---


(Filed by the ENCL orchestrator on Ev's ruling on `[ev]` PR 3380, 2026-09-29.) The rule is D4 ¶1 in `docs/DESIGN.md` as amended by that PR.

## What

Three changes, all following the ruling.

**1. Adoption reads certification as at rest.**
- `geom_brep::recourse::Reading::Adopt` suppresses the tighten offer and ends an undecided approximation in "kernel defect or damaged file". Ev's ruling reads certification at import as at rest.
- So delete `Reading::Adopt`, and render step-import's `Adoption` and `Assembly` at `AtRest`, as `TierInvalid` already is.
- Where the lever should read for someone holding a file, one line in the `Adoption` preamble says levers apply to the source model, then re-export.

**2. The import door's interim size decision.**
- Until the D7 rebuild stage (`work/exch/adoption-rebuilds-caches-at-eps-from-eps-in-interpretation.md`) exists, a band-decided sized refusal whose margin m ≤ ε_in must not offer "if this size is intended, tighten": the file itself declares that size coincident.
- The door decides this with ε_in and ε, which only it knows. It should end in: "this {size} is below the file's declared coincidence distance ε_in = X m, so the file does not state it. Recourse: re-export with the geometry resolved, or its uncertainty declared below m".
- `certify::recourse` stays ε_in-free.
- The spans' definite arms carry no margin (`work/encl/certify-zero-arms-quote-no-margin-without-a-seam.md`). Say which arms the door can and cannot size.

**3. The stopgap.**
- A certification refusal whose miss lies within ε_in but beyond ε names setting ε to ε_in as a stopgap, beside re-exporting more precisely.
- `nist_ftc_09` at 1e-12 today reads "kernel defect or a damaged file", and it is neither.
- Where the definite arm carries no miss value, the door can still compare ε_in with ε.

Re-pin `tier_gate` and remove its `assert_adoption_reading` guard, or replace it with one that holds the new rule.

## The ε_in comparison and the reporting-margin fence (2026-09-29)

Ruled on #3402 (the fence in `real.rs` clause 2) and in force since PR 3418: the reporting margin is for error reporting only. This row's door comparisons (m ≤ ε_in → withhold the tighten offer; a miss within ε_in but beyond ε → the set-ε-to-ε_in stopgap) choose which sentence the import error shows and nothing else. Import, build and refusal outcomes do not change. They therefore fall within the fence's "error reporting" use; no `[ev]` question is needed (confirmed in the orchestrator session, 2026-09-29).

Implement the comparison inside geom-core, beside `sized_recourse`, as a sentence-returning method on the reporting margin, and add it to `reporting-margin-door.sh`'s gated sentence list. The import door never holds the number.

## Closed

2026-10-08. PR 4331 merged at `fd14a18873`. It had a full review, three fix passes, a REQUEST-CHANGES re-review that withdrew my F3(c) order (D4 ¶1 (i)), and an APPROVE-WITH-FIXES delta review. Hosted CI was green on the merged head.

Follow-ups filed:
- `work/encl/no-tighten-offer-quotes-a-value-below-the-runs-resolution.md`
- `work/encl/plane-nurbs-limb-refusal-carries-no-reporting-margin-for-the-import-door.md`
- `work/encl/refused-arm-sign-certain-carries-no-margin.md`
- `work/encl/import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span.md`
- the EXCH row `step-import-placement-pcurve-and-rim-refusals-skip-the-import-doors-reading.md`, widened to cover `TierInvalid`.
