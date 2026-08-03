# The M5→M6 boundary: banked units + three roadmap questions (design conversation)

Status: DRAFT for Evan's review. Two decisions requested (§1, §2);
the technical analysis in §3 rides whichever home §2 picks. A 👍
on the PR is enough to ratify the recommendations as written;
comment to fork any of them.

## 1. The three banked geometry units — where do they run?

M5's reviews banked three reviewed-scope units. M6 is the
error-propagation MVP (ERROR-DESIGN.md, ratification-pending), a
different axis entirely — so "fold into M6" is not the natural
home for any of them. Recommendation per unit:

- **SSI generic-T lift** (Box3/project/certify_branch are f64-only;
  blocks Pcurve::Fitted, which blocks cyl×sphere germ chords and
  NURBS extent tests) — **M5-adjacent, immediately post-exit.**
  It completes M5's own machinery, its blocker map is precise
  (PR 9c dev 2 + the S13 re-gate), and both M6 (interval
  clearance over curved contacts) and M7 (import adoption) sit
  downstream of Fitted pcurves. Small-to-medium, well-specified.
- **Loft/sweep body assembly** (PR 10's §3 caps/walls/seams
  builder + the tier-3 Nurbs-face flips + certify's resolve
  acceptance; design recorded at implementable detail in the
  PR 9c log entry) — **M5-adjacent, after the lift.** It closes
  shape (iii)'s full loft-body story (the exit criterion is
  satisfiable without it only because the substrate row moved to
  PR 7b; the honest complete form wants the body). PR 11's
  quadrature machinery exists; the NURBS-patch flux door is the
  remaining real work.
- **Canal-surface general blend** (C8's approximating-surface
  lane) — **PARK.** No acceptance shape consumes it (the die is
  analytic end-to-end); its natural consumer is variable-radius /
  general-spine fillets (Band 3). Running it now buys reviewed
  code with no caller — the dead-machinery pattern M5's reviews
  repeatedly punished. It re-opens with the milestone that ships
  its first consumer.

Net: two M5-adjacent units (sequenced lift → assembly, one lane,
normal pipeline) run between M5 exit and M6 kickoff; one parked
with its re-open trigger named.

## 2. Three roadmap questions (Evan, 2026-08-03), one shared home

**(a) Ball-and-socket tangency.** A ball seated in a congruent
spherical socket is not a tangency in C7's sense — it is AREA
contact between coincident surface patches. C7's jet certificate
requires the relative transverse normal curvature κ_rel bounded
away from zero; congruent contact is exactly κ_rel ≡ 0 (the
surfaces do not determine a contact locus — there is nothing for
jets to certify). This is the same shape as planar REST contact
(S1's crosslap): a DECLARED coincidence class, verified
structurally (same center, same radius, opposite senses), never
derived numerically.

**(b) Interference fits.** Deliberate overlap breaks the current
world-model (solids are disjoint or booleaned; S13's extent scan
refuses overlapping spheres precisely because silent overlap was
a bug). An interference fit is a DECLARED intent: "this pair
overlaps by design." Consequences once declared: the disjointness
/containment gates skip the marked pair (typed skip, recorded);
assembly-level mass properties either refuse or subtract the
double-counted lens (certified — the lens volume of
sphere-in-sphere and cylinder-in-bore is closed-form for the fits
that matter); STEP has no native concept, so export carries it as
annotation or not at all (stated honestly). Note the deep
connection to **M6's interval clearance**: an interference fit is
a declared NEGATIVE clearance with a tolerance band — the same
measurement machinery, opposite sign convention. M6's
`min_clearance` measure should be designed with signed clearance
from day one so fits fall out of it.

**(c) Tangency against negative-curvature surfaces.** Already
mostly works, and the sign analysis is worth recording: C7's
certificate cares about κ_rel = |κ₁ − κ₂| along the transverse
direction, not the individual signs. A ball of radius r in a
concave groove/socket of radius R > r has κ_rel = 1/r − 1/R > 0,
bounded — the jet system certifies it exactly like external
tangency (and saddle-surface contact likewise, per direction).
The failing regime is CONFORMAL contact — r → R — where κ_rel
enters the band and F6 escalates (correctly: at that point the
contact is transitioning from a curve to an area, and the answer
is (a)'s declared class, not a sharper tangency certificate).
So: nothing new needed for strict-inequality internal tangency
beyond fixtures proving it (cheap, worth adding); the r = R limit
is (a) again.

**Recommended home for all three**: the **curved-census /
declared-contact design doc** that OQ5 explicitly deferred ("the
curved coincidence census waits for its own design doc").
(a) and (c)'s conformal limit are its coincidence classes;
(b) is its intent-declaration vocabulary. Schedule: DESIGN-ONLY
during M6 (it wants co-design with signed clearance),
implementation with the milestone that ships assemblies/contacts
as a feature. Plus one cheap M5-adjacent fixture row proving
internal tangency (c) works today.

## 3. Proposed sequence

M5 exit (PR 12, PR 14) → SSI lift → loft assembly → M6 kickoff
(error-propagation per ERROR-DESIGN, with signed clearance) +
the curved-census/declared-contact design doc drafted mid-M6 →
canal blend and contact IMPLEMENTATION re-open with their
consumers.
