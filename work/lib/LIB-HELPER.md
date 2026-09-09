---
id: LIB-HELPER
kind: unit
title: Expr::length_in and angle_in, mirrored on both sides: one call at every authored number
status: review
branch: lib/helper
opened: 2026-09-09
refs: [expr-seat-costs-a-constructor-call-at-every-authored-number]
pr: 0
---

Closes `expr-seat-costs-a-constructor-call-at-every-authored-number`
under Ev's ruling **(A)** on `[ev]` PR #2266: one composition,
mirrored on both sides, no new type and no new seat.

## Delivered

- **The two Rust doors**, `crates/editor-core/src/expr.rs`, beside
  `written_length` / `written_angle`:
  `Expr::length_in(value: f64, unit: LengthUnit)` is exactly
  `Self::written_length(quantity::WrittenLength::in_unit(value,
  unit))`, and `Expr::angle_in(value: f64, unit: AngleUnit)` is the
  angle twin. One line each, no refusal of their own, no type of their
  own; the rustdoc says it is the composition and names both halves.
- **The two Python mirrors**, `crates/pncad-py/src/py/expr.rs`:
  `Expr.length_in(value, unit)` and `Expr.angle_in(value, unit)` as
  static methods, refusing through `literal_err` — the same
  `LiteralError` path `written_length` raises, carrying the kernel's
  tag and the offending number. Declared in `pncad.pyi`; pinned
  name-for-name by `test_stubs.test_class_attributes_agree_name_for_
  name`, and accounted by the census under its rule 1 (a member the
  Python namesake spells) with no roster edit.
- **974 sites converted**, by a scanner over the exact call shape
  `Expr.written_length(WrittenLength.in_unit(…))` and its angle twin
  with balanced-paren argument capture: 29 test modules, both ty
  fixtures, `crates/pncad-py/README.md`, `docs/GUIDE.md` and four
  guide sub-pages. Three Rust sites too — the tour's `ring.rs` `mm`
  closure and its full-turn revolve, and `diefillet.rs`'s `ang`.
- **The private shorthands are gone**: `_wm(value)` in
  `test_north_star.py` and `test_resolve.py` (19 calls inlined — the
  helper was a one-call wrapper once the composition became one call),
  and the `_0M` / `_1M` pair in `test_monte_carlo.py`,
  `test_face_frame.py` and `test_measures.py`. `SQUARE` STAYS in those
  three: it names the unit square several assertions refer to, which
  is a fixture rather than a spelling shorthand, and the constant that
  existed only for the spelling is the pair it was built from.
- **Stale imports pruned with the sites**: `WrittenLength` /
  `WrittenAngle` left 36 test modules' import lists (ruff F401) and 39
  entries left the guide's and README's executed blocks, where the
  name is now unused. Both types stay bound and stay curated.
- **Pins.** Python `TestTheOneCallIsTheComposition`
  (`test_notation.py`): the helper equals the two calls for both
  dimensions, the saved file reads `"unit": "mm"` / `"value": 0.025`
  through it, and a NaN (length) and an infinity (angle) raise
  `LiteralError` with `kind == "non_finite"`. Rust
  `switch_display_units::the_authored_helpers_are_exactly_the_
  composition`: `bit_eq` against the hand composition for both, the
  stored symbol, the canonical bits, and both refusals.
- **The guide says what the helper is**: §1.3 gains a paragraph naming
  `Expr.length_in`, the composition it spells, and when to reach for
  the two doors underneath instead — the guide had no prose about the
  written door at all, and the sweep would otherwise have left the
  name nowhere in it.
- **`written_length` / `written_angle` are untouched** and stay the
  doors, on both sides; `crates/quantity` has zero diff. The kernel
  diff is the two functions, their doc and the one new test.
