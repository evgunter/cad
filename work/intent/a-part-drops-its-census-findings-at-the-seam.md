---
id: a-part-drops-its-census-findings-at-the-seam
kind: issue
title: A part's census findings do not cross into its parent, which re-finds them inside one copy and refuses them
status: open
opened: 2026-10-16
priority: P2
cost: M
---

A part's product crosses into the document that places it as a `PartValue`,
built in `crates/editor-core/src/eval/parts.rs` from
`product::product_recorded` alone. It carries the mated pairs refused at the
mint (`refused: Arc::new(product.refused_at_rest)`) and the contact records,
but not the child's at-rest census findings (`Product::census`).

So a child whose copies interfere, or which holds a contact its census could
not record (an escalated in-band crossing, a same-key curved conformal patch,
a Zero with no margin), arrives in the parent as unrecorded touches inside
one copy. The parent's gather re-finds them and refuses them unattributed:
`crates/editor-core/src/checks/at_rest.rs` module docs, "Inside one copy's
body an overlap stays the kernel's typed error". That contradicts D10's "nothing
it finds refuses" for the census at rest, and it reports the child's finding
as the parent's.

**Fix shape** (census-over-solids designers, PR 4666): carry the child's
census findings across the seam with the part, re-keyed by the graft as
`refused` already is; report each once, as the child's; match the parent's
re-found in-copy touches against them by solid pair. An in-copy touch that
matches nothing is reported as the copy's door's unrecorded contact. Pairs
whose finding names no entity need
`work/restread/an-escalated-census-finding-names-no-pair.md` to match.

Found by both census-over-solids designers (fork-log row 113 follow-up,
PR 4666, 2026-10-16). Not run; read from `parts.rs` and `checks/at_rest.rs`.
