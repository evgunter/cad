---
id: viewport-pointer-buttons-mirror-a-toolkit-enum-by-hand
kind: issue
title: The viewport maps egui's three pointer buttons to the viewer's three by hand, and nothing forces either side
status: closed
opened: 2026-09-11
closed: 2026-09-12
branch: door/pointer-button-match
---



Filed by the `BooleanOp::ALL` unit. The finding is not new — it is the
fourth instance recorded in the body of
`hand-maintained-mirrors-of-a-kernel-enum-are-unforced`, which that
item declined to file separately because it WAS the file. That item
closes with this unit, so the instance needs its own.

## The finding

`crates/viewer/src/pane/viewport.rs:85-87` maps `egui::PointerButton`'s
three buttons to `crate::input::PointerButton`'s three
(`crates/viewer/src/input.rs:63`), by hand, in production:

    (egui::PointerButton::Primary, PointerButton::Primary),
    (egui::PointerButton::Secondary, PointerButton::Secondary),
    (egui::PointerButton::Middle, PointerButton::Middle),

Both sides are complete at three today and nothing forces either. A
fourth button on either side is silently never produced: no compile
error, no red row.

## Why it is the class's hardest instance

It is the same class as the closed item — a complete list of one
enum's variants, written where no compiler can read it against the
declaration — with the mirrored enum in the TOOLKIT rather than in a
crate this repo owns. The fix that closed the boolean case (ask the
declaring crate to publish its own `ALL`) is unavailable in the
`egui` direction, so this is the instance that most likely wants the
"a row rather than a mechanism" answer: a test that matches
exhaustively over `input::PointerButton` and asserts each arm is
produced by some `egui::PointerButton`, red on the day either side
grows.

The list is not a `const` item, so
`scripts/gates/viewer-vocab-declared-once.sh` does not see it — that
blind spot is stated in the gate's own header ("A LIST THAT IS NOT A
`const` OR A `static` ITEM").

## The mirror is not unforced — it is already WRONG (2026-09-12)

Corrected by the DOOR orchestrator, from Ev's question on PR #2410
("why is there a hand mirrored enum there"). The row was filed as an
instance of "a vocabulary mirrored where no compiler sees the mirror",
which is the wrong classification twice over.

**Why the viewer has its own `PointerButton`, and why that is right.**
`crates/viewer/src/input.rs` is toolkit-free BY DESIGN: it imports only
`crate::camera`, and its module doc states the rule — *"Module kind:
**vocabulary** — it names no driver type and no `app`-only crate."* That
is what lets `InputMap` and the bindings (`orbit_button`,
`alt_orbit_button`, `pan_button`, `select_button`) exist without an
`egui` dependency. `pane/viewport.rs:84-88` is the adapter that
translates at the edge. Keeping the toolkit's types out of the
vocabulary layer is the same instinct as the kernel/chrome split, and
the mirror's EXISTENCE is not the defect.

**The defect is that the adapter is already stale.**
`egui::PointerButton` has **five** variants —
`Primary`, `Secondary`, `Middle`, `Extra1`, `Extra2`, with
`pub const NUM_POINTER_BUTTONS: usize = 5`
(`egui-0.36.1/src/data/input/pointer_button.rs:4-23`). The viewport's
loop polls three. A drag or a click on a mouse side button therefore
produces **no `ViewportEvent` at all**, silently — and the viewer's own
`PointerButton` has no variant that could name one, so no binding could
reach them even if the adapter tried.

So this row is not "nothing forces it, and it could go stale". It went
stale and nothing reported it.

**And it is a fourth kind, not one of `crates/viewer/README.md`'s
three.** Completeness here is a PRODUCT decision — which buttons the
viewport binds — which is nearer the surviving *"deliberately partial"*
kind than the retired mirror kind. But it is not cleanly that either,
because nothing at the site says three-of-five is deliberate. What it
actually is: **an adapter silently incomplete against its upstream, with
no statement either way.**

**What the row now wants**, in order: decide whether binding the side
buttons is wanted; then either add the two variants and their bindings,
or say at the site that three-of-five is the product decision and why.
Only after that does the question of which README kind covers it have an
answer.

**Consequence for the ruling.** This row was the orchestrator's
counterexample against the paragraph #2387 put in place of the retired
kind. The counterexample was misclassified, so it does not support that
argument; see `readme-ratification-amendments-need-ev`.

## The fix shape, settled (Ev's question on PR #2410, 2026-09-12)

Ev asked: *"can we put that enum in a third dependency free crate that
both import or does that not work"*.

**A shared crate does not work, and is not needed.**

**Why not.** That trick needs both sides to be ours. `egui` is a
third-party crate — we cannot make it depend on a crate of ours, so
there is no third home both sides can import. The mirror is not a
duplication we chose; it is the shape of a boundary with an upstream we
do not control, and `input.rs` staying toolkit-free is the thing worth
protecting.

**Why it is not needed.** `egui::PointerButton` is a **plain closed
enum** — no `#[non_exhaustive]`
(`egui-0.36.1/src/data/input/pointer_button.rs:1-20`). So a downstream
exhaustive `match` over it compiles *and is forced*: the day a future
egui adds a sixth button, our build fails at that one site.

That is the whole fix. Replace the hand-written three-pair array at
`crates/viewer/src/pane/viewport.rs:84-88` with a conversion whose match
names all five variants and says what each maps to — `Extra1` and
`Extra2` to a written "not bound, because …", or to new
`input::PointerButton` variants if binding the side buttons is wanted.
Nothing is shared, `input.rs` keeps its zero toolkit dependencies, and
the mirror becomes compiler-held.

**This is the same distinction the mirror class turned on today.**
`topo::ContactClass` is `#[non_exhaustive]`, so downstream *cannot*
enumerate it and the declaring crate has to publish an `ALL`;
`topo::BooleanOp` is closed, so a consumer's own exhaustive match fences
that consumer. `egui::PointerButton` is the `BooleanOp` case, one crate
boundary further out — with the difference that we cannot add an `ALL`
upstream even if we wanted to, which is exactly why the match is the
only lever and happily also a sufficient one.

**What it does not buy.** It is an *upgrade-time* guard, not a runtime
one: it fires when the egui version bumps, which is the right moment and
the only moment the set can change. It also does not decide the open
product question — whether `Extra1`/`Extra2` should bind to anything —
it forces someone to answer it in writing instead of by omission.

## Closed (2026-09-12, branch `door/pointer-button-match`)

**The title is wrong and stays wrong for the record.** It says
"nothing forces either side", which reads as a latent risk. The
defect was not latent: the adapter was already three-of-five against
`egui::PointerButton`, and the click half of it was one-of-five. What
this row actually was is the 2026-09-12 correction's wording — *an
adapter silently incomplete against its upstream, with no statement
either way* — and that is what is fixed.

**The adapter is now compiler-held.** `pane/viewport.rs`'s
`viewer_button` is an exhaustive match naming all five
`egui::PointerButton`s, and `egui_buttons` returns
`[egui::PointerButton; egui::NUM_POINTER_BUTTONS]`. An egui that grows
a sixth button fails to compile twice, at that one pair of sites: the
match goes non-exhaustive (E0004) and the array's length stops
matching the toolkit's own count (E0308). Demonstrated by compiling
the shape with a sixth variant added; both errors fire.

**And the list is a PERMUTATION of the enum, not merely the right
length.** The first fix left `egui_buttons`'s *contents* held by
nothing — `[Primary; NUM_POINTER_BUTTONS]` compiles — which is this
program's own class, re-minted inside the fix that closes an instance
of it (caught by the orchestrator, 2026-09-12). The row
`the_toolkits_buttons_are_each_asked_exactly_once` asserts the entries
are pairwise distinct; distinct, `NUM_POINTER_BUTTONS` long, and an
enum of exactly that many variants (which `viewer_button`'s exhaustive
match holds) compose to *every button exactly once*. Red-checked by
doubling an entry: that row alone reds, and the three behavioural rows
all pass — which is the proof that length was not membership.

**`Extra1`/`Extra2` are NOT BOUND, and the reason is written at the
site.** The alternative — two new `input::PointerButton` variants —
was rejected on three counts, argued in the PR: nothing could bind
them (`InputMap`'s four binding fields are filled by the three main
buttons, `PRESETS` is a code-level registry of one, and no preferences
key, Python binding or `pncad.pyi` entry names a button at all); they
would change no behaviour, since `map` and `pick` drop an unbound
button's events either way; and they would put the toolkit's taxonomy
inside the module whose declared kind is *vocabulary — it names no
driver type*, re-minting
`mate-primitives-is-a-partial-mirror-with-no-growth-alarm`'s shape one
crate further in, across every suite that spells the three buttons.

**The click half was the behaviour bug.** `clicked_by` was called with
a literal `Primary` and pushed a literal `PointerButton::Primary`, so
`InputMap::select_button` — a BINDING, read as a variable by
`InputMap::pick` — could name only one button before selection died
silently, and `pick`'s documented "a click on a button that is not
`select_button`" arm was unreachable in production. **Reachable, not
hypothetical**: `InputMap` is `pub` with `pub` fields and re-exported
at `crates/viewer/src/lib.rs:144`, so an embedder could already write
`select_button: Middle` and get a viewer that selected nothing. Clicks
now come through the same conversion. Three unit rows in
`pane::viewport::tests` hold it, two of them red before the change:
`every_toolkit_button_the_adapter_binds_produces_its_click` (left `[]`,
right `[Click { button: Secondary, … }]`) and
`a_click_selects_through_whichever_button_the_map_binds`.

**No README amendment, and none is owed.** The new list is sized by
the toolkit's own constant rather than by hand, and it is not a `const`
or `static` item, so `viewer-vocab-declared-once.sh`'s roster of
hand-written lists neither sees it nor wants a row for it.

**The pairing is held too, and separately.** Everything above holds
the SET; which toolkit button denotes which of the viewer's is a
naming decision with nothing to derive it from, and every behavioural
row asks `viewer_button` what to expect, so swapping two of its arms
left the whole suite green (the reviewer demonstrated it, and
demonstrated that a docstring of mine claimed otherwise).
`the_pairing_is_the_one_this_module_intends` states the table a second
time; both copies are exhaustive, so neither can fall behind the
toolkit while the other moves. The doc at the site no longer claims
the compiler holds the mapping, because it does not.

**Residue, filed rather than disclosed here:**
`work/view/viewport-adapter-drops-part-of-two-toolkit-values.md` — the
same function reads two of `egui::Modifiers`' five fields
(`viewport.rs:195`) and one of `smooth_scroll_delta`'s two axes
(`viewport.rs:209`), neither stated. One row, because they are one
decision at two adjacent reads. VIEW's ground, not this program's.
