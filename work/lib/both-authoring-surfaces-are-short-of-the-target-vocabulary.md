---
id: both-authoring-surfaces-are-short-of-the-target-vocabulary
kind: issue
title: A CLASS - the target vocabulary is spelled short in three places across the GUI and the Python binding, so neither can author the seam's declared tangent arrival
status: open
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
