# CORES — the boolean and splitting engines as one pipeline (plan)

Opened 2026-10-26 by TOPO's priority-seam cut, the day PIPE folded into
TOPO (Ev, in chat). PIPE's history is recoverable from
`docs/doc-ledger/pipe-leaves-the-tracker.md`; TOPO's is its own `log.md`.

Branch prefix: **`cores/`**. Away-channel tag `(CORES orchestrator)`.
A/B ordinal band **4500–4599**, carried on from PIPE.

## Charter

`crates/topo` has two engines, the boolean (`boolean/`) and the split
(`splitting/`). They run one pipeline, and the cores both use live
inside one of them. The rows here are about that relationship, not
about either engine's answers:

- `topo-shared-cores-hosted-in-one-half` — `finish` and
  `plane_crossing_lane` are shared but hosted in one half.
- `S5` — the two engines are one pipeline built twice. It is parked
  on the row above.

Both are architecture calls with a `docs/DESIGN.md` Layering row
attached, and both move files that several boolean and split programs
have live units in.

## Territory — none, and why

This program claims no paths. Each row is on other programs' ground by
definition, because the finding is that two owned halves disagree.
Every unit runs `work.py territory` on its branch and announces the
seam to the owner of each file it touches.

## The slate

| item | pri | cost | state | where the work lands |
| --- | --- | --- | --- | --- |
| `topo-shared-cores-hosted-in-one-half` | P1 | H, design | open; released by INTENT stage 4 F | `splitting/{finish,classify}.rs` → a shared home; `SplitFinishError`/`SplitJoinError`; `docs/DESIGN.md`'s Layering `topo` row |
| `S5` | P1 | H, design | parked on the row above | `crates/topo/src/{splitting,boolean}/**` |

## Order

`topo-shared-cores-hosted-in-one-half`, then `S5`. The INTENT stage 4
rows both waited on have landed (the first row's `## Released by stage
4 F`), so the next orchestrator:
1. re-reads both rows against the tree;
2. has two designers weigh the layering question: crate-root shared
   cores, two peer lanes, DESIGN.md's `topo` row;
3. opens it with an `[ev]` PR if it is a fork.

## Review posture

Tiers are the orchestrator's call per unit
(`memories/orchestration-model.md`), and the log names each at
dispatch. The structural units draw a dual review: each moves shared
code that two engines and several programs depend on.

## Exit shape

Both rows merged, or re-homed with a reason. No exit criteria are set,
so no walk is owed.
