---
id: revolve-carriers-state-only-the-rim
kind: issue
title: the revolve latitude carriers register the rim identity but cannot state the span one: the builder is never handed the far endpoint
status: open
opened: 2026-09-06
---


**Opened by M10-9's fix pass** (R2 MINOR-5: spec §2 amendment A1's
"the unit of scope is the CONSTRUCTOR" was applied to one constructor
of two).

Two more builders mint a `Curve3::Circle` from a point and a centre
under the same guarantee as the swept arc carrier's:

- `crates/sweep/src/revolve/surfaces.rs`, `revolved_strut_spec` — the
  latitude strut/rim, `center = frame.foot3(point)`, `radius` the
  sketch point's radial extent from the same axis;
- `crates/sweep/src/revolve/full.rs`, the band-2 rim spec — the same
  shape at `cls.verts[..].r`.

**The rim identity IS stated there now** (`‖q − center‖ = radius`,
through `swept::register_rim_identity`, whose doc comment carries the
argument): the centre is the foot of the perpendicular from the sketch
point to the axis, the radius is that point's radial extent from the
same axis, and both are placed by one rigid `frame.place`. Both files
are on `scripts/gates/register-equal-allowlist.sh`.

**The SPAN identity is not, and cannot be, at these sites.** The
carrier's far endpoint is `carrier.eval(param_end)`, and the
registration has to be made against the vertex the topology will pin it
to — `q_to`. `revolved_strut_spec` is handed `point`, `radius`, `q`
(the START point), the frame, `theta` and the carrier axis, and
`full.rs`'s band-2 spec is handed `qpi[i]` and `half`; neither is given
the far vertex, which the caller mints separately. Stating the identity
would mean either passing `q_to` down (a signature change through the
revolve pipeline) or re-deriving it in the builder (a second expression
for the same point, which is exactly the same-object condition
failing).

## What is owed

- A decision on whether the revolve pipeline should hand its carrier
  builders both endpoints, as `sweep::swept::placed_segment_spec`
  already does. It is a signature change, not a tier change, and it
  would make the span identity stateable at three constructors instead
  of one.
- The measurement that would justify it: no M10 driver document is
  bounded by a revolve carrier's endpoint pinning today
  (`m10_5_r1_probes_interval`'s quarter-revolve is the only revolve
  fixture in the driver's suites, and it is not one of the four
  documents whose ceilings M10-9 measures). Until one is, this is
  machinery for zero certificate content (ERROR-DESIGN E6) and the row
  stays filed rather than built.
