# CERT-N3 — Track N's remainder: S235, D31, D98, D244, and C24's analytic member

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md` §CERT-N;
difficulty logged at spec: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting. The
Track N table rows `S235`, `D31`, `D98`, `D244`, `C24` and findings
`S235`, `S32` (the curve-side paragraph), `S18`'s D31 row in
`docs/SMELL-SCAN-2026-08.md` are the primary specification; CERT-N1's
`spline/knots.rs` (`InteriorKnot`, `interior_knot_runs`) and
`curves/nurbs.rs` (`homogeneous::<N>`, `ders`, the order-1 jet door it
lands in its fix pass) are the substrate — read them first. Branch
`cert/n3-track-n-remainder`. Sequenced AFTER CERT-N1 merges.

## The rows, in order

1. **D244** (smallest; filed by CERT-M1): `crates/geom-core/src/spline/
   hull.rs:113-118`'s private `fn bracket<E: CertifiedEnclosure>` is
   `RingInterval::from_certified` under another name with an eight-line
   restatement. Inline at its call sites, delete the restatement, and
   keep `geom-core/tests/decoration_seam.rs`'s header true (CERT-M1
   rewrote it to name this crossing).
2. **D31** — `sweep::skin::make_compatible` (`sweep/src/skin.rs:388`)
   and `geom::curves::fit`'s `deviation_from` (`geom/src/curves/fit.rs:706`)
   are ONE routine written twice: union two knot vectors' interior runs
   and refine both to the union. Give the routine one home in
   `geom-core/src/spline/algebra.rs` (the proposed home; `algebra.rs`
   may not exist — mint it beside `compose.rs` and say why it is not
   `compose.rs`), built on `interior_knot_runs`, and make BOTH callers
   call it. The `sweep/src/skin.rs` call site is this row's (the table
   says so); its error mapping (`SkinError`) stays the caller's. Bit
   identity of both callers' outputs against the retired spellings on
   a corpus that includes rational sections and mismatched
   multiplicities — measured, as CERT-N1 did for C24.
3. **D98** — `KnotVector::unit_segment(degree)` (`spline/knots.rs:560`)
   clamps a degree it could refuse, and the claim licensing the clamp
   is the wrong claim. Read the clamp, its doc, and every caller;
   decide refuse-vs-clamp by the D2 addendum (a degree outside the
   representable set is which row?) and land the typed answer with a
   red-first row at the boundary; if the clamp is kept, the licensing
   claim is corrected to the true one.
4. **S235** — the exact conic box (`geom::curves::boxes::
   {circle_arc_aabb, ellipse_arc_aabb}`, public, tighter on orientation
   AND span) has no production caller while `topo::boolean::boxes::
   EdgeBoxRule::ConicAmplitude` hand-derives the span-blind triangle
   bound (`|û_i|·a + |v̂_i|·b`; four of six carriers on the S16 cylinder
   take the wide branch at 1.366). **The one `topo` call site is this
   row's** — `topo/src/boolean/boxes.rs`'s ConicAmplitude arm
   (`:847` region; `:1107`/`:1283` already cite the exact form as
   ratified). Adopt the exact box there. This is a TIGHTENING with a
   stated obligation: `EdgeBoxRule`'s NURBS bullet says a tighter box
   starts pruning pairs examined today, so the rung-3 operand gate must
   admit the kind first — read that bullet and satisfy it (or show it
   is already satisfied for conics), and it changes the box's SPAN
   behaviour, so the pair-pruning corpus is re-measured before/after
   (which pairs stop being examined; every one of them justified by the
   exact box's soundness argument, which `s16_box_soundness.rs` pins —
   extend it to the adopted arm). The structural half — why two
   constructions existed and the correct one was unused — is answered
   in the body and the answer is what stops a fifth (the `EdgeBoxRule`
   doc points at the one home).
5. **C24's analytic member** — `Curve3::deriv`/`deriv2` on the conic
   arms are the discarded-jet shape; the consumer is
   `topo/src/splitting/neighborhood.rs` (Track Q's, calls both at one
   `t` under "the base-endpoint jet"). Minting a public `CurveJet` is a
   design element (C-R19 tier two). This unit does NOT mint it blind:
   measure the cost at the one consumer (release, the two calls vs a
   jet), and if the cost is nil, close the row with the measurement and
   a pointer to the surface-side precedent; if it is not, write the
   `CurveJet` design question as a filing for Evan (a design
   conversation, not a lane) with the measurement attached, and leave
   the row open naming it. Either way the row stops saying "filed, not
   fixed" without a number.

## Fence and posture

- **Fence (rule 1, plus two drawn seams):** `crates/geom/src/`,
  `crates/geom-core/src/{spline/,linalg/}`, their tests; PLUS
  `crates/sweep/src/skin.rs`'s `make_compatible` (the row's own call
  site; Track T's ground — BLEND/SEAT are live in `sweep/`: re-merge
  main before every push); PLUS `crates/topo/src/boolean/boxes.rs`'s
  ConicAmplitude arm and `s16_box_soundness.rs` (the row's own call
  site; Track Q's ground and the BOOL program is LIVE on
  `topo/src/boolean/` — at dispatch the orchestrator checks for an open
  BOOL PR on `boxes.rs`; if one exists this row is sequenced after it
  and the lane is told). `neighborhood.rs` is read and NOT edited.
- ε: D244/D31/D98 are structural; S235's box is an enclosure claim
  (arithmetic in the ring, no tolerance) whose fixtures ride
  `Tol::witness()` — the issue-1356 shape: `CI-Config: lane=both
  eps=1e-12` with the per-band premise stated and the three-ε local
  sweep on every new/changed row.
- Review: standard v6 dual; S235 is where a wrong answer is reachable
  (a box that is too tight is a soundness hole in every boolean that
  prunes on it) — say so in the body so the reviewers execute it:
  adversarial arcs (span crossing extremal angles, rotated `u_ref`,
  ellipse with extreme axis ratio, degenerate span) against a dense
  sample of the locus.
- Landing (rule 3): delete `D244`, `D31`, `D98`, `S235` (and its finding
  text; relocate the two-halves rule into the `EdgeBoxRule` doc first),
  and `C24` if closed by measurement (else rewrite it with the number);
  Track N's table is then EMPTY — say so in the body, and delete the
  table's header count if one survives. Expect the SMELL-SCAN conflict.
- No `Co-Authored-By`; rows spelled out; push early to
  `cert/n3-track-n-remainder`; the lane rules in full.

## Acceptance

- One union-and-refine routine with two callers, bit-identical to the
  retired spellings; `unit_segment`'s boundary decided by the addendum
  with a red-first row; the exact conic box adopted at the one `topo`
  site with the operand-gate obligation satisfied and the pruning delta
  measured and justified; D244's alias gone; C24 measured.
- Sweep obligation: other hand-derived bounds standing beside a
  shipped exact one (S16's class, all levels) across `topo/src/boolean`
  and `geom-brep/src` — hit list with dispositions, filed on the owning
  tracks; what the pattern cannot match.
- Deviations stated; D2-addendum classification for D98's decision and
  for anything S235's adoption retires.
