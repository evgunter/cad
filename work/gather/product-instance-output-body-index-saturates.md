---
id: product-instance-output-body-index-saturates
kind: issue
title: product gather's instance output-body index saturates at u32::MAX instead of refusing
status: closed
opened: 2026-09-25
priority: P4
cost: E
closed: 2026-09-25
pr: 3141
---

`product.rs::sources_of`, on its `ValuePayload::Instances` arm, tags
instance `i` with the output-body index
`u32::try_from(i).unwrap_or(u32::MAX)`. Past `u32::MAX` every later
instance would share index `u32::MAX`, so `product_named` would read one
body's rows for all of them without anything saying so.

It cannot be reached at a holdable size, and an upstream refusal
already stands in front of it: `names::emit::name_pattern` keys each
instance's rows through `output_body`, which refuses an index past
`u32::MAX` with `NamingError::Emission`, so a pattern that wide fails
before it is ever a product root. The site still breaks the fail-loud
rule, and it reads as though saturation were the intended answer.

The fix is not a one-liner: `sources_of` answers `Option` (`None` is
"this root denotes no body"), so declining on overflow would be a
silent wrong answer. It wants either a typed refusal through the
gather's own error (`ProductError`), or an explicit statement that the
upstream refusal is the guard, with the narrowing spelled as a checked
conversion that names that invariant.

Found by the second sweep of `naming-index-casts-saturate-silently-at-u32-max`
(pattern `try_from(..).unwrap_or(u32::MAX)` over `crates/editor-core/src`).

## Closed

PR 3141. **Option two of this row's two, with the reason the first one
was not taken.**

`sources_of`'s `Instances` arm (`crates/editor-core/src/product.rs`)
no longer saturates. It narrows through `names::output_body`, which is
the one home of that narrowing and the same door whose
`NamingError::Emission` refuses an out-of-range index elsewhere. It
reaches `unreachable!`, naming the instance and the error, only if the
invariant is broken. The invariant holds for this reason: every
`ValuePayload::Instances` is minted by `wire_pattern` (`eval/wire.rs`).
That function puts every flat index through `names::output_body` and
`names::flat_body_index`, refusing with `NodeErrorKind::Naming`, before
the value exists. `Placeable::map` then rebuilds the list one body for
one. The site's comment says this. The shape is the one
`checks.rs`'s shell-count narrowing already uses, and the workspace lint
comment in `Cargo.toml` states the position: an invariant panic is a
kernel bug, not input-reachable failure.

**Why the refusal is not threaded through as a typed error.** It would
need a new arm on three public enums, because none of them carries a
`NamingError`. `sources_of` has eight callers: `product.rs`, three in
`checks.rs` and four in `resolve/pick.rs`. They answer `ProductError`,
`ChecksError`, `NodePickError`, and `Option` (`checks::subject_body`).
Each new arm would then owe a `Display` census row and a Python binding
mapping, all for a refusal the pattern op has already made at the
source. That is not E, and it would add three arms that can never be
reached.

**No test.** The runtime value that could make the conversion refuse is
an instance list longer than `u32::MAX`, and no body list that large can
be held. An assertion over it would be documentation
(`implementer-discipline.md` §2).

**Sweep**, pattern `unwrap_or((u32|u16|u64|i32|usize)::MAX)` outside
`tests/`:

- `product.rs` `sources_of`: fixed here.
- `names/emit.rs` `name_pattern` and `eval/wire.rs` `wire_pattern`
  (`usize::try_from(j).unwrap_or(usize::MAX)`): the saturated value
  feeds `output_body`, which refuses it, so these stay.
- `param_source.rs`'s name-length prefix: a documented saturation that
  keeps the encoding self-delimiting. It encodes, it does not index, and
  its comment argues it. Not this shape.
- `eval/wire.rs` (a `position` sort key), `mate/solve.rs` (a sort key),
  `topo/src/boolean/insert.rs` (a sort key): `usize::MAX` as an
  "absent sorts last" sentinel. A different shape: nothing is aliased.
- `topo/src/euler.rs` `plus`: the saturation is caught by the
  postcondition assert its comment names.
- `topo/src/fixtures.rs`, `topo/src/seqgen/random_op_sequences.rs`,
  `viewer/src/gpu.rs` (a test message): test support.

**Blind spot:** a saturating narrowing spelled some other way (`as u32`,
`.min(u32::MAX as usize)`). This row's parent sweep,
`naming-index-casts-saturate-silently-at-u32-max`, owns the `as` casts
in `names/`. They were not re-swept here.

