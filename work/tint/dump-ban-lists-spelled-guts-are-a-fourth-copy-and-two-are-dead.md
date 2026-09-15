---
id: dump-ban-lists-spelled-guts-are-a-fourth-copy-and-two-are-dead
kind: issue
title: editor-core's `guts` dump ban lists are the shape TINT-1's sweep could not see, and two of them ban identifiers no rendering can produce
status: open
opened: 2026-09-15
---


Found by TINT-5's sweep for the CLASS rather than the spelling
(`work/tint/sibling-display-contract-suites-hand-mirror-their-enums-too`
names the blind spot: *"a list named something other than `dumps`"*).

## The finding

**Six more hand-written variant-identifier ban lists**, all spelled
`for guts in [...] { assert!(!rendered.contains(guts)) }`, none welded
to any enum. `grep` for `dumps`, `assert_f6` or `dumps: &[&str]` — the
sweeps TINT-1 and the sibling row ran — finds none of them, and both
files are inside TINT-1's own fence (`crates/editor-core/tests/**`).

- `crates/editor-core/tests/lib_doors_node_result.rs`, four sites: the
  `MetaVersionError` row, the `NodeErrorKind` forwarding row, the
  `MarginDiag` row, and the resolve/placement row.
- `crates/editor-core/tests/asm_r2b_assembly.rs`, two sites in the
  at-rest refusal rows.

**Two of the six are already wrong**, which is the same live failure
TINT-1 found three of:

- The resolve/placement row (`lib_doors_node_result.rs`, the `cases`
  list that renders `ResolveError::Vanished`, `ResolveError::NodeGone`,
  `PlacementRuleFault::CountSpelling` and
  `PlacementRuleFault::ImproperFrame`) bans
  `["{", "UnknownParam", "NodeGone", "PredicateFlip", "AmbiguousBasin"]`.
  `ResolveError` (`crates/editor-core/src/resolve/mod.rs`) has exactly
  three variants — `Vanished`, `Ambiguous`, `NodeGone` — so **two of the
  three are unbanned, including `Vanished`, which the row itself
  renders**. `PlacementRuleFault` (`crates/editor-core/src/node.rs`) has
  four — `CountSpelling`, `NoPlacements`, `NonFiniteFrame`,
  `ImproperFrame` — and **none of the four is banned**, so either of the
  two arms this row renders could regress to `{self:?}` and pass.
  `UnknownParam`, `PredicateFlip` and `AmbiguousBasin` are not variants
  of either enum: the list bans three identifiers no rendering under it
  can produce while leaving six live ones unbanned.
- The `MarginDiag` row bans `["{", "MarginDiag", "Value("]` over every
  `NodeErrorKind` forwarding case — a type name and a tuple-variant
  fragment, with no arm of the enum being walked named at all.

The `MetaVersionError` row is complete as of today (`NotAMap`,
`MissingVersion`, `VersionNotInt` against a three-variant enum), and so
is the forwarding row's own list as far as it goes; both are the
forecast rather than the instance, welded to nothing.

## The fix

TINT-5 promoted the weld to `test_utils::f6::assert_f6_every_variant`
with `test_utils::census::set_difference` under it, so the shape these
sites need already has a home and adopting it is not a copy. Two
wrinkles this row does not get to skip:

- Several of these rows render **more than one enum's** values in one
  `cases` list, so the per-enum walk `assert_f6_every_variant` expects
  needs the cases split per enum first — the roster and the
  exhaustiveness token are per type.
- `EditError` renders identifiers via `Debug` deliberately (its own
  `Display` header states why), so the property at the
  `MetaVersionError` site is about the PAYLOAD enum, not the carrier.
  A weld there is over `MetaVersionError`.

## Territory

`crates/editor-core/tests/*` is S-TINT's by `paths: [crates/*/tests/*]`,
the same seam the sibling row sits on.

## Out of TINT-5's scope, deliberately

TINT-5's spec decided its three adopting sites by name. These are the
same class in a fourth and fifth file and would have been a scope
widening taken without the decision being made; disclosing a residue is
not scheduling it, so it gets its own file here.
