# PIPE — the topology pipeline (plan)

Opened in the tracker cut of 2026-09-11 (`docs/WORK-TRACKS-2026-09.md`
addendum 3).

Branch prefix: **`pipe/`**. Away-channel tag `(PIPE orchestrator)`.
A/B ordinal band **PIPE = 4500–4599**.

## Charter

`crates/topo` has four live programs on it — TOPO (Euler surgery and
validation), S-BOOL and CURVED (the boolean and splitting engines),
SHELL and TRIM on their own files — and the rows here are the ones that
belong to none of them because they are about **the relationship
between** those parts:

- **Structure.** `topo-shared-cores-hosted-in-one-half` (the cores both
  engines use live inside one of them) and `S5` (the two engines
  duplicate a pipeline). Both are architecture calls with a
  `docs/DESIGN.md` layering row attached, and both move files two
  programs have live units in.
- **The census's answers.** `S350` (what the `ControlNet` arm answers for
  a poisoned net), `D291` (an arm unreachable by construction), and
  `described-net-two-state-reads-…` (thirteen consumers that hand a
  poisoned net to the described arm, in six programs' files).
- **What a refusal may say.** `witness-budget-exhausted-two-caps-one-name`,
  `S14`, `S70`, and `lane-keeping-at-rest-doors-skip-the-m7-8-class`.

## Territory — none, and why

This program claims **no paths**, and that is a statement about the rows
rather than a shortcut: each of them is *on* another program's ground by
definition, because the finding is that two owned halves disagree. Every
unit announces to the owner of each file it touches, and the two
structural rows are not dispatched at all without S-BOOL's and CURVED's
announcement, because they move files those programs have live units in.

`described-net-two-state-reads-…` is a **routing list, not a diff**: its
thirteen sites are in six programs' files, each arm needs its owner's
judgement about whether that consumer is wrong, and the row's value is
the sweep, not a cross-fence patch.

## The slate

| item | pri | cost | state | where the work lands |
| --- | --- | --- | --- | --- |
| `topo-shared-cores-hosted-in-one-half` | P1 | H, design | parked on INTENT stage 4's boolean rows | `splitting/{finish,classify}.rs` → a shared home; `SplitFinishError`/`SplitJoinError`; `docs/DESIGN.md`'s Layering `topo` row |
| `S5` | P1 | H, design | parked behind the row above | `crates/topo/src/{splitting,boolean}/**` |

Every other row is closed and stays here until the program closes (the
log says what each landed as).

## Order

`topo-shared-cores-hosted-in-one-half`, then `S5`. Both wait for
INTENT's `booleans-glue-on-zero` and `declared-pairs-retire`, which
rewrite the boolean half these rows would move (see each row's
`## Parked` section).

When those land, the next orchestrator:
1. re-reads both rows against the tree;
2. has two designers weigh the layering question: crate-root shared
   cores, two peer lanes, DESIGN.md's `topo` row;
3. opens it with an `[ev]` PR if it is a fork. A layering decision
   ratified after the diff is a layering decision the diff made.

## The D10 hold

The hold is `work/intent/d10-one-way-to-say-intent-is-unbuilt.md`; TOPO's
log of 2026-10-03 lists its covered ground. It covers document intent:
- parameters and `Expr`;
- placement;
- declared pairs and contact;
- the undeclared refusals and axis declarations;
- `ParamSource`;
- `Measure`/`Assertion`.

None of this program's live rows stands on that ground. Census reach
boxes, the witness budget and the net-state reads are kernel geometry
and refusal vocabulary. Stage 5's "the at-rest census is a check"
changes what is done with a census answer, not how a face's box is
computed.

The overlap is in files, not ground. `intent/s4-e-glue-on-zero` touches:
- `census.rs` (tests only);
- `chart_region.rs` (the declared-chart path, not the witness budget);
- `boolean/boxes.rs`.

Each local lane runs `work.py territory` and merges main before it
lands.

## Review posture

Tiers are the orchestrator's call per unit
(`memories/orchestration-model.md`), and the log names each at
dispatch. A change to what a census arm answers draws at least a FULL
single review; the structural units draw a dual review. The standing
trap is ordering rule 5: a refusal split into two named faces is a new
vocabulary, and the lane that splits it is the one that will spell the
second face wrong.
