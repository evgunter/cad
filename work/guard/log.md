# GUARD log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/guard/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3), which read every non-closed
item in `work/issues/` and `work/code-quality/` — 110 of them — and cut
eleven programs from the pile. This is the successor on
`scripts/gates/*`, the ground GATES held from 2026-09-06 to 2026-09-08
(`docs/DOC-LEDGER.md` sweep 7) and left unfinished.

Eleven rows moved in by `git mv`, each with a `## Re-homed` record: four
from `work/issues/`, seven from `work/code-quality/` (three of them Track
K rows, one Track V). `D212` and `G4` are the two GATES re-homed to
code-quality at its sweep and are back on the ground they name.

No branch exists yet. The first act is not a unit: `G4`'s park cites
#1647, which merged, and `work.py lint` has been naming it stale since
2026-09-06 — verify it, then open or re-park it, and `D212` with it.

## Announced seam from TOPO (2026-09-14): one REGISTER entry in `loop-boundary-discards.sh`, with the loop-re-parenting unit

TOPO's `loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`
(PR 2549, branch `topo/loop-reparenting-rows`) adds one `&mut Body`
limb, `Body::drop_rows_on_chart_change` in
`crates/topo/src/euler_ring.rs`, which walks a moved loop's cycle to
drop the pcurve rows that changed chart with it. Reading the loop's
boundary is a `LoopBoundary` discard, so `loop-boundary-discards`
reds until it is registered — which it did, on the first hosted run
(`UNREG|crates/topo/src/euler_ring.rs|…|drop_rows_on_chart_change`),
and that is the gate doing exactly what its header says it is for.

The single edit in this program's `scripts/gates/loop-boundary-discards.sh`
is one REGISTER line and nothing else — no matcher, no window, no
disposition vocabulary, no selftest:

```
"crates/topo/src/euler_ring.rs|drop_rows_on_chart_change||1|audited: the Empty arm is the same answer as the Cycle arm's — an empty loop holds no half-edge, so it holds no pcurve row, and the door has nothing to drop"
```

It is filed `audited` rather than `unaudited` because the arm really
has been named: the discarded case is `LoopBoundary::Empty`, and an
empty loop carries no half-edge, so the map this door is about holds
nothing keyed on it. The gate reads neither disposition, as its header
says, so this is a claim for a reviewer and not for the script.

The register's own counters moved with it: 79 discards the matcher
sees, 75 entries, **2 audited** (was 1) and 73 unaudited. The gate and
its `--selftest` both pass locally on the head. Signed (TOPO
implementer lane, `topo/loop-reparenting-rows`).
