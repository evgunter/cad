---
id: LIB-SEATS
kind: unit
title: every dimensioned slot door takes an Expr, and Expr gains literal, written_length and written_angle
status: review
branch: lib/seats
opened: 2026-09-09
refs: [node-slot-literals-erase-the-authored-notation]
pr: 2264
---

Closes `node-slot-literals-erase-the-authored-notation` under Ev's
ruling **(H)** on `[ev]` PR #2233: each dimensioned slot door takes an
`Expr` and nothing else, as the Rust slot does, and `Expr` gains the
kernel's constructors.

## Delivered

- **The four constructors**, `crates/pncad-py/src/py/expr.rs`:
  `Expr.literal(Length | Angle | float)`, `Expr.written_length`,
  `Expr.written_angle`, `Expr.count(int)` — the kernel's four,
  mirrored. The argument's own type is the literal's dimension; there
  is no dimension to spell and no second fact to keep in step.
- **The refusal MOVED and stayed typed.** The `literal` helper left
  `py/doc.rs` for `py/expr.rs`, where it is the constructor's own
  body: `LiteralError` with the kernel's tag and the offending number,
  raised at `Expr.literal` / `written_length` / `written_angle`
  instead of at fifteen node doors. Pinned by
  `test_notation.TestANodeSlotRecordsTheAuthoredNotation`.
- **25 doors take an `Expr`** — the item's 18 plus seven its
  `literal(py, …)` pattern could not see: `Node.datum_frame`,
  `Node.datum_point` (both arrived on main after the item was
  written), the four count slots (`Node.loft`, `Node.pattern`,
  `Node.placed_union`, `PartSelect.instance`, which minted through
  `Expr::count` rather than the helper) and `Doc.sketch_frame`, the
  insert sugar that forwards to `Node.sketch_frame`.
- **The slot's dimension is READ, not restated.** `doc.rs`'s new
  `slot_expr` checks an argument against `SlotId::dimension()` — the
  kernel's own table — and refuses with the kernel's own
  `EditError::SlotDimensionMismatch`, the value `apply` would raise
  for the same expression in the same slot. So the boundary carries
  no second vocabulary for one fault, and the check cannot drift.
  `direction_expr` is its per-axis form over a `VectorSlot`.
- **Deviation, stated: `GeomPred.datum_distance` has no door-side
  check.** Its comparand is not a node slot, so there is no `SlotId`
  to read a dimension from, and the kernel already refuses a
  non-length at `select_where`
  (`SelectRefusal::NotALength`, `names/geompred.rs:407`). Adding a
  binding pre-check there would be the predicted refusal the crate's
  own rule forbids. Recorded in the door's rustdoc and in the stub.
- **Deviation, stated: `Node.sketch_frame` converts `elevation` and
  not `plane`.** A `SketchPlane` is a rigid VALUE — twelve floats
  with no authored notation — and `elevation` is the one authored
  number, which is the frame's origin z slot. It takes an `Expr`; the
  plane's other eight slots lower to canonical literals, which is the
  only remaining `literal(py, …)` in the binding and is deliberate.
- **~976 sites converted** across `crates/pncad-py/tests` (31 files),
  the guide's executed blocks, `crates/pncad-py/README.md`,
  `crates/pncad-py/examples/bracket.py` and both ty fixtures, by
  Ev's note (2): a site that spells a unit keeps its notation
  (`Expr.written_length(WrittenLength.in_unit(25, mm))`), a computed
  quantity takes `Expr.literal`.
- **The ty fixtures moved both ways**: a bare `Length` at a slot is an
  ILLEGAL row (`Node.extrude(solid, 1 * m)`), and the rows that used
  to draw a diagnostic for a DIMENSION mismatch no longer can — one
  type at the seat means the dimension is a door check, not a type
  check — so each says what it now says.
- **No helper, deliberately** (Ev's note 1). The ergonomic cost is
  filed with its site counts as
  `work/lib/expr-seat-costs-a-constructor-call-at-every-authored-number.md`.
- **A sibling the sweep found**, filed rather than taken:
  `work/lib/path-legs-erase-the-authored-notation-one-layer-down.md`
  — the paths vocabulary records `f64` through `RecordedProgram`
  before any `Expr` exists, which is Rust's own shape and so a kernel
  question.
- **Kernel untouched**: `crates/editor-core` and `crates/quantity`
  have zero diff.
