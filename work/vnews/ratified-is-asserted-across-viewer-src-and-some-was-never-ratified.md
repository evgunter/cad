---
id: ratified-is-asserted-across-viewer-src-and-some-was-never-ratified
kind: issue
title: ratified is asserted thirty-five times under crates/viewer/src and some of it names no clause anywhere
status: open
opened: 2026-09-20
priority: P2
cost: D
---


Filed by VNEWS's frame.rs-cluster adjudication (2026-09-20), which
found one instance, proved it, and was told — correctly — that it had
swept for a phrase and stopped. **This row is the census. It does not
fix the population.**

## The instance that started it

`frame::frame_status`'s ranking was called *"the ratified ranking"* by
`work/vnews/rank-one-discards-the-frames-other-news` and *"rank 1's
ratified 'a refusal wins, alone'"* by
`work/vnews/one-line-one-subject-loses-a-mixed-frames-expiry`. It is
neither: the rule's only normative statement is a doc comment an agent
wrote on 2026-08-31, and an agent called it ratified on 2026-09-04.
That row's adjudication section holds the full search and is the
canonical home for the finding; this row is the class it is an
instance of.

## The sweep rule that produces the population

**Every `ratif*` occurrence in a doc comment, module header or code
comment under `crates/viewer/src`.** Case-insensitive, `.rs` files
only. Run today that is **35 lines in 17 files**:

    session/refuse.rs 6   platform.rs 4   session/op.rs 3
    frame.rs 3   display.rs 3   widgets.rs 2   tree.rs 2
    session/select.rs 2   blend.rs 2   theme.rs 1   props.rs 1
    pane/properties.rs 1   lib.rs 1   g1.rs 1   camera.rs 1
    bounds.rs 1   app.rs 1

**Re-derive it; do not quote 35.** The orchestrator's own pass over
the same ground reported **32**, and the gap is the enumeration rule
rather than the tree: this rule counts LINES matching, including the
three `platform.rs` hits where the verb has a different subject
(`:16` *"`scripts/gates/no-ambient-env.sh` ratifies that…"*, `:170`
and `:258` *"CONTRACT-RATIFIED holds vacuously"*), which a rule
counting *claims that a DESIGN DECISION is ratified* excludes. Both
rules are defensible and they give different numbers, which is the
point: a count carries its rule.

The instrument's blind spot, stated because the last one was not: a
grep for `ratif*` cannot see a sentence that asserts ratification
without the word — *"the GUI plan's rulings"*, *"settled"*, *"D5
decides"*. `session/select.rs:166` is inside the population only
because it happens to spell it *"by ratification"*.

## The test that sorts a true claim from a false one

`CLAUDE.md`, in terms: *text is not ratified by sounding official or by
sitting in a file whose companion-table row says Ratified* — **find
the clause, then find the commit that wrote it.** Applied here:

- **TRUE** when the sentence names, or can be traced to, a clause in a
  document `docs/DESIGN.md`'s companion table marks *Ratified*, or an
  Ev ruling recorded in the tracker.
- **FALSE** when the only backing is `crates/viewer/README.md` (the
  implementation record the companion row says *"the program maintains
  itself"*), another doc comment, or nothing.

**The class is mixed, which is why it is a census and not a sweep.**
Three worked examples, all in one file:

| site | claim | verdict |
|---|---|---|
| `crates/viewer/src/frame.rs:565` | *"the ratified expression-driven affordance"* | **TRUE** — `crates/viewer/GUI-DESIGN.md:93-94`, G4: *"Dragging an expression-driven dimension refuses, with an affordance offering to edit the expression"* |
| `crates/viewer/src/frame.rs:1191-1192` | *"the ratified argument the checks badge carries"* | **FALSE so far** — `checks badge` appears 0 times in `crates/viewer/GUI-DESIGN.md` and 0 times in `docs/DESIGN.md` |
| `crates/viewer/src/frame.rs:2040` | *"The ratified pattern is refuse-then-offer"* | **FALSE so far** — `refuse-then-offer` appears 0 times in either, and has already spread to four sites: `crates/viewer/src/frame.rs:2040`, `crates/viewer/src/app.rs:1171`, `crates/viewer/src/pane/properties.rs:237`, `crates/viewer/tests/frame_policy.rs:2620` |

*"so far"* is deliberate: a claim is FALSE only once the search for its
clause has been run and reported, and this row has run it for three of
thirty-five.

## The tracker half of the population

The same assertion travels in item files, where it decides whether a
row believes it needs Ev. **`work/vnews/a-tree-rows-message-line-picks-its-affordance-by-hand:26`**
calls `frame::Affordance::{Read, Opens}` *"the ratified value"*; its
only documentation is `crates/viewer/README.md` and `frame.rs`'s own
doc comments, which is the implementation record this adjudication
argues is not a gate. It was opened **2026-09-20**, the same day as the
adjudication that found the class — so the class is still minting
members, which is the argument for the census going before any of the
fixes.

## What this row delivers

The population, each member's verdict with the clause or the failed
search beside it, and the correction of whichever members come back
FALSE. **Not** a rule that `ratified` may not be written: the word is
right thirty-odd times here and the census is what tells them apart.

`crates/viewer/README.md` and `crates/viewer/tests/*` members are
VDOC's by `work/vnews/program.md`'s `keep_out` and are filed there, not
fixed here. `platform.rs`, `theme.rs` and `blend.rs` are among the
eleven files `work/view/viewer-src-files-no-successor-claims` reports
as claimed by no dispatching program.

## A tracker-side member, and it is the orchestrator's — 2026-09-20

`crates/viewer/src/session/select.rs`'s `Standing` doc calls the clause
*"the affordances that need a live entity switch off"* **GQ7's recorded
constraint**. Verified by grep over all three candidate documents:

- `crates/viewer/GUI-DESIGN.md` — **0 occurrences**, and its GQ7 is
  selection mechanics (single-select, pick priority, `PickKinds`),
  which defers vanishing-entity semantics elsewhere;
- `docs/SELECT-DESIGN.md` — **0 occurrences**;
- `crates/viewer/src/session/select.rs` — **1**, the assertion itself.

So this is the same shape as `frame.rs`'s *"ratified"* ranking: a source
comment naming a ratified home that does not contain the rule. It is
listed here as a source-side member.

**What makes it worth recording separately is where it went.** The VNEWS
orchestrator read that comment, did not check `GUI-DESIGN.md`, and wrote
*"ratified design in `crates/viewer/GUI-DESIGN.md`, so the shape I
recommended would have needed an `[ev]` PR"* into a dispatch, into
`work/vnews/plan.md` §Dispatch rules, and into the merged body of
PR #2942 — **two hours after ruling that the status line's ratification
was an agent's word nobody had checked, and while this row was being
filed.** The properties-pane lane caught it.

That is the class's real cost, and it is not that a comment is wrong.
**An unchecked ratification claim does not stay in the comment**: it is
believed by the next reader, who writes it somewhere more binding, and a
plan or a merged PR body is harder to correct than a doc line. The
sorting test this row states — find the clause, then the commit — has to
run at **the document that would carry the gate**, never at the comment
that cites one.

Corrected at `work/vnews/plan.md` §Dispatch rules, which now carries the
retraction rather than a quiet rewrite.
