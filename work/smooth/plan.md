# SMOOTH — the plan

No orchestrator yet. Live state is `work/smooth/log.md`'s tail and the
item files beside this plan.

Branch prefix **`smooth/`**; away-channel tag `(SMOOTH orchestrator)`.

## The staging, and why

The build changes a vocabulary every corpus file speaks. It is staged
**additive, then subtractive**:

1. **Add** the inherited flavour with `line_to`'s checked row, and the
   `End` mode with its fused rows, beside the old verbs. Each old verb
   is re-pointed at the new row's kernel in the same unit, so there is
   one kernel per act from the first unit on and never two
   implementations of one arc or one straight leg.
2. **Retire**, one vocabulary at a time and each atomically: first
   `tangent_arc_to`, then `continue_to` with the bare leg-end rows.
   Each retiring unit deletes the old row and migrates every caller in
   the same PR, so main never holds a half-renamed call site set.

Why not one atomic unit: the whole of it is about 580 occurrences of the
two verbs in some 120 files, plus the bare leg-end sites. Put in one diff,
the new semantics (a type-level flavour, a checked row, a new arc mode,
two new fused acts) would be reviewed inside a mechanical sweep, and a
reviewer could not tell a respelling from a behaviour change.

Why not migrate before deleting: a migration with the old rows still
present has nothing to enforce it. Other lanes keep writing the old
spellings, and the record-level, document-level and Python sites are
found only by a run. With deletion in the same PR, every site that is
left breaks loudly: at compile for typed chains and `Step::` variants,
and at replay or load for the rest.

Why the two retirements are separate units: `tangent_arc_to` is found
almost wholly by the compiler (its `Step` variant and its typed method
both go). The bare leg-end `line`/`fillet` rows have no variant to delete.
Their record, document and Python sites refuse only at replay, so that
unit needs several CI rounds. Splitting them keeps the compile-found
half from waiting on the runtime-found half.

**The schema step** comes in two parts. The additive part,
`ProgramArcData::End`, lands with the `End` unit: documents that do not
name it still load. The breaking part, retiring `ProgramStep::TangentArcTo`
and then `ProgramStep::ContinueTo`, lands with each retiring unit. A break
here costs one re-bless and nothing else. The format has no version
(`persist/mod.rs`); the in-tree documents naming the retired verbs are
`tests/golden/golden.cad` (`M4_PR6_BLESS_GOLDEN=1`) and
`tests/corpus/wire_rv_bytes.json` (`PNCAD_BLESS=1`), plus the literal pins
in `tests/switch_program_vocabulary.rs`. The historical
`tests/bool13_goldens/` bytes are not re-blessed: they are asserted to
refuse, naming `unit`. Each schema-bearing unit announces on the away
channel before it lands.

## The slate, in order

| # | unit | cost | tier | waits on |
|---|---|---|---|---|
| 1 | `a-tangent-tip-is-its-own-state-and-line-to-is-checked-from-it` | H | single (FULL) | — |
| 2 | `arc-to-gains-the-end-mode` | M | single (FULL) | — |
| 3 | `the-fused-verbs-take-end-and-radius-at-every-directed-tip` | M | single (FULL) | 2 |
| 4 | `tangent-arc-to-retires-into-arc-to-end` | M | single (STYLE) | 2 |
| 5 | `continue-to-and-the-bare-leg-end-rows-retire` | H | single (FULL) | 1, 3, 4 |

- 1 and 2 run at once: different rows of one table, coupled only in
  that whichever lands second makes its rows generic over the other's
  addition (2's `End` row over 1's flavour). Both touch
  `transition_table!`, the Python door's path module and the census,
  so textual conflicts are routine.
- 3 and 4 run at once after 2.
- 5 is last. It needs 1's checked `line_to` and 3's `Radius` at a
  Directed tip as the respellings of what it deletes, and it follows 4
  because both sweep the same test files and the same recourse
  sentence.

The parent row, `tangent-arc-to-needs-an-explicit-tangent-where-line-takes-the-directed-point`,
is parked on unit 5 and closes with it.
`no-fused-verb-takes-a-departure-tangent-arc-toward-a-point` closes
with unit 3.

## Exit criteria

- No `tangent_arc_to`, `continue_to`, `TangentArcTo` or `ContinueTo`
  is left in `crates`, `demos`, `tools` or the Python stub, outside
  `RETIRED_VERB_TAGS` and quoted history.
- `Verb::states` admits no heading-taking verb at `DirectedPoint`.
- §3's table and `transition_table!` agree row for row. The
  `JunctionTurnWithinTolerance` recourse matches §4 item 1.
