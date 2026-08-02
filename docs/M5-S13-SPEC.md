# M5 S13 — the die-pips enablers: containment-fallback re-cut + the plane×sphere germ arm (binding spec)

Branch `ev/m5-s13-pips-enablers` from current main (post-#157/#158).
Origin: S12 deviation 1 + the PR 9c blocker map. **This unit aims
to make PR 12's die pips reachable WITHOUT the fitted-chord lane**:
a pip is a spherical cap bitten out of a PLANE face, whose section
curve is an EXACT circle (C5 plane×sphere), not a fitted chord —
the cyl×sphere fitted-chord lane (PR 9c dev 1, behind the SSI
generic lift) is NOT needed for pips. Two halves, one unit:

## 1. The containment-fallback re-cut (S12 deviation 1)

- Read memories/curved-containment-fallback.md FIRST — mechanism,
  witness (union(slab, poking ball) = 16.0 pinned at
  finding_sphere_class_containment_fallback_is_wrong_today),
  scope (sphere-only today), and the NURBS hazard.
- Replace the vertex-probe with a curved-extent test for the
  sphere class: a shell's true extent must consult curved face
  extents (a sphere face's extent = center ± r over its chart
  window — the PR 9c azimuth/window machinery and certified boxes
  from PR 9's boxes.rs are the substrate; exact structure or
  certified enclosures, never sampled normals). The fallback's
  containment question (no crossings ⇒ which operand contains the
  other?) then answers correctly for poking-but-not-crossing
  configurations — or refuses typed where the extent test cannot
  certify (in-band extents escalate, two-tolerance shape).
- The finding row FLIPS to its construction row: union = 17.30900
  bracketed, 1 shell... (check: poking ball ∪ slab is ONE shell —
  derive the true shell count and pin it), volume closed-form.
- NURBS: per the memory's hazard, the extent test for NURBS
  operands is UNWRITABLE today (implicit_residual poisons; f64-
  only projection) — re-gate the class explicitly (typed refusal
  at the fallback naming the lift blocker) so a future NURBS body
  constructor cannot re-open the silence. Pin the gate.

## 2. The plane×sphere germ arm (the pips join lane)

- The join dispatch (boolean/join.rs germ arms) gains
  (Plane, Sphere): section curve = exact Circle (C5 table, PR 5
  lineage — verify the intersect route yields it); the window
  analog = the sphere-side chart window via PR 9c's sphere-door
  machinery (the group-arm/representative discipline applies —
  read solid_contain.rs's closed_sphere_group) + the plane-side
  planar window (S9 lineage). Selection by exact containment,
  four metered margins, S9 pattern.
- mef inheritance is already in (S12) — fragments of reversed
  sphere faces keep their bits; the S12 guard row's audited-answer
  arm goes live for the sphere class it covers.
- The S12 per-class door narrows: sphere operands whose germs are
  ALL plane×sphere flip to live for ∖/∩ (and ∪'s fallback is
  fixed by §1); operand pairs still needing cyl×sphere chords or
  cone/torus keep typed refusals naming the remaining blockers.
- **The die-pips smoke row goes GREEN**: slab ∖ ball (a pip-
  shaped cavity) — exact volume (slab − cap volume, closed form),
  tier-3 valid, certified pcurves on the seam circle, both lanes.
  The refusal pin flips per its own doc comment.

## 3. Acceptance

- §1: the finding row flipped (union bracketed + shell count);
  poking-cylinder and torus configurations still refuse typed
  (the S12 probes keep passing); the NURBS re-gate pinned;
  in-band extent escalation row (band-scaled).
- §2: slab ∖ ball green (volume, tier-3, pcurves, Interval lane);
  slab ∩ ball = the cap (closed form); additivity row; a
  TWO-pip row (two disjoint balls out of one slab — the group
  arm under multiple sphere surfaces); the die-pips smoke row
  flipped.
- Every retired refusal re-pinned as construction (S9 pattern);
  multi-ε honesty (placements from the resolved band; the
  budget-refusal arms honest per the FitSampleBudget precedent);
  two-tolerance on every new arm INCLUDING definite arms.

## 4. Out of scope

Cyl×sphere fitted chords (PR 9c dev 1, behind the SSI generic
lift); cone/torus operands; NURBS operands (re-gated, §1); the
fillet machinery itself (PR 12 — this unit hands it live sphere
subtraction); any marcher change.

## 5. Process

One implementer + one blinded adversarial reviewer + one fix
pass. Review charter musts: merge-base reproduction of the 16.0
union; independent extent-test soundness derivation (can a
poking-but-not-crossing configuration fool the certified
extents? construct adversarially); the circle-section exactness
of the plane×sphere germ (residual identically zero — attack
with tilted planes); the two-pip group-arm row; the NURBS
re-gate cannot be bypassed. Local scope by iteration-speed:
touched crates (topo, sweep) default ε + the new rows at
Interval; CI proves the matrix. Push per unit; foreground only;
OUTPUT DISCIPLINE per standing process.
