---
id: a-census-witness-is-stated-in-the-products-computing-frame
kind: issue
title: An at-rest census finding states its witness point in the product's computing frame (its first source's), not the world's
status: open
opened: 2026-10-27
priority: P3
cost: E
---


Found by INTENT stage 3 E (`intent/e-operation-frame-of-reads`), whose
gather computes in its first non-empty source that is in its space's
coordinates (`Own`), else its first source's frame
(`crates/editor-core/src/product.rs`, `gather_sources`), and censuses
the aggregate there. While E's gather computed in its first source's
frame, `perf12_census_goldens` (`kitchen_sink`, every ε row) read the
same 29 findings on the same keys at the same margins and its seven
`UnrecordedContact` witnesses 0.375 further along z than main. With the
`Own` source preferred it reads main's witnesses again, because that
product holds a source in the world's coordinates; a product whose
sources are all copies under maps still states its witnesses in the
first copy's frame, and no row holds one.

A witness is a point a reader is told to look at. Nothing draws one
today (the viewer and pncad-py read a finding's words, not its point),
so no surface misleads yet, but a finding should state its witness in
the space the product is in. The census runs in `topo`, which knows no
frame, so the composition belongs where the finding leaves the gather:
map each witness through the product's frame (`Product.frame`), as the
interrogation doors answer in the space (`InItsSpace::moved_by`).

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to ATREST's charter (at-rest census work; ATREST's stage 5 C makes the census a check at each `⊔`, the same door). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
