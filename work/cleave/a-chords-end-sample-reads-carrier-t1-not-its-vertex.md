---
id: a-chords-end-sample-reads-carrier-t1-not-its-vertex
kind: issue
title: Certification reads an edge's end sample at carrier(t1), one rounding off its vertex, so at the band's edge its verdict depends on the edge's orientation
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: cleave/chord-end-sample
closed: 2026-10-11
pr: 4732
---


## What

Found by the review of PR 4625 (`cleave/folded-graze-join-desync`).
`geom-brep/src/certify.rs` `certify`'s check 4 (the per-sample
`Intersection` residuals, `let p = spec.carrier.eval(t)` near line
2647) reads every sample, the two ends included, through the carrier.
For a `line_between` chord (`EdgeCurveSpec::line_between`, ~1035) the
start sample is `origin + 0·dir`, the start vertex bit for bit, and the
end sample is `origin + len·(p1 − p0)/len`, which lands up to one
rounding off the end vertex.

Witness: `sweep` `near_tangent_pierce_poses`' `EDGE_ON_FACES`, e.g.
`Ltop nt e2 a13 d-1e-9` at ε = 1e-9. The far vertex of the prism's
vertical edge lies 9.999999994736444e-10 m off the cube's near face
(exact over the stored `f64`s: 5e-19 inside the zero band's edge). The
section chord pierce → vertex evaluates the vertex as
(1, 0.9999999999999999, 0), whose residual is 1.0000000518091955e-9:
just in band, so the run refuses `Escalated { EdgeOnFaces }`. The
chord vertex → pierce, in the other three runs of the pose, reads the
vertex as its origin, in the zero band, and builds. One input, two
verdicts, by the edge's orientation.

## Owed

(Q1) Re-pose the end samples at the edge's vertices, the points the
endpoint checks (`EndpointStart`, `EndpointEnd`) already pin the
carrier to, so check 4 reads the same point at either orientation; or
argue that the carrier, not the vertex, is what check 4 certifies and
that the orientation dependence at the band's edge is admissible.
Either way the `EDGE_ON_FACES` pin's run list moves with it.

## Built (branch cleave/chord-end-sample)

Decided: the carrier, not the vertex, is what check 4 certifies; the
orientation dependence at the band's edge stands. No kernel change.

- **The ratified text.** D4 ¶2 puts the invariant on the derived cache
  ("residual ≤ ε for every derived item"), and the carrier is that cache.
  Re-posing the end samples at the vertices bounds a carrier end only
  through check 3: vertex ≤ ε off the surface plus carrier end ≤ ε off
  the vertex, up to 2ε. That is a decision inferred from two others, which
  Q1 rules out ("Closeness within ε is not an identity"). The
  tangent arm's C2.2 sup bound (`TangentHull`, `tangent.rs`
  `TangentSpanBounds`) is a bound on sampled carrier residuals plus the
  sag between them, so it needs carrier samples at the ends too.
- **Not Q1's amplified error.** `carrier(t₁)` is one rounding off the
  vertex: 1.1e-16 m at the 1 m model extent, moving the residual by
  5.2e-17 (9.999999994736444e-10 at the vertex, 1.0000000518091955e-9 at
  the carrier's end). That is the few-ulp noise K's headroom presumes.
  The in-band reading escalates, which is escalate-never-guess. Both runs
  are sound: one certifies a carrier inside ε, and the other refuses an
  in-band reading with the input's recourse.
- **What a re-pose would have moved** (prototype: check 4's
  `Intersection` arm reading `start`/`end` at samples 0/8; full
  `near_tangent_census_probe`, main against prototype):
  - ε = 1e-9: the 12 `EDGE_ON_FACES` refusals (4 poses × 3 runs) all
    build, and the census calls them SOUND. They only pass because the
    validator re-certifies through the same re-posed check: each stored
    carrier ends 1.0000000518e-9 or 1.0000000241e-9 off the face, past ε.
  - ε = 1e-6: no change.
  - ε = 1e-12: 6 runs (`w345 nt e1 a12 d±1e-6`, the `cp` ops) change
    their refusal. On main they refuse with the input's own coincidence,
    `Escalated Coincidence(VertexOnFace)` / `Containment` in band. Under
    the re-pose they refuse earlier, as a `CrossingInsertion` certification
    escalation on `SurfaceResidual`, because the inserted vertex is now
    read against the surfaces in place of the carrier.
- **Pin.** `geom-brep/tests/end_samples_read_the_carrier.rs` holds two
  families at every ε row, each at both orientations:
  - a carrier end just past the zero band (r ∈ {1.01, 1.07, 1.14}·ε)
    whose vertex reads inside it escalates `SurfaceResidual` at the far
    end's sample;
  - a carrier inside the band whose vertex reads in band certifies.

  Both go red under the prototype re-pose. The `EDGE_ON_FACES` pin is
  unchanged. Its docs, and `BooleanDecision::EdgeOnFaces`'s, now state
  the invariant rather than cite this row; `certify`'s check-4 rustdoc
  states it at the owning decision.
- **Filed:**
  `work/restfront/tier-3s-planar-arm-never-reads-an-edges-carrier-ends.md`.
  It covers the converse found by the sweep: tier 3's planar arm, and
  `attach.rs` `edge_on_plane` after it, read an edge's ends at its
  vertices and never at its carrier.

## Closed (PR 4732, 2026-10-11)

Ruled the second way: check 4 certifies the carrier (D4 ¶2), so its end samples stay carrier points. Reading the vertex instead would bound the carrier's end only through check 3 plus ε, which infers a decision from two others (Q1). The one-rounding move is K-headroom noise, not an amplified error, and both orientations answer soundly (build, or escalate in band). The rule is pinned by `geom-brep` `end_samples_read_the_carrier`, which went red under a vertex-reading prototype.
