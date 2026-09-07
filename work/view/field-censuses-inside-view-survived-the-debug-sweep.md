---
id: field-censuses-inside-view-survived-the-debug-sweep
kind: issue
title: three hand-listed field censuses inside crates/viewer/src, one of them in the file this sweep edited, are the same class and were not swept
status: closed
opened: 2026-09-06
refs: [2093]
closed: 2026-09-07
branch: view/censuses
pr: 2103
---



Found by the style review of #2093.

The item this PR closes says the class that matters is **a hand-listed
field census of any kind, not the `Debug` trait**, and the sweep it
shipped grepped the trait. Three instances of the stated class sit
inside `crates/viewer/src/` and none is named. One of them is in the
file the PR edited.

## `PickCache::forget` — the same walk, eleven lines below the one that was fixed

`crates/viewer/src/pickcache.rs:335-340` clears four of `PickCache`'s
five fields by hand:

```
fn forget(&mut self) {
    self.index = None;
    self.attempted = None;
    self.outstanding = None;
    self.error = None;
}
```

`PickCache` is declared at `:147-161`. A sixth field added there is
E0027 in the `Debug` walk this PR wrote (`:172-178`) and **silently
not forgotten here** — and this walk is the one with a correctness
consequence, since its own doc (`:319-334`) argues that leaving
`attempted` set is what makes a late answer install "an index of a
document nobody is looking at, over a scene of a third one, with
nothing running and nothing said."

This is `Derived::none`'s shape without `Derived::none`'s property, in
the file the sweep was reading, in the crate the sweep was scoped to.

## `impl PartialEq for Camera` — the in-fence instance the sweep places out of fence

`crates/viewer/src/camera.rs:116-127` compares eight coordinates of
`Camera`'s six fields (`camera.rs:99-106`) by hand. A seventh field
added to `Camera` is silently outside equality, with no compile error
and no `finish_non_exhaustive`-shaped marker to hedge it.

The PR's blind-spot paragraph says the `PartialEq` hazard is
out-of-fence: *"the `PartialEq` impls beside the hull/window `Debug`s
above are hand-written field-by-field comparisons with the same
hazard, unswept because they sit in the same out-of-fence files."*
`crates/viewer/src/camera.rs` is in the fence, and it is the crate's
only hand-written `PartialEq` — one `rg -n 'impl[^=]*\bPartialEq\b
for' crates/viewer/src` away.

## `DisplayState::clear` — three of four fields, by hand

`crates/viewer/src/display.rs:839-846` clears `hidden`, `moves` and
`free_move` and deliberately leaves `revision` monotonic;
`DisplayState` is declared at `display.rs:578-595`. A fifth field is
silently not cleared. `crates/viewer/README.md:406-409` and
`session.rs:225-230` both lean on `DisplayState::clear` being the
reset door for the display half of a new document, so a field that
missed it would be the same stale-across-`Open` defect
`session-clearing-walk-is-hand-maintained-three-times` closed for
`Derived`.

`work/view/display-clear-drops-free-move-placements-silently-while-prune-reports-them.md`
is about a different defect in this function (a silent drop) and does
not cover its exhaustiveness.

## Where else to look

`crates/viewer/src/app.rs:791-795` writes five `ViewerApp` fields in
one block and `:946-948` three more;
`crates/viewer/README.md:510` already notes that `ViewerApp` "still
be bookkept by hand". Whether those are the same class or an ordinary
update is a judgement this file does not make — but they are the next
place to look, and the sweep looked at neither.

## `PickCache::forget` is taken on #2093; two of three remain, and five more are added

**`forget` now destructures**, with `seam: _` as the arm that must
stay, so a sixth `PickCache` field is E0027 there as well as in the
walk eleven lines above it. That one was taken rather than filed
because it is the instance with a live consequence — its own doc argues
that a missed `attempted` is what lets a late build install an index of
a document nobody is looking at — and because it sits in the file the
sweep was reading.

`impl PartialEq for Camera` and `DisplayState::clear` stay filed here.
Neither is in the file #2093 edited, and `DisplayState::clear`'s
deliberate `revision` exception means its `_` arm carries an argument
that wants writing rather than a mechanical destructure.

**Five more instances, from re-deriving that PR's false blind-spot
claim** (`debug-sweep-display-blind-spot-claim-is-false`): of the 36
`impl … Display for` under `crates/viewer/src`, five are over structs
and read fields by hand —

| site | value | reads | of |
|---|---|---|---|
| `crates/viewer/src/prefs.rs:304` | `StoreError` | `doing`, `because` | 2 |
| `crates/viewer/src/frame.rs:320` | `Message` | `text` | 2 |
| `crates/viewer/src/frame.rs:706` | `Withdrawal<'_>` | `kind`, `withdrawn` | 2 |
| `crates/viewer/src/frame.rs:1847` | `Disagreement` | `from_gpu`, `from_ray` | 2 |
| `crates/viewer/src/blend.rs:128` | `BlendTarget` | `node`, `body` | 2 |

None is missing a field today; `Message`'s omission of `subject` is
deliberate, since the subject routes the message rather than appearing
in it. What none has is a compile-time tie, and `Disagreement`'s own
doc argues that both halves it renders are load-bearing — the sentence
a third field would falsify.

So this row now carries seven instances across four hats — `PartialEq`,
a clearing walk, and five `Display`s — which is the point the sweep's
own greps could not reach.

## Closed (2026-09-07)

All seven converted. The mechanism is exhaustive destructuring, as in
#2093 and in `session-clearing-walk-is-hand-maintained-three-times`
before it, and it fits all four hats — but for four different reasons,
which is the part that was not mechanical.

**`impl PartialEq for Camera`** (`camera.rs:116-127` on base). The
comparison moved behind a private `Camera::coordinates`, which
destructures `Camera` once and returns the eight numbers as an array;
`eq` is now `self.coordinates() == other.coordinates()`. One pattern
rather than two, because a census stated twice is a census that can
disagree with itself. The second pattern in that function names
`Point3`'s `x`, `y` and `z` rather than reading `target.x`: expanding
the point by hand was where the census stopped at the crate boundary,
and it did not have to.

**`DisplayState::clear`** (`display.rs:839-846` on base) destructures.
`revision` needs no `_` arm at all — the walk *uses* it, bumping it
when the reset was visible — so what the pattern buys is that the
field is named at the site where its exception lives, with the reason
(the chrome's rebuild key must not go backwards) written beside it.

**The five `Display`s over structs** all convert, and the argument is
not "we did it to the others". The cost here is one line, not five: two
of the five (`Withdrawal`, `StoreError`) come out SHORTER than they
went in, because the destructure replaced a `let withdrawn = …` and an
inline-capture `write!` respectively. The gain is that each of these
five renderings is meant to be a complete account of its value —
`StoreError`'s sentence is the only public face that value has,
`Withdrawal`'s whole job is to word itself, `BlendTarget`'s sentence is
how a refusal names the scope it refused on, and `Disagreement`'s own
doc *argues* that both halves are load-bearing, so the pattern is what
holds that paragraph to the value instead of leaving it asserted.

`Message` is the fifth and the exception: its account is deliberately
partial, and `subject: _` is where that decision now lives — the
subject routes the message (it is what `StatusUpdate::Expire` retires
and what ranks it), and a line printing its own routing would say to
the user what the chrome says to itself.

**Witness experiment.** Seven witness fields added at once, one per
value, `cargo check -p viewer --features app`; E0027
(pattern-does-not-mention-field) at every converted site —
`camera.rs:121`, `display.rs:851`, `prefs.rs:310`, `frame.rs:332`
(`Message`), `frame.rs:723` (`Withdrawal`), `frame.rs:1889`
(`Disagreement`), `blend.rs:137` — and E0063
(missing-field-in-initializer) at ten struct literals, which is
`Derived::none`'s error, not this one. The `Point3` pattern was
witnessed separately, by dropping `z` from it: E0027 at
`camera.rs:128`.

**Behaviour: nothing changed, and here is what would have caught it.**
Deleting `impl PartialEq for Camera` and driving the compiler to a
fixpoint names every consumer of camera equality: `Folded`'s derived
`PartialEq` (`camera.rs:807`, transitive and invisible to any grep for
`==`), `pane/viewport.rs:510`, `tests/input_mapping.rs:332,358,362` and
`tests/review_gui0_r2.rs:171,354` — the last two files compare WHOLE
cameras to check that `fold` agrees with sequential `apply` and that
`map_stream`'s camera agrees with folding its own ops, so a coordinate
outside `eq` is a coordinate those properties do not check. Perturbing
all five renderings and running both suites in both feature
configurations names every assertion on them: four lib tests on
`Withdrawal` and `frame_policy::the_agreement_check_compares_names_and_ignores_answers_nobody_asked_for`
on `Disagreement`; nothing asserts on `Message`, `StoreError` or
`BlendTarget`, though deleting those three impls shows all three ARE
rendered, at twelve sites. Both suites are unchanged on head.

**Receipt, with its rule.** The enumeration rule for the `Display` hat:
every `impl … Display for T` under `crates/viewer/src`, with `T`'s
declaration looked up in the crate and classified `struct` or `enum`.
36 impls, 36 distinct subjects, **5 structs** — `BlendTarget`,
`Disagreement`, `Message`, `StoreError`, `Withdrawal` — and 31 enums,
which matches the count this file already carried. For the `PartialEq`
hat: every `impl … PartialEq … for` under `crates/viewer/src`, which is
**1**, `Camera`. What the rule cannot see: an impl generated by a
macro, and a `Display` derived by a proc macro. Both are closed by
inspection rather than by the grep — `vocab.rs` holds the crate's only
`macro_rules!` and it generates neither `Display` nor `PartialEq`, and
`crates/viewer/Cargo.toml` depends on no `thiserror`/`derive_more`/
`strum`/`displaydoc`. `crates/viewer/tests/` contains no `Display` or
`PartialEq` impl at all, and the crate has no `impl ToString`.
