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

**Not all five are equally reachable, and the unit that takes this
should measure each one rather than assume.** One of the five is already
measured, and the measurement corrected this lane's own first guess:
`NoBodies` **is** reachable, through `clearance::clearance`, which takes
a caller-authored `Selection` and reaches `interrogate::output_body`
with no name in the picture. But the rung it produces is destroyed one
frame up by a `map_err(|_| ..)` — see
`work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`,
which carries an executable repro — so a row for it is **blocked on
SHELL's repair**, not on reachability. The other four are unmeasured
here.

The transferable half: *"the arm looks unreachable"* is a claim about a
call graph, and `memories/refusal-text-is-not-cause.md` applies to it
exactly as to a refusal's prose — run the door, read the payload.

## Fence

`crates/editor-core/tests/lib_u5_interrogate.rs` is S-TINT's and
S-TCOST's shared `crates/*/tests/*` glob. Filed here rather than on
S-TCOST because the defect is an enumeration nobody updated, not a cost
lever — S-TINT's own charter shape, and the same shape as
`r2-m10-6-header-roster-omits-the-suites-heaviest-row` and
`decoration-seam-header-names-no-pin-for-enclose` already on this slate.
