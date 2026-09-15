---
id: load-door-is-the-construction-door-for-expressions-and-not-for-profile-programs
kind: issue
title: persist/wire.rs carries two rebuild policies and states one: expressions rebuild through their constructors, profile programs do not
status: open
opened: 2026-09-15
---



## Found by

PORT-DIMS-1's reachability sweep (2026-09-15), asking `tags.rs`'s
`persist_error_tag`, `edit_error_tag` and `path_error_tag` the question
#689's false premise made worth asking: *is this refusal reachable from
the LOAD path as well as the authoring path?*

## The finding

`crates/editor-core/src/persist/wire.rs` rebuilds two kinds of thing
from the wire, under two different policies, and its prose states the
first as though it covered both.

- `WireExpr::rebuild` and `WireMeasureExpr::rebuild` call the **smart
  constructors** — `Expr::add`, `Expr::sin`, `MeasureExpr::mul` and the
  rest — so every refusal those constructors can raise is reachable
  from `load`. The measure rebuild says so in its own words: *"the load
  door is the construction door, so a file cannot carry a tree the
  authoring API refuses."*
- `<ProfileProgram as Deserialize>::deserialize`, in the same file,
  is a **structural mapping**. `WireLoopProgram::Chain(steps)` becomes
  `LoopProgram::Chain(steps.into_iter().map(WireStep::into_step))` and
  nothing fallible is called. No path constructor runs, so no
  `PathError` is reachable from `load` at all — which is why
  `path_error_tag` has exactly one caller
  (`crates/pncad-py/src/py/path.rs`) and it is the authoring door.

That second policy is not by itself wrong: profile programs are
re-checked afterwards by the shared validator, and
`persist::check::ProgramFault`'s `Lattice` arm records the decision
explicitly — *"geometry refusals and resolve failures deliberately PASS
this door: they are V1 class 2, legal at rest, surfaced as typed node
errors at evaluation."* Two of `ProgramRefusal`'s four arms are
accounted there.

## What is actually open

**Two things, and the first is the cheap one.**

1. The module's prose reads as a general claim about the load door and
   is true of one of its two rebuild families. A reader who takes it
   generally will conclude — as #689 concluded about the authoring
   doors, in the opposite direction — that a refusal is reachable from
   `load` when it is not, or the reverse. The fix is a sentence beside
   `ProfileProgram`'s impl saying that this one trusts the structure
   and names the validator that re-checks it.

2. `ProgramRefusal::Validate(profile::ProfileError)` — the fourth arm —
   has **no stated disposition**. `ProgramFault` has two arms,
   `SlotDimension` and `Lattice`; the doc quoted above accounts
   `Geometry` and `Resolve`; nothing accounts `Validate`. Either a
   hand-written file can carry a program whose replay would fail
   `Profile::validate` and load clean (and the refusal arrives later,
   at evaluation, which may well be the intended answer), or it cannot
   and the reason is unwritten. This lane did not settle which; the
   probe that would is a save whose loop program is a bowtie.

## Where to look

- `crates/editor-core/src/persist/wire.rs` — `WireExpr::rebuild`,
  `WireMeasureExpr::rebuild`, `impl Deserialize for ProfileProgram`.
- `crates/editor-core/src/persist/check.rs` — `ProgramFault` and its
  `Lattice` doc, the replay probe.
- `crates/editor-core/src/program.rs` — `ProgramRefusal`'s four arms.
- `crates/pncad-py/src/tags.rs` — `path_error_tag`; its one caller is
  `crates/pncad-py/src/py/path.rs`.
