---
id: lib
kind: program
title: LIB — usable as a library
status: ready
opened: 2026-08-06
area: api
prefix: lib/
ab_band: 300-399
paths: [crates/pncad/*, crates/pncad-py/*, docs/LIBRARY-DESIGN.md, docs/RECIPE-DOORS-DESIGN.md, docs/GUIDE.md]
keep_out: [kernel crates are VERBS and SEAT ground (LIB carries recipe doors and bindings only), the viewer is GUI-era ground, the analysis lane is M10's, evaluate's signature and the resolver door are design conversations before they are units]
priority: P3
---

**The doors, after the 2026-09-20 cut.** LIB makes the kernel usable as
a library under `docs/LIBRARY-DESIGN.md` (the contract, ratified #229),
and what it keeps now is the half of that with a measured instrument:
the doors `docs/guide/north-star-audit.md` is waiting on.

The certified locally-valid range query has no Python door. The STEP
import declaration channel has no Python spelling, so `import_step`
withholds `declared_contacts`. The frame constructors are reachable
only as `pncad::geom_core::linalg::frame::*` while Python meets them as
`Frame` methods. Three `EvalOptions` fields defer to an evaluation door
that carries a box with width, and none exists. The two witness
`DocEdit` arms need a facade type before Python can build them.

The bindings' own housekeeping — stubs, tag pins, censuses, the prose
beside them — went to BIND at the cut (Ev, in chat: the north-star half
is medium priority and the rest is low). LIB keeps its band 300-399.

Live narrative: `log.md`'s tail; order in `work/lib/plan.md`.
