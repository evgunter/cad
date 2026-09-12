---
id: edge-nurbs-computes-the-chart-image-and-discards-it
kind: issue
title: edge_nurbs derives a full chart image at certification time and returns only scalars — the compute-and-discard sibling of TCOST-K3
status: closed
opened: 2026-09-03
closed: 2026-09-11
---


The TCOST-K3 shape — *a derivation is computed to decide something,
the decision is returned, and the object is thrown away* — has a
named sibling that another spec already found and could not use.

`crates/geom-brep/src/edge_nurbs.rs:330`-`:336` derives the chart image
of a declared carrier on a NURBS wall (33 certified foot points,
interpolated on the carrier's own parameter, `on_carrier_domain`-lifted)
at EDGE CERTIFICATION time, and returns only the `PlaneNurbsLimbs`
scalars. `docs/PCURVE-P2-SPEC.md:59` names this site verbatim: it says
the derivation "already derives exactly this image ... and then THROWS
IT AWAY", and that a P-2 consumer needing that image should prefer an
existing producer to writing a third.

So there are two consumers of one derivation and no way to hand it
over — the TCOST-K3 situation exactly, one layer down.

**Why this is a candidate and not yet a unit.** Unlike K3, the cost
has not been measured: nobody has instrumented how much of an edge
certification the image is, nor how often a caller that certifies also
wants it. K3's own stop clause is the model — measure first, and if
the second derivation is a small fraction of its caller, the
redundancy is not what the finding says.

**Places to look**, from the K3 sweep for this shape:

- `crates/topo/src/census.rs`' `census_and_certify` — called by the
  tier-3′ door immediately after its check-7 certificate, and its
  product is a verdict vector;
- `PropsQuadLane::recertify_approx` — re-derives a surface certificate
  per validation pass, by design (tier 3's never-trust posture), which
  is the case where the discard is CORRECT and the unit would be wrong
  to collapse it. It is listed so the sweep's blind spot is stated:
  not every compute-and-discard is redundant, and telling them apart
  is the work.

## Closed (2026-09-11): measured, and the premise is false

Measured at the S-TCOST re-sort, after the board's sort put this row in
"unclear — no measurement exists". The stop clause this file set for
itself (*"the cost has not been measured: nobody has instrumented how
much of an edge certification the image is"*) is now spent, and the
answer closes the row twice over.

**1. The citation drifted, and the image is NOT discarded.** At
`d6a9b948` the derivation is `edge_nurbs.rs:308-336` (`:330-336`, this
file's citation, now lands inside the transversality hook's tail, not on
the derivation). Thirty lines later `edge_nurbs.rs:354-362` passes it
straight into the certificate:

```
let cert = certify_rung3(carrier, Some(&pcurve), …)
```

and `ssi/certify.rs:795 certify_branch` REFUSES `UnsupportedCertificate`
without it, in two places: `:810-815` (*"a NURBS operand's limbs need
the traced pcurve"* — limbs 1 and 2 on the wall side) and `:851-854`
(limb 3's chart uniqueness tube). Every `PlaneNurbsLimbs` field except
`min_sin_theta` comes out of that certificate. Delete the image and the
lane returns nothing. There is no compute-and-discard here to remove.

What is true is weaker and is not what this row claimed: the image is
not *returned*, so the pcurve mint (`pcurve_cache.rs:1229
general_image_lane`) derives its own. That is a redundancy ACROSS
PASSES, not within one.

**2. And the prize is small anyway.** LOCAL readings on this container
(4 cores / 15 GB, shared), single-threaded medians over 5 certifying
calls, driven by `m7_8_plane_nurbs_edge` — an iteration tool and NOT a
result of record (`memories/perf-measurement-lane.md`); a unit would
need a hosted before/after:

| profile | `chart_image` | whole `run_checks` edge certification | share |
|---|---|---|---|
| dev (opt 0) | 2.79 ms | 22.16 ms | **12.6 %** |
| release (debug-assertions on) | 0.461 ms | 4.86 ms | **9.5 %** |

One call per plane×NURBS edge per validation pass, not in a loop: 33
cold `NurbsSurface::project` calls against limb 1's 9 warm
`project_from_seed` ones. So ~0.46 ms per such edge in release.

**3. Collapsing the cross-pass redundancy is a design conversation, not
a cost lever.** It means the certifier reading the stored cache, and
`topo/src/validate.rs:3297-3299` ratifies the opposite posture —
*"Re-certification re-derives; it never trusts the stored
certificate."* That is the same reason this file's own
`PropsQuadLane::recertify_approx` sibling is listed as the case where a
discard is CORRECT. Anyone who wants it takes the never-trust question
to Ev first, on evidence this row does not have.

Closed as a negative result. The mint side is unmeasured — no geom-brep
suite reaches `general_image_lane`, and a `topo`/`sweep` build for it
was out of the measurement's budget; that is recorded here rather than
left as an implied gap, and nothing on any slate waits on it.

Residue, filed rather than disclosed in prose (`work/README.md`): the
stale sentence this row was built on is still in the spec, and
`docs/PCURVE-P2-SPEC.md` and `edge_nurbs.rs` are both TRIM's territory,
so it is filed there as
`work/trim/pcurve-p2-spec-says-edge-nurbs-throws-the-image-away.md`.
