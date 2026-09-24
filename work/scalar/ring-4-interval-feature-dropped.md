---
id: ring-4-interval-feature-dropped
kind: unit
title: RING-4: the interval feature is dropped — the certified code compiles in every build, CI's lane axis collapses, Q1 says what is true
status: closed
opened: 2026-09-24
closed: 2026-09-24
branch: scalar/ring-4
pr: 3154
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

## Closed (2026-09-24) — PR 3154

Signed off by Ev on the PR's decision section (Q1, `DESIGN.md:266`, the
GUI-DESIGN wasm paragraph, the two `memories/` examples,
`docs/prompts/implementer-discipline.md` §2), including the cost: the
§0 gate read GO on the three-run aggregate (+31 % job / +36 % step;
one sample +52 %), and the run afterwards costs about what the base did
(136.8 job-min vs 133–148) — the fold's saving is spent on the certified
code compiling everywhere. The `interval` feature is deleted, not
no-op'd: 16 manifest entries, 54 `src` cfg lines, 10 in `demos/tour`,
5 in examples, 116 whole-file and 172 item-level test gates, six
`all(probe, interval)` → `probe`, the six loud-skip rows deleted
(`test_utils::loud_skip_marker!` stays — the viewer's three `app` rows
use it). CI's lane axis collapsed onto the interval rows under the
default rows' keys; the backend and oracle jobs stay. Coverage receipt:
before-interval = after (9523 at the reviewed head), before-default −
after = the six loud-skip rows. The tour's two certified narrations run
as `demo-tour certified` in `demos tour suite`, not in the render and
tess-budget walks. Single full Opus review, APPROVE-WITH-FIXES 0/3/6;
fix pass took everything but S2. Head `c9b4e232e7`, run 35983416425
green. Mooted and closed with it:
`ciw/interval-only-selection-premise-restored`,
`ciw/interval-cfg-gate-names-the-wrong-cause-for-an-attribute-order`,
`scalar/gate-on-the-type-in-prose-outside-geom-core`. C9's feature
clause stays false until RING-3 (#3153) lands.
