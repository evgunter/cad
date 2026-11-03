# CARVE leaves the tracker — 2026-11-03

CARVE (what a loft or sweep builds from its sections, and the bodies it builds that it should refuse) opened 2026-09-17 as BLEND's successor. It was cut into BAND, STRUT, CARVETAIL, CARVEREST and LATHE. It closed on 2026-11-03 (Ev, in chat) as a `blocked` track: two parked rows behind eleven closed ones and nothing dispatchable. Its plan set no `## Exit criteria`, so it closes without an exit walk.

**Where its live rows went.** Both moved to **LATHE** by `git mv`, ids and bodies unchanged, each with a dated note. LATHE holds the loft and sweep path they gate:
- `self-overlapping-spines-build-and-validate` (P0, H), parked on CLEAR's `the-at-rest-gate-certifies-embedding`;
- `the-sweep-doors-return-bodies-tier-3-refuses` (P3, M), parked on ONEHOME's `the-at-rest-result-gate-and-its-typing-live-in-five-homes`.

CARVE's "when they release" notes are now in LATHE's plan, and `crates/sweep/src/loft/*` joined LATHE's `paths`. CARVE's other paths stay with the programs that already claim them: BAND holds `blend/*`, `fillet.rs` and `chamfer.rs`, and STRUT holds `test_support.rs`.

**Citations.** Citations of the two live rows were rewritten to `work/lathe/`. No shipped file cited a closed CARVE row. Header references from CARVEREST's rows to closed CARVE rows were dropped.

Its A/B band 5600-5699 in `docs/MODEL-AB-LOG.md` closes with it.

Recover its program, plan, log and closed rows at 0f01217666:

    git show 0f01217666:work/carve/<FILE>
