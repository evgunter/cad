---
id: field-censuses-inside-view-survived-the-debug-sweep
kind: issue
title: three hand-listed field censuses inside crates/viewer/src, one of them in the file this sweep edited, are the same class and were not swept
status: open
opened: 2026-09-06
refs: [2093]
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
