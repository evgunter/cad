---
id: sibling-display-contract-suites-hand-mirror-their-enums-too
kind: issue
title: mesh and topo display-contract suites hand-mirror their enums too, and mesh's list is already one variant short
status: closed
opened: 2026-09-15
closed: 2026-09-15
pr: 2694
---


Found by TINT-1's sweep
(`work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`),
whose fence was `crates/editor-core/tests/**` — these three lists sit
outside it, so they were disposed of rather than fixed.

## The finding

TINT-1's row said `grep -rn assert_f6` finds the helper and its callers
only in `crates/editor-core/tests/display_contract.rs`. That is wrong as
a repo-wide claim: **`crates/topo/tests/display_contract.rs` declares
its own `assert_f6`**, a near-verbatim copy with the same
`dumps: &[&str]` hand-written ban list. A sweep for the ban-list SHAPE
(`for dump in`, `dumps = [`, `dumps: &[&str]`) rather than the helper
name finds it, and finds a third inlined copy in `crates/mesh/tests/errors.rs`.

**One of them has already drifted**, which is the same live failure
TINT-1 found three of in editor-core:

- `crates/mesh/tests/errors.rs`, `let dumps = [...]` inside the
  tessellate-error Display row, holds **14** identifiers.
  `mesh::TessellateError` (`crates/mesh/src/types.rs`, `pub enum
  TessellateError`) has **15** variants: `Band` is missing. The
  suite's own case list constructs a `TessellateError::Band { .. }`
  case, so a rendering of that arm IS exercised with its identifier
  unbanned.

  **What that actually leaves uncaught, corrected.** As filed this row
  said *"the one identifier that would catch it going back to `Debug`
  is the one the ban list does not hold"*. That is false, and TINT-5's
  fix pass corrected it. The same inline check carried
  `!shown.contains('{')` **and** `assert_ne!(shown, format!("{err:?}"))`,
  and `TessellateError::Band` is a STRUCT variant — so an arm that went
  back to its full `Debug` dump reddened on the brace whatever the
  roster held. What the missing entry left uncaught is a **brace-free
  leak of the word `Band`** (`write!(f, "Band: …")`), which is real and
  strictly smaller. The red-on-arrival evidence in TINT-5's PR stands:
  the weld does red on a live defect and names `["Band"]`; only the
  claim about what would otherwise have passed was wrong.
- `crates/topo/tests/display_contract.rs`:
  `["Contradicted", "Escalated", "Undeclared", "NotCertifiable"]` against
  `topo::ContactRefusal` (4 variants) and
  `["Dangling", "NoCanonicalFrame", "NoCarrier"]` against
  `topo::readback::ReadbackError` (3 variants) are both complete **as of
  2026-09-15**. Nothing welds either to its enum, so both are the
  forecast TINT-1's row wrote, not the instance.
  (`together_edge_disagreement_display_is_true_at_all_three_meters`'s
  one-entry `&["TogetherEdgeDisagreement"]` is a deliberate single-variant
  row, not an enum mirror — leave it.)

## The fix

The shape TINT-1 landed, applied per suite: `test_utils::f6::assert_f6`
instead of a local copy of the predicate, a wildcard-free `match` used
as an exhaustiveness TOKEN (returning `()`, naming no identifiers), the
identifier roster written out once, and a set difference that welds the
roster to the cases — with the covered identifiers read off each value's
own `Debug` (`test_utils::f6::variant_identifier`) rather than typed
beside a pattern, because rustc checks the pattern and never the string.
See `crates/editor-core/tests/display_contract.rs`'s
`assert_f6_every_variant` and its per-enum `*_is_exhaustive` /
`*_VARIANTS` pairs. Adding `TessellateError::Band` to mesh's list is the
smaller half; the weld is what stops the next one.

`topo::ContactRefusal` and `topo::readback::ReadbackError` are ordinary
enums, so the compiler can be the census there with no exception of the
kind `SelectRefusal` forced (`work/wire/select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`).
Check `mesh::TessellateError` for `#[non_exhaustive]` before assuming the
same.

## Territory

Both paths are S-TINT's by `paths: [crates/*/tests/*]`;
`scripts/work.py territory` reports `crates/mesh/tests/errors.rs` also
claimed by `mesh` and `tcost`, and `crates/topo/tests/display_contract.rs`
also claimed by `tcost` — double claims on the documented `*/tests/*`
seam, not crossings.

## Sweep blind spot inherited

The shape sweep (`for dump in`, `dumps = [`, `dumps: &[&str]`, `&dumps`)
cannot match a ban list that spells the loop differently — an
`assert!(!shown.contains("Variant"))` written out once per identifier,
a list named something other than `dumps`, or a ban list held in a
`const` and passed positionally. `crates/viewer/tests/error_display.rs`'s
lone `assert!(!duplicate.contains("PatchId"))` is that shape at N=1 and
is not an enum mirror.

**And TINT-1 added two instances of that blind spot itself**:
`display_contract.rs`'s `*_VARIANTS` consts and `m4_pr4_hit.rs`'s
`HIT_TEST_ERROR_VARIANTS` are exactly "a ban list held in a `const` and
passed positionally", so a `dumps = [` / `for dump in` sweep will not
find them. They are welded to their cases by a set difference, which is
why they are not the defect this row is about — but a sweep for the
CLASS has to look for the const shape as well as the inline one, and
this is the note that says so.

## Closed by TINT-5 (PR #2694, `645e4d6d1` on main, 2026-09-15)

All three sites adopted the weld, and the weld got a home:
`assert_f6_every_variant` and `set_difference` moved out of
editor-core's test binary into `test_utils::{f6, census}`, so the three
adopting suites call one copy rather than carrying three.

**The live defect closed, and it reddened on arrival.**
`crates/mesh/tests/errors.rs` banned 14 identifiers against a
15-variant enum; the weld went red naming `["Band"]` before the entry
was added, then green after. Not a plant — the first guard in this
program to red on a real defect in the tree.

**This row misstated its own defect, and the correction is the more
useful half.** It said *"the one identifier that would catch it going
back to `Debug`is the one the ban list does not hold."* **False.** The
pre-merge check at `crates/mesh/tests/errors.rs:219` carried
`!shown.contains('{')` **and** `assert_ne!(shown, format!("{err:?}"))`,
and `TessellateError::Band` is a struct variant — so a full `Debug`
regression reddened on the brace whatever the roster held. What the
missing entry actually left uncaught is a **brace-free** leak of the
word `Band`. Real, and strictly smaller than this row claimed. Found by
the style review, which reproduced both halves.

**What the weld does NOT enforce.**

- The **prose half**. `fields` — the field punctuation a rendering must
  not leak — is a hand-written roster at every site, welded to nothing,
  and `f6`'s module doc argues why it is not derived (it was tried; it
  false-positives on a door whose own prose opens with a field name,
  `MeshPickError::PositionOutOfRange`'s *"pick index: …"*). The three
  new rosters are correct today and nothing checks that they stay so.
  What they buy is narrower than first claimed: the brace ban and the
  exact-dump refusal already catch a `{self:?}` arm, so the 18 tokens
  buy exactly a brace-free field-token leak.
- A variant **rendered twice** is indistinguishable from once — the
  comparison is a set difference.
- A **hand-written `Debug`** that opens on the type name yields a wrong
  covered token, the roster gets "fixed" to match it, and the weld goes
  green over a roster mirroring nothing. `variant_identifier` asserts
  only that the first token starts alphabetic or `_`.
- One site is not macro-built: `SelectRefusal` is `#[non_exhaustive]`,
  so rustc forces a catch-all and it goes through a named
  `hand_written` door that says what that costs.

**The token half is now welded**, which it was not when this row was
cut: `test_utils::f6_variants!` writes the exhaustiveness `match` and
the identifier roster from one list of idents, so they cannot disagree,
a wildcard is a macro grammar error, and a no-op token is not
expressible.
