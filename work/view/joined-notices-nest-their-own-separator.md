---
id: joined-notices-nest-their-own-separator
kind: issue
title: A frame's joined notices nest NOTICE_SEPARATOR and the em-dash inside themselves, so the line is ambiguous at two notices
status: review
opened: 2026-09-05
refs: [the-news-vocabulary-has-no-expiry, status-line-writers-bypass-the-ranking, 1886]
branch: view/joined-notices
pr: 2665
priority: P1
cost: E
---


Found by #1886's style review. Pre-existing in shape, made reachable
by that unit: `frame` had one notice producer and now has two, so two
notices in one frame is an ordinary state rather than a hypothetical.

## What the line reads

`frame::frame_status` joins rank-2 notices with `NOTICE_SEPARATOR`
(`"; "`), and `render_causes` joins each notice's own causes with the
same string. With two superseded placements and one dropped hide the
status line is:

> free move: 2 committed placements were discarded — A; B; hide: … — C

A reader cannot tell where the cause list ends and the next notice
begins, because the inner join and the outer join are the same
character. The em-dash nests the same way: `DisplayFault`'s own
`Display` arms contain one (`FusedGeometry`,
`crates/viewer/src/display.rs:199-209`), and so does each notice's
preamble.

## Why it is a design question and not a formatting nit

The obvious fix — a different inner separator — is a choice about how a
composed sentence tells a reader its own structure, and this crate has
no rule for that. It is also the same question from a different side as
`the-news-vocabulary-has-no-expiry`, which is on `[ev]` PR #1883: if a
message carried its subject, several notices about different subjects
would have a structure to render rather than a string to concatenate,
and the ambiguity would not arise. Answering the separator alone would
be a local patch over a vocabulary gap.

One hazard worth recording separately, because it is mechanical and
present today: **`DisplayFault::NonRigidFrame`'s `Display` contains a
`"; "` of its own** (`display.rs:180`). Any reading of the joined line
that counts separators — including the assertion at
`crates/viewer/src/frame.rs:1186` — is wrong the moment that arm
reaches the line. #1886's fix pass was asked to stop that assertion
lying; the ambiguity it is a symptom of is this file.

## Home

VIEW's: `crates/viewer/src/frame.rs`. Sequence after #1883's answer on
the news vocabulary.

## #2026 multiplied the reachability again, by an order of magnitude

This item's premise was *"`frame` had one notice producer and now has
two, so two notices in one frame is an ordinary state rather than a
hypothetical."* `status-line-writers-bypass-the-ranking` took the notice
producers from four to eighteen — ten `tool_news` sites in
`pane/create.rs`, `pick_refusal`, `unindexed_refusal`,
`delta_refusal`, `delta_not_a_number`, `store_refusal`,
`Disagreement::notice`, `fold_status`'s refused fold and the two
`Withdrawal` notices — and every one of them now lands in the same
`Vec<Message>` that `frame::frame_status` joins with
`NOTICE_SEPARATOR`.

**Multi-notice frames stopped being unusual.** A create-pane refusal
and a camera fold refusal are one drag apart; a δ that will not parse
and a pick the index refuses are one click apart. Before this unit the
only way to get two was two withdrawals from one accepted edit.

That does not change the finding — the ambiguity is the same
ambiguity — but it changes what it costs and what a fix has to
survive. The line a reader now has to disambiguate can be a camera
refusal, a tool refusal and a preferences-store failure joined with
`"; "`, each of which may contain a `"; "` or an em-dash of its own.
The hazard recorded above (`DisplayFault::NonRigidFrame`'s embedded
`"; "`) is one of several rather than the one.

It also gains a second side, which belongs to
`one-line-one-subject-loses-a-mixed-frames-expiry`: those notices no
longer share a subject, so the joined sentence has a structure in the
type as well as in the prose, and `joined_subject` throws it away to
pick one `Subject`. Answering the separator without answering that
would render a structure the value no longer has.

## Note (`view/gesture-doors`, 2026-09-11): the assertion citation is stale

`crates/viewer/src/frame.rs:1186` is named here as *"the assertion"*
and is a doc-comment line inside `delta_not_a_number` at the merge base
of this note. The finding is untouched; the pointer was already wrong,
so it is disclosed rather than moved — a number that was never about
its subject has no shift to apply.

## Answered (`view/joined-notices`, 2026-09-15)

The outer level is fixed and the rule is on the type.
`frame::NOTICE_SEPARATOR` is now `" • "`, built from
`frame::NOTICE_MARK`, and goes between two of a frame's notices; the
new `frame::LIST_SEPARATOR` (`"; "`, the old spelling) goes between
the items of a list ONE notice carries — a `Withdrawal`'s causes, the
preferences path's startup notices. Neither rendering changed at the
inner level.

What holds it is two private-field consequences, not an assertion:
`Message::new` — the only public door — rewrites a `NOTICE_MARK` out
of every text that reaches it, and the only constructor that WRITES
one, `Message::joined`, is module-private and takes `&[Message]`
rather than strings. So the only way to a boundary mark is to have had
two notices, and `line.split(NOTICE_SEPARATOR)` returns exactly the
notices that went in. Three rows in `crates/viewer/tests/frame_policy.rs`.

### What this item claimed, checked

- The outer and inner joins being one string: **true**.
- `DisplayFault::NonRigidFrame`'s `Display` carrying a `"; "`:
  **true**, and it is no longer a boundary mark now that a boundary is
  a bullet. Its collision with the INNER join survives and is filed as
  `withdrawal-causes-join-on-a-mark-a-fault-may-contain`; that item
  also records why no prune path reaches it today.
- `render_causes`, named here as the site that joins a notice's own
  causes: **no such function today**, and it was real when this row
  was written. `fn render_causes` lived in `app.rs` from `6877a40ff1`
  (2026-09-04) and was deleted by `4db112ada0` the next day, the day
  this item was opened. `stale-file-citations-after-the-split` records
  the citation as *left as written* because naming a successor would
  have been a guess. **The successor is nameable now**: the same
  commit moved the join into `Display for Withdrawal`, where the cause
  loop still is, and this unit is what read it there. The sentence's
  substance was correct throughout — a notice's own causes were and
  are joined with the same string the notices were.
- The em-dash "nesting the same way": **it is not the same way.** No
  join anywhere writes an em-dash — it is in-sentence punctuation in
  `Withdrawal`'s preamble and in two `DisplayFault` arms — so it was
  never a boundary a reader could mistake for the outer one, and the
  outer fix needs nothing from it. What remains is in-sentence
  structure inside one notice, which is the inner item's ground.
- `crates/viewer/src/frame.rs:1186` as "the assertion": already
  disclosed as stale by the 2026-09-11 note; unchanged here.
