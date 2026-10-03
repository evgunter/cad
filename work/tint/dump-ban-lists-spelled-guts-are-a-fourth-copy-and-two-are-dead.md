---
id: dump-ban-lists-spelled-guts-are-a-fourth-copy-and-two-are-dead
kind: issue
title: editor-core's `guts` dump ban lists are the shape TINT-1's sweep could not see, and the resolve/placement list bans two of the nine identifiers it renders
status: open
opened: 2026-09-15
priority: P3
cost: E
---


Found by TINT-5's sweep for the CLASS rather than the spelling
(`work/tint/sibling-display-contract-suites-hand-mirror-their-enums-too`
names the blind spot: *"a list named something other than `dumps`"*).

**This row was filed with a false headline and corrected by TINT-5's fix
pass.** As filed it said the resolve/placement list bans three
identifiers *"no rendering under it can produce"* and that two of the
six sites were dead. All three of those identifiers are live, and one
site — not two — is dead. The correction is below; the filename keeps
its original id because the id is how the row is cited.

## The finding

**Six more hand-written variant-identifier ban lists**, all spelled
`for guts in [...] { assert!(!rendered.contains(guts)) }`, none welded
to any enum. `grep` for `dumps`, `assert_f6` or `dumps: &[&str]` — the
sweeps TINT-1 and the sibling row ran — finds none of them, and both
files are inside TINT-1's own fence (`crates/editor-core/tests/**`).

- `crates/editor-core/tests/lib_doors_node_result.rs`, four sites: the
  `MetaVersionError` row, the `NodeErrorKind` forwarding row, the
  `MarginDiag` row, and the resolve/placement row
  (`the_document_layers_own_payloads_render_their_own_stories`).
- `crates/editor-core/tests/asm_r2b_assembly.rs`, two sites in the
  at-rest refusal rows.

**The class is looser than a roster.** Only ONE of the six — the
`MetaVersionError` list — is a single enum's variant roster. The
`NodeErrorKind` forwarding list, the two `asm_r2b_assembly.rs` lists
and the resolve/placement list are MIXED lists of type names and
variant names spanning several enums at once, which is why no per-enum
sweep found them and why adopting the weld at any of them needs the
cases split per enum first.

**The resolve/placement list is badly incomplete, and unwelded.** Its
`cases` list renders **five** types, not two — the test's own doc header
says so (*"the D54 set: `EvalError`, `ResolveError`, `WitnessBifurcation`,
`PlacementRuleFault`"*, plus the nested `Diagnosis` and
`BifurcationKind` payloads) — and it bans
`["{", "UnknownParam", "NodeGone", "PredicateFlip", "AmbiguousBasin"]`.

- All three of `UnknownParam`, `PredicateFlip` and `AmbiguousBasin` are
  LIVE identifiers this row renders: `EvalError::UnknownParam` is its
  first case (`EvalError`, `crates/editor-core/src/expr.rs`);
  `Diagnosis::PredicateFlip` is the payload of the
  `ResolveError::Vanished` case (`crates/editor-core/src/resolve/mod.rs`);
  `BifurcationKind::AmbiguousBasin` is the `kind` of the
  `WitnessBifurcation` case (`crates/editor-core/src/witness.rs`). The
  row as filed narrowed "either enum" to `ResolveError` and
  `PlacementRuleFault` and reported the residue as dead. It is not.
- What IS wrong is the other direction. `ResolveError` has exactly
  three variants — `Vanished`, `Ambiguous`, `NodeGone` — and only
  `NodeGone` is banned, **including `Vanished`, which the row itself
  renders**. `PlacementRuleFault` has exactly four — `CountSpelling`,
  `NoPlacements`, `NonFiniteFrame`, `ImproperFrame` — and **none of the
  four is banned**, so either of the two arms this row renders could
  regress to `{self:?}` and pass on the identifier. `EvalError` has
  seven and one is banned.

**One site is genuinely dead.** The `MarginDiag` row bans
`["{", "MarginDiag", "Value("]` over every `NodeErrorKind` forwarding
case. `MarginDiag::Value` is a TUPLE variant, so a `Debug` dump of it
opens `Value(` and never on the TYPE name — `"MarginDiag"` is an
identifier no rendering under that list can produce.

The `MetaVersionError` row is complete as of today (`NotAMap`,
`MissingVersion`, `VersionNotInt` against a three-variant enum), and so
is the forwarding row's own list as far as it goes; both are the
forecast rather than the instance, welded to nothing.

## The fix

TINT-5 promoted the weld to `test_utils::f6::assert_f6_every_variant`,
with `test_utils::census::set_difference` under it and
`test_utils::f6_variants!` generating the `match` and the identifier
roster from one list of idents, so the shape these sites need already
has a home and adopting it is not a copy. Two wrinkles this row does not
get to skip:

- Several of these rows render **more than one enum's** values in one
  `cases` list, so the per-enum walk `assert_f6_every_variant` expects
  needs the cases split per enum first — the census is per type.
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
