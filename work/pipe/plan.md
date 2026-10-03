# PIPE — the topology pipeline (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

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

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `S350` | **M** | One arm, but wrong-answer semantics: choose `None` vs `CensusUndecidable`, new test needed | `crates/topo/src/census.rs` (~:1694-1722 `ControlNet` arm), `crates/topo/tests/*` (new poison case; cf. `crates/geom/tests/net_placeholder_width.rs`) |
| `D291` | **M** | Comment rewrite plus deleting two by-construction-unreachable arms; two files, needs verification | `crates/topo/src/census.rs` (~:1951 conic arm, `None => conic_extent`), `crates/topo/src/boolean/boxes.rs` (`edge_axial_span` `params: None`) |
| `witness-budget-exhausted-two-caps-one-name` | **M** | Split one variant into two named faces plus levers; three consumers, unclaimed owner. | `crates/topo/src/chart_region.rs` (:304-318, :779, :1981, :2003, `WITNESS_BUDGET`), `crates/topo/src/census.rs` (:1695, :2814), `crates/topo/tests/census_g2_carrier.rs:399` |
| `described-net-two-state-reads-hand-a-poisoned-net-the-described-arm` | **H** | Thirteen sites in six programs' files; each arm needs owner's judgement | `crates/topo/src/{pcurves.rs,props.rs,replace_face.rs,transform.rs,census.rs}`, `crates/mesh/src/{chords.rs,trimmed.rs}`, `crates/step-import/src/adopt.rs`, door at `crates/geom/src/surfaces/nurbs.rs` (`net_state` already landed) |
| `topo-shared-cores-hosted-in-one-half` | **H** | Two extractions, public error-path and K-name questions, plus a DESIGN.md architecture call | `crates/topo/src/splitting/{finish,classify}.rs`, `crates/topo/src/boolean/{finish,ops,rest,reduce}.rs`, `crates/topo/src/chord_join.rs` + new crate-root modules, `SplitFinishError`/`SplitJoinError` re-exports in `crates/pncad/`, `docs/DESIGN.md` layering row |
| `S5` | **H** | Unifies two engines: ~267 tests over 36 files, error-type API, design conversation | `crates/topo/src/splitting/**`, `crates/topo/src/boolean/**` (`sectors.rs`, `join.rs`, `reduce.rs`, `solid_contain.rs`), `docs/DESIGN.md` layering row |
| `lane-keeping-at-rest-doors-skip-the-m7-8-class` | **H** | Renaming public tier-3′ doors evicts `Body<Dual64>`; H-R3/H5 ruling first | `crates/topo/src/validate.rs` (door names/bounds), `crates/pncad-py/src/py/value.rs`, `demos/tour/src/{main,letterforms,probe}.rs`, `crates/pncad/src/prelude.rs` |
| `S79` | **H** | Three new public API surfaces; explicitly no row owed, scheduled only as GitHub issues. | `crates/topo/src/` (public census/genus query), `crates/topo/src/boolean/` (declaration producer), `crates/pncad/src/authoring.rs`, `demos/tour/src/*`, `demos/wild/src/main.rs`, `crates/topo/tests/common/mod.rs` |
| `S70` | **H** | Is a ruling; blocks on Ev deciding whether `graft_disjoint_all_keyed` gets real atomicity | — |
| `S14` | **H** | Amends D9; gates ~45 fillet sites and S19/S70; Ev's affordability call. | — (ruling; if ruled "restructure", work would land in `crates/topo/src/instance.rs`) |

## Order

The three local rows first — `S350`, `D291`,
`witness-budget-exhausted-two-caps-one-name` — because each is one or two
files and each sharpens what the census and the refusals actually say
before anything structural moves them.

Then `described-net-…` as a routing pass: read the thirteen, file each on
its owner, keep the sweep here.

`topo-shared-cores-hosted-in-one-half` and `S5` are the program's real
weight and go together in that order — extract the shared cores, then ask
whether the two pipelines are one. Both open with an `[ev]` PR carrying
the `docs/DESIGN.md` layering row, because a layering decision ratified
after the diff is a layering decision the diff made.

**Not takeable.** `S14` and `S70` are rulings; `lane-keeping-at-rest-…`
is gated behind the H-R3/H5 ruling; `S79` was explicitly scheduled as
GitHub issues rather than rows and that disposition stands until Ev
changes it.

## Review posture

Full v6 dual with Fable specs on the structural units and on any change
to a census arm's answer. Ordering rule 5 is the standing trap: a
refusal split into two named faces is a new vocabulary, and the lane
that splits it is the one that will spell the second face wrong.
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
