---
id: census-sees-an-inert-attribute-but-not-a-missing-one
kind: issue
title: The deny_unknown_fields census is one-directional: nothing reds when a named field arrives with no attribute
status: open
opened: 2026-09-15
priority: P3
cost: D
---


`crates/test-utils/tests/deny_unknown_fields_census.rs` reds when
`#[serde(deny_unknown_fields)]` sits on a declaration with no named
field to deny. **Nothing reds in the other direction** — a declaration
that derives `Deserialize`, grows a named field, and carries no
attribute passes every row in the tree, silently drops an unknown key,
and reads as governed to anyone who trusts the census.

That complement is not hypothetical. `MatePrimitive::PlanarRest` is a
confirmed instance, filed on msolve as
`mate-primitive-accepts-a-stray-field-the-module-docs-say-refuses`,
established by execution: a stray key inside `PlanarRest` loads and is
dropped, while `crates/editor-core/src/persist/mod.rs`'s module docs
rule the opposite for the whole format. That row is ONE INSTANCE,
scoped to `crates/editor-core/src/` and not swept beyond it. **This row
is the class and the instrument.** It belongs on CENSUS rather than on
msolve because it is the charter stated exactly: a vocabulary whose
census can see one direction and not the other, where the blind
direction is invisible AND certified by the instrument that covers the
other half.

## What a two-directional instrument has to decide

The hard part is not the walk — the same classifier already tells a
named-field declaration from one with nothing to deny, and the same
`test_utils::source` views already reach every `.rs` file. The hard
part is the verdict key, because **not every `Deserialize` type with a
named field wants the attribute**, and a guard that says otherwise is a
lint nobody can satisfy:

- **A type that is not document vocabulary at all.** Test-local
  stand-in producers (`meta`'s own `mod tests` has two), fixture types
  and anything deserialized from a source that is not a saved
  document. Refusing an unknown key there buys nothing and the
  attribute would be noise.
- **A type behind `#[serde(flatten)]` or an untagged arm**, where
  `deny_unknown_fields` is documented by serde as incompatible with
  flattening — the attribute cannot be added, so the guard must be able
  to say "exempt because it CANNOT carry one" without a hand-written
  list, which is this program's trap.
- **A type whose unknown key is meant to be tolerated** — the viewer's
  preferences file is the standing example of the opposite posture
  (`crates/viewer/src/prefs.rs`: an unknown key reports and the rest of
  the file applies), and a document-side type could legitimately want
  it.
- **Whose rule is being enforced.** The persist module docs state the
  refuse-don't-drop rule for the DOCUMENT format. A guard that reads
  every crate enforces it on types that never reach a document. So the
  population the instrument scans is itself a decision, and scoping it
  by crate is the hand-written list again.

An exemption list is refused here for the same reason the existing
census refuses one at its own header: a hand-written list of
declarations is a fresh instance of the class. So the row's substance
is finding a mechanical property that separates "owes the attribute"
from "does not" — reachability from the persist door is the obvious
candidate and it is a type-graph question, which a source-text walk
cannot answer.

Until that is decided, the asymmetry is stated at the two sites that
would otherwise read as a guarantee: the census header says what it
cannot see, and `persist/mod.rs` says the rule is the format's intent
and not a property its types enforce everywhere it is asserted.

**Not to be taken by a lane that cannot also change what a document
accepts** — closing the instance found so far means adding an
attribute, which changes what a document accepts, which
`docs/CENSUS-INERT-DENY-SPEC.md` §What this unit is NOT places outside
the sweep that found it.
