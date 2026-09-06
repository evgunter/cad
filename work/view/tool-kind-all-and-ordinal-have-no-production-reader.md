---
id: tool-kind-all-and-ordinal-have-no-production-reader
kind: issue
title: ToolKind::ALL and ToolKind::ordinal have no production reader — a pub pair kept for one test, whose doc names a chrome consumer that does not exist
status: closed
opened: 2026-09-04
refs: [opoutcome-superseded-has-no-production-reader, viewer-session-god-module-split]
closed: 2026-09-06
pr: 2046
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

## Closed (VIEW, unit `const-all`, 2026-09-06)

Taken with `viewer-const-all-tables-have-no-exhaustiveness-guard`, as
both items said it had to be: whether these tables should EXIST was
settled before anything was guarded.

### The reader counts, re-verified on `origin/main` at `167dc4f84`

Unchanged and confirmed. `git grep 'ToolKind::ALL'`, `git grep
'Seat::ALL'` and `git grep '\.ordinal()'` across the whole tree:
`ToolKind::ALL`, `ToolKind::ordinal`, `Seat::ALL` and `Seat::ordinal`
have **zero** production readers; every non-prose hit under
`crates/viewer/src/` is a doc comment, and every call site is in
`crates/viewer/tests/combine_ops.rs` or `blend_authoring.rs`.

### The decisions, per item

**`ToolKind::ordinal` and `Seat::ordinal` — DELETED.** Each existed for
exactly one job, stated in its own doc: to be "the compiler-forced half
of that list's completeness", read against the hand-written `ALL` by
`every_tool_kind_is_listed_in_all` and by the seat sweep's ordinal
bookkeeping. Once `ALL` is projected from the enum's declaration
(`crates/viewer/src/vocab.rs`) there is no second list for a match to
be read against, and a "place in `ALL`" written by hand is a second
declaration of the order. The rows that read them went with them:
`every_tool_kind_is_listed_in_all` is deleted outright (its subject
cannot fail), and the seat sweep's `seen`/`ordinal` bookkeeping is
dropped from
`every_seats_wanted_kind_is_the_one_its_door_refuses_by`, which keeps
its real job. The one test that needed an index —
`only_one_modal_tool_is_open_at_a_time` — needs none now: its expected
flags are `ToolKind::ALL.map(|kind| kind == opened)`.

Rejected for the pair: keeping them with a corrected doc (they would be
a hand-written order with nothing reading it), and `#[cfg(test)]`
(these are integration tests; `cfg(test)` does not reach them).

**`ToolKind::ALL` and `Seat::ALL` — KEPT, `pub`, projected.** The
complaint was a `pub` hand-list maintained for a test. The macro
removes the "hand-list" half: the array is now a projection of the
declaration, costs nothing to maintain and cannot go stale, so what is
left is a `pub` projection whose readers today are the suites. That is
the right trade, and the alternatives are worse:

- **Delete it and let the suite keep its own list** — the suite's list
  would be hand-written, unforced, and invisible to the compiler. That
  re-mints the exact defect the sibling item is about, one directory
  over.
- **Move it behind the suite** (`#[cfg(test)]`, or a test-local
  vocabulary) — not available. `crates/viewer/tests/*` are integration
  tests and see only this crate's public surface, so `pub(crate)` and
  `cfg(test)` both hide it from its only readers.

Both docs now say the suites are the only readers and why the item
stays `pub`; `tools.rs`'s module header no longer calls `ALL` "the one
list a compiler cannot force", because it is not one any more.

### The test-side residue — DONE, VIEW's territory now

VIEW's territory includes `crates/viewer/tests/*` (Ev, in-chat,
2026-09-04), so the half this item said neither program could touch was
taken here.

`open_flags`'s docstring (`combine_ops.rs:1248-1256`) said "`open_kind`
is a PRIORITY SCAN: it answers with the first tool it finds open, so it
cannot see a second one left behind it". Unit 1d removed that
mechanism. Rewritten: `open_kind` reads the single `Option<OpenTool>`,
so a second tool left behind has no spelling in that value at all —
the row's REASON for reading through the per-tool accessors is
unchanged and still good. The same pass made the function's
position-per-kind correspondence explicit (`ToolKind::ALL.map(|kind|
match kind { … })`) instead of leaving seven accessors in an order the
reader had to check by eye.

### The sweep — run, and its pattern is too coarse

The item's pattern is "`pub` items whose grep hits outside `src/` are
all under `tests/`". Run over `crates/viewer/src` at `167dc4f84` it
returns **twenty** names, and **no new instance of this item's class**.

- The four already named appear as two of the twenty: `ordinal` (both
  declarations, `tools.rs:89` and `seats.rs:137`).
- `ALL` does **not** appear, and that is the item's own stated blind
  spot biting inside the item's own class: the sweep keys on a bare
  NAME, and `Theme::ALL` shares the name and has production readers, so
  the two reader-less `ALL`s are masked by a third that is fine.
- The remaining eighteen are read-back doors — `pick::{id_of, key_of,
  ids_of, ids_in, face_at, edge_at, hovered_at, op_for}`,
  `display::{is_hidden, free_move_of, probing}`, `frame::{
  delta_not_a_number, unindexed_refusal,
  SUBJECTS_WITH_AN_EXPIRY_ISSUER}`, `session::evaluation_arc`,
  `bounds::{from_nodes, is_edge}`, `scene::as_requested`,
  `input::map_stream`. Every one of their docs describes what it
  answers; **none promises a production consumer that does not
  exist**, which is the half of the class that makes an instance a
  defect. They are the crate's stated posture ("every move a user can
  make is a typed operation on a state value, callable with no
  renderer present"), exercised headlessly.

**So the pattern over-collects by roughly ten to one and under-collects
on collided names.** The discriminator is the DOC'S PROMISE, not the
reader count. Its other blind spot is the one the item names and this
run cannot fix: an item reached through a re-export or a trait method,
which a name grep does not resolve to its definition.
