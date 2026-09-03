# VERBS-RIMCAP — the partial-revolve rim for circle-profile walls

RATIFIED 2026-09-03 (orchestrator) against main `a6072d812`, from
the 2026-09-02 draft (drafted against `69640aaba` from measured
state: TORAX's elbow-split STOP + its fix pass's corrected
capability record; C5ARMS PR-1's opening measurement; the wedge
counterexample). Cites re-verified at ratification; drifted lines
refreshed. The corpus-first law binds — the implementer re-measures
every anchor at dispatch.

Branch `verbs/rimcap-1`. Difficulty pre-logged: PR-1 **M** (the
sphere half). PR-2 (the torus half) is a DESIGN CONVERSATION and
does not dispatch from this spec.

## What this unit is

A partial revolve's rim is the wall × meridian-cap boundary. For
walls whose profile constraint is a LINE (cylinder, cone, plane),
the rim already hollows: the wedge (`sf2b_axial.rs:81-109`,
`the_partial_revolve_wedge_hollows_to_its_closed_form` at `:184`)
is the shipped precedent, and its docs carry the load-bearing
fact this unit extends: **the moved meridian caps travel inward
along their own normals, so they stop containing the axis, and
the cavity of a wedge is not a wedge** — the azimuth half of the
corner solve already handles that displacement.

For walls whose profile constraint is a CIRCLE (sphere `Ball`,
torus `Torus` — `offset_axial.rs:970-979`), the rim does not
hollow, at TWO measured doors:

- **The elbow door** (torus wall): the rim corner meets ONE
  profile constraint plus a meridian, and the corner solve's
  under-determination arm refuses — `TogetherAxialCorner
  { surfaces: 2, what: "one profile constraint meets here and the
  vertex is not on the axis, so its station is determined but its
  radius is not" }` (`offset_axial.rs:1020-1025`, constructor at
  `:925`; pinned by
  `verbs_shell::the_klein_wall_pair_waits_on_the_partial_revolve_rim`
  `:637` and
  `torax_axial::torax_the_partial_revolve_rim_has_one_profile_constraint`
  `:497`).
- **The lune door** (sphere wall): the CORNERS solve through the
  axis-pole arm, and the rim EDGE fails —
  `TogetherEdgeDisagreement`, the two ends solved a wall-thickness
  apart (`torax_axial.rs:20-29` module doc; pinned by
  `torax_the_sphere_lune_refuses_the_rim_at_the_other_door` `:581`
  with `gap == 0.05` asserted).

The two doors are one gap seen from two sides: the rim's moved
geometry is (moved wall) ∩ (moved cap), the moved cap is a plane
parallel to a meridian plane at distance `t` off the axis, and
that section is **an off-axis circle for a sphere** (planes cut
spheres in circles, always) and **a spiric quartic for a torus**
(`verbs_shell.rs:619`; C5ARMS measured the spiric at half-width
`0.22520271607754455` vs half-height `0.22500000000000003`,
Δ 2.03e-4 — a real quartic, not a perturbed circle at kernel
tolerances). The kernel has closed-form machinery for the first
and NO carrier kind for the second. Hence the split below.

Reading-level corroboration, recorded at ratification:
`torax_axial.rs`'s own module doc already states the lune
mechanism as "the moved rim circle is centred off the axis and
the mint has no arm for one" (`:20-29`). That sentence is a
reviewer's reading, not an executed diagnostic — opening
measurement item 4 still runs, and the mechanism STOP still
binds if the run contradicts it.

## Opening measurement (before any code, in the PR body)

Re-run at the unit's head, payloads and raising sites quoted:

1. The klein elbow, `shell_open` and sealed `shell` — expected
   `TogetherAxialCorner { surfaces: 2, what: "one profile
   constraint…" }`.
2. The sphere lune (the `torax_axial.rs:581` fixture) — expected
   `TogetherEdgeDisagreement { gap = wall thickness }`.
3. The wedge — expected GREEN (the control; its closed form must
   not move at any point in this unit).
4. NEW, the diagnostic the halves hang on: for the lune, print
   the two pole solves and the minted carrier — confirm the
   mechanism is (a) the pole arm answering ON-AXIS stations while
   the true moved corners sit OFF the axis (the moved cap no
   longer contains it), and/or (b) the meridian great circle
   falling to `mint_carrier`'s circular-edge arm, which refuses
   any circular edge "whose centre is off the axis" / "whose
   plane is not normal to the axis" (`offset_axial.rs:1470-1516`).
   The mechanism story is derived from reading, NOT yet executed —
   this measurement is what makes it true or re-cuts item 1.

## PR-1 — the SPHERE half (the off-axis circle rim)

1. **The rim corner with a circle profile.** The corner's profile
   half gains the carried-datum rule the SHELLFIX-2b lineage
   already uses one constraint up: with ONE profile circle, the
   corner's `(ρ, h)` is the OLD corner's profile point moved
   concentrically with the circle (radius `r → r + d`, centre
   fixed — the same datum `mint_carrier`'s sphere-seam arm
   already trusts at `offset_axial.rs:1369-1391`), and the
   AZIMUTH solves from the moved cap exactly as the wedge's
   does today (`ρ·sin(Δφ) = t` — the existing meridian solve,
   fed the off-axis offset). No new public type; the new arm
   lives beside the pole arm and refuses typed when the datum
   is not a circle or the azimuth solve has no root
   (`|t| > ρ`).
2. **The rim edge carrier.** `mint_carrier` (`offset_axial.rs:1325`)
   gains the **off-axis-circle** arm for a circular edge between
   two distinct charts (sphere wall × plane cap): centre
   `sphere.center + n̂·t` where `n̂` is the moved cap's normal,
   radius `√((r±d)² − t²)`, plane parallel to the old meridian
   plane — the `plane_sphere_section` closed form, computed
   inline in the door's own arithmetic (NOT by calling the
   section function — see fences). The existing arms are
   untouched; this is a sibling arm gated on its own named
   predicates, each registered in the audit doc. The ordinal-111
   guard lesson binds: the new predicates cover their FULL
   stated conventions, not just the binding relation.
3. **`param_on` unchanged** — its circle arm reads an angle
   difference and meters the endpoint gap; if the corners and
   carrier above are right, it passes; if not, its refusal is
   the net. Do not weaken the `offset_axial_edge_agreement`
   meter.
4. **Acceptance.** The sphere lune HOLLOWS to a closed form
   (derive the cavity volume exactly — two concentric spheres
   cut by two planes at azimuthal half-angle φ, one of them
   displaced; the derivation goes in the test doc), tier-3
   valid, rigid-re-pose parity; the lune's refusal row flips to
   the hollow row WITH its old door recorded; a planted red
   keeps `TogetherEdgeDisagreement` reachable (a deliberately
   disagreeing fixture, or mutation-demonstrated red-then-green
   if unbuildable — state which). The wedge, the full-revolve
   suites, and every TORAX row bit-identical.
5. **Differential.** The klein elbow (torus) still refuses —
   the SAME door as the opening measurement or one door deeper
   with the payload quoted; the torus half's boundary is stated,
   not glossed.

## PR-2 — the TORUS half (design-gated; NOT an implementation unit)

The moved rim is a spiric quartic. `Curve3` has no quartic
carrier; the honest options, for adjudication and Ev rather
than for a lane to pick:

- (a) **Fence it** (the standing state, made permanent for this
  family): partial revolves with torus walls refuse at a door
  that names the spiric; klein-elbow rows stay measured-red.
- (b) **Exact spiric carrier kind** — a real design conversation
  (evaluation, parameterization, meters, STEP export), the same
  family as #1377's valence-4 machinery; if funded it should be
  one design doc covering both.
- (c) **NURBS-fitting the rim** — REJECTED in advance: it is the
  fenced approximation class (a plausible body whose meters lie
  near the cap), recorded so it is not rediscovered as a
  shortcut.

PR-2 does not dispatch from this spec; it gets its own doc if Ev
funds (b), via an `[ev]` PR per the tracker contract.

## Fences (PR-1)

- **No section functions called from the offset path** — the
  off-axis circle is computed in `mint_carrier`'s own arithmetic
  (the module's "No marching, no SSI" law at `offset_axial.rs:112`
  and its "does not widen the C5 table" law at `:114` both
  stand).
- No route flips, no C5 table changes, no `reduce`/`chord_join`/
  `join` work.
- The TORAX bit-identity rows (all `torax_axial` closed forms,
  the seam decides) unmoved; the wedge's closed form unmoved;
  the pole arm's existing answers unmoved for full revolves.
- The spindle/horn refusal (`R > r > 0`) and the wall-clearance
  layer untouched.

## STOP conditions (pre-registered, PR-1)

1. **Machinery-shape STOP**: if the carried-datum corner rule
   cannot be expressed without widening a PUBLIC type or adding
   a constraint KIND to `Constraint` that couples azimuth and
   radius, STOP for adjudication — that is a design widening
   this spec deliberately does not authorize.
2. **Mechanism STOP**: if opening-measurement item 4 falsifies
   the derived mechanism (the failure is not the moved-cap
   displacement — e.g. the pole arm's answer is right and
   something else moves the endpoints), STOP and report; the
   spec re-cuts on the measured mechanism.

## Difficulty (pre-log)

- **PR-1 (sphere half): M.** Two arms beside existing siblings,
  one closed form, no public types. The risk is the azimuth
  solve's interaction with the carried window (the |δ|=π
  lessons apply — the moved cap's azimuth is small but signed).
- **PR-2 (torus half): unknown-pending-design.** Not dispatched
  from this spec.

## What each half unblocks

- Sphere half: the lune family; sphere-walled partial revolves
  generally; NOT the klein elbow.
- Torus half (if ever funded): C5ARMS rows 3/4/8 (the klein
  elbow shell_open self-retirement, the demo re-authoring, and
  the elbow's KERNEL-VERBS row) — those rows stay measured-red
  until then. The C5ARMS spec's hold note now points here.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Suites via hosted CI verified at the STEP
level (read the change filter's own output; a green job name is
not evidence); local compute for touched suites, probes, and
mutations only. Opening measurement before code, payloads quoted
(a refusal's text is not evidence of its cause). Deviations
declared with schedules. Long jobs: setsid + output file +
foreground poll; never end a turn parked on a background wait.
