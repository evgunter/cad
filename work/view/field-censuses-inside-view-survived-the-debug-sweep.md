---
id: field-censuses-inside-view-survived-the-debug-sweep
kind: issue
title: eight hand-listed field censuses inside crates/viewer/src, across four hats, are the same class and were not swept
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

`crates/viewer/src/app.rs:791-798` writes six `ViewerApp` fields in
one block and `:945-948` three more;
`crates/viewer/README.md:622` already notes that `ViewerApp` "still
be bookkept by hand". Whether those are the same class or an ordinary
update is a judgement this file does not make — but they are the next
place to look, and the sweep looked at neither. **Dispositioned in
`## Closed` below**, with a file for the half that is a defect.

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

**`impl PartialEq for Camera`** (`camera.rs:116-127` on base,
`camera.rs:116-153` at head). The comparison moved behind a
`coordinates` helper, which destructures `Camera` once and returns the
eight numbers as an array; `eq` is now
`coordinates(self) == coordinates(other)`. One pattern rather than two,
because a census stated twice is a census that can disagree with
itself. The second pattern in that helper names `Point3`'s `x`, `y` and
`z` rather than reading `target.x`: expanding the point by hand was
where the census stopped at the crate boundary, and it did not have to.

The helper is a `fn` NESTED IN `eq` (`camera.rs:139-150`), not a method
on a second `impl Camera` block. It exists for `eq` alone, so nesting
puts it at its only call site and keeps `impl Camera` a single block —
a reader of the type's inherent surface sees all of it in one place,
which is the shape `four-debug-walks-are-spelled-and-placed-two-ways`
is open on. It reads the fields and not the six public accessors
(`camera.rs:490`, `:495`, `:500`, `:515`, `:520`, `:526`) because an
accessor call is a field READ: a census assembled from accessors is a
hand list again, and the E0027 tie — the whole point of the helper —
would be gone. That argument is now written at the helper.

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
subject ROUTES the message. It is what `StatusUpdate::Expire` retires
it by, and what a joined rank-2 line takes as its own subject
(`frame::joined_subject`, `frame.rs:560`). It does **not** rank:
`frame::frame_status` (`frame.rs:507`) ranks by SOURCE — a refusal,
else the frame's notices, else the batch's verdict — and no rank reads
a subject. The first draft of this row and of the README passage both
said "what ranks it"; both are corrected. A line printing its own
routing would say to the user what the chrome says to itself.

**An eighth, found by this row's own fix pass: `BlendTool::clear`**
(`blend.rs:491-494` on base, `blend.rs:497-501` at head). It set
`self.target = None; self.edges.clear()` by hand over a two-field
struct (`blend.rs:306-309`) — the same hat as `DisplayState::clear`,
355 lines below the `BlendTarget` census this PR had already converted
in the same file, and its doc says *"Drop every pick"*, which is a
census claim. Converted. A third `BlendTool` field is now E0027 there
rather than surviving a door whose contract is that the tool holds
nothing afterwards.

**Witness experiment, re-run on head.** Seven witness fields added at
once, one per value, `cargo check -p viewer --features app`. **18
errors: 7 x E0027 + 11 x E0063.**

E0027 (pattern-does-not-mention-field) at every converted site. Line
numbers below are HEAD's, each read back after the witness fields were
removed:

| site at head | pattern |
|---|---|
| `camera.rs:140` | `let &Camera { … } = camera;` in `eq`'s `coordinates` |
| `display.rs:850` | `DisplayState::clear` |
| `prefs.rs:309` | `Display for StoreError` |
| `frame.rs:334` | `Display for Message` |
| `frame.rs:724` | `Display for Withdrawal` |
| `frame.rs:1889` | `Display for Disagreement` |
| `blend.rs:136` | `Display for BlendTarget` |

E0063 (missing-field-in-initializer) at **11** distinct struct
literals, which is `Derived::none`'s error and not this one — the
count is ten in the first draft of this row and it was wrong. Its
enumeration rule is *every struct-literal expression that constructs
one of the seven witnessed types*, and it produces `prefs.rs` x5
(`:333`, `:401`, `:410`, `:416`, `:421`), `frame.rs` x3 (`:304`
`Message::new`, `:712` `Withdrawal::of`, `:1959` `Disagreement`),
`blend.rs` x2 (`:102`, `:120`, both `BlendTarget`) and `camera.rs` x1
(`:414`, `Camera::new`) — witness-tree numbers, since these sites are
what the errors name and the tree they name is the witnessed one.
`--all-targets` reports 12 errors, not 12 sites: `frame.rs:712` is reported
once per target.

The `Point3` pattern was witnessed separately, by dropping `z` from
it: E0027 at `camera.rs:148`. `BlendTool` was witnessed separately
too, being the eighth (below): one error, E0027 at `blend.rs:498`, and
no E0063 anywhere, because `BlendTool` derives `Default` and every
construction goes through it.

**Behaviour: nothing changed, and here is what would have caught it.**
Deleting `impl PartialEq for Camera` and driving the compiler to a
fixpoint names every consumer of camera equality: `Folded`'s derived
`PartialEq` (the derive at `camera.rs:813` over the `camera` field at
`camera.rs:817`, transitive and invisible to any grep for
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

**The `## Where else to look` lead, dispositioned.** That section named
`ViewerApp`'s two hand-written field blocks and said the sweep looked
at neither. Both were read. **Neither is a census** — `ViewerApp` has
32 fields (`app.rs:273-459`) and neither block's population comes from
the declaration: `app.rs:791-798` installs one rebuild's six outputs
(*"Marked current ONLY on success"*), and `app.rs:945-948` sets the
three intentions a replaced document invalidates while most of
`ViewerApp` is session chrome that must survive an `Open`. The second
block is a real defect of a different class, and it has its own file:
`work/view/viewerapp-document-derived-state-has-no-boundary.md`.

`session.rs:1583` `clear_for_new_document` is the same shape and is not
an instance either, for the reason its own doc gives (`session.rs:1555-1560`):
the census was collapsed into one value rebuilt from nothing
(`Derived`) *"rather than a field-by-field walk each door has to
remember"*.

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

**Two sweeps the fix pass added, with their rules and their blind
spots.**

*The 31 enum `Display`s.* A `match` is exhaustive over VARIANTS, not
over a variant's FIELDS, so "exhaustive by its `match` already" settles
nothing about them. Rule: in each of the 31 impl bodies, a `{ .. }` or
`, ..}` in a pattern, a `_ =>` or bare-binding catch-all arm, and a
`Self::X(_…)` tuple pattern below the variant's arity. **Zero
catch-alls over a subject enum, zero tuple-arity drops, exactly two
`..`** — `CameraOp::Frame` dropping `bounds` (`camera.rs:357`) and
`MateToolEvent::PickLost` dropping `resolution` (`matetool.rs:353`).
Neither is converted: rendering either would change what the chrome
says, and this row's claim is that no rendering moved. `MateToolEvent`
already carried its argument at the impl (`matetool.rs:349-350`);
`CameraOp::Frame` carried none, and one is written at the arm now
(`camera.rs:345-357`) — `Aabb` has no `Display` in this workspace, the
two errors that provoke the sentence name the box's condition
themselves, and `aspect` is the half a reader can act on. Two more
sites match the rule and are not instances, both in
`Display for Withdrawal`: a `matches!` on another type's variant
(`frame.rs:728`) and a catch-all over `withdrawn.len()`
(`frame.rs:751`).

*The writing hat.* Rule: every `fn` under `crates/viewer/src` naming
two or more distinct `self.<field>` assignments, `.clear()`s or
`.take()`s, each hit read against its struct's declaration. **23 hits
at head, none a census** — a converted census has no `self.<field>`
write left, so it does not match the rule at all and a clean sweep is
the receipt. The 23 are bookkeeping (`ViewerApp::sync_scene`,
`BlendTool::load_all_edges`, `PickCache::sync` and `land`, the two
`Drop`s in `evalseam` where the language's own drop glue is the
exhaustive part). What this rule cannot see: a census spelled through
accessors rather than fields, a census over a value that is not
`self`, and one written by a macro — `vocab.rs` holds the crate's only
`macro_rules!` and it writes no such walk.
