---
id: interrogate-ladder-header-claims-every-rung-and-pins-five
kind: issue
title: lib_u5_interrogate's header claims every rung of InterrogateError is pinned; five of ten have no row
status: closed
opened: 2026-09-12
priority: P3
cost: E
closed: 2026-09-22
pr: 2707
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

## Re-derived (2026-09-15, lane C)

**VERDICT: REPRODUCES** — unchanged in every particular. Five of ten
rungs, the same five, and the header still claims all ten.

**The header.** `crates/editor-core/tests/lib_u5_interrogate.rs` still
opens *"What is pinned HERE is the part a doctest cannot reach
comfortably: every rung of [`InterrogateError`], and `edge_frame` against
a body whose edges are lines"*, and still argues for itself on that scope
*"An untested ladder is one where two rungs silently collapse into each
other."*

**The table, re-derived** (`grep -c "\b<variant>\b"` per variant over the
file; `InterrogateError`'s ten variants read off its `pub enum` in
`crates/editor-core/src/names/interrogate.rs`):

| rung | hits in the suite |
| --- | --- |
| `NodeNotEvaluated` | 3 |
| `NoSuchName` | 4 |
| `Ambiguous` | 1 |
| `WrongKind` | 3 |
| `WholeBody` | 1 |
| `NodeFailed` | **0** |
| `NodePoisoned` | **0** |
| `NoBodies` | **0** |
| `NoSuchBody` | **0** |
| `Readback` | **0** |

Identical to the table filed at `62c0e277e`: the five absent rungs do not
appear in the file at all, so the gap is five and the header's scope
sentence is five rungs wider than its evidence.

**The enum has not grown.** Ten variants exactly —
`NodeNotEvaluated`, `NodeFailed`, `NodePoisoned`, `NoSuchName`,
`Ambiguous`, `WrongKind`, `WholeBody`, `NoBodies`, `NoSuchBody`,
`Readback` — so the denominator in the title is still right.

**The `NoBodies` blocker is still in place.**
`work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`
exists on SHELL's slate, so the "blocked on SHELL's repair, not on
reachability" disposition stands and the four unmeasured rungs
(`NodeFailed`, `NodePoisoned`, `NoSuchBody`, `Readback`) are still
unmeasured.

**Adjacent, same enum, worth knowing before a unit is cut.** The row
`assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums` on this
same slate re-derived `InterrogateError` at the same base: its `dumps`
list in `crates/editor-core/tests/display_contract.rs` **is** complete at
ten. So `InterrogateError` is fully enumerated in the F6 suite and half
enumerated in its own ladder suite — a unit that adds the five missing
rungs has a ready source for their identifiers.

**Blind spot of this re-derivation.** The count is a whole-word grep over
the file text, so a rung constructed through a helper that names the
variant elsewhere would read as 0 here, and a rung mentioned only in a
doc comment would read as present. Both directions were checked by eye for
the five "yes" rows (each is constructed or matched in code) and the five
zeros are absolute — the identifiers do not occur in the file in any
form.

**Recommendation (orchestrator's call).** Keep open, unchanged.

## Routed OUT of the roster class (S-TINT orchestrator, 2026-09-15)

This row was grouped with `r2-m10-6-header-roster-…`,
`test-headers-name-fns-that-exist-nowhere` and TOPO's
`review-d18-probes-header-miscounts-its-own-rows` as four rows wanting
one executable check. **That grouping was this seat's and it was wrong
about this row.**

A feasibility probe built and ran the roster mechanism (read the row
list off libtest's own `--list`, welded to the source by a `roster!`
macro whose ident feeds the compile check, the compared string and the
printed text). It covers a header that enumerates **the file's own
`fn`s**. This row's header does not do that: it claims coverage of an
**enum's variants** — every rung of `InterrogateError` — and a roster of
test names says nothing whatever about `InterrogateError`.

**Its welded shape is TINT-1's, not the roster's**: an exhaustive
`match` over the enum in the test file, no wildcard, so a rung added
tomorrow makes the file fail to compile, with the covered set compared
against the full set by a set difference. That unit landed
(`work/tint/assert-f6-dump-lists-are-hand-written-mirrors-of-error-enums`,
PR #2648) and `crates/editor-core/tests/display_contract.rs` already
carries a complete, welded enumeration of `InterrogateError`'s ten
variants to copy from — including the correction that the identifier is
read off `Debug` rather than hand-typed.

**So this row is cheaper than it looked and its mechanism already
exists.** It is not blocked and it is not part of the roster unit; it
wants a small lane of its own applying a landed pattern. Whether the
five unpinned rungs should be pinned, or the header's claim narrowed to
the five that are, is the row's own open question and is unchanged.

## Closed by TINT-6 (PR #2707, `ccf32a73d` on main, 2026-09-22)

The header claimed every rung of `InterrogateError` and the suite reached
five of ten. It now **drives eight through a door** and **excludes two by
name with the measurement that excludes them**, and the accounted set is
welded to the enum by `display_contract.rs`'s existing `f6_variants!`
census — no second roster was minted for one enum in one binary.

**The reachability answer, which is the unit.** Each is a door that was
run and a payload that was read, not a call-graph argument
(`memories/refusal-text-is-not-cause.md`):

| rung | reachable | what the door returned |
| --- | --- | --- |
| `NodeFailed` | yes | `NodeFailed { node: RecipeNodeId(5) }` after a zero-distance extrude |
| `NodePoisoned` | yes | `NodePoisoned { node, through }` via a union over that failed node |
| `Readback` | yes | `Readback(NoCanonicalFrame { carrier: "nurbs surface" })` at `loft_prism` |
| `NoSuchBody` | **no** | 13324 name-table rows driven, zero occurrences; `clearance` answers `SelectionRefusal::NoSuchBody`, a different enum |
| `NoBodies` | **no** | zero corpus nodes carry a no-body payload AND a non-empty name table |

**Structural, not a gap in the corpus**: a read door reaches
`output_body` only through `entity_of`, which indexes with the name
table's own body index — emission's, never a caller's. The one
caller-authored index is `clearance`'s, destroyed a frame up by a
`map_err(|_| ..)`. That is
`work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index`,
still open; a row for either rung today would pin that defect.

**The zero is guarded, not asserted in prose.**
`sweep_the_corpus_for_body_index_rungs` re-takes the measurement every
run — every name of every corpus node through every read door — so the
exclusion reds the day a corpus node makes one reachable. It guards the
zero and non-vacuity (`off_first_body > 0`) and pins neither count, so
it does not rot as the corpus grows. ~1.0s for 13324 rows. Both arms
shown red: a planted kernel condition names the node, the name and the
door; a shrunken corpus reds the vacuity arm.

**What it does NOT enforce.** The header's narrowed sentence is prose
and nothing checks it — said at the site rather than left for review.
Also unenforced: that a driven rung is driven once (the comparison is a
set), and the `carrier: "nurbs surface"` string, which asserts geom's
answer rather than welding anything. `ReadbackError`'s own vocabulary is
declared out of scope; the row pins two of its payloads and not the
enum.
