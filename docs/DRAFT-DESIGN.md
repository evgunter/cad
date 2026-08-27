# Draft — the molding-taper design conversation

**Status: RATIFIED (Evan's sign-off on PR #908, with a note-fold round — planes-only to start is Evan's own note). The status line lagged the ratification until 2026-08-27 — the sign-off itself is the PR record.**
(VERBS program; the register's "no design record yet — needs its own
conversation" row). Proposals DR1–DR6. Substrate anchors verified on
main 2026-08-21 by the survey lane; the row is genuinely greenfield —
no open issue mentions draft, and nothing in docs/ pre-derives it
beyond LONGTERM-IDEAS' checker note.

Vocabulary: **draft** replaces a solid's walls with walls tilted a
small angle away from a **pull direction**, so the part releases from
a mold; the tilt pivots about a **neutral plane** (where the wall's
trace stays fixed). A drafted plane is another plane; a drafted
cylinder is a cone; a drafted general wall is a ruled surface.

## DR1 — Scope v1: plane walls only, cylinder walls refuse typed

The honest first door is **plane-wall draft**: pivot each selected
plane wall about its neutral-plane trace. Everything it needs
exists — general plane frames, general affine pcurve charts,
plane×plane re-intersection.

Drafting a CYLINDER wall mints a cone, and the survey shows the cone
is a second-class citizen in exactly the places draft's output would
land next:

- where the new cone meets a drafted plane neighbor at generic tilt,
  `plane_cone_section` routes to rung 3 **permanently** — "parabola
  and hyperbola are outside the conic inventory **by decision, not
  by omission**" (R1, `intersect.rs:1069-1076`). Cone SSI coverage
  is otherwise zero (cyl×cone, cone×cone, cone×sphere all
  unimplemented in the C5 table).
- a single cone face makes the whole body boolean-unavailable
  (per-face-KIND gate) and fillet-unavailable (`build.rs` reads
  plane supports only) — the drafted body would be one-way.

**Recommendation**: v1 ships plane-only with a typed
`DraftUnsupported`-class refusal naming the wall kind.

**The cylinder arm's cost, corrected per Evan's #908 note.** R1's
"permanent" refusal bars only the *exact conic special cases*
(parabola/hyperbola never join the analytic curve inventory); a
generic plane×cone section as a **fitted NURBS curve** is fine —
the ordinary Q8-family approximating-curve route the plane×NURBS
lane already ships. So the cylinder→cone arm does NOT re-open R1:
its real content is a **plane×cone fitted-SSI lane** (march + fit +
certify, the `plane_nurbs_ssi` shape with a cone chart), sequenced
with Wave 2's cone-operand work since they share the substrate.
Still a separate later unit — but plumbing, not a ratified-decision
change — plus (or after) Wave 2's per-face-KIND operand-gate
re-scope so the drafted body is not boolean-dead.

## DR2 — Mechanism: a certified re-geom pass (a new small op class)

For an all-plane body, draft changes **no topology**: every selected
wall gets a new plane, every affected strut a new line carrier, every
affected vertex a new point; face/edge/vertex keys all survive. That
is not the M6-1 split/graft shape (surgery is a graft executor for a
plan that adds and kills entities); it is a **geometry replacement
pass**, and the kernel has partial vocabulary for it:

- `set_face_surface` / `set_edge_curve` exist (attach layer), with
  the documented ordering obligation (surfaces before edge
  descriptions, then whole-body `mint_pcurves`) and the
  no-certification-here contract — the caller owns re-description.
- **The vertex half has no public door**: `get_vertex_mut` is
  `pub(crate)`; the only whole-body point rewriter is
  `transform_rigid`, which is rigid by checked contract.

**Recommendation**: draft is implemented as a kernel verb whose body
follows M6-1's discipline verbatim — decide everything first (all
new planes, carriers, vertex points, and every validity predicate,
read-only against the source), clone, apply through the attach layer
plus a **new `pub(crate)`-widened certified vertex-placement step
owned by the pass** (not a free-standing public vertex mutator — the
D4 posture stays: uncertified intermediate states never escape, the
pass validates once at the end). The alternative — recipe-layer
re-authoring (a drafted extrude is exactly a loft between two scaled
profiles, buildable today) — is honest sugar but cannot draft an
imported or boolean-produced body, which is the verb's actual
territory; it can land independently as a recipe convenience without
this conversation.

Validity predicates, per the ratified pre-construction stance
(DESIGN.md:1823): neutral-trace non-degeneracy per wall (the pivot
line must actually cross the wall), angle vs adjacent-face
consumption (a drafted wall must not sweep past its neighbor — the
face-clearance shape from the fillet battery, re-instantiated), and
global wall–wall non-interference at the requested angle. Each a
named margined Q1 trilean.

## DR3 — Selection: the pull-direction predicate is a SELECT-DESIGN amendment

Draft's natural spelling is "every face whose outward normal leans
against the pull direction". No such predicate exists —
`GeomPred` has surface-kind and datum-distance arms only, and
SELECT-DESIGN makes any new geometric predicate a DECIDED one with
its own `sel_*` K-funnel row and in-band refusal. **Recommendation**:
a `GeomPred::NormalLeans { direction, comparison }`-shaped predicate
(margin = the lever-folded sine against the pull direction,
trilean, in-band refusal per the selection ladder) lands as a small
co-requisite unit, proposed as a SELECT-DESIGN amendment in the same
PR that implements it. Kernel-direct callers keep the hand-scan
(the register's "selection is document-layer only" note stands; the
predicate at least gives the document layer the natural spelling).

## DR4 — Neutral plane: bare point+normal, no datum machinery

The house pattern is `SplitPlane { origin, normal }` passed by value
(the split door — the only "operation takes a plane" precedent).
Draft takes the same shape plus the pull direction and angle; datums
appear only when the recipe layer wraps the node (`Node::Split`'s
tool-evaluates-to-DatumValue pattern, reused). Derivable; adopted
here unless Evan objects.

## DR5 — Naming: drafted faces are survivors

A drafted face keeps its `FaceKey` and gets a new surface — under
the fillet-surgery naming discipline that is a **survivor, no birth
row**. Only a future neutral-plane *split* variant (drafting a wall
that straddles the neutral plane both ways) would mint, and
`SplitNaming` is the existing template. Derivable; adopted.

## DR6 — Sequencing and the checker twin

- Draft lands **after VERBS-RIM/CHAMFER** (shared review bandwidth,
  no code dependency) and is indifferent to Wave 2 in v1 (plane-only
  output keeps the body boolean-live). The cylinder arm, if ratified
  later, queues behind Wave 2's operand-gate work by DR1's own
  logic.
- LONGTERM-IDEAS' **moldability checker** ("1-1 along the pull
  direction with derivative bounded — hull bounds on surface normals
  vs pull direction; the M5 normal-enclosure substrate exists;
  nearest-term of the four") shares draft's vocabulary exactly.
  **Recommendation**: the checker rides as a sibling unit when draft
  lands — same pull-direction input, and it is the natural
  acceptance instrument for the verb (a drafted body should CERTIFY
  moldable at its own angle).
  **Its reach is NOT plane-limited (answering Evan's #908
  question)**: the checker consumes certified normal enclosures,
  which exist per KIND, not per verb — closed forms for the five
  analytic kinds and the patch-hull machinery for NURBS (the M5
  substrate LONGTERM-IDEAS names). So it ships kind-general from
  birth: it can certify (or honestly refuse, where an enclosure
  is missing) the moldability of bodies the draft VERB cannot yet
  touch — including imported parts — while draft itself is
  plane-only. The asymmetry is useful, not accidental: the checker
  is analysis over existing enclosures; the verb is construction.
