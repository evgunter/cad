# CONTACT-3 — a curved wall's chart trim stops answering for a class it misstates

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-3.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/3-wall-trim`. The row this unit carries is
`work/contact/cylinder-wall-trim-overcovers-a-tilted-section`. Read all of
it; it has the measurement, the traced ray and the cause. The pinned
witness is `crates/sweep/tests/pis_arc_capped_poses.rs::the_cut_cylinder_reads_its_truth`
(`#[ignore]`d today, green when this is fixed).

## The invariant

`boolean::solid_contain`'s ray lane decides whether a ray's hit on a
curved wall face lies ON that face. It currently answers from
`cylinder_chart_trim`'s azimuth × height rectangle, which is read off
the face's boundary VERTICES. That rectangle is the face's chart region
only when every edge of the face is iso: a rim (constant height) or a
meridian (constant azimuth). A planar section that is not
perpendicular to the axis maps to a curve `h(az)` in the chart. The
rectangle then over-covers the face on one side of the section and
under-covers it on the other. The result is **62 false `In` answers
on the cut cylinder**: confident wrong answers from the certified walk.

**After this unit, the ray lane never answers a hit on a wall face from
a region that is not that face's region.** A refusal is acceptable. A
wrong answer is not.

## Settled design

**S1. One class predicate, shared.** The face-level door
(`contain::curved_face_containment`) already asks whether a wall is
iso-bounded (`iso_bounded_wall`) before it reads the trim. The ray
lane must ask the same question through the SAME predicate. Do not
write a second spelling of it. Where the predicate currently lives
privately, widen it.

**S2. Exact where the premise is exact, confined refusal elsewhere.**

- **Exact case: walls whose non-iso edges are planar sections.** A plane
  that is not parallel to the axis meets each ruling of the cylinder
  exactly once. So membership in such a wall is: the hit's azimuth lies
  in the face's azimuth range, and the hit lies on the face's side of
  each bounding plane (rim planes and section planes alike). Decide
  each side with `decide` under the run band, as a margin in metres.
  Derive this argument yourself and state it at the site; do not take
  it from this spec. If it fails for a pose you find (a section that
  meets a ruling twice, or a wall that wraps past 2π), refuse that
  pose.
- **Everywhere else:** refuse typed, confined the way ATREST-9's
  `EdgeCarrierUnsupported` is confined. Refuse only a hit where the
  rectangle and the true region could disagree, not every hit on a
  non-iso wall.

**S3. The class, not the instance.** The cone and sphere chart trims
fold boundary images the same way, per `torus_chart_windows`' "this is a
class" paragraph, and are unmeasured under tilted sections. Measure a
tilted cut of a cone and of a sphere through `point_in_solid` on a grid
of probes clear of the boundary, at several rigid poses, as the row
did for the cylinder.

- Where either gives a false answer, fix it under the same S1/S2
  discipline in this unit if it is the same shape.
- Otherwise, file it as its own row.
- Report the measurement either way.

## Rows

- Un-`#[ignore]` `the_cut_cylinder_reads_its_truth`. It must pass:
  zero false answers. Refusals are allowed, but report their count
  against the base's 282 `VolumeUncertified`.
- Add a row for each bounding-plane side, including one with the probe
  in band of the section plane, so that it escalates rather than
  guessing.
- Add an iso-bounded wall that must still answer exactly as before (a
  plain cylinder, a meridian-cut sector).
- Add a sweep of the ray lane's other chart-trim readers. List every
  place a trim rectangle answers membership, and give its disposition.

## Out of scope

- Exact chart parity for general non-planar edge images. That is
  CHART's pcurve ground; file it if you meet it.
- `VolumeUncertified` for the tilted-section wall, which the props lane
  owns.

## Review

A **dual** review. This unit changes a certified walk that answers
wrongly today, and it makes a class decision (exact vs refuse) that
later walls will inherit.
