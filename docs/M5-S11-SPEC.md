# M5 S11 — concave arc walls mint sense:false (binding spec)

Branch `ev/m5-s11-concave-sense` from post-S10 main. Origin: S10
deviation 1 (MAJOR, returned) + its review's e2e upgrade: **main
ships a silent wrong union today** — extrude mints every cylinder
wall sense:true, but a CONCAVE arc's material lies outside its
carrier cylinder, so point_in_solid's cylinder door misreports
the notch interior and the boolean containment fallback swallows
a body placed there (executed witness: union(notched, pellet) →
volume 3.000 vs truth 3.008, 1 shell vs 2, no refusal).
**MERGE PRIORITY on the S9/du_of_rims precedent.** Required
predecessor of the revert-wiring unit (reverting wrong senses
flips a lie into a lie).

## 1. The fix

- Sweep's extrude wall-minting: a concave arc segment (the
  material side is outside the carrier cylinder — the criterion
  is the arc's turn sign against the loop's orientation, exact
  structure from the profile's stored winding, never a numeric
  decide) mints its wall face with `sense: false`. Convex arcs
  and all planar walls stay `true`. Any other constructor that
  mints curved walls (revolve; grep for Face literals with
  curved surfaces) gets the same audit — enumerate in the
  report.
- The sense criterion must be EXACT: derive it from the same
  stored bulge/turn structure the profile validation already
  trusts; a numeric derivation from sampled normals is a review
  MAJOR.

## 2. Flips and pins

- The S10 finding rows FLIP to construction rows (S9 pattern):
  `finding_concave_arc_wall_sense_is_wrong_today` becomes the
  pinned correct-door row (point_in_solid reads Out in the
  notch); the adopted merge-base union-drops-pellet witness
  becomes the pinned correct-union row (2 shells, volume 3.008
  bracketed).
- The S10 acceptance rows (tier-3 inside-out catch, props flip,
  tessellation flip) must still pass — the concave wall's
  sense:false is now CORRECT, so tier-3 check 6, props flux
  (rimless-band s_f and the A/B discipline), boolean doors,
  sector classification, and STEP same_sense all consume a
  genuinely mixed-sense body for the first time. Add one
  end-to-end row: a concave-notched body validates tier-3,
  meters its exact volume, tessellates watertight, and exports
  STEP with same_sense=.F. on the concave wall.
- Every consumer the S10 audit marked "sense-invariant" is now
  exercised with a real mixed-sense body — any that lied
  surfaces here. Battery + the S10 probe suites are the net.

## 3. Out of scope

Revert wiring (the follow-on unit — this PR makes it safe);
curved subtract/intersect (still gated on revert); any door or
audit change beyond what the mixed-sense body forces.

## 4. Process

One implementer + one blinded adversarial reviewer + one fix
pass. Review charter musts: independent derivation of the
concavity criterion from the profile winding (attack orientation
conventions: CW loops, holes/rings, full-circle segments, the
S2 arc-leg fillet corners); merge-base confirmation that the
pellet witness reproduces before and is fixed after; a
mixed-sense adversarial sweep over the S10 "sense-invariant"
dispositions; CODE QUALITY REPORT (fixed rubric). Local scope by
iteration-speed: touched crates (sweep, topo) default ε + the
door/union rows at Interval; tour battery only if a demo is
touched; CI proves the matrix. Push per unit; foreground only;
OUTPUT DISCIPLINE per standing process.
