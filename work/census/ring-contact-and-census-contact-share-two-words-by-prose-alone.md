---
id: ring-contact-and-census-contact-share-two-words-by-prose-alone
kind: issue
title: ring_contact_tag and census_contact_tag share two words, held equal by a doc sentence
status: open
opened: 2026-09-15
priority: P3
cost: D
---



Found by CENSUS-PY-GETTERS' sweep (2026-09-15), which had to decide
for seven word pairs whether a shared spelling was one concept or two.
`crates/pncad-py/src/tags.rs` holds one more pair that was already
DECIDED — decided as one concept — and held to that decision by a
sentence.

## The pair

`ring_contact_tag` mints `vertex_vertex`, `vertex_on_edge` and
`edge_along_edge`; `census_contact_tag` mints `vertex_vertex`,
`vertex_on_face`, `vertex_on_edge` and five more. The first two words
are shared on purpose and that map's doc says so: *"The words are the
census vocabulary's where the shape is the same one
([`census_contact_tag`]), because a caller reading two contact words
off one finding should not have to learn two spellings for one
coincidence."*

**Nothing executes that sentence.** Rename `census_contact_tag`'s
`vertex_on_edge` and `TAG_INVENTORY` reds on that one function, the
inventory is updated, and `ring_contact_tag` keeps the old spelling
with its doc still claiming the two agree. The inventory pins each
map's vocabulary and says nothing about two maps meaning one thing.

## Why it is a row and not a line of that unit

CENSUS-PY-GETTERS pinned the two pairs its own fence covered
(`entity_kind_tag` with `entity_id_tag`, `class_admission_tag` with
`mint_refusal_tag`) by construction in `crates/pncad-py/src/tests.rs`,
and stated the general rule in `src/tags.rs`'s header: a tag word is
scoped to its map, and the pairs that must AGREE are the ones that
owe a pin. This pair owes one by that rule and is outside the fence —
both its maps were already in `tags.rs` and neither is one of the
seven second spellings the unit was dispatched over.

The fix is three lines beside the two that exist: build one
`RingContact` and one `CensusContact` per shared word and assert the
tags equal. What wants a moment's thought is whether
`edge_along_edge` and `edge_edge_overlap` are also one concept under
two names, which is a kernel question rather than a binding one.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.

## The family is three, not two (fix pass, 2026-09-15)

`stale_declaration_tag` is the third map in this family and the row
was written without it. It mints `vertex_vertex` **and**
`vertex_on_face`, sharing the first with both maps above and the
second with `census_contact_tag`, and its own doc reasons about them
as contact-record GRANULARITIES — *"a `vertex_vertex` or
`vertex_on_face` record names entities, so the repair is at those
entities"* — which is the same claim `ring_contact_tag`'s doc makes
and the same one nothing executes.

So the fix this row proposes — build one contact per shared word and
assert the tags equal — is a half-fix if it pins two maps. The shared
words across the three, measured from `TAG_INVENTORY`:

- `vertex_vertex`: `census_contact_tag`, `ring_contact_tag`,
  `stale_declaration_tag`
- `vertex_on_edge`: `census_contact_tag`, `ring_contact_tag`
- `vertex_on_face`: `census_contact_tag`, `stale_declaration_tag`

`sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read`
is the general population this pair sits in, and the roster test it
left behind is what would notice a fourth map joining the family.
