---
id: no-instrument-guards-a-public-method
kind: issue
title: A CLASS - the surface censuses guard exported NAMES and declared MEMBERS, and nothing in the tree guards a public METHOD: every method added to or removed from a carried type is invisible to both
status: open
opened: 2026-09-12
refs: [2487]
priority: P3
cost: D
---


## Finding

Filed by WIRE's `linear-door` lane (PR 2487), on META's slate because it
is a gap in the tree's own guard apparatus rather than in any kernel
crate, and META already owns the sibling class
(`doc-citations-no-gate-checks-rot-silently`).

PR 2487 deleted a **public method** — `Frame::linear<T>` from
`crates/editor-core/src/placement.rs` — from a type that is exported at
`editor_core`'s root, carried on `pncad::document`, and bound in Python.
The brief expected one of the two façade censuses to have an opinion.
**Neither did, and neither could.** Both ran and both passed; the
removal was invisible to them. What caught it was the compiler, which
catches a deletion and would catch nothing about an ADDITION.

## Why: what each census's alphabet actually contains

- **Rust.** `crates/pncad/tests/all.rs`'s
  `every_document_layer_root_export_is_carried_or_listed` builds its
  export set with `module_pub_use_names` over
  `crates/editor-core/src/lib.rs` — the LEAF NAMES of that root's
  `pub use` statements. A method is not a leaf name of a `pub use`, so
  it never enters the set.
- **Python.** `crates/pncad-py/tests/test_binding_census.py` has two.
  `test_every_curated_name_is_bound_or_listed` compares curated façade
  NAMES against `pncad.pyi`'s top-level declarations — again names, not
  methods. `test_every_member_of_a_matched_type_is_spelled_or_listed`
  goes one level in, and is the one that looks like it should catch
  this, but its Rust-side member list comes from `declared_members` in
  `scripts/payload-rung-sweep.py`, whose own doc says what it reads:
  *"An enum's VARIANT names and a struct's bare-`pub` FIELD names — the
  two things a consumer of the declaration can name one level in."* It
  parses the declaration BODY and **never reads an `impl` block**.

`Frame` is the clean demonstration. Its two `pub` fields (`columns`,
`translation`) ARE guarded by the member census. Its nine public methods
— `translation`, `rotate_then_translate`, `from_affine`, `is_finite`,
`determinant`, `affine`, `compose`, `is_identity_bits`, `bit_eq` — and
its `IDENTITY` associated constant are guarded by nothing, which is why
a public one could be deleted with both censuses green.

## Why it is a class and not this PR's footnote

The asymmetric case is the expensive one. A method DELETED is caught by
the compiler at every call site. A method **added** to a carried type is
caught by nothing at all: it lands on the public Rust surface, no
census names it, no `NOT_BOUND` entry is owed for it, and the Python
side is never made to decide whether to bind it or to say why not.
That is exactly the drift the two censuses exist to stop, one level
below where they look, and it accumulates silently — which is how the
assembly, checks, picking, expression-read and content-pin families
accumulated before the name-level census was written.

## Where the work would go

The blind spot is TEXT-REACHABLE on both sides, like the two the Rust
census already lists: `impl` blocks for a carried type are in the same
source the scanners already parse, and `payload-rung-sweep.py` already
has the chunk-splitting machinery an `impl` body would need. That is
scanner work in the two census files plus the shared script, not a
toolchain. What it needs first is a decision about the bar: every `pub
fn` in every `impl` of a carried type is a large alphabet, and whether
a `NOT_BOUND`-shaped exclusion list is the right price is the question
this row asks.

Cited as a fourth blind spot from the doc comment on
`every_document_layer_root_export_is_carried_or_listed` in
`crates/pncad/tests/all.rs`, so a reader of that guard meets it there.
