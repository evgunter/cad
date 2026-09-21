---
id: props
kind: program
title: PROPS — enclosure certificates and interval honesty
status: open
opened: 2026-09-03
area: kernel
prefix: props/
tag: (PROPS orchestrator)
ab_band: 2400-2499
paths: [crates/geom-core/src/k_stats.rs, crates/editor-core/src/stackup.rs, crates/geom-brep/src/props/*, crates/geom-core/src/*, crates/geom/src/*, crates/editor-core/src/analysis.rs, crates/editor-core/src/distribution.rs, crates/editor-core/src/drive.rs, crates/editor-core/src/measure.rs, crates/editor-core/src/mc.rs, crates/editor-core/tests/m10*, crates/editor-core/tests/e4_dual*, docs/ERROR-DESIGN.md, docs/DUAL-DESIGN.md]
keep_out: [OPENED 2026-09-06 at S-CERT's exit walk (#1924, ratified by Ev) — the inherited items and territory are in the paths above, code-quality Tracks R and N are claimed whole as S-CERT claimed M and N, the DL6 contract (docs/DUAL-DESIGN.md) is ratified — the linalg lane is an audit against it and not a design conversation, bounds-allowlist.sh is Track K's — the overshoot-payload ruling names the seam, the tess-budget re-baseline is this program's per S-MESH's keep_out, M10's analysis lane ARRIVED at M10's sweep 2026-09-13 (docs/DOC-LEDGER.md sweep 13) — analysis.rs and distribution.rs and drive.rs and measure.rs and mc.rs and the m10*/e4_dual* suites and ERROR-DESIGN.md and DUAL-DESIGN.md are in the paths above now and four of M10's rows came with them, crates/geom-core/src/sym.rs and sym/* are SYM's since that same sweep — Ev's call that the tier is a successor program and not this program's inheritance — and the fence is written on both sides: this program keeps the rest of geom-core/src/* including real.rs and ring_interval.rs and the linalg lane and SYM announces every seam it needs there, crates/bvh/src/* was M10's by this clause and is in NO program's paths after that sweep — a row on it is filed wherever its consumer lives until a program claims it, ssi* and pcurve_cache are TRIM's ground behind PCURVE P-2 and the coefficient doors' consumers there move by announced seam only, CUT 2026-09-20 with Ev's agreement in chat — three programs were cut out of this one when the slate passed sixty rows: QUAD (band 5700-5799) took crates/geom-brep/src/props/quad.rs and the seven quadrature rows, ENCL (5800-5899) took offset_fit.rs and patch_bound.rs and offset_meters.rs with the twelve certified-enclosure rows and is S-CERT's second successor, FRAME (5900-5999) took the eight rows of the who-answers-give-me-a-frame family and inherits the rest of it when this program lands the sign-hull unit (PR #2468, Ev's option-1 ruling on #1944, holding on a fold SYM owes it); this program keeps the closed-form flux arms in props/curved.rs and props/mod.rs, the verdict and escalation channel, and the geom-core scalar and spline doors, and SHARED GROUND WITH THOSE THREE IS EXPECTED rather than a conflict (Ev, in chat, 2026-09-20: it is ok if units have shared ground, they should just be aware of each other if working at the same time) — props/* is claimed by this program and by QUAD, geom-core/src/* by this program and by FRAME, and what the programs owe each other is awareness when a lane is live on the same file, carried by the per-branch territory check and the announced-seam convention rather than by a partition]
priority: P0
---

**The flux and rim arms, after the second 2026-09-20 cut**: what the
property layer measures about a face, and the faces it will not
measure at all.

Its foundational row is that `props_rim_side` and `props_rim_dir_group`
read whichever rim the loop walk from `Cycle::first` meets first, so
**their recorded signs are facts about cycle order rather than about
the face** — and everything else here rests on them. Beside it, the
rim-level rule's structurally-impossible arm feeds `f64::NAN` into
`classify` and `unreachable_zero` returns a 4-tuple of NaNs into live
flux arithmetic.

Three more are faces the kernel builds and this layer refuses: a cone
bounded by one rim with the apex interior, the lune family the sphere
flux arm's coplanar premise excludes, and a meridian that arrives in
lineage pieces where the torus arm folds pieces and the sphere arm does
not.

PROPS was cut TWICE on 2026-09-20 (Ev, in chat): first at 66 open rows,
opening QUAD, ENCL and FRAME; then at 108.5 budget points once every
row carried a band, opening NURBS, LINALG and VERDICT. PROPS keeps its
band 2400-2499.

Charter and lanes: `work/props/plan.md`; narrative in `work/props/log.md`.
