# LIB-LOG — orchestrator log for the usable-as-a-library program

Program contract: `docs/LIBRARY-DESIGN.md` (RATIFIED, PR #229).
This log is the program's operational record — unit dispatches,
unilateral orchestrator decisions (LB-numbered), and resting state
— in the M*-LOG tradition. This program runs concurrently with the
M6/M7 close-out (its own orchestrator, its own logs); the fence
between the two lanes' footprints is recorded per-spec.

## Rulings absorbed at program start (Evan, in-chat, 2026-08-06)

Recorded in LIBRARY-DESIGN.md §L8; operational consequences here:

1. **U1 + U2 authorized to start now** (LQ5 execution); units past
   that are delegated to orchestrator judgment where footprints
   are independent — Evan: "things past that likely are also
   viable." Genuine design forks still escalate.
2. **Façade placeholder crate name: `pncad`** ("pending-name CAD")
   — greppable, carries the Q9 rename debt visibly. See the
   name-candidates memory for the rename-time grep note.
3. **v2 profiles-as-programs spec timing**: the design-conversation
   draft waits for U2's algebra to be implemented AND the demo
   corpus reworked onto it — the rework is the evidence base for
   what the representation should be. Still ahead of U9 (§L3's
   "Python never ships the opaque-profile state" stands).
4. **A/B**: library-program implementation dispatches draw from
   their own LIB-labeled block series in MODEL-AB-LOG (no
   collision with the M7-N series the other orchestrator draws).
5. **Lane slots**: Evan is building flock-based build-slot locks
   (`cargo-slots.txt` is RETIRED in place); until the script
   lands on main, the 10 GB / two-parallel-cargo-lanes ceiling is
   enforced by this log's slot line.

## Dispatch record

| Unit | Spec | Model (draw) | Lane | Status |
|---|---|---|---|---|
| U1 façade | docs/LIB-U1-SPEC.md | OPUS (block LIB-1 draw byte 13 = opus,fable; difficulty S logged pre-draw) | lib-u1 | dispatched 2026-08-06 |
| U2 PATHS | docs/LIB-U2-SPEC.md | fable (block LIB-1 remainder; difficulty L logged pre-draw) | lib-u2 | dispatched 2026-08-06 |

## Orchestrator decisions (LB-numbered)

- **LB1 (2026-08-06)**: U2 is staged as two PRs — PR-1 the algebra
  + lowering + differential tests (touches `crates/profile` only),
  PR-2 the demo-corpus profile rework (touches `demos/tour`),
  sequenced after U1's façade rework merges. Rationale: the two
  authorized units otherwise collide in `demos/tour`; PR-1 is
  also the natural review boundary (algebra semantics vs
  mechanical rework).
- **LB2 (2026-08-06)**: the U1 façade re-exports the full
  authoring surface as modules + a curated prelude; the
  SurfaceKind-leak closure is specified as a CLOSURE PROPERTY
  (every type reachable through re-exported public error enums is
  importable from the façade) with a compile-level test, not as a
  one-off re-export.

## Resting state (2026-08-06)

Slots: 1 = lib-u1 (Opus), 2 = lib-u2 (fable). Monitors:
disk-watchdog + hourly-checkin armed in this session;
away-channel NOT armed (Evan present in-chat; watchlist empty).
The v2 representation design conversation is QUEUED behind U2
PR-2's merge (ruling 3 above). Next units in judgment scope after
U1/U2: U3 (SectionSegments retirement) and U5 (read-back) are the
natural nexts; U7 unblocked (M6-5 merged #219/#220).
