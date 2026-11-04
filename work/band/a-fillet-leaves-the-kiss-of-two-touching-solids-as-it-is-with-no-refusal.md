---
id: a-fillet-leaves-the-kiss-of-two-touching-solids-as-it-is-with-no-refusal
kind: issue
title: the fillet leaves the kiss of two touching solids as it is, with no refusal
status: open
opened: 2026-11-04
priority: P1
cost: M
design: true
refs: [6510]
---


Filed from the knife-edge fillet unit (`a-knife-edge-blend-is-unbuilt-though-its-rolling-ball-is-defined`), as the orchestrator ruled on its question B (2026-11-04).

## Measurement

Two unit tubes extruded over `z ∈ [0, 1]`, about `(0, 0)` and `(2, 0)`, kiss along the ruling `x = 1, y = 0`. `topo::union(discA, discB)` returns an `Assembly` of two solids. Each solid keeps its own edge on the kiss, a seam between its two arcs, and `topo::contact_marks` reads both edges `SmoothUnderdetermined` (`crates/sweep/tests/wedge_end_doors.rs:831` pins the touch at every op).

Requested with `fillet_edges` or `chamfer_edges` at 0.1, each kiss edge is read by predicate 5 as a smooth seam (`crates/sweep/src/blend/battery.rs:781`), resolved as `Resolved::Smooth` (`battery.rs:1195`) and left out of every chain (`battery.rs:2035`). Both verbs return the operand **as it is, with no refusal**. A user who asked for "a fillet between the two tubes" gets nothing, and nothing says why. `crates/sweep/tests/a_knife_edge_fillet_builds_where_its_ball_is_defined.rs:626` pins this outcome today.

Each edge is a smooth seam *of its own solid*: on one solid the kiss is wedge π. The valley the user means lies between the two solids, and neither solid's edge carries it.

## The route that works

A plate whose top face passes through both tube axes and the kiss, unioned with the two tubes in either order (`(plate ∪ left) ∪ right` or `(left ∪ right) ∪ plate`), is one solid (`Seamed`) with one knife edge (a slit) on the kiss. The fillet fills that valley exactly (`a_knife_edge_fillet_builds_where_its_ball_is_defined.rs:379`, ΔV checked against the closed form).

## Options (Ev's call)

1. **A blend that fuses the two solids**: read the request across the touch as the valley's two supports, fill both valleys of the kiss (above and below the line of centres), and return one solid. This is a new kind of band: it changes the result's solid count, and it needs a rule for which side or sides of the kiss it fills.
2. **A refusal that names the touch**: the kiss edge of an assembly refuses typed (the edge touches another solid of the assembly, so its blend is between solids), with the plate-union route as the recourse.
3. **Keep the silent no-op**: the kiss is a seam of each solid, and a requested seam is left as it is everywhere else.
