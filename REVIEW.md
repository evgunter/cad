# Review r2: PR #4345, a plane across a one-face wall (JOIN)

Frozen head `a92e7cbd`, base `f3755cd4`. Lane isolation held: I read no other review branch, session or PR comment.
Everything ran in release, base and head in separate target dirs. Probes: `crates/sweep/tests/wrap_r2_probes.rs`
(registered in `all.rs` on this branch only). Their output lines are in `probes/wrap-r2/`. Every build was read through
`common::differential::outcome`, then tessellated and `check_mesh`ed.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 5 · NOTE 6. No wrong body was found. Every failure below is a typed
refusal, a test gap, or a defect that reproduces on base or on a two-arc control.

## Claims

1. **Holds.** I re-derived three closed forms independently: the CLEAVE tube's overlap 0.16π; the near-end plane π(c+½),
   from the mean height c + m(1 − x̄) with x̄ = 0; and the beside-edge segment ¼·acos 0.8 − 0.12. Beyond the six
   witnesses I ran 70 poses × 6 ops:
   - flat planes off-centre, with the wrap edge at azimuths 0, 2 and 4;
   - tilts about x of 0.05°–44°;
   - tilts about y of 0.5°–88°. At 78°–88° the plane meets the wrap edge at 12°–2° (120-wide box, F1s);
   - five tilt-axis azimuths, with r from 0.37 to 1.17;
   - planes 1e-2, 1e-4 and 1e-6 above the end vertex;
   - tubes r ∈ {[.3,.5], [.05,.1], [.9,1.2], [.45,.5]}, off-centre, under slabs turned 3° and 15° about x and z.

   Every op in both orders builds to its closed form within 1e-7, passes t2, t3′ and the certificate, and meshes
   clean, apart from NOTEs 1–3. Base refuses all of them `SingleSiteSectionLoop`. One true crossing refuses (MINOR 5).
2. **Holds where reachable; unsure for two clauses.** These non-transverse one-site loops refuse `SingleSiteSectionLoop`:
   - a plane tangent to the cap at the end vertex (F1v);
   - a circle touching a y-poled ball's seam meridian without crossing it, at β = 60°, 30° and −45° (F6). The control
     0.05 inward builds SOUND;
   - **the parked parent row's in-face conic**, on supported lanes (F4d): a bored tube whose wall turns from cylinder
     to sphere on y = 0, inside the box face y = 0. The result is `{count:1}` in all 12 ops: the in-face circle
     refuses, and the inner wall's crossing passes the gate.

   A plane through the end vertex transversally builds, the same on base. Two loops on one locus pair (one-face
   torus) and a curved partner (coaxial cone or ball) refuse earlier, `GermFrameUnsupported` /
   `CurvedPairUnsupported`, so `shared` and the both-sides-wrap arm are never reached (MINOR 2).
3. **Holds by execution.** Every op in both orders is SOUND for:
   - plate holes inside the conic (0.3, 0.2) and outside it (2, 2);
   - a hole at the conic's centre;
   - a boss inside the conic;
   - two concentric aligned-seam one-site loops, the outer conic enclosing the inner pierce ring.

   The Euler steps check their own preconditions: `mfkrh`'s ring, and `kfmrh`'s `FaceHasRings` on the hole.
   `join_lone_ring` checks two walled faces and a definite, opposite winding. A pending lone ring hits
   `place_pending`'s `RingHomingAmbiguous` (`l1 == l2`), which is typed. The face-sense argument
   (`chord_join.rs:3391-3396`) rests only on the tiers passing. The hole choice is pinned in one order only (MINOR 1).
4. **Holds.** Base against head, timing stripped, lines sorted where threads interleave:
   - `pinch_runs_battery`: 3 030 lines, 0 differ;
   - `corner_pairs_battery`: 16 387 lines, 0 differ;
   - `rc_wide_battery` shards 0, 41 and 83 of 84: 486 lines each, 0 differ;
   - **`pocket_ring_steep_ellipse`** (not run by the PR; curved pierces of a wall with rings): all rows, 4 404 lines,
     0 differ;
   - `one_segment_loop`, `germ_coplanar_conic`, `review_cleave_wrongarc` and `m6_tube`: pass sets differ only by the
     PR's rename and deletion; TALLY lines identical.
5. **Partly holds.**
   - **M3, the hole choice flipped:** all six witness tests go red, but only through their rows whose PLANAR operand is
     A (`Pcurves{LoopNotClosed}`). See MINOR 1.
   - **M1 (both sense checks removed) and M2 (`shared` dropped), built together:** all 21 rows of the three test files
     still pass, and every probe line is unchanged (MINOR 2).
6. **Partly holds.** Each listed site is right. The sweep missed the readers and prose of "the join's one
   enumeration" (MINOR 4). The REST disposition is acceptable: `rest.rs:533` falls back to the join refusal, and the
   parent row puts the REST half under D10 stage 4.

## Findings

**MINOR 1: the hole choice is unpinned when the planar face is operand B** (executed; `chord_join.rs:3423-3437`).
Instrumented M3 prints `wa=Positive wb=Negative disc=9 hole=8` in A ∪ B of the slab, so `kfmrh` receives the face
wound WITH the outer loop. Even so, AuB, AnB and A−B of every F1 pose stay SOUND, with exact volume, cert and t3′.
Only BuA, BnA and B−A refuse. So `one_segment_loop.rs` and `germ_coplanar_conic.rs`, which always have the wall as A,
cannot detect a flipped choice, and something downstream repairs or accepts a co-wound ring on B. *Unsure* which.

**MINOR 2: three guards have no row that can go red** (executed; `join.rs:667` `shared`, `join.rs:670-691` the
`is_up` and `rotational_sense` checks). M1 and M2 survive everything. `shared` looks unreachable on today's lanes,
since every torus × plane pair refuses first. The both-sides-wrap arm (`na > 0 && nb > 0`, a curved partner) is
exercised by no row.

**MINOR 3: no row reaches `SingleSiteSectionLoop` through the kernel any more** (inspection; the PR deletes
`germ_coplanar_conic.rs` `a_closed_section_loop_with_one_site_refuses_typed`). Only editor-core's message table
remains. The refusal at `join.rs:1124-1135` is untested. F1v, F6 and F4d reach it, and F4d is the in-face fixture the
parent row says the tree lacks.

**MINOR 4: the "one enumeration" contract is broken** (inspection).
- `join.rs:500-501` and `rest.rs:40-41` say `section_segments` is the enumeration that the join's surgery and REST both
  read. The join now also reads `wrap_site_segments`.
- `join.rs:745-757` `segment_sites`, exposed as `test_support::boolean_segment_sites` (`lib.rs:468`), omits wrap
  segments.
- The `fragments` doc (`chord_join.rs:673-681`, "faces the chord mefs minted") now also gets the `mfkrh`-promoted
  face.
- The module's `join` contract (`chord_join.rs:13-37`) lacks the lone-ring branch.

**MINOR 5: a true pair of wrap crossings passes the gate and refuses** (executed; F3 annulus). Take a one-segment
annular tube r ∈ [0.5, 1] through a plate's top face. With its two wrap edges at the same azimuth it builds SOUND. With
them at 0/π or 1/4, all 6 ops refuse `Join(SectionLoopUndecided)` (`join.rs:3131`). That is a typed frontier, but it
is the washer or annular-boss shape PATHS unit 4 (unblocked here) will produce. It owes an issue row.

**NOTE 1.** Every tilted-plane build is `operand=false` (far-brick union `Containment(VolumeUncertified)`). The n = 2-arc
cylinder control fails identically: pre-existing.
**NOTE 2.** The tube witness's B∖A fails t3′ `CensusUndecidable` across its two lumps. Two nested disjoint
full-revolve tubes, united, fail identically on base (F7): the census frontier, not this arm. The witness file never
checks t3′.
**NOTE 3.** At tilt-y 85° and 88°, with the seam at the ellipse's major-axis end, the result meshes non-manifold at
chordal 2e-2 and 5e-3 with `tessellate` returning `Ok`. The operand and the two-arc result are clean. The **base split
lane** gives the same edge ids, so this is a pre-existing tessellation defect, newly reachable from the boolean. No
`work/tess/` row covers it; it should be filed.
**NOTE 4.** The plane z = m(1−x) + 0.8y through the end vertex: ∪ and A−B refuse `VolumeUnmeasured`, the same on base.
**NOTE 5.** Retracted: my first 80°+ rows used an 8-wide box whose sides reached the wall, and base mis-measured them
identically. The F1s rows replace them.
**NOTE 6.** The "0 lines moved" claim rests on `#[ignore]`d batteries with no register (Q6). Standing practice.

## Style (Q1–Q8 exercised; Q8 partial: `chord_join.rs`' header and its join, lone-ring, rehome and pending regions)

- **Q1** `join.rs:635-638` and `join.rs:1124-1131` spell "is a one-site record" twice: unused slots, equal loci and a
  conic frame. The gate also rebuilds `open` and re-marks the matched slots, which `section_segments` already
  knew. *sure*
- **Q1** One concept has two words: "wrap edge" in code and docs, "seam" in the message (`chord_join.rs:463`). *likely*
- **Q7** `wrap_site` returns a count that is only tested `> 0`. The predicate at `join.rs:662` is a puzzle where an
  enum (wrap / pierce / other) would state it. *likely*
- **Q7** `join_lone_ring` re-enters `self.join`. Only `lone_ring` rejecting an outer loop stops recursion, and nothing
  states it. *unsure*
- **Q2** The `set_face_sense` comment (`chord_join.rs:3391-3394`) is justified only by the tiers, and MINOR 1 suggests
  they would pass either way. *unsure*
- **Q3** The witness file checks tiers 1–3 and volume, but not t3′, the certificate, operand legality or the mesh.
  NOTE 2 went unseen there. *sure*
- **Q5** The message's "not a one-face wall's seam crossed by a plane" also covers F6's touching circle. *likely*
- **Q6** The REST residue for wrap crossings is scheduled only by prose on the in-face-conic row. *likely*

REVIEW COMPLETE
