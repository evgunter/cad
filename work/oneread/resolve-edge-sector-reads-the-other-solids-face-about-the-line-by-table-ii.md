---
id: resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii
kind: issue
title: recl::resolve_edge_sector reads the other solid's material about the line (one face's half-turn) by Table II's side codes, beside the line cone
status: open
opened: 2026-11-01
priority: P1
cost: M
---


Filed by `recl-edge-edge-composes-the-other-solids-material-beside-the-cone`'s
sweep (branch `oneread/recl-line-cone`), which made
`recl::resolve_edge_edge`'s membership read the other solid's line cone
(`boolean::sectors::line_fans`, `link::Cone::reads_tied`).

## What

`crates/topo/src/boolean/recl.rs` `resolve_edge_sector` decides an
edge of one solid lying inside a face sector of the other's (the
reference): it keys the edge's two flankers by their sides of the
reference plane (`flank_key`, through `side_code`), reads Table II on
the keys, and Table III's lump for an On flanker. The other solid's
material about the edge's line there is the reference face's half-turn,
which is a line cone of one face the line runs inside
(`LineSheet::Across`): the same fact the edge-edge site now reads by
parity, read here by side codes and a table. The class is confirmed:
both answer "which of the edge's flanks lie in the other solid's
material about the common line".

What the cone does not yet hold: the declared-`Tangent` second-order
key (`curving`, `sectors::tangent_relative_side`), which the edge-edge
site keeps beside the cone as its coplanar ladder does, and the
first-read reference sector (`find_ref_sector` over `first_read`).

## Owed

Weigh folding `resolve_edge_sector`'s keys into the line cone (Table II
as the cone's reading of the two flanks, Table III's lump as the `On`
ladder), keeping `flank_key`'s construction and the tangent descent;
or say why the table reads something the cone does not.
