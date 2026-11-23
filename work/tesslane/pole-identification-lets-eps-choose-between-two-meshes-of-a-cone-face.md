---
id: pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face
kind: issue
title: A vertex the band places ON a cone's apex at one ε and OFF it at another is meshed two ways at one δ: the guard DESIGN says pole identification owes is not built
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Found by unit 4c's implementer (PR 7096) in its sweep for places where ε chooses between two meshes, 2026-11-23. The trigger is main's (one classification of one vertex against the apex); on a head with the walk its consequence is wider, as a review of that pull request noted.

## What

`docs/DESIGN.md` `[tolerance.chordal]`: "The mesh layer never reads ε for sizing … the one ε read that is a classification — pole identification — owes a guard so an ε flip can never move emitted coordinates with δ held fixed (#896)."

The read is `AnalyticRow::is_singular` at the run's band, through `trimmed::singular_vertices` on the structural arms and through the route's apex fact (`chart_table::at_a_singular_point`, `trimmed::walkable`) on the walk. Where the band cannot place a vertex on or off the apex, both refuse `SphereTrimShape { SingularUndecided }`: a refusal against a mesh, which the page allows.

Where the band DECIDES the vertex on the apex at one ε and off it at another (a split nearer the apex than the band at 1e-6 and farther than it at 1e-12), the face is meshed two ways at one δ: with an apex row, or with the vertex as a rim point. On the structural arms that is two polygons in one lane at one target. With the walk it also switches the LANE (walked against structural), and with the lane the face's target (δ against the headroom's share of it) and so the chords of the rims its neighbours share. `split_rulings::a_split_beside_the_apex_of_a_needle_cone_refuses_undecided` pins a cell that refuses at the default ε and meshes walked at 1e-12. That is the case the page says owes a guard, and no guard is built on either lane.

## What closing it takes

A body that reaches it, pinned at two ε (a cone face with a side split within 10ε of its apex at the larger ε); then the guard the page names: a decision that does not move emitted coordinates when ε moves, or a refusal on the cells where it would.
