---
id: curved-edges-passes-hand-composes-on-carrier-and-span-place
kind: issue
title: solid_contain's curved_edges::passes composes on_carrier and Span::place by hand beside Span::read
status: open
opened: 2026-11-03
priority: P2
cost: E
---



Found by the design fork on `a-foots-parameter-is-read-without-its-certificate`.

`crates/topo/src/boolean/solid_contain/curved_edges.rs` `passes` does the following by hand:
- it reads a ray's crossing of a curved edge by iterating the foot (`Foot::seed`);
- it reads `Curve3::foot(..).on_carrier(band)` into its own `Gap`;
- it then reads `geom_brep::curve_pair::Span::place` at the same point.

That is the composition `Span::read` already makes, a carrier reading plus a place against the ends. It is also the one point-on-span reading (`geom_brep::curve_pair` module docs). The hand composition reads the place even where the carrier said `Off` (it returns first, so the order holds today), and it maps the refusals itself.

Fix: read `Span::read` (or `Span::contact`) at the iterated point, and map its `Reading` onto `Passes`. Check that `Passes::End`'s graze reading survives, since `Reading::End` carries the same fact.
