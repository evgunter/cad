---
id: both-authoring-surfaces-are-short-of-the-target-vocabulary
kind: issue
title: A CLASS - the target vocabulary is spelled short in three places across the GUI and the Python binding, so neither can author the seam's declared tangent arrival
status: closed
closed: 2026-09-17
opened: 2026-09-12
---



(WIRE orchestrator, 2026-09-12) D364's lane reported the `pncad-py`
half from its `Target::Start` sweep (PR 2445). **The review lane found
the third instance inside that same sweep's own output** — the sweep
returned 55 hits, nine of them `viewer`'s, and the disposition listed
five and named no viewer site. Filed here as the class, because a row
naming only the binding is the half-fix the class-not-instance rule is
about, and because this row's first draft made exactly that mistake.

## The vocabulary

`profile::Target` has three forms — `Point`, `Start` and
`StartArriving`, the last ruled by Ev in-chat 2026-09-02 as the seam's
DECLARED tangent joint: *"the seam is the one junction whose arriving
leg is the later-authored one, so the declaration that elsewhere rides
the departing leg rides the target here"*
(`crates/profile/src/path/program.rs:116-135`). The kernel CHECKS the
arriving direction against `Start`'s own and refuses a seam that
contradicts it, so the declared and undeclared closes are two different
things, not a convenience pair.

## Six spellings, three of them short

| spelling | site | forms | forced? |
|---|---|---|---|
| `profile::Target` | `profile/src/path/program.rs:116` | 3 | the vocabulary itself |
| `ProgramTarget` | `editor-core/src/program.rs:49` | 3 | exhaustive matches |
| `WireTarget` | `editor-core/src/persist/wire.rs:216` | 3 | forced both ways (`from_target`/`into_target`, `:231-244`) |
| **`PathTarget`** | **`viewer/src/sketch.rs:57`** | **2** | **nothing** |
| **`PyTarget`** | **`pncad-py/src/py/path.rs:225`** | **2** | **nothing** |
| **`Tgt`** | **`pncad-py/src/py/path.rs:255`** | **2** | **nothing** |

Each short spelling is lowered by an exhaustive match over ITS OWN
enum — `viewer`'s `program_target` (`sketch.rs:369-373`), `Tgt::of`
(`path.rs:261-267`) — so all three compile happily and will keep
compiling however the kernel vocabulary grows. `grep -rn StartArriving
crates/viewer/ crates/pncad-py/` returns nothing.

The consequence is not "less convenient": **neither the GUI nor Python
can author the declared tangent arrival at all.** `PathTarget`'s own
doc even explains the `Start`/close distinction carefully
(`sketch.rs:51-54`) while carrying two of the three forms that
distinction produced.

## The part that is a question, not a task

Carrying the form through each surface is mechanical (a variant, a
lowering arm, the `.pyi`). What is not mechanical is whether the
omission is deliberate — it may be that the authoring surfaces
deliberately offer the undeclared close only, and the seam declaration
is a typed-surface affair. **Nothing at any of the three sites says
so**, which is this row's real subject: an absence with no statement is
indistinguishable from an oversight, and this one has survived a
ruling. Answer that first; the code follows from it either way.

## The instrument to close it now exists

`crates/pncad-py/src/surface_census.rs:598-619`'s
`every_arc_mode_has_a_python_spelling` already keys on `ArcMode::ALL`
and asserts both that `pncad.pyi` declares a class per mode and that
each class appears in some signature. D364 (PR 2445) gives
`profile::Target` a `TargetKind` with an `ALL` projected from the
variant declaration, which makes the exact analogue writable for the
first time — and the same anchor serves `viewer`'s side. Before that
`ALL` existed there was nothing to key such a census on, which is why
all three lists drifted unobserved.

## Owners

`pncad-py` is LIB's (DOCM's `keep_out`: *"LIB keeps the pncad facade
and bindings"*), `crates/viewer/src/sketch.rs` is VIEW's or CHROME's.
The row is filed whole rather than split so the ruling is asked once;
whoever takes it should expect to hand the other half across.

## The table is five spellings, not six (2026-09-16, EDIT's PR #2738)

`WireTarget` no longer exists. EDIT's `C6` `WireStep` unit collapsed
`editor-core`'s persisted step vocabulary into its document one:
`WireStep`, `WireTarget`, `WireArcData` and `WireLoopProgram` are
deleted, `from_target`/`into_target` with them, and
`ProgramTarget` now derives serde on its own declaration — so the row's
third line is gone and the `forced?` column's answer for it was always
"by two matches that no longer exist".

**Nothing about this row's subject moved.** The deleted spelling was
one of the three that carried all three forms, so the count of SHORT
spellings is unchanged at three (`PathTarget`, `PyTarget`, `Tgt`),
and neither the GUI nor Python can author the declared tangent arrival
today any more than before. What changed is the denominator: the
vocabulary is now spelled five times, three of them short.

The census anchor the row asks for is in place on the `editor-core`
side and unaffected: `ProgramTarget::ALL_NAMES` and
`profile::TargetKind::ALL` both still project from their declarations,
and `editor-core/tests/switch_program_vocabulary.rs` keys
`every_target_form_is_a_document_program` on the latter. The analogue
this row wants written for `viewer` and `pncad-py` is still unwritten.

`viewer`'s half has since been filed as its own row on CHROME's slate
(`viewer-sketch-claims-a-compile-break-a-probe-verb-does-not-cause`),
which names the same `Tgt::of`/`program_target` shape from a different
angle — a false claim in `sketch.rs`'s prose that the lowering breaks
when the document vocabulary grows. That row is the instance; this one
is still the class and the ruling.

## The ruling, and the viewer half (2026-09-17)

**Ev, in chat, 2026-09-17: the omission was not deliberate** ("indeed
it was not deliberate to leave out declared arrival"). That answers
this row's question. The authoring surfaces are meant to carry the
declared tangent arrival, so what is left is carrying it.

The viewer half is closed on branch `viewer/path-form-uses-kernel-step`.
`PathTarget` is deleted, and the path form edits the kernel's own
`Target<f64>` through a control that walks `TargetKind::ALL`, so
`StartArriving` is offered at every target and the lattice refuses the
verb and mode rows that do not take it. **Two short spellings remain,
both in `pncad-py`** (`PyTarget`, `Tgt`), and this row now owns only
those.

## Closed (2026-09-17)

The pncad-py half is closed on branch `lib/py-declared-arrival`.
`Start.arrives_tangent()` is now a Python value, typed
`ArrivesTangentToken`, and it spells the same thing as in Rust.
`line_to`, `tangent_arc_to` and `arc_to(Bulge(...))` take it, which
are exactly the closers the kernel's `ArrivesTangent` serves. `Via`
and `Center` refuse it with a TypeError at construction, mirroring the
missing trait impls in Rust; lifting that is
`work/paths/via-and-center-arc-closers-declared-arrival.md`.
`PyTarget` now carries all three forms. The binding still spells the
vocabulary in its own enums, and there are four of them now, on
purpose: `PyTarget`/`BulgeTgt` carry three forms and
`ThroughTarget`/`Tgt` carry two, because `Via` and `Center` do not
take the declaration (each pair is an extraction enum and its lowered
twin).

The census this row asked for now exists, and here is what it forces.
`surface_census.rs`'s `every_target_form_has_a_python_spelling` keys
on `TargetKind::ALL` through an exhaustive `target_class` match. So a
new kernel form fails to compile until it is given a Python class
name, and fails the test until the stub declares that class and some
signature accepts it. It does NOT force the binding's own enums: those
are still hand-written, and a form could reach the stub while
`PyTarget` refuses it at runtime. The runtime half is guarded only by
`tests/test_paths.py`. While
writing it, the census found a blind spot in the stub scanner that is
fixed here too: `Stub::defs` kept only the FIRST `@overload` of each
`def`, so the census never read any type accepted only by a later
overload, including every closing overload of `line_to`,
`tangent_arc_to` and `arc_to`.

`continue_to` is still unbound in Python. That is a verb gap, not a
target gap, and the census already records it as `NotBound`.
