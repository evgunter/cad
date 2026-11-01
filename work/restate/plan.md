# RESTATE — the describing contract of the Euler doors (plan)

Opened 2026-10-26 by TOPO's priority-seam cut. TOPO's history is its
own `log.md`; this slate was TOPO plan's "describing-door family".

Branch prefix: **`restate/`**. Away-channel tag `(RESTATE
orchestrator)`. A/B ordinal band **16600–16699**.

## Charter

A door that moves an edge, loop or run from one face's chart to
another's, or mints an edge under a caller's description, has to leave
two things true: the edge's description names the surfaces of the faces
it bounds, and every pcurve row the destination face stores is
certified against the chart that face now wears. The rows here are the
places one of those two is not yet true, or is true only by a caller's
ordering.

## The slate

All P3 unless marked. `python3 scripts/work.py status --program
restate` is the live view.

- **The kills across keys and the restater** (ruled in outline on PR
  3970, Ev 2026-10-06; details wait on the intent refactor):
  - `kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`
    (design; the PR 4562 fork F1/F2 is open on it);
  - `restatement-derives-each-moved-edges-kind`.
- **The describing door** (`set_face_surfaces_describing`, `attach.rs`):
  - `set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart`;
  - `the-describing-door-refuses-a-wall-re-charted-reversed-in-u`;
  - `a-listed-spec-cannot-name-a-fresh-chart-a-neighbour-keeps-the-old-key-of`;
  - `set-edge-curve-scans-the-edge-arena-per-call` (E, its cost).
- **The moving doors that half-mint** (`kef`, `kfmrh`, `ring_move`,
  the `_minting` bands):
  - `a-moving-door-carries-minted-rows-onto-an-unminted-face-and-half-mints-it`
    (design);
  - `a-moving-door-leaves-a-complete-spline-destination-half-minted`
    (design);
  - `kef-minting-clears-a-complete-face-whose-merged-loop-it-cannot-chart`.
- **The minting doors and the re-basing gate:**
  - `minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key`;
  - `the-re-basing-gate-reads-a-derived-image-where-the-mint-read-the-stored-one`
    (unpriced; price it at pickup).

## Order

One designer-first thread: the rows share one question, how an edge a
door moves gets described and which rows travel with it. Re-read
`kef-and-kfmrh-across-keys-…`'s ruled outline and the PR 4562 fork
first; the half-minting rows and the restater follow from its answer.
The describing-door rows and the E cost row can be taken beside it.
Where a row's detail is one Ev said the intent refactor will change,
check INTENT's state before building.

## Review posture

Per unit, at spec time, on the tiers in
`memories/orchestration-model.md`, as TOPO's: the orchestrator's read
for mechanical changes, a single review by default, a dual for an
architectural decision that would be hard to change later (the
restater is one). The tier and its one-line reason sit on the unit's
item or in the log at dispatch.

## Exit shape

Every row merged or re-homed with a reason. No exit criteria are set,
so no walk is owed.
