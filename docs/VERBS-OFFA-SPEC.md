# VERBS-OFF-A — analytic offsets: the mint table and its refusals

Wave 3 unit 1 of `docs/VERBS-PLAN.md`, per the ratified
`docs/OFFSET-DESIGN.md` O1 (Evan 👍, #907). Branch `verbs/offa`,
PR to main. Difficulty logged pre-dispatch: **S**. Substrate
evidence: `docs/Q8-SUBSTRATE-2026-08-21.md` §1 (re-verify anchors —
the snapshot predates Waves 1–2).

## The verb (geometry-layer, no topology)

`geom_brep::offset::offset_surface(&Surface<T>, d, band) ->
Result<Surface<T>, OffsetError>` (home and module name are the
implementer's call within `geom-brep` — the door needs the decide
funnel, which `geom` does not depend on; a sibling table to
`revolve/surfaces.rs::wall_surface`'s shape). Signed d: positive
along the stored chart normal. The mint is struct-update on public
fields, exactly O1's table:

- Plane → `origin + d·normal`, u_ref carried.
- Cylinder → `radius + d` (sign folded via the chart normal's
  outward sense — the parameter d is along the NORMAL, so the
  radial change is +d by the chart convention; state this and pin
  it, the sign convention is the unit's one subtle spot).
- Sphere → `radius + d`, center/axis/u_ref carried.
- Torus → `minor_radius + d`, all else carried.
- Cone → apex slides `d/sin(half_angle)` along the axis (direction
  from the chart normal's side), `half_angle` carried. The slide
  direction's sign derivation gets the ARMS-2 treatment: derived
  from stored structure (axis + half-angle + the normal's side),
  never a numeric branch; both signs unit-rowed at closed forms.
- Nurbs → typed refusal (`OffsetError::NotClosedUnderOffset` —
  names the approximating-surface route as the coming door; OFF-B's
  territory, never a silent fit here).

## Refusals (door-owned, per the ratified O1 stance)

Named margined Q1 predicates over the INPUTS, decided BEFORE any
mint (DESIGN.md's pre-construction stance; the TUBEWALL precedent
for metered input validity — plain lengths in meters, Margin::of):

- `offset_radius_floor` — cylinder/sphere/torus-minor inward:
  margin `radius + d` (refuse at ≤ 0; in-band escalates — a
  collapsed or near-collapsed offset is never minted). The TUBEWALL
  lesson applies: the metered quantity is the REALIZED radius, so
  there is no large-scale rounding regime where the check passes
  and the mint collapses.
- `offset_torus_ring` — torus: margin `major − (minor + d)` (the
  ring convention R > r, the same quantity tier-3's #889 net
  checks; refusing here keeps the net a second net, not the first).
- `offset_cone_apex` — cone: the apex slide crossing the chart's
  represented region is a refusal only if the resulting surface's
  stored form degenerates; the apex itself moves legally (a cone
  offset is a cone). What must refuse: `half_angle` at the chart's
  validity edge after the slide — derive what actually degenerates
  from the stored form and meter THAT; if nothing does, document
  why the cone case needs no refusal and drop the predicate rather
  than metering vacuously (the ARMS-2 predicates-1/3 lesson: never
  meter a non-question).

## Fences

- **Geometry only.** No topology, no body door, no shell, no
  face-replacement — OFF-C/D consume this. No `Surface::Approx`,
  no fitting (OFF-B).
- No changes to existing mint sites (`wall_surface`, the blend
  tables); they may later route through this, not now.
- The demo rule does not bind (no demo — substrate); acceptance is
  closed-form unit rows.

## Acceptance

- Round-trip: `offset(offset(S, d), −d)` reproduces S bit-exactly
  for plane/sphere/torus/cylinder (pure parameter arithmetic both
  ways) and to one IEEE operation's reproducibility for the cone
  slide (state which).
- Definitional: for each kind, sampled points of the offset lie at
  exactly |d| from the base along the base normal (defining-
  equation rows, the ARMS-2 test style — independent spellings,
  not the mint's algebra restated).
- Both cone slide signs; both d signs per kind; each refusal
  planted red (incl. the realized-radius floor at a large-scale
  fixture — the TUBEWALL collapse-regime lesson as a test).
- Interval rows: enclosures contain, refusals escalate honestly
  in-band.
- Existing suites untouched and green.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft
(`~/.local/share/cad-work/verbs-offa-*.md`). Merge origin/main
before opening the PR titled "VERBS-OFF-A: analytic surfaces close
under offset — the mint table and its refusals"; confirm CI runs
STARTED; note the drawn point; watch to completion. Do not merge.
