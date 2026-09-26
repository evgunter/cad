# CONTACT-4: contfp reads every loop on its edges' own carriers

This file is the PR body for the combined landing. The orchestrator
folds it into that PR and deletes it at merge.

Carries `work/contact/contfp-walks-the-vertex-polygon-of-an-arc-bearing-loop`
(P0) and `work/contact/point-on-arc-endpoint-zone-compresses-by-sin-half-width`
(P1).

## What changed, and why

### `contfp`'s interior walk

`boolean::contain::contfp` used to pick a walk per loop from
`loop_shape`:
- `Disc` went to `disc_side`;
- `Polygon` and `ArcParity` went to `point_in_loop`, the vertex-polygon
  ray parity;
- `NoWalk` was refused as `ArcLoopUnsupported`.

`ArcParity` was answered from a polygon that is not the loop's region.
An outward arc leaves region between the polygon and the boundary, and
a point there read `Out`.

`contfp` now reads its outer loop and every ring through
`splitting::containment::point_in_carrier_loop` (ATREST-9). The walk's
`None` becomes `ContainError::ArcLoopUnsupported { loop }`. `None`
means a spiric or spline edge the walk cannot cross, with `q` inside
that edge's reach ball. The walk's `OnBoundary` still maps to
`Escalated(bool_contfp_boundary)`, as before. By then the typed
pre-pass has already said `q` is off every edge it can name.

What happened to each `LoopShape` arm inside `contfp`:

| arm | fate in `contfp` | why |
|---|---|---|
| `Polygon` | deleted | The carrier walk delegates an all-line loop to `point_in_loop` unchanged (`edges.iter().all(Chord)`), so the answer is bit-identical. |
| `Disc` | deleted | The walk is exact on a loop of one circle's arcs: one conic row, and the same radial question as the pre-pass `conic_on` row. It refuses nothing `disc_side` answers, because it refuses only spiric or spline edges. No hot path was identified where one radial decide beats ≤16 one-root rays, so no speed claim is made and none is needed to delete it. |
| `ArcParity` | deleted | This is the defect: a polygon that is not the region. |
| `NoWalk` | deleted | Its refusal is answered. A half-disc cap, a half-cylinder cap and a lens are walked on their carriers. |

`LoopShape`, `loop_shape`, `LoopCircle` and `disc_side` stay in
`contain.rs`. Their only consumer is now tier 3's check 9
(`validate.rs`), which ATREST-12 moves onto the same walk. After that
they have no caller, and ATREST retires them. Their docs were
rewritten to say they are check 9's classification. `validate.rs` is
not touched.

### `boundary_pre_pass` and `point_on_arc`

- **`UnrowedCarriers` is deleted, and with it the `Chord` arm.** Its
  only user was `contfp`. That arm decided an `Ellipse`, `Spiric` or
  `Nurbs` edge by its chord. On a planar face an elliptical rim's chord
  runs through the interior, so a chord verdict there was a wrong
  `OnEdge`. The pre-pass now has one mode: a `Line` takes the chord
  rows, a `Circle` takes `point_on_arc`, null scaffolding is read as
  its chord (as the region walk reads it), and every other carrier gets
  no verdict. For an ellipse, the region walk that follows reads the
  edge on its conic. A point on the arc comes back `OnBoundary` and
  escalates typed.
  - One side effect on the curved door (`curved_boundary_containment`):
    a null-scaffolding edge used to be skipped and is now read by its
    chord rows. The operand gate refuses scaffolding upstream, so no
    test moved.
- **`point_on_arc` keeps its callers:** the pre-pass for both the
  planar and the curved door. So it is rewritten rather than deleted.
  Its angular test was the cosine window
  `Margin::levered(r̂·m̂ − cos(w/2), R)`. For a point an arc length `s`
  past an end, that margin is about `−sin(w/2)·s`, so the window's
  `Zero` reached `ε/sin(w/2)` along the carrier and its escalation
  `10ε/sin(w/2)`.
- **The new shared helper is `splitting::containment::arc_trim`.** It
  works in the conic's unit coordinates:
  1. The chord distance to either end. `Zero` gives `ArcTrim::End`,
     which the vertex pass owns.
  2. Otherwise, the chordal-defect sum
     `(|e − m| − |p − m|) + (|p + m| − |e + m|)`. This is ATREST-11's
     `validate::window` construction, derived in the helper's doc. The
     sum is `2(g(α) − g(w/2))` for `g(x) = cos(x/2) − sin(x/2)`, whose
     slope is at least ½ on `[0, π]`. So the margin is never smaller
     than the arc length to the nearer end.
- **The carrier walk's own boundary pre-pass now reads `arc_trim`**
  instead of `ConicArc::in_window`, which had the same compression.
  `in_window` still serves the ray's crossing count. There a `Zero`
  abandons the ray, so the compression costs a short arc rays, never a
  count.

**The `point_on_arc` row: retired, but not by the switch alone.**
Measured, not assumed. The walk's pre-pass used the same cosine window,
so switching `contfp` onto it would have moved the compressed zone, not
removed it. On the base, the rows below read as follows:
- a point 50ε inside the end of a `w = 0.02`, `R = 10` arc escalates
  `bool_contfp_boundary`;
- on the near-full mirror (`w = 2π − 0.02`), the same point reads
  `Out` (wrong).

Both doors read `arc_trim` now, and the rows are green at every ε row.

### The refusal's meaning

`ContainError::ArcLoopUnsupported` keeps its name, because five callers
and the attribution suite match it. It now means "a spiric or spline
edge within reach of the point". Its doc and `Display`, and
`BooleanError::ArcLoopContainmentUnsupported`'s doc and message, say so
now. `validate.rs`'s `classify_contain` still renders the old sentence
("arcs over fewer than three corners"). That is ATREST's ground, filed
there.

### Predicate roster

There are four new names, each pair carried by an `ArcTrimRows` value:
- `point_in_arc_loop_conic_{end,trim}`, for the walk's pre-pass;
- `bool_contact_arc_{end,trim}`, for `point_on_arc`.

`docs/K-REPORT.md` has a roster addition and
`docs/predicate-dimension-audit.md` has two rows.
`point_in_arc_loop_conic_window` now decides only ray crossings, and
`bool_contact_arc_span` only `point_on_arc`'s period guard.

## Per-caller mapping

Every caller already matched all four `ContainError` arms, and none of
their match arms changed. What changes is which inputs reach which arm.

| caller | `Escalated` | `RayExhausted` | `Corrupt` | `ArcLoopUnsupported` | non-`Out` verdicts |
|---|---|---|---|---|---|
| `reduce.rs` edge-crossing, root lane (`contfp(y, face, …)` after `roots.first()`) | `BooleanError::Escalated` | `ClassificationInvariant` | `CorruptOperand` | `ArcLoopContainmentUnsupported { operand, loop }` | trace-accepted |
| `reduce.rs` edge-crossing, line lane (the `d1/(d1 − d2)` crossing) | same `esc` | same | same | same | same |
| `reduce.rs` vertex-on-face (`vertex_face_contact`, `esc(e, x_is.other())`) | same `esc` | same | same | same | `In` → vf contact; `OnEdge` → split the edge; `OnVertex` → vv |
| `ops.rs` extent scan (`contfp(y, yf, normal, witness, …)`) | `Escalated` | `ClassificationInvariant("extent scan: …")` | `ClassificationInvariant("… corrupt …")` | `ArcLoopContainmentUnsupported` | `Out` skips, `In` escapes |
| `census.rs` `contain` helper | `CensusEscalated` | `CensusUnsupported(Containment)` | same | same | the census's own arms |
| `chart_region.rs` witness search (`inside`) | not a witness | not a witness | not a witness | not a witness | only `In` counts |

**Answer → refusal**, each with its reason:
- **A planar face with a spiric or spline edge, asked about a point
  inside that edge's reach ball** (the torus's `R + r`, or a NURBS
  control hull's reach). The base answered from the vertex polygon,
  which is not the region. The head refuses `ArcLoopUnsupported`: the
  walk has no crossing row for the edge.
  - The sectioned torus vessel's cavity carries such a cap
    (`spiric_rim.rs`'s `vessel_cavity`), and its
    `validate_pseudomanifold` stops at `VolumeUncomputable` before the
    census.
  - No test on the tree reaches this change. The full `topo` and
    `sweep` suites are green.
- **A point ON an elliptic arc edge** (off its chord). The base
  answered `In`/`Out` from the polygon, which is wrong because the
  point is on the boundary. The head gives `Escalated(bool_contfp_boundary)`
  from the walk's `OnBoundary`. Filed as
  `work/contact/contfp-has-no-typed-on-edge-row-for-an-elliptic-arc`.

**Wrong → right.** Each row below is red on the base and green at the
head, at every ε row:

| row | base | head |
|---|---|---|
| bored D-rod (flat 0.3, bore 0.15, both creases filleted), transverse caps `z = 0, L`, lune point `(−0.3, 0)` via `contfp` | **`Out`** | **`In`** |
| the same cap with a brick standing in the lune, through `validate_pseudomanifold` (public door) | 0 `VertexOnFace` contacts: the body passes the vertex-on-face sweep silently | 4 `VertexOnFace` contacts on the cap |
| slot (4 vertices, two semicircles), 41×25 grid | `Out` inside the semicircular ends, e.g. `(−1.4388, −0.2390)` | every probe matches the analytic region |
| rounded rectangle (8 vertices, four quarter arcs), same grid | not reached on the base: the row stops at the slot's first wrong probe | every probe matches the analytic region |
| pie slice `w = 0.02`, `R = 10`: 50ε and 500ε inside the end | `Escalated(bool_contfp_boundary)` | `OnEdge(arc)` |
| the same, 50ε and 500ε past the end | not reached on the base (the row stops at its first case) | `Out` |
| pac-man `w = 2π − 0.02`: 50ε inside the end | **`Out`** | `OnEdge(arc)` |
| the same, past the end (in the mouth), and deep in the body | not reached on the base (the row stops at its first case); its vertex polygon is the mouth triangle, not the region | `Out`, and `In` |
| `point_on_arc` unit row, both arcs, `s ∈ {20, 50, 500, 5000}ε` on both sides of both ends | `None` inside the compressed zone | `Some(true)` / `Some(false)`; `None` only within the band |
| `verbs_pierce_r1` half-disc cap × box, both senses | the containing sense refuses `ArcLoopContainmentUnsupported` | buried truth `π + 0.09` and disjoint `π + 0.18`, one each |
| `verbs_pierce_r2` half-disc cap × box, both senses | refuses | buried and disjoint truths |
| `verbs_pierce_r2` lens cap × box | refuses | `va + 0.04` (exact union) |
| `census_containment_cause` lens under a box | `CensusUnsupported(ArcLoopUnsupported)` ×N | 4 `VertexOnFace` contacts, no refusal |

The spec said the slot and the rounded rectangle "must answer as
before". That holds only where the polygon and the region agree. In the
arc bulges the base was wrong, and the head is right there.

## Rows moved, added, deleted

- **Added.** `crates/sweep/tests/contfp_reads_arcs_on_their_carriers.rs`
  has five rows: D-rod lune via `contfp`; D-rod lune via the census;
  slot and rounded rectangle; short arc; near-full arc.
  `contain::tests::an_arcs_end_zone_is_the_bands_own_width` is the
  sixth.
- **Re-signed to the right answer.** `verbs_pierce_r1_probes::r1_a_box_through_a_half_disc_cap`,
  `verbs_pierce_r2_probes::r2_a_box_through_a_half_disc_cap_measures_the_mixed_loop_remainder`
  and `…::r2_a_box_through_a_lens_cap_measures_the_all_arc_remainder`
  asserted the refusal. They now assert the exact volumes.
- **Reshaped.** `census::tests::a_region_walk_refusal_on_a_separated_vertex_is_the_filters_to_answer`
  became `a_separated_vertex_in_a_half_disc_caps_plane_is_decided_by_both_sweeps`.
  The half-disc cap no longer refuses, so the pruning filter's
  refusal-class member has no fixture. Both strategies now decide the
  far vertices, with no containment refusal.
- **Deleted.** `census_containment_cause::no_fabricated_containment_margin`
  and `…::the_arc_loop_refusal_reaches_the_user_as_itself`. Their lens
  fixture no longer refuses, so neither could fire. A spiric-cap
  replacement was tried and stops at `VolumeUncomputable` first.
  `…::the_lens_cap_is_read_as_a_region` replaces them. Filed as
  `work/contact/census-containment-refusal-carriage-has-no-executed-fixture`.

## Sweep

**Pattern 1: a vertex-polygon walk handed a loop that may carry an
arc.** `grep -rn 'point_in_loop('` over `crates/`:

| hit | disposition |
|---|---|
| `boolean/contain.rs` `contfp` (`Polygon`/`ArcParity` arm) | **fixed** here |
| `splitting/containment.rs` `point_in_carrier_loop`'s all-`Chord` delegation | correct: every edge is a line |
| `solid_contain::point_in_face` | fixed by ATREST-9 (reads `point_in_carrier_loop`) |
| `chord_join::rehome_rings` (`point_in_loop` over the split's run) | not this unit: REACH/TANG ground, already filed as `work/reach/rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk` |
| `validate.rs` check 9 nesting arm | gated to `Polygon`/`Disc`, so correct as gated; ATREST-12 moves it onto the walk |
| `topo/tests/review_m3_pr3_pil.rs`, `m3_pr3_split.rs` | test fixtures on polygon loops |

**Blind spot of pattern 1: a walk that reaches `ray_parity` directly,
not through `point_in_loop`.** Second pass: `grep ray_parity::` for
consumers of `on_boundary`, `on_segment`, `ray_verdict` and
`ray_crossings`.

| hit | disposition |
|---|---|
| `chart_region.rs` `point_in_polygon` | not an instance: its polygon comes from `loop_uv_polygon`, which refuses every non-straight chart image (`Harmonic` with live trig channels, `Fitted`, `General`), so every edge is exact |
| `chart_bound.rs` | not an instance: it carries each arc's envelope `E_L` beside the chord polygon by construction |
| `splitting/containment.rs` | the walk itself |

**Pattern 2: a cosine-window end zone read as a verdict.**
`grep -rn 'cos_half\|cos(w/2)\|r̂·m̂\|(width \* half).sin_cos'`:

| hit | disposition |
|---|---|
| `contain::point_on_arc` | **fixed** (reads `arc_trim`) |
| `containment` pre-pass (`in_window`) | **fixed** (reads `arc_trim`) |
| `containment::conic_crossings` (`in_window`) | kept: `Zero` there is a graze that abandons a ray, never a verdict |
| `solid_contain::point_on_wall_in_face` / `point_on_cone_in_face` / `point_on_torus_in_face` (`chart_azimuth_margin`), `point_on_sphere_in_face` | not this unit: a `Zero` there is a boundary graze on a ray lane (the doc's own ledger row F8 owns the narrow-window fix) |
| `curved_face_containment` period guard | a chart-form question, not an end zone |

`solid_contain`'s by-hand site list for the construction is updated:
`point_on_arc` is off it, and `ConicArc::in_window` (never listed) is
on it. The count stays at three restated sites.

**The pattern's blind spot:** an end-zone test spelled without a
cosine, such as an `atan2` window. `grep atan2` over `topo/src/boolean`
and `topo/src/splitting` finds five sites, and none is a window
membership test:
- `join.rs`: a test's round trip onto an ellipse;
- `boxes.rs` ×3: a box's extremal angles;
- `classify.rs`: a crossing root's phase, behind its own frame trilean.

## Territory seam

- `crates/topo/src/splitting/containment.rs` is REACH's path, with the
  walk authored by ATREST-9. The edits there are the `arc_trim` helper,
  `ArcTrimRows`/`ArcTrim`, `ConicArc::span`, and the pre-pass switch.
  They are announced on `work/atrest/log.md`, which is where the walk's
  author reads. `scripts/work.py territory` flags this path.
- `validate.rs` is untouched. After this change `LoopShape` has one
  consumer, check 9. That seam is announced on `work/atrest/log.md`,
  and its stale text is filed as
  `work/atrest/check-9-and-classify-contain-describe-contfps-retired-polygon-walk`.
- `work/tang/arc-aware-point-in-loop.md` (#1076) has a dated note that
  the `contfp` site is closed.
- `docs/KERNEL-VERBS.md`: two sentences that described `contfp`'s
  posture on `ArcParity` and its use of `disc_side` are re-worded to the
  moved code. They were never ratified: `git log -S` finds them first
  written by `4d7faeb02`, an agent commit.

## Filed

- `work/atrest/check-9-and-classify-contain-describe-contfps-retired-polygon-walk` (P2)
- `work/contact/census-containment-refusal-carriage-has-no-executed-fixture` (P3)
- `work/contact/contfp-has-no-typed-on-edge-row-for-an-elliptic-arc` (P2)

## Local verification

- `cargo nextest run -p topo -p sweep --no-fail-fast` at eps unset,
  1e-6 and 1e-12: 3145 tests.
  - The first two rows ran before the three `verbs_pierce` re-signs and
    failed exactly those three.
  - The 1e-12 row ran after them and is fully green.
  - The three re-signed suites, the new suite and
    `census_containment_cause` were re-run at unset and 1e-6: green.
- The same filter was run against the base kernel (`crates/topo/src` at
  `origin/main`, head tests): all nine new or re-signed sweep rows are
  red.
- `cargo clippy -p topo -p sweep --all-targets -- -D warnings`: clean.
- `scripts/doc-gate.sh --skip-viewer-toolkit`: OK.
- `python3 scripts/work.py lint`: ok.
