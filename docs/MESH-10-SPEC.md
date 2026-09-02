# MESH-10 — issue 1562: the torus extent from a split seam

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **S**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1562 is the primary specification; it was filed from MESH-7's
D9 finding (PR #1565) and issue 723 / CERT-1 are its class. Read
MESH-7's reverse red-first row
(`crates/mesh/tests/iso_rectangle_door.rs::a_split_seam_donut_pins_issue_1562s_torus_extent_limitation`),
the sweep constants' doc at `crates/mesh/src/curved.rs` (the
`TOTAL_REFUSED` paragraph), and `props/curved.rs`'s `torus_parse` /
`torus_rims_at_extremes` / `torus_ends` in full — MESH-7 factored
them to one copy for the flux lane, `boundary_material_sign` and the
shape door, so this unit changes ONE home.

## Situation

`torus_ends` takes a torus face's v-extent from the ANCHOR meridian's
stored span (`m0.dt`) — the first meridian `torus_parse` met. Sound
while a meridian is one edge. After `split_edge` on the seam meridian
the anchor reads half the span, the far rim is "not at an extreme",
and `require_rims_at_extremes` refuses `NotIsoRectangle
{ props_rim_level }` on a genuine chart rectangle — at the shape door
and at `mass_properties` alike. MESH-7's issue-653 sweep moved from
(254 meshed, 4 refused) to (250, 8) on exactly the four split-seam
donut configurations; the split-RIM donut is fine on both sides.
Reach: `split_edge` on a torus seam then `tessellate` or
`mass_properties`; or a blend whose surgery splits a torus meridian
(unmeasured). Import's tier 3 refuses such a body today by the same
limitation.

The fix is props-side, in the extent derivation, and it is the
torus arm's version of what CERT-1 did for the sphere (issue 723:
extent from the arc's span, not its endpoints).

## Deliverables

1. **The extent from the whole meridian, not the first edge.** Decide
   ONE of two shapes and say why at the site: (a) FOLD consecutive
   meridian arcs on one carrier (same minor circle: same centre on the
   tube's centre circle, same carrier axis, endpoints chaining) into
   one span before `torus_ends` reads it — the parse's job, so
   `torus_ends` stays as written; or (b) take the torus extremes from
   the RIMS (the linearly-leveled kinds' `min_max` move; the torus's v
   is periodic, which is why the anchor-span derivation exists — argue
   how the periodic ambiguity is resolved from the rims plus the
   meridian orientation before choosing this). (a) is the smaller
   change and keeps the derivation the S58/#723 record calls "the
   torus's own"; take (b) only if (a) cannot be made exact. Either
   way: no new comparand unless the fold decides carrier identity
   (then a named key, banded, stated per band — the props band, the
   same `RimArms` levers).
2. **The flip**: MESH-7's reverse red-first row
   (`a_split_seam_donut_pins_issue_1562s_torus_extent_limitation`)
   goes red on this change — rewrite it as the positive pin (the
   split-seam donut meshes AND measures V = 9.8696… bit-identical to
   the unsplit donut through `mass_properties`); the issue-653 sweep's
   constants return to (254, 4) with the `TOTAL_REFUSED` paragraph
   rewritten; the door's doc paragraph "the torus is the known case
   (issue 1562)" and the flux lane's twin sentence retire. Red-first
   the other way: the fold must not admit a meridian carried by two
   edges on DIFFERENT minor circles (two distinct carriers meeting at
   a vertex — that is a corner, not a subdivision): a row with two
   meridian arcs on different carriers still refuses as today.
3. **Every consumer flips together**: the flux lane (`curved_face`'s
   volume/area on the split-seam donut equals the unsplit donut's to
   the ulp — state the ulp), `boundary_material_sign`, the shape door,
   `mass_properties` through `topo`. One home, three callers, one
   receipt.
4. **D9 / behaviour**: mesh bytes identical on every body that meshes
   today (MESH-4's two-build digest at three ε rows over the tour
   corpus and the suites' bodies); the ONLY behavioural change is the
   four split-seam configurations now meshing — show their meshes are
   the unsplit donut's meshes up to the extra seam vertex (say how
   you compared).
5. **ε posture** (issue 1356): the fold's carrier-identity decision, if
   any, at props' band; three-ε battery; the trailer decision (the
   torus rows are `Decide`-generic — ask for the interval lane or say
   why not).
6. **Class sweep** (discipline §5): every other per-kind extent
   derivation that reads ONE edge's span (sphere's
   `sphere_meridian_span_levels` post-CERT-1 folds per arc — does it
   fold consecutive arcs on one great circle?; cylinder/cone `min_max`
   are endpoint-based and immune); every consumer of `torus_ends`
   found by compiling. Report, do not act outside the torus arm.
7. **Issue 1562 CLOSES at this merge** — say so in the PR (the
   orchestrator closes); the S-MESH slate's MESH-10 entry and the
   MESH-7 log entry's pointer are the orchestrator's to update.

## Acceptance

- The split-seam donut meshes and measures identically to the unsplit
  donut through every consumer; the sweep at (254, 4); the
  different-carrier refusal red-first; D9 digest identical elsewhere;
  hosted CI green; gate record per head.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 1562" spelled out, no
  closing keywords.
- Scope fence: `crates/geom-brep/src/props/curved.rs`'s torus parse /
  ends / rims-at-extremes only (Track R ground on this program's
  leave; NOT `props/quad.rs`, `patch_bound.rs`, the area lanes —
  S-CERT's; NOT the sphere/cylinder/cone arms beyond the sweep's
  report), `crates/mesh`'s sweep constants and the 1562 rows,
  `crates/geom-brep/tests` for the flux-lane rows. NOT: `topo`,
  `sweep::blend::surgery` (report the reach, do not measure it here),
  `validate`.
- Re-merge main before opening the PR.
