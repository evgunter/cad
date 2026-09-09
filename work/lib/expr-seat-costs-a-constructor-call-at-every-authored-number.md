---
id: expr-seat-costs-a-constructor-call-at-every-authored-number
kind: issue
title: the Expr seat spells a written length in 55 characters, at 976 sites
status: open
opened: 2026-09-09
refs: [node-slot-literals-erase-the-authored-notation]
---



Ev's note (1) on the `(H)` ruling anticipated this and set its terms:
"if the seat wants to be more ergonomic, add helper functions on BOTH
the Rust and Python sides, mirrored, rather than a Python-only
convenience". LIB-SEATS built the seat and deliberately built no
helper; this is the measurement that says whether one is worth it.

## What it costs, measured

Every dimensioned slot now takes an `Expr`, so an authored number is
spelled through a constructor. The written form is the one the
conversion rule reaches for, and it is two nested calls:

```python
Node.extrude(profile, Expr.written_length(WrittenLength.in_unit(25, mm)))
```

55 characters where `25 * mm` was 7. After LIB-SEATS the tree holds
**904 `Expr.written_length`**, **72 `Expr.written_angle`**, **460
`Expr.literal`** and **41 `Expr.count`** call sites across the Python
tests, the guide's executed blocks, the README and the examples.

Where it bites hardest is a coordinate LIST, because the cost is per
COMPONENT rather than per call. A four-corner square was one line:

```python
Node.polygon([(0 * m, 0 * m), (side * m, 0 * m), (side * m, side * m), (0 * m, side * m)], plane=f)
```

and is now six, one per corner plus the brackets, each corner 110
characters wide. `crates/pncad-py/tests/test_shell.py:53` is that
diff. Three test files bind `SQUARE` to a module constant and two
define a local `_wm(value)` shorthand rather than repeat the spelling
inside a comprehension (`crates/pncad-py/tests/test_north_star.py`,
`test_resolve.py`) — which is the ergonomic pressure showing up as
private helpers in the corpus, and the evidence this row exists to
record.

## What a helper would have to be

Mirrored on both sides, Rust first, per Ev's note. The Rust side has
the same shape and does not feel it as sharply, because
`Expr::written_length(WrittenLength::in_unit(25.0, mm))` is written
once per authored number in a demo rather than 976 times in a corpus
— so the question a decision here has to answer is whether the RUST
door is wanted for its own sake. A `quantity`-side sugar that mints a
`WrittenLength` from a value and a unit already exists
(`WrittenLength::in_unit`); what is missing is the composition of
that with `Expr::written_length`, and its name.

Not decided here, and deliberately not built at LIB-SEATS: a Python
convenience alone is the thing the ruling refused.

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR)

Your note on (H) set the terms: a helper, if the seat wants one, goes
on BOTH sides, mirrored, never as a Python-only convenience. LIB-SEATS
built the seat and no helper, and measured the cost above: 55
characters where `25 * mm` was 7, at 976 written-form sites, and the
pressure already showing as private `_wm(value)` shorthands in two
test files and module constants in three. Is the mirrored helper
wanted, and what is it?

- **(A) One composition, mirrored: `Expr::length_in(25.0, MM)` /
  `Expr::angle_in(90.0, DEG)` in `editor-core` beside
  `Expr::written_length`, and `Expr.length_in(25, mm)` /
  `Expr.angle_in(90, deg)` in Python** — each exactly
  `written_length(WrittenLength::in_unit(v, unit))`, refusing through
  the same `DimensionError`; no new type, no new seat, one call at
  every authored number, the notation kept. The Rust door is honest on
  its own: the tour spells that composition by hand at
  `demos/tour/src/ring.rs:190` and `diefillet.rs:78`. Recommended;
  mechanical afterwards (the ~976 sites and the two private
  shorthands convert to it).
- **(B) Leave it.** The corpus keeps its private shorthands; the cost
  is recorded and accepted.
- **(C) A `quantity`-side sugar only** (`written(25.0, MM)` minting a
  `WrittenLength`) — shortens one of the two calls and leaves the
  other, so the site is still two calls.

Recommendation: **(A)**; names are yours to change (`length_in` /
`angle_in` mirror `WrittenLength::in_unit`'s own verb).

## Ruled (2026-09-09, Ev on `[ev]` PR #2266): (A)

One composition, mirrored on both sides: `Expr::length_in(value,
unit)` and `Expr::angle_in(value, unit)` in `editor-core` beside
`Expr::written_length`/`written_angle`, each exactly
`written_length(WrittenLength::in_unit(value, unit))` and refusing
through the same `DimensionError`; `Expr.length_in(value, unit)` and
`Expr.angle_in(value, unit)` in Python, the mirrors. No new type, no
new seat. The written-form sites and the corpus's private shorthands
convert to the one call. Mechanical unit LIB-HELPER.
