# The chord's arc side: one rule, held by the pairing, not re-derived by the chord

## For Ev

**Recommendation (likely).** One arc-side rule, and `chord_spec` does not evaluate it. The rule is
the **crossing rule**: where the section plane crosses a face's boundary, the section leaves that
crossing *into the face* along `n_plane × n_out` when the boundary passes there from the plane's
positive side to its negative side, and along the opposite direction when it passes the other way
(`n_out` the face's outward normal; the loop runs with the face on its left). Both lanes already
hold this answer before any chord is minted, so the chord takes the arc as **data** — the
rotational sense, about the section plane's normal, of the section's departure from one named
site — and mints it. The azimuth-window selector (`select_arc`), the run-side selector
(`select_arc_by_run_side`), the `azimuth_monotone` flag, the window the planar side is handed by
value, and every refusal that exists only because a window could not be read
(`ArcWindowCase`, `ArcSideCase`, `SectionNotPolar`) leave the chord path.

Terms. *Section plane*: the plane a face is cut along (the split plane, a plane germ face, or a
sphere pair's radical plane). *Crossing*: a null edge where the plane meets a face's boundary.
*Pierce*: a null edge in a face's interior, a ring of its own. *Segment*: the piece of the
section curve between two consecutive null edges; each side mints it as a *chord*, an arc of the
section conic, and the two arcs between the same two points are the *candidates*. *Run*: the
boundary between a chord's two null halves. *Azimuth window*: the hull of a face's boundary in
its chart's angular coordinate. *Germ*: the boolean's record of a null edge, per operand.

**Premise check.** The brief asks which of two selectors should pick the arc. Neither should:
the arc is fixed when a segment's two ends are paired, and both selectors re-derive it from the
face afterwards — a second copy of one fact in each lane, kept in step by nothing.

- The boolean's germ carries `dir`, the section's outgoing direction at the site toward the
  segment's other end, set by the crossing classification for crossings and pierces alike
  (`insert.rs`, `vtxfac.rs`). `germs_face_each_other` already reads its rotational sense about the
  section frame to decide which germs pair. That sense *is* the arc. (sure)
- The split's `conic_pairs` walks the conic along `n_plane × n_out` and pairs each down crossing
  (loop passing above→below) with the next crossing in walk order — the crossing rule, applied to
  pair. It runs only on faces crossed more than twice; a two-crossing face reads the same two
  facts (the half's sense, one heading sign) and gets the same arc. (sure)
- `chord_spec` then asks the face a third time, by window or by run side. (sure)

The planar side's gap is the same fact seen from the plane: the plane has no chart, but the
pierce germ on it has a `dir`. The "missing cue" was never missing; the planar lane reads the
mate's window instead of the datum it already holds. The three selectors agree because they are
three evaluations of one rule, two of them through a chart or a metric that fails where the rule
does not (a tilted sphere section; a cone apex tie — DR-37's wrong half; a fitted image).

The run-side rule's own open premise (the reflex run end): at a run end where the run departs
strictly off the plane, the inside candidate always leaves on the run's left and the complement
on its right, whatever the face's corner angle (derivation below). The reflex gate guards an
upstream configuration — a reflex vertex on the plane with both adjacent edges on one side, where
the reduce has to mint two null edges or refuse — not a flaw of the reading. The P3 item closes
with the rule. (likely)

**Ratified text.** Neither selector is a ratified decision: S9 is an archived M5 spec
(`M5-S9-SPEC.md` in the doc ledger; PR 145), not DESIGN.md, and no companion design page names
either rule. One DESIGN.md sentence moves as a consequence: frontier entry (d) says cyl×sphere
germ chords lack "the join window itself"; under this design no chord needs a window, and (d)'s
blocker is the C5 arm and the fitted carrier's chord description. A re-wording the change causes,
not a second decision.

**The answers as final states.**

*A — the arc is pairing data (recommended).* The lane that pairs a segment's ends hands the join
the arc: the boolean from the entry germ's `dir` (its sense about the shared section frame — the
test `germs_face_each_other` already passes); the split from the down half's sense and the face's
heading sign (`split_join_conic_heading`), one function serving both `conic_pairs` and the chord.
`chord_spec` keeps the C5 table and `oriented_arc` and gains one argument. Makes true: one source
of truth per segment; both operands' chords are one arc by construction, not by two rules
agreeing; the chord needs no chart, so every conic the table mints (cylinder, cone incl. its
apex, polar or tilted sphere; later cyl×sphere) selects the same way; a pierce germ on any
carrier has its arc. Leaves possible: a wrong `dir` ships a wrong body, as a wrong window does
today — the pairing's own checks (facing, alternation) are the certificate, and they are stronger
than hull containment. `JoinLane::BoolPlanar` keeps the wall by value and loses `window`;
`ChordRun`, `cross_loop_window_cycle`, `run_azimuth_window`, `ChartFrame`, `cone_chart_lever`,
the certified-run walk and the corner gate retire; `face_azimuth_window` stays for its other
readers (point-in-solid, pcurves). The planar side of a tilted plane×sphere cut builds; the
tilted split of a sphere body composes when the split admits spheres. Reversible: the retired
selectors are a `git revert` away, and the datum is one field.

*B — one selector, the run-side rule, with the wall's arc handed to the planar side.* The brief's
fold. Chart-free, so the window machinery retires too. Makes true: one selector. Leaves: the
selector re-measures (a side trilean, a tangency arm, a corner gate) what the reduce classified
and the pairing already used; it refuses at every pierce ring (no certified run), so the planar
side and a ring on a wall still need the hand-over — two paths to one arc remain, and the
pairing and the selector can disagree without anything noticing.

*C — the window where azimuth is monotone, the run side elsewhere, plus a run-side planar arm.*
Today plus a patch. Keeps the chart premise and the apex lift on the chord path, and the three
readings. Rejected.

Worked example: `brick((0.5, 3), (−2, 2), (−2, 2))` against the y-poled unit ball. The section
circle lies in the box face `x = 0.5` and crosses the ball's seam twice. On each half-band the
germs are crossings of the seam: the half-band's arc leaves each seam crossing into the band
(the rule; also what the run side reads). On the box face both germs are pierces, both candidate
arcs lie in the face, and the face has no cue of its own — but each pierce germ's `dir` names
the segment's direction, so the planar chord is the same arc as the band's chord. Today the
planar side refuses `SectionNotPolar`, and a run-side planar arm would refuse `NoCertifiedRun`.

Confidence. Recommendation: likely. Both lanes already hold the arc: sure. `dir` is sound for
every germ class (fan, strut, pierce): likely — read from its docs and its mint, not driven. The
reflex derivation: likely. Nothing downstream of the chord needs the window: likely.

## For the orchestrator

- **Derivation (reflex premise).** In the tangent plane at a run end, with `n_out` toward the
  viewer and the face on the loop's left, put the in-face "up" direction (the projection of
  `n_plane`) at 90°, so `d = n_plane × n_out` is at 0° and `−d` at 180°. A run bounding the
  below part departs at some α ∈ (180°, 360°). The ccw angle from the departure to `d` is
  `360° − α ∈ (0°, 180°)`: `d` is on the run's left. To `−d` it is `540° − α ∈ (180°, 360°)`: on
  the right. Nothing used the corner's other side, so the full sector's angle is irrelevant.
  Equivalently `(n_out × travel)·d = −travel·n_plane` for `travel ⟂ n_out`: the run-side sign is
  the half's sense. Both candidates are on the left only if the run departs in the plane, which
  is the along-edge case the rule already routes. A sector containing both `±d` needs the loop
  to pass below→below at a vertex on the plane: the reduce's "reflex-corner tilted crossings"
  frontier (DESIGN.md's standing entry), one layer up.
- **Not checked.** Provenance via `git log -S` timed out (the checkout is shallow, 2335
  commits); S9's origin is from `docs/MODEL-AB-LOG.md` row 24 (PR 145, 2026-07-31) and the doc
  ledger's archive line. I did not drive `dir` through the dangling-strut facing swap in
  `insert.rs` (the halves swap, the germ meta is per slot, so the datum should follow the half);
  an implementer should pin that the germ whose `dir` is read faces the half the join starts at.
  I did not confirm that the "off the seam plane" pierce-ring poses in `tilted_sphere_pair.rs`
  build end to end under A (the props' ring arm may stop them); the lens closed forms are the rows.
- **Errors in the items.** `planar-side-…-no-arc-cue` names `bool_between_arc_window`; nothing
  of that name exists. The boolean's adjacency skip is `skip_adjacent_chord` on
  `SegmentEdge::Is` from the germ locus — no window — and `between_edge_is_section` refuses the
  boolean lanes outright. The live azimuth premise on the planar side is the up-front
  `face_azimuth_window` in `bool_connect` and `select_arc` itself; both go under A. The P3 item's
  premise is doubtful (above); file the reduce's on-plane reflex vertex as the real owner if it is
  not already the standing frontier entry.
- **Rows that move under A.** `a_tilted_section_stops_at_the_pierce_ring_and_the_planar_side`'s
  planar half flips to builds against the planar-side item's closed forms (quarter cap
  `20 + 4π/3 − c/2`, `20 − c/2`, `c` the cap beyond `x = 0.5`); the `SectionNotPolar` variant,
  `split_sphere_section_polar`, `split_arc_window`, `split_arc_chart_orientation`, the
  `split_arc_run_*` predicates and their k-lint rows retire; `cert4r1_the_centred_anchoring_…`
  and the `select_arc` unit tests go with the selector. Bits of surviving chords should not move:
  the same arc through the same `oriented_arc`.
- **Off the question.** No tier-3 check compares a trim against the face it bounds
  (`run_azimuth_window`'s own doc says so); A removes the window as a selector, not as a possible
  certificate — if a face-window containment row is wanted, it is a tier-3 question, separate.
  DESIGN.md frontier (d) needs its re-wording with the landing PR.
