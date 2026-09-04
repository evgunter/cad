---
id: tool-kind-all-and-ordinal-have-no-production-reader
kind: issue
title: ToolKind::ALL and ToolKind::ordinal have no production reader — a pub pair kept for one test, whose doc names a chrome consumer that does not exist
status: open
opened: 2026-09-04
refs: [opoutcome-superseded-has-no-production-reader, viewer-session-god-module-split]
---


Filed by unit 1d's fix pass, at the moment the last production reader
went (`docs/prompts/reviewer-style-lane.md` Q6: a narrowing owes a
named unit, and "deferred" is not a schedule). Unit 1d made the
condition true; it is not unit 1d's to answer, because deciding what
the pair is worth is a question about the test suites that hold its
only readers, and those are CHROME's glob.

## The reader counts, verified on the 1d branch

`git grep 'ToolKind::ALL'` and `git grep '\.ordinal()'` across the
whole tree:

- **`ToolKind::ALL`** (`crates/viewer/src/tools.rs:74`, `pub`).
  Production readers: **zero**. Before unit 1d, exactly one:
  `Tools::open_kind`'s scan over the seven `Option<…Tool>` fields.
  1d replaced that with `self.open.as_ref().map(OpenTool::kind)`, so
  nothing under `crates/viewer/src/` reads the array — the remaining
  hits there are prose (`tools.rs:22`, `tools.rs:65`,
  `seats.rs:110`, `crates/viewer/README.md`). Test readers:
  `crates/viewer/tests/combine_ops.rs:1258,1278,1281,1282,1290,1302,1314,1315,1746,1764,2132`
  and `crates/viewer/tests/blend_authoring.rs:773,778,782`.
- **`ToolKind::ordinal`** (`crates/viewer/src/tools.rs:86`, `pub`).
  Production readers: **zero, and zero before 1d as well**. Its only
  two callers are `crates/viewer/tests/combine_ops.rs:1291` and
  `:1316`.

So the pair is now `pub` API whose whole function is to let one test
sweep a list only that test reads, with a second test
(`every_tool_kind_is_listed_in_all`, `combine_ops.rs:1307`) guarding
the sweep. The chrome that `tools.rs`'s doc comment promised does not
exist and did not exist before 1d: `crates/viewer/src/pane/create.rs`
names each `ToolKind::` variant literally at 24 sites and iterates
nothing. 1d corrected both doc sentences (`tools.rs:65`,
`crates/viewer/README.md`) to say the suites are the only readers; it
did not remove the items, which is this item.

## The class

**A `pub` item whose only readers are tests, while its doc names a
production consumer.** The promise is what makes the next reader trust
the item, so "no production reader, undocumented" is the one answer
that cannot be right — the same argument, on the same crate, as
`work/view/opoutcome-superseded-has-no-production-reader.md`, where
`OpOutcome::superseded` is computed, handed to the GUI and read only
by two test files while the type still promises it. The difference is
the direction of the fix: `superseded` has a consumer that ought to
exist (the status line), while `ALL`/`ordinal` may simply belong to
the suite that uses them.

## What this program cannot touch

The test-side residue is CHROME's glob (`crates/viewer/tests/`), so
1d could not fix it and neither can VIEW:

- `combine_ops.rs:1248-1256`, `open_flags`'s docstring, still says
  "`open_kind` is a PRIORITY SCAN: it answers with the first tool it
  finds open, so it cannot see a second one left behind it". After 1d
  that is no longer the mechanism — `open_kind` asks one value which
  kind it is, and a second tool left behind has no spelling. The
  reason the row reads through the per-tool accessors is still good
  (they are the door the chrome uses); the mechanism sentence is
  stale.
- Whether `ALL` and `ordinal` should move behind the suite that reads
  them — a test-local list, or `#[cfg(test)]`, or kept `pub` with a
  doc that says so — is a decision about those files.

## The sweep, which has NOT been done

This is one instance and the class is not swept. `crates/viewer/src/`
has at least one sibling of the same construction, unexamined:

- **`Seat::ALL` / `Seat::ordinal`** (`crates/viewer/src/seats.rs:121`
  and `:134`, both `pub`). Same construction, same two-test pattern:
  the only readers are `combine_ops.rs:1407,1408` (`ALL`) and
  `combine_ops.rs:1409` (`ordinal`) — zero production readers either
  side of 1d. `seats.rs:110` carries the same prose ("the shape
  `crate::tools::ToolKind::ALL` uses, for its reason"), so whatever
  answer `ToolKind`'s pair gets, this pair inherits it.
- **`forms::BOOLEAN_OPS` / `forms::MATE_PRIMITIVES`**
  (`crates/viewer/src/forms.rs:41` and `:485`) are the neighbouring
  axis and **not** an instance of this class: both are `pub(crate)`,
  not `pub`, and both DO have production readers
  (`pane/create.rs:876` and `:129,144`). What they share is being
  self-declared hand-maintained mirrors of a kernel enum
  (`crates/viewer/README.md:274`) that no compiler forces — worth the
  same sweep's attention for the mirror question, not for the
  reader-count one.

The pattern to sweep with is `pub` items whose grep hits outside
`src/` are all under `tests/`; its blind spot is items reached
through a re-export or a trait method, which a name grep will not
resolve to their definition.

## Home

VIEW's for the `src/` side (`tools.rs`, `seats.rs`); the
`crates/viewer/tests/` half needs CHROME.
