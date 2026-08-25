# Long-term ideas (parked, non-binding)

Evan's running list of things the kernel should eventually support
(2026-08-03 batch, in-session). NON-BINDING: nothing here is
scheduled; each idea names its natural prerequisites so the
milestone that makes one cheap can pick it up deliberately. GUI-
specific ideas live at the bottom of GUI-DESIGN.md instead.

## I1 — Custom "unit tests" for parts

A part carries user-defined checks that run like the kernel's own
validity gates — certified where the geometry supports it, honest
heuristics where it doesn't (labeled as such, never silently
mixed). Named instances Evan wants eventually:

- **(0) The scale-relative sliver lint — the FIRST and easiest
  member (Evan, 2026-08-03, resolving #89's display half).** A
  margin that is numerically definite (≫ Kε) yet below
  display-distinguishability at model scale is *probably* a
  mistake — warn, never refuse; the modeler confirms intent ("the
  2 µm step is deliberate"). Easiest because no new geometry is
  needed: the margins are already recorded (the K-funnel
  Probe/verdict telemetry), so this is a threshold sweep over
  existing data plus a warn channel — and it is the PROTOTYPE of
  this whole lane's shape (per-part, advisory, honest about being
  a heuristic threshold). #89's kernel half — the K value itself —
  is CLOSED separately: K = 10 permanent (docs/K-REPORT.md).
- **(0b) The connectedness lint (Evan, 2026-08-10, #328 — born from
  the assembly-design conversation). SHIPPED 2026-08-25** as the
  checks registry's first resident (`editor_core::checks`,
  DISCIPLINES-DESIGN DS6 round 4 — which also narrows this entry's
  "no heuristic threshold anywhere": the count is combinatorial, the
  void exclusion is a certified decided orientation read). Warn when a body at rest has
  more disconnected components than expected — a stray solid usually
  means a boolean that didn't reach its operand or an instance placed
  nowhere. Unlike the rest of this lane it is *exact*: connectivity
  is combinatorial (the shell-partition components tier 1 already
  computes), no heuristic threshold anywhere — the lane's first
  fully-certified member. "Expected" is an input: assembly structure
  / file splits mark expected disconnection naturally, and an
  explicit per-part expectation mark is the lint-input form (both
  Evan's framing, #328; the split-as-signal half is a natural
  default, not a requirement). Warn, never refuse, per the lane's
  charter. Prerequisite: the assembly design's multi-solid evaluation
  (docs/ASSEMBLY-DESIGN.md A2), which is what makes "disconnected on
  purpose" a common, expressible state — the ASM program has since
  shipped it, so the prerequisite is met.
- **(a) Injection-molding / draft**: the shape is 1-1 along the
  pull direction (a function), with derivative everywhere below a
  max — equivalently minimum draft angle everywhere. This is a
  certified-geometry check (hull bounds on surface normals vs the
  pull direction — C9-class machinery; the tessellation/normal
  enclosures built in M5 are the substrate). Nearest-term of the
  four.
- **(b) Thermal expansion**: materials in metadata; at target
  temperatures, scale each part by its CTE and re-run the
  constraint/clearance checks — do all mates/fits still hold?
  Prerequisite: M10 signed clearance + material metadata vocabulary
  (and the interference-fit declarations, which are temperature-
  dependent by nature — see #161's declared-contact doc).
- **(c) Tool access**: can a hand holding a screwdriver reach each
  screw (wrench equivalents etc.)? Swept-volume / clearance-corridor
  queries against the assembly — needs assemblies, swept volumes,
  and a tool-envelope library; genuinely far out, parked so the
  assembly design leaves room for volume queries.
- **(d) Machinability heuristics** per machining method: relies on
  heuristics we do not currently know; explicitly the
  labeled-heuristic class (never certified language).

## I2 — Design-for-measurement

Make it natural for designs to favor easy measurement:
- **(a)** once interval tolerancing exists (M10), make "the required
  tolerance at the places that will actually be MEASURED" a
  first-class query — the designer sees measurable-point tolerances,
  not just whole-feature ones.
- **(b)** bridge the gap between how a machinist measures (calipers
  at one or two points on a width) and what the design needs (the
  tolerance holding across the whole surface): pick point-tolerances
  that suffice for the whole-surface requirement under declared
  assumptions about surface variation (a form-error budget). This is
  a certified-inference feature: point bound + variation assumption
  ⇒ surface bound, with the assumption recorded as a declared input
  (D4-honest: the assumption is data, not a guess).

## I3 — Handbook-lookup definitions

Define standard engineering choices (the named case: interference
fits) by lookup against machinist's-handbook norms keyed on
materials/sizes in question, rather than hand-entered numbers —
i.e. `fit: H7/p6-per-handbook(steel, 12mm)` as a declared,
versioned data source. Prerequisite: the declared-contact/
interference vocabulary (CONTACT-DESIGN C4/C6, ratified) + a
data-provenance story (the handbook table is an
input with a version, like a tolerance). Pairs naturally with
I1(b) and I2.

## I4 — SVG output lanes (Evan, 2026-08-09, in-session)

The kernel can draw pictures of itself in a vector format with no
renderer in the loop. Three members, in ascending difficulty; the
first has landed, the other two are parked here.

- **(0) The UV trim-loop dump — LANDED 2026-08-09**, as the demo
  tour's third montage lane (`demos/render-uv.sh` →
  `renders-uv/montage-uv.svg`; emitter `demos/tour/src/uvdump.rs`).
  Each face's `(u, v)` chart with its trim loops drawn on it. Needs
  no projection, camera or silhouette machinery because the chart is
  already 2-D, so the lane has NO external dependency at all —
  unlike the two 3-D lanes, which need headless FreeCAD. It is a
  diagnostic, not a depiction: loop closure gaps, winding, seam
  crossings on periodic charts, and stored-vs-derived pcurve
  provenance are all measured and drawn per face. Recorded here
  because (a) it is the substrate the other two reuse, and (b) the
  reason it was cheap is a design fact worth keeping: the pcurve
  caches already exist, so the lane is a serializer over data the
  kernel had.
- **(a) Projected-edge SVG wireframe** — the fast alternative to
  `render.sh` for the inner dev loop. Project the B-rep's edge
  carriers onto a view plane and write them as SVG paths. The
  projection is EXACT and nearly free: both orthographic and
  perspective projection are projective maps, which act linearly on
  a rational curve's homogeneous control points, so a
  `Curve3::Nurbs` projects to a 2-D rational NURBS of the same
  degree with the same knots, and `Line`/`Circle`/`Ellipse` project
  to a line or an ellipse. SVG's own `L` and `A` (elliptic arc)
  commands then carry those exactly; only the general rational case
  needs the standard subdivide-to-cubics-within-ε approximation.
  Prerequisite: none beyond I4(0)'s curve→path layer. **Explicitly
  NOT a replacement for the montage**: with no hidden-line removal,
  a curved solid draws with no silhouette (a cylinder is two circles
  and nothing joining them) and a finned body is spaghetti. The
  eyeball gate stays with the shaded lanes.
- **(b) Drawing-grade projection with hidden lines** — already filed
  in DESIGN.md's Band 3 ("Engineering drawings"), and it stays
  there: the blocker is silhouette curves, which on a general NURBS
  surface means tracing the implicit `n(u, v)·d = 0` in parameter
  space (SSI-grade) plus a 2-D visibility pass. Noted here only so
  the cheap two above are not mistaken for progress toward it. The
  analytic-only middle tier (closed-form silhouettes for the five
  analytic charts) is NOT worth taking alone — the picture is wrong
  at every analytic/NURBS face boundary, and the tolerance-nasty
  visibility pass is still owed.

## Process note

Items graduate from this file by being written into a milestone
plan with Evan's sign-off; the file records the idea's origin date
and any design-fact dependencies discovered since parking.
