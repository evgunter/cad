---
id: a-count-slots-cast-still-saturates-for-a-finite-value-too-large
kind: issue
title: A finite Count value outside i64 still saturates to i64::MAX, and there is no refusal vocabulary for it
status: parked
opened: 2026-09-21
priority: P2
cost: E
refs: [need-count-spells-every-failure-as-a-pattern-count, a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it]
blocked_on: [need-count-spells-every-failure-as-a-pattern-count]
---


## Finding

Found by the refusal-floor sweep while closing
`a-count-slot-launders-a-typed-nan-into-zero`. That row's fix makes
`props::SlotValue::of`'s `Count` arm refuse a value that is not
finite, by the name `Expr::literal` uses for the continuous half
(`DimensionError::NonFiniteLiteral`). **The other half of the same
cast is untouched**: `value as i64` saturates for a finite value
outside `i64`'s range too, and that arm still commits a number nobody
typed.

`crates/viewer/src/props.rs`, `SlotValue::of`'s `Count` arm — the
`value as i64`.

Executed under `rustc -O`:

| typed | committed |
|---|---|
| `1e30` | `9223372036854775807` (`i64::MAX`) |
| `-1e30` | `-9223372036854775808` (`i64::MIN`) |
| `9.3e18` | `9223372036854775807` |
| `f64::MAX` | `9223372036854775807` |

The route is the one the parent row establishes and is unchanged by
its fix: `props::field_edit` reads anything `f64::from_str` accepts as
a `FieldEdit::Number`, `crates/viewer/src/widgets.rs`'s typed arm
hands it to `SlotValue::of`, and `crates/viewer/src/pane/properties.rs`'s
`slot_field` sets no `egui::DragValue` range, so `1e30` is a value the
field accepts. The four Count-dimensioned slots are `SlotId::Count`,
`VDegree`, `Stations` and `Instance`
(`crates/editor-core/src/node.rs`, `SlotId::dimension`'s `Count` arm).

## Why it was not fixed with the parent row

**There is no refusal to raise.** The parent's fix works because
`DimensionError::NonFiniteLiteral` already exists and already says the
right sentence — *a literal value must be finite* — so the viewer
raises the kernel's own word rather than minting one. For "this count
does not fit an `i64`" there is no variant, in `DimensionError` or
anywhere else the viewer may raise: `crates/editor-core` is EDIT's and
MSOLVE's, and `work/vgeom/program.md`'s `keep_out` says *"a numeric
door the viewer consumes is a hand-off and never a diff from here."*

So this needs a decision before it needs a diff, and the fork is:

- **a door in `editor-core`** — a `DimensionError` (or `Expr::count`)
  arm for a count outside the representable range, which is where the
  continuous half's finiteness refusal already lives; or
- **a refusal of the viewer's own**, in `session::Refusal`, which is
  VNEWS's vocabulary and would be the first numeric refusal the viewer
  words for itself.

Whichever is taken, the shape at the door is the parent's: `of`
already returns a `Result` and the call sites already take it.

## Not established here

Whether `Expr::count` or the document below it bounds a count of
`i64::MAX` anyway. If it does, the user still gets a refusal about the
wrong thing — a count out of range rather than a number that was never
a count — which is the parent row's own "Not established here",
unmeasured for the same reason.

## Fence

`crates/viewer/src/props.rs` — this program's, with the decision half
reaching `crates/editor-core` (EDIT's and MSOLVE's) or `session/`
(VNEWS's and VSEAM's).

## The fork re-derived against the tree, 2026-09-21 (VGEOM orchestrator)

Held out of the 2026-09-21 wave because it needs a decision before a
diff. The register's rule is that an item's own menu of options is a
claim like any other, and that costing it against the tree of TODAY
rather than against the tree it was written from is what a fork costs
— skipping it is how a ruling gets made on a question nobody still
has. Done here, and **it moved the question in three places.**

### 1. "Not established here" is now established: nothing bounds it

The row asks whether `Expr::count` or the document below it bounds a
count of `i64::MAX` anyway. **Neither does.**

- `crates/editor-core/src/expr.rs`'s `Expr::count(value: i64)` is
  **total** — it wraps the `i64` in an `ExprKind::CountLiteral` and
  checks nothing. There is no `Result` and no guard.
- `crates/editor-core/src/eval/wire.rs`'s pattern node guards `n < 1`
  and then runs `for j in 1..n`, allocating and placing
  `master.len()` bodies per iteration. Its only ceiling is
  `names::output_body(...)` **inside** the loop, so it refuses after
  roughly `u32::MAX` placements rather than before the first — and
  `eval/mod.rs` says the cancel token is *"checked BETWEEN nodes"*, so
  the loop is not cancellable either.

So the row's worry — that a user might get "a refusal about the wrong
thing, a count out of range rather than a number that was never a
count" — does not arise, because there is no refusal about either.
Filed as `work/wire/a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it`
(WIRE's ground by `work.py territory --files -`, not EDIT's as this
row guessed).

### 2. The fork's premise — "there is no refusal to raise" — is half wrong

There is no variant that says *this count does not fit*. But the tree
does not lack the SENTENCE; it lacks a place to put it, and the nearest
variant is already being made to carry it:

`crates/editor-core/src/eval/wire.rs`'s `need_count` maps **every**
failure of `usize::try_from(n)` onto `NodeErrorKind::NonPositiveCount`,
whose `Display` is *"pattern count {count} is not at least 1"* — and
`need_count`'s only call sites are a loft's `SlotId::VDegree` and a
sweep's `SlotId::Stations`, never a pattern count. On `wasm32`, where
`usize` is 32 bits and which this workspace builds, a V-degree of `5e9`
is reported as *"pattern count 5000000000 is not at least 1"*: wrong
slot, wrong fault, false about the number in front of the reader. Filed
as `work/wire/need-count-spells-every-failure-as-a-pattern-count`.

**That is the same refusal this row wants minted**, one crate over and
already load-bearing. So the two are one decision, not two.

### 3. Which reshapes the fork

The row offers (a) a door in `editor-core` or (b) a first numeric
`session::Refusal` of the viewer's own. **(b) is now clearly the worse
of the two**, and not for the reason the row gives: a viewer-side
refusal would be a SECOND spelling of a fault the kernel is already
trying to spell and getting wrong, so it would leave two wrong words
where there is one. (a) is also the shape the parent row's landed fix
already uses — the viewer raises `DimensionError::NonFiniteLiteral`,
the kernel's own word, rather than minting one.

**This is a re-derivation, not a ruling.** The siting decision is still
WIRE's and EDIT's to take, and it is still owed before this row has a
diff. What changed is that it is now one decision across three rows
with the evidence in each, rather than a VGEOM question about a
vocabulary nobody had looked for.

**What was NOT checked**, so the next reader does not inherit a
negative result stronger than the search behind it: no outer budget or
timeout above `crates/editor-core/src/eval/` was looked for (the eval
service in `crates/viewer/src/session/` is VSEAM's), and nothing here
was driven end to end through the GUI. The chrome-side chain
(`props::SlotValue::of`'s saturating cast, `slot_field`'s absent range)
is the parent row's and was established there.

## Parked on WIRE's siting decision (2026-09-22, VGEOM orchestrator)

Taken up in the sitting that dispatched the rest of this program's
slate, and **parked rather than dispatched, because the diff this row
wants is one this program may not write.**

The re-derivation above establishes the fork's answer — (a), a door in
`crates/editor-core`, because a viewer-side `session::Refusal` would
be a second spelling of a fault the kernel is already trying to spell
and getting wrong. What it does not establish is a right to write it.
`work/vgeom/program.md`'s `keep_out` is explicit: *"a numeric door the
viewer consumes is a hand-off and never a diff from here"*, and
`crates/editor-core` is EDIT's and MSOLVE's. So VGEOM can hold the
evidence and cannot mint the variant.

**Nor is there a VGEOM half that lands alone.** The chrome-side repair
is to stop `props::SlotValue::of`'s `Count` arm saturating — and to
refuse instead of saturating, the arm needs a word. The parent row's
fix works precisely because `DimensionError::NonFiniteLiteral` already
existed. For *this count does not fit* there is no variant anywhere
the viewer may raise, so a chrome-side diff today would either invent
the viewer's first numeric refusal (option (b), re-derived as the
worse of the two) or go on saturating. Neither is worth a lane.

**Parked on `need-count-spells-every-failure-as-a-pattern-count`**,
which is the same decision one crate over and already load-bearing:
`eval/wire.rs`'s `need_count` maps every failure of
`usize::try_from(n)` onto `NodeErrorKind::NonPositiveCount`, so on
`wasm32` a V-degree of `5e9` is already reported as *"pattern count
5000000000 is not at least 1"*. The variant that row needs minted is
the variant this row needs raised. When WIRE and EDIT site it, this
row is a small chrome-side diff and nothing else.

**Announced to WIRE rather than left to be found**: the trigger is
named in `blocked_on`, and this note says what VGEOM will do when it
fires. Nothing here asks WIRE to schedule it.
