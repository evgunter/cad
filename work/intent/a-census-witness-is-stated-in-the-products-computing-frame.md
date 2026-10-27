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
gather computes in its first non-empty source's frame
(`crates/editor-core/src/product.rs`, `gather_sources`) and censuses the
aggregate there. `perf12_census_goldens` (`kitchen_sink`, every ε row)
reads the same 29 findings on the same keys at the same margins, and
its seven `UnrecordedContact` witnesses 0.375 further along z than
main: the product's first source's frame is a translation along z.

A witness is a point a reader is told to look at. Nothing draws one
today (the viewer and pncad-py read a finding's words, not its point),
so no surface misleads yet, but a finding should state its witness in
the space the product is in. The census runs in `topo`, which knows no
frame, so the composition belongs where the finding leaves the gather:
map each witness through the product's frame (`Product.frame`), as the
interrogation doors answer in the space (`InItsSpace::moved_by`).
