---
id: boolean-side-codes-refuse-a-levered-on-as-a-kernel-bug
kind: issue
title: The boolean's side codes read a long chord's levered Zero as On, and the join refuses the pose as a kernel bug where a metric reading answers it
status: open
opened: 2026-09-28
priority: P2
cost: M
---


Filed by CONTACT-9, which traced every Zero of the boolean's and the
splitting lane's side codes and found no wrong result. What it did
find is a false refusal, worded as a kernel bug.

## What

`boolean::sectors::side_code` (`crates/topo/src/boolean/sectors.rs:336`)
reads a chord's side of a face plane as `d̂·n̂ × arm`, and turns its
Zero into `SideCode::On` (`:347`). The arm is the shorter chord of the
sector:
- `vtxfac.rs:178`, per sector;
- `pair_search` (`sectors.rs:631`), the shorter of the pair;
- `recl.rs`'s `flank_key` (`:371`) and edge-edge membership (`:674`).

So a 10 m chord beside a 1 mm one reads Zero while its far end stands
10⁴ times the reading off the plane. The On is then:
- resolved from its neighbours in `vtxfac` (`:333`);
- or taken as an on-edge event in `recl_edges`.

Either way a section germ at the vertex is dropped or moved, so
each face flanking the chord keeps a section end with no partner.
The join then refuses `Join(UnpairedLooseEnds)`
(`boolean/join.rs:530`), and the message ends "(kernel bug)".

## Evidence

`crates/topo/tests/contact9_side_codes.rs`, at every ε row:
- A needle's tip on a slab's face, or on a block's corner, has its
  10 m edge dipping `500·ε`. With 1 mm edges beside it, every op
  refuses at the join. The same dip with 1 m edges answers, and point
  membership in the sliver is correct.
- The splitting lane reads a line edge at its far vertex
  (`splitting/neighborhood.rs:286`, `split_vertex_side` in metres). It
  splits the 1 mm needle and returns the analytic sliver.

## The fix's shape

For a line chord, read its far vertex's signed distance from the
plane, `Margin::of(n̂·(q − p))`. This is the splitting twin's reading,
and CONTACT-7's metric principle. A conic chord keeps its departure
reading at its own extent. The rows above then flip from refusals to
answers. The bisector entries need no change: CONTACT-9's trace shows
that their Zero changes no topology.
