# ONEDOOR — the plan

No orchestrator yet. Live state is `work/onedoor/log.md`'s tail and the
item files beside this plan.

Branch prefix **`onedoor/`**; away-channel tag `(ONEDOOR orchestrator)`.

## Charter

Each boolean door these rows name has one home, and every caller reads
it there.

## Posture

Sort each row before dispatch (ONEHOME's plan, Posture): design signal
(`design: true` — the copies exist because nothing says where the logic
belongs; designers first) or mechanical (delete one copy and repoint its
users). `G9`, `S173` and `description-staleness-ladder-three-spellings`
carry `design: true` from the cut; the rest are unsorted.

## From CORES (merged 2026-11-02)

CORES opened on 2026-10-26 by TOPO's priority-seam cut, with PIPE's two
live P1 rows (`docs/doc-ledger/pipe-leaves-the-tracker.md`). It merged
here when the budget came to count only the rows a sitting dispatches:
23 and 8 points, 31 against a budget of 31
(`docs/doc-ledger/cores-merges-into-onedoor.md`). Both rows moved by
`git mv`, ids and bodies unchanged.

| item | pri | cost | state | where the work lands |
| --- | --- | --- | --- | --- |
| `topo-shared-cores-hosted-in-one-half` | P1 | H, design | open; released by INTENT stage 4 F | `splitting/{finish,classify}.rs` → a shared home; `SplitFinishError`/`SplitJoinError`; `docs/DESIGN.md`'s Layering `topo` row |
| `S5` | P1 | H, design | parked on the row above | `crates/topo/src/{splitting,boolean}/**` |

Order: `topo-shared-cores-hosted-in-one-half`, then `S5`. The
orchestrator re-reads both rows against the tree, has two designers
weigh the layering question (crate-root shared cores, two peer lanes,
DESIGN.md's `topo` row), and opens it with an `[ev]` PR if it is a
fork. The structural units draw a dual review: each moves shared code
that two engines and several programs depend on.

## Layering units (`[topo.homes]`, ratified on PR 6426, 2026-11-03)

The rule is in `crates/topo/README.md` `[topo.homes]`. Its guard's allowlist is the move list. Order:

1. `engine-directories-are-leaves-guard`: the guard and the shrink-only allowlist; the per-module placement prose is deleted.
2. `crossing-doors-leave-topo-for-geom-brep`: the Body-free doors, plus the `split_conic_*` K rename.
3. `topo-shared-cores-hosted-in-one-half`:
   - `carve`/`single_solid` become typed body surgery at the root;
   - `SplitJoinError` becomes `JoinError`, wrapped by both engines;
   - `SplitFinishError::Corrupt` leaves with `carve`.
4. `S173`: `face_carrier` and both `face_plane`s move to `face_normal`. `chart-window-walk-written-twice`: the chart-window walk leaves `chord_join` for a chart-window module.
5. The `boolean/` and `splitting/` library moves: `boxes`, records, `contain`, `solid_contain`, `containment` and the arc readers, face reach, the sector levers, and the ops readings. One module per PR as the allowlist shrinks. They are too many rows for this track's budget, so a successor program takes them once the guard lands, chartered by the allowlist.

`S5` is re-read after unit 5. The dual tier applies wherever a unit moves shared code that several programs read.
