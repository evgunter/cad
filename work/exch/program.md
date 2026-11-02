---
id: exch
kind: program
title: EXCH — STEP import recognition and promotion certificates
status: ready
opened: 2026-09-03
group: kernel/exchange
prefix: exch/
tag: (EXCH orchestrator)
ab_band: 2100-2199
paths: [crates/step-import/*, docs/STEP-BANK.md, scripts/step_import_check.py]
keep_out: [split 2026-10-29 into EXCH INTERP FOREIGN REIMPORT IMPORTGATE along their semantic seams (Ev approved in chat) - the five share crates/step-import by design so announce the seam where changes are coupled, code-quality Track U's STEP and STL rows are claimed — D343 moved into this directory, C13 and C14 closed in work/code-quality before the move; its pncad and pncad-py rows (D341) are LIB's and stay in work/code-quality until LIB claims them, topo/src/pcurves.rs is TRIM's — the ExtrudedPoint rung of nurbs_iso_derive is filed as a TRIM row and EXCH consumes it, geom-core/src/spline/compose* is PROPS' glob since S-CERT's exit (Track N claimed whole by PROPS, DOC-LEDGER sweeps 7 and 11) — the derivative channel and the tensor hull are filed as PROPS rows when their units are cut, geom-brep surface recognition (the M7-6 lane) is PROPS' (Track R claimed whole) — announce, the pncad-py Python side of any option-surface change is LIB's unit, tcost and tint hold crates/*/tests/* — their test-cost and test-integrity rows on the STEP crates' tests stay theirs and EXCH's unit edits to those tests ride unit PRs, dm1's arc-rim MapResidual frontier is TRIM's on both files — EXCH consumes work/issues/arc-rim-mint-frontier-on-foreign-charts and does not build it]
---

**What the importer recognises, what it promotes, and what it certifies
about the promotion.** Certified curve recognition on import (the
circle's map obligation; open arcs, ellipse, helix), the plane and
cylinder surface promotion, and the certificates those promotions
carry: a sup rooted to nearest, a refused locus dropped by `f64::max`,
a round-to-nearest bound called a certificate, a chart sense read at an
unchecked midpoint.

EXCH was cut on 2026-09-20 (Ev, in chat) into EXPORT and a remainder,
and on 2026-10-29 (Ev, in chat), at 133 budget points against 30, into
this track and four successors: INTERP (D7's ε_in interpretation),
FOREIGN (foreign faces at the door), REIMPORT (the round trip and the
writers) and IMPORTGATE (the door's validation and its refusals). EXCH
keeps its band 2100-2199.

Charter and unit order: `work/exch/plan.md`; narrative in `work/exch/log.md`.
