---
id: rank-one-discards-the-frames-other-news
kind: issue
title: A frame's refusal discards its notices, and a discarded free-move placement is not recoverable
status: open
opened: 2026-09-04
priority: P0
cost: D
---

Disclosed by `opoutcome-superseded-has-no-production-reader`'s fix,
which put a new kind of message into rank 2 and so made the rank's
cost concrete.

## What happens

`frame::frame_status` (`crates/viewer/src/frame.rs`) ranks a frame's
news, and **rank 1 wins ALONE**: a refusal is shown and every notice
the same frame produced is dropped, not queued, not joined.

```
match batch_status(ops, refusal) {
    refused @ StatusUpdate::Show(_) => refused,
    ...
}
```

That is the ranking as `frame_status`'s own doc comment states it, and
it is deliberate — two sentences about different things are worse than
one about the loudest. **It is not ratified**; this row said it was,
and the correction is the last section of this file. The
question this item raises is whether it should still hold when the
dropped notice reports something the user **cannot get back**.

A batch is one frame's ops, and one frame can carry several: a panel
committing a mate and a drag emitting a gesture op that refuses is one
batch. When that happens today:

- the mate lands, `DisplayState::prune` discards the free-move
  placement the user positioned by hand
  (`crates/viewer/src/display.rs`, `prune`),
- `frame::Withdrawal::superseded` renders it,
- and `frame_status` drops that notice for the gesture's refusal.

The refusal is about an op that did nothing. The notice is about a
value that is **gone** — discarded, not zeroed and not parked, so no
undo of the refusing op restores it. A declined pick can be re-picked;
this cannot.

## Why it is not simply "add it to the refusal"

The joined form (`NOTICE_SEPARATOR`) already exists for several
notices, so the mechanism is there. What is missing is the JUDGEMENT:
which rank-2 messages are important enough to ride alongside a
refusal, and whether that is a property of the message (a lost value)
or of the pair. Deciding it for supersessions alone would put a second
un-stated rule in a module whose whole point is that its rules are
stated.

The same question is open for the tool notices already in rank 2 —
this item is not about supersessions alone, they are just the case
where the loss is irreversible.

## Where to look

- `crates/viewer/src/frame.rs` — `frame_status`, the ranking and its
  doc comment.
- `crates/viewer/src/app.rs` — `perform_batch`, where a frame's ops
  and its notices are collected together.
- `crates/viewer/tests/frame_policy.rs` —
  `a_superseded_free_move_is_news_the_ranking_shows` is the row that
  would grow a refusing sibling op.

## Adjudicated 2026-09-20 (`vnews/frame-cluster-order`)

### The ranking has no ratification, so this row is this program's to decide

**This section is the canonical home for the finding.** Five sibling
rows and `work/vnews/plan.md` §Order 7 reached the same conclusion in
the same adjudication; they cite this section rather than restating it,
for the reason `work/vnews/plan.md` §The register gives about the lane
register itself — *a claim fixed in one place and stale in another
contradicts itself*, and six copies of one search guarantee divergence
the first time anything moves.

This row called rank 1 *"the ratified ranking"*, and
`one-line-one-subject-loses-a-mixed-frames-expiry` inherited the word
from it. Run against the tree, the word does not hold. What was
searched and what each search returned:

- `crates/viewer/GUI-DESIGN.md` — the Ev-gated document, the one
  `docs/DESIGN.md`'s companion table (`docs/DESIGN.md:33`) marks
  *Ratified* — **contains no clause about the status line at all.**
  Grepped for `rank|ranking|alone|status|notice|news|frame_status`:
  **three hits, `:176`, `:206` and `:234`, all of them the word
  `alone`** — *"bevy is … left alone"*, *"generation alone"*, *"left
  alone because it is an observation"*. None is about the status line.
  (A first pass of this row listed `:4` and `:159` among the hits.
  They are not hits for any of the seven patterns; the correction
  strengthens the conclusion, which is exactly why it is made —
  `work/view/plan.md`'s rule is that *a receipt offered as evidence and
  wrong about its own file is worse than no receipt*.)
  Read rather than grepped: G3 (`:78`) is what v1 is; G4's
  presentation clause (`:93-99`) says *"Presentation is decided case by
  case"*; G5 (`:104`) is colour; GQ1–GQ7 (`:130-167`) are the solver
  boundary, partial builds, persistence, document scope, typed
  quantities, the toolkit posture and selection mechanics.
- `docs/DESIGN.md`'s own open questions Q1–Q9 (`docs/DESIGN.md:1302`
  to `:1402`) — none of the nine is the status line.
- A pickaxe search for the rule's own words in its own file returned
  exactly one commit, `66d7fe11f` (*"GAUTH-5 fix pass: the frame's
  notice composition…"*, 2026-08-31), author **Claude**. That commit
  wrote the `# The ranking` list in `frame_status`'s doc comment,
  which is the rule's only normative statement anywhere in the tree.
- The same search over the whole repository returned two commits,
  `66d7fe11f` and `b3ac2f792`, both authored by **Claude**.
- A `--grep` for `[ev]` over this file's history returned **no
  commits**: no `[ev]` PR has ever touched `crates/viewer/src/frame.rs`.
- A pickaxe search for the phrase *the ratified ranking* returned one
  commit, `3ef3f9df7` (*"viewer: a superseded free move reaches the
  status line"*, 2026-09-04, author **Claude**) — the commit that wrote
  that sentence in THIS file. The phrase occurs nowhere else in the
  tree.
- `crates/viewer/README.md` mentions the ranking and nowhere states
  rank 1's alone-ness. Grepping `ranking` gives **five hits — `:674`,
  `:737`, `:743`, `:755`, `:1192` — of which four are about
  `frame_status`'s ranking**; `:1192` is a different subject (layer 3's
  refusal ranking, *"Layer 3 adds nothing but the ranking, so it stores
  the payload and forwards the text"*). A first pass of this row listed
  `:867` instead of `:1192`: `:867` is not a `ranking` hit at all — the
  sentence at `:866-867` says *"joined into rank 2 by `frame_status`"*,
  which is `rank`. It is still evidence for the conclusion, just not
  evidence of the kind the sentence claimed, and the correction is made
  here rather than absorbed because `work/view/plan.md`'s rule is that
  *a receipt offered as evidence and wrong about its own file is worse
  than no receipt*.
  It is not an Ev gate either way: the companion table's GUI-DESIGN row
  says that page *"beside it is the implementation record, which the
  program maintains itself"*, and it has no row of its own in that
  table.

So the chain is: an agent wrote the rule as a doc comment on
2026-08-31; an agent called it ratified on 2026-09-04; this row and
`one-line-one-subject-…` have carried the word since. `CLAUDE.md`'s
test — *text is not ratified by sounding official or by sitting in a
file whose companion-table row says Ratified* — settles it. **There is
no ratification, so changing rank 1 is this program's decision and not
an `[ev]` gate.**

### What that does and does not license

It does not make the change free. The rule is asserted in
`crates/viewer/tests/frame_policy.rs` and in `frame.rs`'s own test
module, `crates/viewer/README.md` describes it at the four sites above,
and `frame_status`'s doc comment is where the rule is stated — a lane
changing rank 1 re-states it there, re-baselines the rows it moves, and
says what moved. What the finding removes is the belief that a question
has to be asked before any of that can start.

### Citations re-derived against the tree (2026-09-20)

- The ranking is `crates/viewer/src/frame.rs:665-669` (`frame_status`'s
  body); the `# The ranking` list in its doc comment is `:610-623`.
- `batch_status`'s refusal arm is `crates/viewer/src/frame.rs:583-585`,
  and it spells the refusal `Subject::Document`.
- `frame::Withdrawal::superseded` is `crates/viewer/src/frame.rs:991`;
  that its notice is `Subject::Document` is asserted at
  `crates/viewer/src/frame.rs:2502-2507`.
- `crates/viewer/tests/frame_policy.rs:2779`,
  `a_superseded_free_move_is_news_the_ranking_shows`, is the row named
  here and is there.

**A consequence this row should carry.** Because a refusal and a
supersession are BOTH `Subject::Document`, the per-subject line that
`one-line-one-subject-loses-a-mixed-frames-expiry`'s arm 2 proposes
would NOT deliver this row's case. The two rows are adjacent and
independent rather than one conversation; that row's own adjudication
section states the same finding from its side.
