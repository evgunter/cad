---
id: carve
kind: program
title: CARVE — what a loft or sweep builds from its sections, and the bodies it builds that it should refuse
status: ready
opened: 2026-09-17
area: kernel
prefix: carve/
tag: (CARVE orchestrator)
ab_band: 5600-5699
paths: [crates/sweep/src/blend/*, crates/sweep/src/revolve/*, crates/sweep/src/extrude.rs, crates/sweep/src/fillet.rs, crates/sweep/src/chamfer.rs, crates/sweep/src/skin.rs, crates/sweep/src/swept.rs, crates/sweep/src/loft.rs, crates/sweep/src/test_support.rs, crates/sweep/src/lib.rs, crates/sweep/README.md]
keep_out: [OPENED 2026-09-17 in BLEND's cut as its one successor on BLEND's ground - BLEND closed at its exit walk the same day (docs/DOC-LEDGER.md sweep 17) and every path above is this program's alone, crates/sweep/tests/* and crates/test-utils/* are S-TCOST's and S-TINT's by declaration - units add rows there as ordinary tests and a test-mechanism change is announced to both, crates/profile/* is PATHS' - the profile fillet door's rows moved there in the same cut and a fix on that side is filed on PATHS and never diffed from here except by announced seam, geom-brep certify.rs and dihedral.rs and geom-core predicate.rs are PROPS-adjacent - the must-carry wrapper and the TangentParallel margin and the gap-sentence newtype are announced to PROPS, topo/src/query.rs is unowned and TOPO edits it as its own ground - rim_of's row moved there, editor-core verbs/blend.rs and names/emit_*.rs are EVAL's ground and a name-emitter row found here is filed there, the S14 ruling holds the typed-error announcement sites in blend/ - they stay typed errors while it is open]
priority: P1
---

**The loft and sweep path's own bodies, after the 2026-09-20 cut**, and
specifically the ones it builds when it should refuse. A loft or sweep
whose spine revisits itself — a planar arc past a full turn — builds
and validates. A two-section loft whose top section's plane normal
points DOWN, against the sweep, builds. `loft_geometry` takes the whole
surface's `v` from the first strip, so a section authored rolled
relative to the first changes the body rather than being rejected or
normalised. `skin.rs` refuses coincident sections by a bare `f64`
strict comparison, which is the unmargined predicate Q1 forbids.

These are the expensive failures: not a refusal a user can work
around, but a body they will carry downstream believing it sound.

CARVE was cut on 2026-09-20 (Ev, in chat) from 73 budget points into
three tracks meant to run in parallel: BAND (the blend and rim bands),
STRUT (one sweep rule with several homes) and this remainder. CARVE
keeps its band 5600-5699.

Charter and order: `work/carve/plan.md`; narrative in `work/carve/log.md`.
