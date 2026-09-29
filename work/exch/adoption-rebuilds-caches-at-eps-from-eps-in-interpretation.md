---
id: adoption-rebuilds-caches-at-eps-from-eps-in-interpretation
kind: issue
title: step-import: adoption certifies the file's parsed carrier at ε instead of interpreting at ε_in and rebuilding the caches at ε (D7)
status: open
priority: P1
cost: H
opened: 2026-09-29
---


(Filed by the ENCL orchestrator on Ev's ruling on `[ev]` PR 3380, 2026-09-29: "the real fix should be filed as p1".)

## What

D7 says an adopted entity's caches are "recomputed from its description by our own algorithms and certified at ε", with ε_in governing interpretation only. The adoption code does not do that. Each rung is "a full `set_edge_curve` certification of the **parsed carrier**" (`crates/step-import/src/adopt.rs` module docs, `adopt_edges` → `set_edge_curve_nurbs_lane(…, tol)`), and `crates/step-import/src/lib.rs` ("Two tolerances") calls the ε_in-consuming stage "M7-2+", i.e. unbuilt. So a residual refusal at import measures the file's print precision against ε.

**Witness:** `crates/step-import/tests/tier_gate.rs`, `(NIST09, 1e-12, "file", Refused(ENDPOINT_START_MAPPED_CURVE))`. `nist_ftc_09` declares ε_in ≈ 3.4e-5 m and prints about 12 digits. It imports at ambient 1e-9, but refuses at 1e-12 on an endpoint residual that lies within its own declared coincidence distance.

**Same seam, symptoms of this stage:**
- `work/exch/step-import-eps-in-ambient-two-dial-strand.md`
- `work/exch/step-import-adopts-frame-directions-on-eps-in-that-check-1-refuses-at-eps.md`

## Repair shape

Interpret each file entity at ε_in: which description its data is evidence of, where healing may move geometry by up to O(ε_in), and report any move. Then rebuild the caches with the kernel's own constructions from those descriptions (exact lines and circles; the SSI families `ssi.rs` covers), and certify at ε like native geometry.

Refusals of the file's evidence become interpretation refusals, named in ε_in with the value and direction ("import with ε_in ≥ a if the file is less exact than it declares"). A surface pair the kernel has no construction for keeps the file's curve as the cache and keeps the stopgap text.

**When this lands:**
- The ENCL door's interim size decision (`adoption-certification-reads-as-at-rest-with-the-eps-in-stopgap`) moves to the interpretation stage.
- The set-ε-to-ε_in stopgap in D4 ¶1 is retired, except for that leftover class.

Design record: `[ev]` PR 3380 and `docs/DESIGN-FORK-LOG.md` row 7.
