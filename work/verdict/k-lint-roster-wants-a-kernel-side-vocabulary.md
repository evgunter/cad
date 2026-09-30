---
id: k-lint-roster-wants-a-kernel-side-vocabulary
kind: issue
title: k-lint's roster pin parses kernel source because the fence forbade the two cheaper shapes
status: open
opened: 2026-09-07
priority: P4
cost: D
---


## What

`tools/k-lint/tests/predicate_roster.rs` pins `EPS_COUPLED_PREDICATES`
against the kernel by **parsing Rust source text**: `include_str!` over
`crates/geom-brep/src/props/quad.rs`, two blanked views from
`test_utils::source`, a bracket walk to find each
`classify_len::<T>(` call, and a comma split to read its first two
arguments. That is roughly 200 lines of machinery policing a
one-element array.

Two cheaper shapes exist and neither was available to the unit that
wrote it:

- **a `#[test]` inside `geom-brep`** asserting the crate's own minted
  predicate names against a list, which the roster then reads. No lexer,
  no cross-root text read, and the assertion sits where the names are.
- **a `const` slice exported from `geom_core::k_stats`** — the
  vocabulary as data rather than as `&'static str` arguments — which
  `k-lint` already dev-depends on and could simply import, the way
  `tests/outcome_vocabulary.rs` imports `SampleOutcome`. The roster
  then becomes a subset assertion over an imported set, with nothing to
  parse and the ADDED direction checkable for the first time.

Both are edits under `crates/`. METER's fence covers `tools/k-lint`,
`tools/tess-meter` and `docs/K-REPORT.md`; `crates/geom-brep/src/props/*`
and `crates/geom-core/src/k_stats.rs` are PROPS' seam. **The fence, not
the problem, chose the instrument** — which is worth writing down
because a future reader finds a hand-rolled parser and no statement
that it was second choice.

## Finding

The second shape is strictly the better one and it composes with
`k-lint-eps-coupled-criterion-unwritten`: a kernel-side declaration of
the vocabulary is the natural place to also declare *which* of those
names are metered against an ε-derived target, which is the criterion
that item says nothing in the tree states. One PROPS unit could land
both, and the roster pin would shrink to an import and two set
comparisons.

Until then the parser stays, because a roster with no pin at all was
the defect `k-lint-predicate-roster-unpinned` filed, and a textual pin
that reds loudly on every spelling change is a worse instrument than an
import and a better one than nothing.

**Confidence:** sure that both shapes are outside METER's fence and
that neither exists today; likely that the `const` slice is the shape a
PROPS unit would actually pick, unsure whether `k_stats`' `&'static str`
signature is load-bearing somewhere that makes a slice awkward.

## Was

Raised by the style review of `meter/klint-roster-pin` (NOTE-4 and
NOTE-6, `sure`), which observed that a reviewer would not have written
a Rust parser to police a one-element array and named both alternatives.
Recorded rather than acted on by that PR's fix pass, per the fence.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.

## Moved to PROPS (2026-09-16)

Moved from `work/instr/` by `git mv`, id unchanged; the directory is the
claim. **Both shapes this row proposes are edits under `crates/`** —
a `#[test]` inside `geom-brep` over its own minted names, or the better
one, a `const` slice exported from `geom_core::k_stats` — and both sit
on PROPS' seam (`work/instr/program.md`'s `keep_out` names
`crates/geom-brep/src/props/*` and `crates/geom-core/src/k_stats.rs` as
PROPS'). INSTR held it only because METER's fence chose the instrument,
which is the finding itself.

INSTR owns the consequence, not the fix:
`work/instr/k-lint-eps-coupled-criterion-unwritten` is now `parked` on
this row, and closes when `tools/k-lint/tests/predicate_roster.rs`
trades its hand-rolled parser for an import and gains its ADDED
direction. **Nothing here waits on INSTR** — this row is dispatchable
on PROPS' own schedule, and the roughly 200 lines of source-parsing
machinery in `predicate_roster.rs` is what it buys back.

The move is authorised by Ev on 2026-09-16 (*"if the item would be
better sited in props itself then you can just move it there"*), and
the move is the notification: the row reaches PROPS' board through
`work/STATUS.md` rather than through a message.

## Evidence from INSTR unit 12 (2026-09-16): the tooling side now carries TWO hand-maintained lists

`k-lint-last-round-is-eps-coupled-but-unrostered` ruled that
`props_quad_last_round` stays off rule (4), and recording that ruling
mechanically meant adding a second roster beside the first:
`k_lint::EPS_COUPLED_UNRULED`, names ε-coupled by the criterion's
spelling and deliberately not under rule (4)'s floor, each with its
reason. It moved out of `tools/k-lint/tests/predicate_roster.rs`'s
`NOT_ROSTERED` and into `tools/k-lint/src/lib.rs` because the CLI reads
it too.

**That list exists only because the tooling side cannot evaluate the
criterion.** The two lists are a partition of one kernel property — the
names metered against an ε-derived target — into "under rule (4)" and
"ruled out of it", and both halves have to be maintained by hand in a
workspace-excluded crate because nothing on the kernel side declares
the property. The `const` slice this row proposes, *"declared beside
which of its names are ε-metered"*, does not merely shrink the parser:
it makes the partition's DOMAIN checkable, so a name that acquires the
property lands in one list or reds instead of being invisible to both.

`tools/k-lint/tests/predicate_roster.rs`'s
`every_target_len_mint_is_rostered_or_excused` is the current stand-in
and it is bounded by the same spelling the parser is: `target_len`, in
`MINT_SOURCES`, and nothing wider.

**One correction to this row's Confidence line while it is being read.**
*"unsure whether `k_stats`' `&'static str` signature is load-bearing
somewhere that makes a slice awkward"* — a slice and the signature do
not conflict: `k-lint`'s own `EPS_COUPLED_PREDICATES` is a
`[&str; 1]` matched against the recorded name by string compare, and
the ruling above added a `[(&str, &str); 1]` beside it the same way. A
kernel-side `const` slice of `&'static str` would be imported and
compared identically, with `decide`'s signature untouched.
