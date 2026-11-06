---
id: kept-refusal-loops-the-ladder-row-never-listed
kind: issue
title: Loops that keep a first refusal with get_or_insert, outside the hand-rolled-ladder row, each undispositioned
status: dispatched
opened: 2026-11-05
priority: P2
cost: M
branch: onecopy/refusal-standing
---


Found by `onecopy/ladder-climb-3`'s sweep. The row
`door-searches-left-on-the-hand-rolled-ladder` lists door searches found
by `escalation-discards.sh` and by review. Census held three more of the
same shape that no row named (`planar_interiors_meet`,
`shared_interior_witness`, `arc_interiors_meet`); that branch ported them.
So the row's list is not the class.

The shape swept: `get_or_insert(` in production sources: a loop keeping
its first refusal while it reads on. Of 59 hits, most are a first
anchor, a first sense or a first id: no refusal. Each hit below keeps a
refusal or an escalation in a loop. Each is to be read to say whether it
is a search's kept reading (port onto `geom_brep::ladder::climb`, with
its `Answering` declared and its refusal's rank on its type), or not, and
why:

- `crates/topo/src/splitting/order.rs:220` (`undecided`)
- `crates/topo/src/splitting/pieced_arc.rs:137` (`in_band`)
- `crates/topo/src/census.rs:8749` and `:10072` (`refused`)
- `crates/topo/src/boolean/records.rs:830` (`structure`)
- `crates/topo/src/boolean/overlap.rs:559` (`refused`)
- `crates/topo/src/merge_faces.rs:3168`, `:3176` (`zero`, `unsupported`)
- `crates/topo/src/link.rs:1560` (`in_band`) and `:3798` (`refused`)
  (`:1984` is `Cone::reads_given`, rowed separately)
- `crates/topo/src/clearance.rs`: the engine's `lower_refusal` and
  `upper_refusal` (about a dozen sites). This is a bound engine's two
  refusals, likely not a ladder; say so.
- `crates/geom-brep/src/props/spiric.rs:1092` (`first`)
- `crates/editor-core/src/refactor.rs:792` (`missed`) and
  `names/emit_topo.rs:4633` (`escalated`)

What this pattern cannot see: a kept refusal held as `= Some(why)` under
an `if`, as the census loops were (found there by grepping the census's
own `InteriorsUnread::`), or a refusal folded into a counter or a `bool`.
A second pass shaped at those is owed: `= Some(` assignments inside
`for` bodies whose type is an error.
