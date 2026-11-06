---
id: kept-refusal-loops-the-ladder-row-never-listed
kind: issue
title: Loops that keep a first refusal with get_or_insert, outside the hand-rolled-ladder row, each undispositioned
status: open
opened: 2026-11-05
priority: P2
cost: M
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

## Disposition (`onecopy/refusal-standing`, 2026-11-06)

| hit | disposition |
|---|---|
| `splitting::order::axis_plane` (`undecided`) | Not a ladder. The answer is a quorum: two components agreeing. No single reading answers, so no rung can. |
| `splitting::pieced_arc::PiecedArc::contact` (`in_band`) | **Ported.** The walk's kept readings rank through `ladder::Evidence`: in band, then unsettled, as before. Its ends' `?` is filed as `an-escalation-returned-by-question-mark-ahead-of-a-later-answer`. |
| `census::Germ::of_holds` (`refused`) | Filed: `touch-unread-has-no-standing-and-two-loops-rank-it-by-hand`. |
| `census::sweep_cross_solid_backstop`'s vertex probe (`refused`) | Filed: `census-vertex-probe-keeps-a-point-in-solid-abort`. |
| `boolean::records` landing loop (`structure`) | Not a ladder. It collects every landing, not a first answer. The kept `ContactPairError` is a structure fact read by D1's exemption (`fused`, `glues`), not a band refusal. |
| `boolean::overlap::edge_witness` (`refused`) | Filed: `overlap-witness-search-ranks-its-refusals-first-of-any-kind`. This is the door's `Any` search across both faces' pieces, ranked first-of-any-kind. |
| `merge_faces::merged_outline_ring` (`zero`, `unsupported`) | Not a ladder. The answer is a uniqueness count (exactly one positive loop). A zero winding is a decided reading, and the escalation-before-unsupported order is its documented contract (`# Errors`). |
| `link::Cone::place` (`in_band`) | Rides `cone-reads-given-reference-loop-waits-on-its-unsettled-classes` (same `Unsettled<T>` classes). A section was added there. |
| `link::judge` (`refused`) | Filed: `touch-unread-has-no-standing-and-two-loops-rank-it-by-hand`. |
| `clearance` (`lower_refusal`, `upper_refusal`) | Not a ladder. It is a branch-and-bound frontier whose cells subdivide. Each bound keeps its own first refusal in frontier order, and the receipt counts every refused cell. Nothing argues the first-in-frontier order across `EngineRefusal`'s kinds; no row, by ruling. |
| `props/spiric.rs` (`first`) | Test code: a probe recording a lane's first stop. No refusal. |
| `editor-core::refactor` (`missed`) | Not a ladder. It records the first variable a remap misses, for naming. It is one kind, with no band. |
| `editor-core::names::emit_topo::merged_chord_rim` (`escalated`) | Not ported. It reads every side so that its `unreachable!` (a rim on both) and a later side's `?` refusal stand, and `climb` would stop at the first `Holds`. It keeps one kind, so its order agrees with `Evidence`. |

**Second pass** (`= Some(` of an error inside a loop). It finds
`boolean::ops`'s `torn` (a callback's error kept beside its `Unread`
stand-in), `mesh::tessellate` and `mesh::trimmed` (a first refusal that
stops the loop), `profile::path::arc_fillet`'s `whole_refused` (every
corner is needed, one kind, its own documented outranking), and
`ssi`'s and `ssi::march`'s kept values (not refusals). The viewer's
two forms are UI. None is a ladder.

It also found the class nothing kept: `?` on an escalation ahead of a
later reading (`containment::CarrierLoop::place_off`). That is filed as
`an-escalation-returned-by-question-mark-ahead-of-a-later-answer`.
