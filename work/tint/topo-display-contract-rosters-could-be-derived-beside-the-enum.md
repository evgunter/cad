---
id: topo-display-contract-rosters-could-be-derived-beside-the-enum
kind: issue
title: topo's two display-contract enums could carry a compiler-derived variant list beside the enum, as its own src rows already do
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Found by TINT-5's style review, reading `test_utils::f6`'s claim that
the variant list is not derivable, against what `topo` already does.

## The finding

`test_utils::f6::assert_f6_every_variant` used to state, unconditionally
and in the one home all its adopting sites point at, that closing its
coverage hole *"would need the variant list itself to be derivable,
which safe Rust does not offer without a macro or a derive over a type
the asserting crate does not own."* **That is false for one of the three
adopting crates, and that crate has the counter-example landed.**

`crates/topo/Cargo.toml` carries `strum` with `derive`, applied under
`cfg(test)` only, and `topo`'s own `src` rows already do exactly the
thing the sentence said was unavailable, twice:

- `crates/topo/src/euler.rs` derives `EnumDiscriminants` /
  `EnumCount` / `EnumIter` on `EulerOpError` to size
  `every_euler_op_error_once() -> [EulerOpError; <EulerOpErrorKind as strum::EnumCount>::COUNT]`
  in declaration order. `every_error_displays` in the same file is the
  F6 smoke test over that derived array — the same job as the
  display-contract rows, with the coverage hole closed by the compiler.
- `crates/topo/src/validate.rs` does the same for `ValidationError`.

The escape clause in that sentence did hold for `crates/mesh/tests` and
`crates/editor-core/tests`, which own neither their enums nor a strum
dependency. The sentence was stated without it.

## What it implies, and what changed under it

TINT-5's fix pass removed the sentence, because it removed the hole it
described: `test_utils::f6_variants!` now writes the `match` and the
identifier roster from ONE list of idents, so a variant added to the
enum stops the suite compiling, and writing it into the block is
writing it into the roster, which then reds until the variant has a
case. What survives as a residue is one step smaller: the idents are
still **transcribed** beside the enum's test suite rather than
**derived** from the enum. rustc checks the transcription in both
directions (`E0004` on a missing ident, `E0599` on a wrong one), so it
cannot be silently wrong — but for `topo`'s two display-contract enums
the transcription could be dropped entirely.

`work/wire/select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`
already says where such a row belongs: a unit test beside the enum,
inside the crate that owns it, where `#[non_exhaustive]` and the
foreign-crate limits do not apply. That is the same home this would
take. `ContactRefusal` lives in `crates/topo/src/contact.rs` and
`ReadbackError` in `crates/topo/src/readback.rs`; a derived array
there, walked by an F6 row, would replace the transcription in
`crates/topo/tests/display_contract.rs`.

## Why it is filed rather than taken

Moving `topo`'s half of the display contract out of `tests/` and into
`src/` is a scope decision about what a unit covers, not a defect in
what TINT-5 landed, and TINT-5's fences put `crates/*/src/**` out of
reach. The same argument would then apply to the F6 rows of every crate
that owns its enums, which is a program-sized question rather than a
unit-sized one.

## Territory

`crates/topo/tests/*` is S-TINT's by `paths: [crates/*/tests/*]`;
`crates/topo/src/*` is not, which is part of why this is a separate
row.
