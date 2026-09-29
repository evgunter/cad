---
id: adoption-certification-reads-as-at-rest-with-the-eps-in-stopgap
kind: issue
title: step-import certification refusals read as at rest, with the door's ε_in size decision and the set-ε-to-ε_in stopgap (D4 ¶1, [ev] PR 3380)
status: open
priority: P2
cost: M
opened: 2026-09-29
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
