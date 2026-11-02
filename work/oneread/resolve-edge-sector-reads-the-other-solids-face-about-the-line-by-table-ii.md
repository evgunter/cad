---
id: resolve-edge-sector-reads-the-other-solids-face-about-the-line-by-table-ii
kind: issue
title: recl::resolve_edge_sector reads the other solid's material about the line (one face's half-turn) by Table II's side codes, beside the line cone
status: closed
opened: 2026-11-01
priority: P1
cost: M
branch: oneread/edge-sector-line-cone
pr: 6363
closed: 2026-11-02
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

## Decided (2026-11-02, ONEREAD orchestrator, on the lane's weighing)

The line cone is not the right home at this site. Here the other solid's material about the edge's line is one face's half-turn, so the cone's parity over it reduces to a flank bound's side of that one face. Since TANGRIM's stage 2 (PR 6253), the kernel's one reading of a bound's side of a face is `sectors::bound_read` (`flank_keys` / `flank_read`), with its descent one order down on a tie. Routing the site through the cone's first-order `link_plane` would make a second reader of that side and regress what stage 2 decides. The lane built the cone reading against main before 6253: one germ moved, the `sliver_shell_role` fin at exactly Kε, on a one-ulp point-versus-direction rounding. Merging over 6253 would have refused the both-tied undeclared curved flanks it now decides.

What lands: Tables II and III fold into what they decide, on 6253's keys unchanged. A crossing is exactly one flanker in the reference's material, a flanker on the face lumped (Eq. 15.3, or a glued tangency's `tangent_lump`), and the germ is placed as Table II named it. `table_ii`, `TableIiVerdict`, `table_iii_rule` and `resolve_verdict` are gone. Both flankers on the face after the descent read no crossing, as Table II's first row did.

The duplication left at the edge-edge site (its line cone reads flank-plane sides at first order beside `bound_read`) is the orchestrator's own row, coupled with TANGRIM.
