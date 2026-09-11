---
id: pair-subject-witness-strings-unswept
kind: issue
title: the pair-subject sweep could not read witness STRINGS: UndeclaredContact and ContactContradicted carry their subject in format! text, unexamined
status: closed
opened: 2026-09-04
branch: fix/census-pair-order
closed: 2026-09-11
refs: [census-witness-string-repeats-the-subject]
---


## The blind spot, stated as its own item

PR 1750's Half A swept the shape *a refusal whose subject is a set,
carrying one member the arena's ordering picked*, anchored on
`EntityId::Face(...)` and on `face:`/`entity:` struct-field payloads.
That pattern is blind to a subject that reaches a reader inside a
**string**, and `crates/topo/src/census.rs` has fourteen `witness:`
sites. Eleven render a POINT through `witness()` — a coordinate, which
is the right thing and is what makes those findings actionable. Three
render arena keys instead:

- `census.rs:1635` — `format!("{fa:?}~{fb:?}")`, the conformal sweep's
  undeclared-contact finding;
- `census.rs:2728` — `format!("{:?}~{:?}", c.face_a, c.face_b)`, the
  patch confirm pass's contradiction;
- `census.rs:2673` — `format!("{:?}", c.witness)`, the curve pass's
  witness edge.

## What is NOT this item

The `{:?}`-where-`Display`-belongs half is already owned by
`tier-3-prime-findings-render-through-debug`, which measured it at
LIB-B-VALIDATE4 and names `witness()` and two `validate.rs` arms. This
item does not re-file that.

Nor is any of the three the S190 defect: each carries its pair or its
edge in a TYPED field beside the string, so a consumer resolves from
the field and never has to read the prose. Nothing here is a
correctness gap.

## What IS this item

**Order.** PR 1750 settled that a census face pair is UNORDERED as a
subject, and wrote that into the type: `CensusSubject`'s `PartialEq`
compares `(a, b)` against `(b, a)`. The two pair strings above were not
revisited, and they still print one order — the arm's, which is the
arena's. So a reader is shown `f~g` for a subject the kernel now
holds to be the same as `g~f`, and two runs that differ only in arena
order produce two different messages for one finding.

The work is to read the three sites and decide, per site, whether the
rendering says what its arm means: normalise the order, say in the
message that the pair is unordered, or record why the arm's own order
is the right thing to show. It may well close with no code change and
a sentence — that is a legitimate outcome, and better than a blind spot
nobody wrote down.

## Provenance

This is a SWEEP gap, not a measured defect: PR 1750's own "what the
pattern could not match" sentence, given a file so it survives FIX's
directory being deleted at close.

## Home

`crates/topo/src/census.rs`.

## Closed: the order is right at all three sites; the row's premise does not hold

Re-derived at merge base `8851abb`: the sites are `census.rs:1677`,
`:2830` and `:2770` (the body's `1635`/`2728`/`2673` are stale). The
site census above is also a raw grep: three different `witness` fields
of three different types are counted together. The
`ValidationError` **string** witnesses in the file number **eleven**,
not fourteen — **eight** render a position, **three** render keys.
`:2736` is `StaleDeclaration::CurveLocus.witness: EdgeKey`, and
`:2989`/`:3415` are `EdgeDescriptionSpec::Intersection.witness:
Point3`; none is a string.

**`:2770` is not in this class.** Its witness is one `EdgeKey`, and a
single key has no order to normalise. It names a locus genuinely
distinct from the pair the same sentence already states, so it is the
one of the three whose string adds something.

**At `:1677` and `:2830` the arm's own order is the right thing to
show**, and this is now recorded at both sites. The row reads the
unordered-pair settlement as reaching the rendering; the settlement
says the opposite in the same clause — the order is kept in the value
and in what `Debug` prints, and dropped from the COMPARISON only
(`crates/topo/src/validate.rs:317-344`). `CensusSubject`'s own
`Display` prints the arm's order for the same reason
(`validate.rs:349`). The stated harm — two runs differing only in
arena order — is not a state D9 admits: same build and same inputs
give bit-identical outputs, so the arm's order is a function of the
input. Across *different* inputs the keys themselves differ, which no
ordering of them can stabilise.

**What is wrong at those two sites is the content, not the order**:
the string repeats the subject in a slot documented for the witnessing
position. That is its own row —
`work/fix/census-witness-string-repeats-the-subject.md` — because it
needs a decision about `ValidationError`'s shape and a change inside
CURVED's predicate, neither of which is this item.
