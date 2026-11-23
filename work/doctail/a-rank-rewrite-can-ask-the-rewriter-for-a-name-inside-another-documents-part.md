---
id: a-rank-rewrite-can-ask-the-rewriter-for-a-name-inside-another-documents-part
kind: issue
title: names: a rewritten name's rank rule reads a seam through an InPart wrapper and asks the rewriter for the image of another document's name
status: closed
opened: 2026-09-30
priority: P3
cost: E
closed: 2026-10-26
---

(EDIT, found by the name-nesting row's sweep, `edit/name-nesting-stack-safe`.
A shape, not probed.)

## What

Every rewrite of a published name (`StableName::rewrite_path`,
`crates/editor-core/src/names/role.rs`) leaves an `InPart` argument
verbatim: it names ANOTHER document's nodes, which the rewrite's map
does not cover (`RoleSeg::rewrite`'s `InPart` arm). But the canonical
form it ends in (`names::canonical::rewritten` → `RankRule::derive` /
`RankRule::along`) finds the seam line a rank lies on through
`seam_pair::seam_line_pair`, whose `head` passes THROUGH `InPart`
(`crates/editor-core/src/names/seam_pair.rs`, `head`), and then asks the
rewriter for the image of that seam's two sides.

So a name whose rank lies along a seam an instantiated part minted —
`[FromA(InPart { of: <the part's seam edge> }), Fragment(OrderAlong)]`,
a boolean cutting an instance's seam edge — has the part's own seam
sides handed to the rewriter. The split's re-map (`refactor::Remapping`)
maps their nodes through THIS document's map: an `Unmapped` refusal for
a name the split should carry, or, where the ids happen to coincide, a
seam side rewritten into the wrong document's space and the rank's rule
read off it.

## What would close it

Decide whether a rank's line is read through the document seam at all;
if it is, the image of a name inside `InPart` is that name (the
rewrite does not cross the seam). A row that splits a document holding
such a name.

## Moot (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). The code this row is about is deleted: ranks, `RankRule`, canonical's rank rewrite and the carrier-parameter rank readers. PR 5758's FULL review checked that no premise of this row remains in editor-core.
