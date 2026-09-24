---
id: ring-4-interval-feature-dropped
kind: unit
title: RING-4: the interval feature is dropped — the certified code compiles in every build, CI's lane axis collapses, Q1 says what is true
status: dispatched
opened: 2026-09-24
branch: scalar/ring-4
---


## What

H5 ruling 1 cut (iii), its feature half. The `interval` cargo feature
is deleted (fail-loud, not a no-op): its 16 manifest entries, ~69 code
cfg sites and ~251 gated test files compile unconditionally; the six
loud-skip rows that only said the feature was off go; five editor-core
modules, `eval::leaf` and pncad's re-exports become default API; CI's
lane axis collapses onto the interval lane (already the superset), the
backend and oracle jobs stay, `check-interval-cfg-additive.py` retires;
coverage before and after is the receipt. A cost gate comes first:
hosted and local build cost measured against the ratified ~+36 %
ceiling, stop above +50 %. Q1, `DESIGN.md:266`, C9's feature sentence,
`GUI-DESIGN.md`'s wasm step and one `memories/` example re-worded (Ev's
text — the PR is `[ev]`). Spec: `docs/RING-4-SPEC.md` (deleted at merge).

**Review tier: SINGLE, full** (one Opus reviewer, claims plus the style
lane) — the design decision is Ev's and already made; the unit's risk is
coverage silently lost in the CI fold and a missed cfg site, which a
reader can check against the before/after listings and the grep, not a
tricky algorithm. Outside the suspended A/B protocol: Opus throughout.
