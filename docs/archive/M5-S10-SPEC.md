# M5 S10 — face orientation sense (binding spec)

Branch `ev/m5-s10-face-sense` from current main. Origin: PR 9c
deviation 3, RULED by Evan in-session 2026-08-02: **option (a),
`sense: bool` on `topo::Face`** — the ratified fix for the
contract gap (a face's outward normal was its surface's chart
normal; axisymmetric chart normals are outward for cyl/cone/torus
by odd-in-r parity and for spheres under the r>0 convention, so
`revert` had no representation to write for curved faces).
Options (b) Surface::Reversed, (c) NURBS conversion, and (d)
negative-radius spheres were considered and rejected (grounds in
M5-LOG and the #154 pinned messages). Unblocks curved revert →
subtract/intersect → PR 12 die pips. This unit is the DESIGN.md
touch plus the mechanical sweep — curved revert itself is the
follow-on unit, not this one.

## 1. The contract change

- `topo::Face` gains `sense: bool` (true = material side agrees
  with the chart normal; false = reversed). DESIGN.md D1's face
  text amended to state the bit and its meaning — a one-paragraph
  revision citing the ruling, applied in this PR (the ruling IS
  the sign-off; no further consultation needed).
- Every `Face { … }` literal (~82) gains `sense: true` — at M5
  every constructor mints material-agrees-with-chart faces; the
  reversed value becomes REACHABLE only when curved revert lands.
  A doc note at the field says so (honest: the bit exists ahead
  of its first writer).
- STEP alignment noted in rustdoc: `advanced_face.same_sense` is
  this bit; PR 13 consumes it directly.

## 2. The consumer audit (the real work)

Audit and thread the bit through every "which way is out"
consumer. Known population (verify by grep, enumerate in the
report): tier-gate dihedral/contact classification (validate.rs
uses per-face outward normals); props flux (divergence theorem
signs — curved.rs and loop_area); boolean sector classification
(sectors.rs outward normals) and point_in_solid arms (the
plane/cylinder/sphere doors' material-side reads); tessellation
winding (mesh crate triangle orientation); splitting
classify/rules local-normal reads; STEP/STL export winding;
revert.rs (which becomes a trivial sense-flip for curved faces —
but WIRING it is the follow-on unit; here it keeps its typed
refusal with the message updated to name this unit as landed and
the wiring unit as next).
- Each consumer site either (i) multiplies its normal read by
  the sense sign, or (ii) documents in place why it is
  sense-invariant (e.g. consumes both faces of an edge
  symmetrically). NO site is silently skipped: the report
  carries the full audited list with per-site disposition.
- Since every minted face has sense=true, ALL behavior is
  bit-identical at this PR — pinned by the battery running
  unchanged. The audit's correctness for sense=false is pinned
  by unit rows that construct a body with a hand-flipped face
  (test-only door) and assert the consumer flips: a tier-3
  refusal (inside-out face caught), a props sign flip, a
  tessellation winding flip. These rows are the review's attack
  surface.

## 3. Persistence

Faces re-derive from recipes (bodies are not serialized), so NO
schema change. Assert this in the report (grep the wire format);
if anything DOES serialize a face, that is a numbered deviation
and a design conversation, not an improvisation.

## 4. Acceptance

- Battery unchanged at default ε + Interval (bit-identical
  claim pinned by the existing suites passing untouched).
- The three hand-flipped-face rows (§2). Two-tolerance shape on
  any new arm (the flipped-face tier-3 refusal is structural —
  say so per the PartialSphereFace precedent).
- DESIGN.md amendment included; revert.rs message updated;
  k-lint clean (no new numeric predicates expected — sense is
  exact structure, never a decide).

## 5. Process

One implementer + one blinded reviewer + fix pass. Review
charter: independently re-derive two audit dispositions of the
reviewer's choosing per consumer class (esp. props flux signs
and tessellation winding); attack the sense-invariant claims;
verify the hand-flip rows discriminate. Local runs by the
iteration-speed principle (this is sweep-shaped: workspace
check + the three new rows + ONE suite per consumer class,
default ε; CI proves the matrix). Push per unit; foreground
only; OUTPUT DISCIPLINE per standing process.
