---
id: python-dimensionerror-names-the-quantity-check-not-the-dimension-check
kind: issue
title: Python's DimensionError names the quantity-operator check while the real dimension checker surfaces as LiteralError and PersistError
status: open
opened: 2026-09-15
refs: [S107, 694, 689]
priority: P3
cost: E
---


## Ruled a defect (Ev, in-chat, 2026-09-15)

`S107` asked whether the Python-visible confusion left by the
`DimensionError` untangling is a defect or a deliberate compatibility
choice. Ev ruled **defect**, on the general principle: *Python should
always match Rust where it can.* This row is the work that ruling
releases; `S107` closes carrying the verdict.

## The defect

The untangling renamed the Rust type only. `QuantityOpMismatch`
(`crates/pncad-py/src/errors.rs`) is the quantity boundary's
operator check, and it is published to Python as the class
**`DimensionError`** (`ErrorClass::Dimension => "DimensionError"`,
`errors.rs`; the registration in `py/mod.rs`). Meanwhile the document
layer's own `DimensionError` — the type that actually checks
dimensions — reaches Python as `LiteralError` on the
literal-construction door and as `PersistError` with
`variant == "parse"` on the load door.

So from Python the name `DimensionError` denotes the thing that is not
the dimension checker, and the dimension checker denotes under two
other names. Rust has no such ambiguity; only the binding does, which
is exactly the case Ev's principle covers.

The compatibility defence the ruling rejects had no installed base
to protect: `Cargo.toml` carries `publish = false` and the project has
no name yet (Q9), so nothing downstream catches the class today.

## What the fix touches

The rename itself is small; the **re-documentation is the bulk**. Every
site below exists to reconcile the two spellings, which is Q2's shape
in `docs/prompts/reviewer-style-lane.md` — a comment doing work the
code should do — and the ruling licenses deleting them rather than
maintaining them:

- `crates/pncad-py/src/errors.rs` — the class-name mapping, the
  doc-comment on `QuantityOpMismatch` explaining that the class is
  called something else, and the module-header paragraph.
- `crates/pncad-py/src/py/mod.rs` — the class registration and the two
  docstrings that disclaim `DimensionError`.
- `crates/pncad-py/src/py/analysis.rs`, `py/measure.rs`,
  `py/quantity.rs`, `py/doc.rs`, `py/expr.rs` — the door docstrings
  that name the class.
- `crates/pncad-py/pncad.pyi` — the class declaration and the prose
  that distinguishes it from the document layer's type.
- `crates/pncad-py/src/tests.rs` and the Python suite's stub tests.

Cite by name; the line numbers in `S107`'s body were taken on
2026-08-20 and are not re-taken here.

## The shape question the unit decides

Renaming the Python class to `QuantityOpMismatch` frees the name
`DimensionError`. **Whether the document layer's real `DimensionError`
should then take it is not ruled** — Ev ruled the mismatch a defect, not
what the vacated name is used for. Two readings, and the unit picks one
with its reasoning in the PR:

1. Leave the name vacant. The document layer's refusals keep reaching
   Python as `LiteralError` and `PersistError`, and the defect closed is
   only the lie.
2. Give it to the document layer's type, so Python's `DimensionError`
   means what Rust's does. This is the same principle applied twice and
   is the reading this row prefers — but it is a larger change, and it
   is **entangled with `load-path-stringifies-structured-refusals`**:
   that row destringifies the document layer's refusals at the load
   door, and the class they should arrive as is this decision. The two
   rows are one door and should be specced together.

## Home

PORT announces; `crates/pncad-py/*` is **LIB's** territory and LIB may
take this row instead (`work/port/program.md`'s `keep_out`). The
kernel-side type is not edited by this work.
