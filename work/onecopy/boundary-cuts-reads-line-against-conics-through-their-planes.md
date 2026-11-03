---
id: boundary-cuts-reads-line-against-conics-through-their-planes
kind: issue
title: boolean::boundary_cuts reads a line against boundary conics and lines by plane crossings and end offsets, a proxy of the curve-pair meeting question beside meet's LineConic and LineLine arms
status: dispatched
opened: 2026-11-02
priority: P1
cost: M
refs: [6327]
branch: onecopy/curve-pair-reads
---

Found in the review of ONEHOME's PR 6327, which moved `reduce::boundary_meets_circle_only_at` off the same kind of proxy and onto the boundary pass (`geom_brep::curve_face::read`).

`boolean/mod.rs` `boundary_cuts` lists where a face's boundary can meet a line (`origin + t·dir` over `span`). It answers that with its own rows:
- a straight edge: its ends' offsets across the line, interpolated;
- a circle or an ellipse: the line's crossing of the conic's plane (`bool_tangent_line_cut_plane`), or the conic's in-plane roots where the line lies within the band of the plane (`bool_tangent_line_cut_swing`);
- a spiric or a spline edge: `None`.

That is the curve-pair meeting question, a line span against each boundary edge's span. `geom_brep::curve_pair::meet` answers it through its `LineLine` and `LineConic` arms, and its walk covers spiric and spline edges. DESIGN `[one-way.readings]` and C5 say one reading per question.

The two can decide one fact differently in the band:
- `meet` polishes a plane crossing to the closest pair;
- `meet` reads a level line against the conic as a first harmonic with its own extreme rows.

`boundary_cuts` keeps parameters that are no meeting (they "only cut a run in two"). So the route is to take the boundary pass's meetings (`curve_face::read` over the line's span, or the per-edge `meet`) as the cut list, after checking whether its caller needs anything the meetings do not carry.

## Slate triage (2026-11-03, ONEHOME cut)

- Moved to ONECOPY (`work/onecopy/`) by `git mv`, id and body unchanged. ONEHOME measured 70 budget points against 35 when its orchestrator ran out of usage; the rows it had not started (no lane, no design fork, no in-flight PR riding them) were cut onto ONECOPY, and what is in flight or staged stayed on ONEHOME.
