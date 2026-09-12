---
id: interrogate-ladder-header-claims-every-rung-and-pins-five
kind: issue
title: lib_u5_interrogate's header claims every rung of InterrogateError is pinned; five of ten have no row
status: open
opened: 2026-09-12
---


## Finding

Found by WIRE's `names-vocab` lane (PR 2474) while looking for a home
for a row about `interrogate::output_body`. Accurate at `62c0e277e`.

`crates/editor-core/tests/lib_u5_interrogate.rs`'s module header says
what is pinned there is *"every rung of [`InterrogateError`]"*, and
argues for itself on exactly that scope: *"An untested ladder is one
where two rungs silently collapse into each other."*

`InterrogateError` has ten variants. The suite mentions five:

| rung | rows in the suite |
|---|---|
| `NodeNotEvaluated` | yes |
| `NoSuchName` | yes |
| `Ambiguous` | yes |
| `WrongKind` | yes |
| `WholeBody` | yes |
| `NodeFailed` | **none** |
| `NodePoisoned` | **none** |
| `NoBodies` | **none** |
| `NoSuchBody` | **none** |
| `Readback(..)` | **none** |

(`grep -c` per variant name over the file; the five "yes" rows are
constructed or matched, the five "none" ones do not appear at all.)

This is the scope-sentence shape `docs/prompts/implementer-discipline.md`
§5 names: a header that reads as completeness over evidence that does
not share its scope. The harm here is the one the header itself
identifies — `NodeFailed` and `NodePoisoned` are the two rungs a caller
most needs to tell apart after an upstream edit, and nothing in the tree
stops them collapsing into each other.

**Not all five are equally reachable, and the row should say so rather
than assume.** `NoBodies` in particular looks unreachable from outside
the crate: `output_body` is `pub(crate)` and reached through
`entity_of`, i.e. only after a name has already resolved in that node's
table — and a datum, profile, mate, measure or assertion node names no
boundary entities. So the repair for that rung may be a sentence in the
header rather than a row, and the unit that takes this should measure
each of the five before writing any of them.

## Fence

`crates/editor-core/tests/lib_u5_interrogate.rs` is S-TINT's and
S-TCOST's shared `crates/*/tests/*` glob. Filed here rather than on
S-TCOST because the defect is an enumeration nobody updated, not a cost
lever — S-TINT's own charter shape, and the same shape as
`r2-m10-6-header-roster-omits-the-suites-heaviest-row` and
`decoration-seam-header-names-no-pin-for-enclose` already on this slate.
