# PIPE leaves the tracker — 2026-10-26

PIPE (the topology pipeline, its shared cores and what a refusal there
may say) opened in the 2026-09-11 tracker cut for the `crates/topo`
rows that were about the relationship between its engines rather than
any one of them. It claimed no paths. Its wave 1 landed on 2026-10-10:
the described-net routing pass (PR 4483), `S350`'s census `ControlNet`
arm (PR 4482), and `D291` with the witness-cap split (PR 4486). `S14`
(the no-panic ruling) closed on Ev's answer on PR 4006, `S70` (the
graft footnote) with the staged graft of PR 4022, and `S79` once its
three demo API gaps had closed or become rows of their own.
After that only the two structural rows were left, and the track was
handed back.

Ev folded PIPE into TOPO in chat on 2026-10-26, with no exit walk (the
plan set no exit criteria). Its two live P1 rows,
`topo-shared-cores-hosted-in-one-half` and `S5` (parked on it), moved
with their ids kept. TOPO's split of the same day then cut them, with
nothing else, into CORES (`work/cores/`), whose charter is theirs: the
boolean and splitting engines as one pipeline.

Three closed rows moved to `work/topo/` unchanged, because live rows
cite them in their text for context they do not restate:
- `S14`, whose body quotes Ev's ruling in full and is cited by path
  from TOPOTAIL, COMB, CLEAVE and VACUITY rows;
- `S350`, named as the root class of
  `work/issues/aabb-poison-is-per-lane-and-a-one-lane-nan-box-prunes.md`
  and as the origin of NURBS, SHELF and PROBE rows;
- `described-net-two-state-reads-hand-a-poisoned-net-the-described-arm`,
  the surface-side sweep that WALKS's spelling row and NURBS's
  curve-side net-state row each define themselves against.

The other five closed rows (`D291`, `S70`, `S79`,
`lane-keeping-at-rest-doors-skip-the-m7-8-class`,
`witness-budget-exhausted-two-caps-one-name`) were deleted. Where they
were cited only as `refs:` entries, those entries were pruned (Ev, in
chat: keep a closed row only where it gives the citing row context).
The prose citations of `work/pipe/` paths now point at `work/topo/`.
PIPE's band 4500–4599 passes to CORES with its two rows.

Recover the program and its closed rows from the parent of the commit
that deletes `work/pipe/`:

    git log --diff-filter=D --format=%H -1 -- work/pipe/program.md
    git show <that sha>^:work/pipe/<FILE>
