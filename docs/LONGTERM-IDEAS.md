# Long-term ideas (parked, non-binding)

Ev's running list of things the kernel should eventually support
(2026-08-03 batch, in-session). NON-BINDING: nothing here is
scheduled; each idea names its natural prerequisites so the
milestone that makes one cheap can pick it up deliberately. GUI-
specific ideas are the last section, below.

## I1 — Custom "unit tests" for parts

A part carries user-defined checks that run like the kernel's own
validity gates — certified where the geometry supports it, honest
heuristics where it doesn't (labeled as such, never silently
mixed). The lane's ratified shape is DISCIPLINES-DESIGN DS6 — grade 4,
the advisory-check registry (`editor_core::checks`); members still
graduate by the process note below. Named instances Ev wants
eventually:

- **(0) The scale-relative sliver lint — the FIRST and easiest
  member (Ev, 2026-08-03, resolving #89's display half).** A
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
- **(0b) The connectedness lint (Ev, 2026-08-10, #328 — born from
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
  Ev's framing, #328; the split-as-signal half is a natural
  default, not a requirement). Warn, never refuse, per the lane's
  charter. Prerequisite: the assembly design's multi-solid evaluation
  (crates/editor-core/ASSEMBLY.md A2), which is what makes "disconnected on
  purpose" a common, expressible state — the ASM program has since
  shipped it, so the prerequisite is met.
- **(a) Injection-molding / draft**: the shape is 1-1 along the
  pull direction (a function), with derivative everywhere below a
  max — equivalently minimum draft angle everywhere. This is a
  certified-geometry check (hull bounds on surface normals vs the
  pull direction — C9-class machinery; the tessellation/normal
  enclosures built in M5 are the substrate). Nearest-term of the
  four, and the only one with a design record: DRAFT-DESIGN DR6
  sequences it as the draft verb's checker twin and settles its reach
  as kind-general, not plane-limited.
- **(b) Thermal expansion**: materials in metadata; at target
  temperatures, scale each part by its CTE and re-run the
  constraint/clearance checks — do all mates/fits still hold?
  Prerequisite: M10 signed clearance + material metadata vocabulary
  (and the interference-fit declarations, which are temperature-
  dependent by nature — CONTACT-DESIGN C6).
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

## I4 — SVG output lanes (Ev, 2026-08-09, in-session)

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

## GUI ideas (non-binding sketchpad)

Ideas captured during the GUI and solver design conversations. Each
builds on a ratified contract (the witness mechanism W1–W9 in
`crates/editor-core/README.md`, the naming rules in
`crates/editor-core/src/names/README.md`) and none is scheduled.

- **Wall-mode drag** (on W2/W4) — the default click-and-drag refuses
  to cross the discriminant locus: as the drag approaches it the
  preview solver's branch margin shrinks and at the wall the dragged
  point sticks, with the parameter-space wall indicated. Every
  default-mode drag is then a fold-free homotopy, so the drag-end
  re-witness is uniquely branch-selected and needs no dialog. A
  modifier key crosses the wall; the keypress is the recorded intent
  to flip branches. Only the recorded endpoint enters the solution.
- **Bulk re-witness on clean certificates** (on W4) — certified
  same-branch re-witnessing is semantically invisible, so do it in
  bulk on commit edits; dialogs are reserved for certificate
  refusals, which concentrate at degenerate geometry.
- **Margin as an ambient affordance** (on W3) — the branch margin is
  a live scalar during editing; surfacing it (proximity shading near
  walls) turns "why did it ask?" into something the user saw coming.
- **Scale-relative sliver lint** — a document-layer lint that reuses
  the K machinery verbatim and compares predicate margins against a
  display-relative threshold (viewport scale × pixel size, not ε),
  badging features that render indistinguishably from exact
  coincidence. The kernel never refuses such a feature; K guards
  certification honesty, not intent.
- **Painted operands through booleans** — joining painted bodies
  never errors (paint keeps resolving on the operand node); the GUI
  renders the displayed node's appearance, and when a boolean is
  appended above appearance-carrying names the suggestion ladder
  offers one-click `Rebind`s for the wrapping derivations. Silently
  following faces by topological-naming heuristics is the N5-banned
  shape.
- **Undo as a history tree, not a stack** — edit history as a DAG of
  document states: undo moves a pointer toward the root, an edit
  after undo mints a sibling branch, and the abandoned branch stays
  reachable. Nearly free here because `Doc` is an immutable value and
  every `DocEdit` a recorded value, so the tree is parent pointers
  over states already materialized; the structural node-granular
  diff shows what a branch changed. v1 ships the tree-shaped state
  under linear chrome (`viewer::history`); the branch picker and a
  separable history sidecar are the banked GUI-6 unit. The on-disk
  form stays snapshot + edit log, so "state without history" is a
  save with a compacted log, an explicit operation that says what it
  drops, and the main document never depends on the sidecar.

## Process note

Items graduate from this file by being written into a milestone
plan with Ev's sign-off; the file records the idea's origin date
and any design-fact dependencies discovered since parking.
