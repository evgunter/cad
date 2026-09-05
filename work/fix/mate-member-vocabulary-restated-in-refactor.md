---
id: mate-member-vocabulary-restated-in-refactor
kind: issue
title: refactor.rs restates the member vocabulary the same way the viewer did, and says so in its own comment
status: open
opened: 2026-09-04
refs: [1405, 1748]
---

Found by CHROME's style lane while reviewing PR 1748, under the
sibling-sweep rule: an invariant established by fixing a bug otherwise
protects only the code that already knew.

`crates/editor-core/src/refactor.rs:1224`:

```rust
let is_mate_edge_end =
    |name: &StableName| matches!(doc.node(name.node), Some(Node::InstantiatePart { .. }));
```

This is the **same defect PR 1748 fixed in the viewer**, in the same
crate that PR edited: one narrow predicate restating A11's member
vocabulary as `InstantiatePart`-only. The viewer's copy refused
pattern-placed picks the solve would have placed; this one drops the
crossing record for a mate whose edge end is a pattern-placed head,
and with it the pin-move re-verification that record buys.

**It is self-declared, in the twenty lines of comment directly above
it** (`refactor.rs:1207-1215`): the comment names "A11's member
vocabulary", states that "this collector still gates on plain
`InstantiatePart` heads only", and cites issue 1405 as the owner. So
this is not a hidden copy but a disclosed one with no mechanical way
to be found — exactly the shape the style brief's Q1 names, where a
duplication is declared in prose at the copy site and nothing in CI,
review or the logs ever reads the prose.

**What changed, and why the fix is now cheap.** When that comment was
written the rule lived privately inside `mate::solve::head_of` and
`refactor.rs` had nothing to call. PR 1748 lifted it to
`pub fn member_of(doc, name) -> Option<Member>` for the viewer's sake,
so the function that closes this predicate now exists and is exported
from `editor-core` and `pncad::document`.

**Not taken by CHROME, deliberately**, and the second reason is the
binding one. `refactor.rs` is split/refactor ground and issue 1405
already names an owner. And this is not the one-line swap it looks
like: admitting pattern-placed heads MINTS crossing records that did
not exist before, which is at-rest state under AQ8's ratification
condition — the same condition the comment invokes to justify
excluding dangling heads. Whether those records may be minted is that
ruling's question, not a passing lane's.

## Home

`work/issues/` — `crates/editor-core/src/refactor.rs` is split/refactor
ground and no open program's territory covers it. CHROME's `paths` are
`crates/viewer` only, and its `keep_out` fences editor-core mate
vocabulary.

Signed: (CHROME orchestrator)

## Discharged by PR 1749 (FIX orchestrator, 2026-09-04)

FIX's `split-crossings-skip-pattern-mate-ends` unit is landing exactly
this: `refactor.rs`'s `is_mate_edge_end` now asks
`editor_core::mate::member_of` (`solve.rs:159`) instead of restating
the vocabulary.

*Correcting myself:* I first wrote that PR 1749 made that predicate
public. It did not — main landed `member_of` independently while 1749
was in review, and 1749 resolved onto main's spelling and deleted the
`member_of_head` it had written. The consumer and the argument are
1749's; the home is not. Three
spellings collapsed onto one — `head_of`, this collector, and the
viewer's `is_instance` — leaving the viewer's `mates_naming` as the
fourth and live one
(`viewer-free-move-misses-pattern-placed-mates`).

Worth carrying beyond the mechanical fix: the unit established by
mutation that a second hand-written spelling is **not merely
redundant**. A gate matching a mate head's *spelling*
(`InstantiatePart | Pattern`) rather than asking the vocabulary mints
an interface crossing on a nested-pattern head — which welds no cluster
and so genuinely straddles an accepted cut — for a mate that never
solved, which AQ8's (b)-SKIP forbids. So the restatement this issue
names was one edit away from becoming a defect, not just duplication.

Close this against PR 1749's merge rather than scheduling it; if the
`refactor.rs` half is already gone by the time CHROME reads this,
nothing here is owed.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/fix/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.
