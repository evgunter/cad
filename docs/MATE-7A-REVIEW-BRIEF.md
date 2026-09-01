# MATE-7a stored symmetric review brief (substitute {R}, {PROBES}, {OTHER})

You are a blinded adversarial REVIEWER (label: {R}) for unit
MATE-7a of the S-MATE program (repo evgunter/cad).

UNIT UNDER REVIEW: PR #1477 — "MATE-7a" (issue 968 items 1–2 + the
ratified π arm of the shared-rim routing). FROZEN review head
`530eb8f59303bccc3914f8359f0a0bd840528192` on branch
`mate/7a-torus-rest`; review THAT head. Hosted CI run 33478558188
gated it green (lane=both ASKED via trailer × eps 1e-6 drawn; all
four test legs green).

READ FIRST, by path: docs/prompts/reviewer-style-lane.md,
docs/MATE-7A-SPEC.md, and docs/MATE-7-TANGENCY-DESIGN.md (RATIFIED
— the ruling chain this unit executes; all on main); the PR #1477
body in full (its five deviations are audit targets). The unit's
HEADLINE DEVIATION reverses a spec premise — deliverable 5 (lily
wall 1 retires) did NOT land, on a measurement claim. That claim
is your highest-value target in either direction: confirm it
independently or break it.

CLAIMS TO FALSIFY (execution outweighs inspection):
1. The LILY WALL 1 measurement: the operand gate's refusal names
   the stem's tube wall against the arch's FAR cap ~2 m away —
   the exact loci never approach, and the overlap is the
   WHOLE-TORUS bounding box (a 22° arc of a 5 m ring boxed as the
   full 10 m ring; the artifact VERBS-GATE fixed for the cone and
   deferred for the torus). Further: the weld has NO torus contact
   to declare (stem tube 0.060 vs arch tube 0.052 — the walls
   share only the plane they end on). RE-MEASURE all of this
   yourself from the lily scene. If it holds, the spec's premise
   was wrong and the deviation is honest; if it breaks, the unit
   failed its demo deliverable and the PR misdescribes why.
2. The COVERED-PAIR GATE (item 1): `first_unsupported_pair` takes
   a `covered` predicate; a declared cross-operand pair is not an
   offending pair; what a declaration may name is bounded by
   `carrier_eq`'s rung list ("one list decides both"). Attack: can
   a declaration smuggle an uncertifiable kind or a same-operand
   pair through? Do ∖ and ∩ genuinely keep their roster verbatim
   (the never-covered predicate argued at the site)?
3. The TORUS CARRIER RUNG (item 2): centre compared WHOLE (not
   axis-projected — two coaxial tori slid along the axis are
   different carriers, pinned), axis line, both radii; four
   metered NUMERIC margins. Verify the margins' certified
   arithmetic; mutation-check that each of the four rows can red;
   check the declaration inventory widening.
4. The π-ARM PRICE (item 3): 34 metered rows for one rim (27
   classification = 9 CERT_SAMPLES × 3 predicates, 5 rim
   identification, 2 conformal screen); the tier-3 fold IMPORTED,
   not restated (no new numerics). Verify the count and the
   no-new-numerics claim.
5. BOTH ROUTING ARMS pinned (item 4): the G1 tube chain
   classifies π → `Seam`; the kissing torus pair → `Slit` (2π)
   refuses via new `BooleanError::RimArmUnbuilt` CITING the
   ruling and naming the arm. Red-first? Is the refusal typed,
   reachable, and its citation correct?
6. FLIPPED PINS: NONE claimed — klein walls 3/4 and lily 1/2 name
   exactly what they named before (the spec EXPECTED re-blessing
   onto a widened admission). Verify the pins are truly unmoved
   and that this is consistent with the gate change (why does the
   widened admission move nothing? — the honesty of that story).
7. Deviation 2: the covered rung is NEAR-INERT on today's
   whole-torus boxes — reachable only where neither operand
   carries a non-torus face — pinned both ways. Verify both pins
   and the reachability argument.
8. Deviation 3 (the fence blocker): the lane cannot complete an
   operation because `geom_brep::circle_residual_harmonics` has
   no torus arm; the circle rung takes the frontier door BEFORE
   the C8 declared-cover rung, and every edge of a torus body is
   a circle. Two rows hold that boundary. Verify the call-order
   claim against the code and the two boundary rows.
9. Deviations 4–5 (out-of-fence touches): the one-line twopeg.rs
   repair (independently landed on main as #1474 — after the
   main merge the final diff should carry NO twopeg change;
   verify) and the face_normal.rs registration (the routing reds
   the hand-multiply inventory by construction). Verify both
   dispositions; then check the WHOLE diff against the merge base
   for any UNdisclosed fence crossing (the fence:
   crates/topo/src/boolean/ except vertex_on_curved_face,
   carrier_eq's home, topo+sweep TEST files, demos/tour/src/lily.rs,
   klein pins; NO geom-brep beyond the disclosed line, no
   census.rs/chart_region.rs, no tangent_locus, no editor-core).
10. D2-ADDENDUM honesty in the PR body: with lily wall 1 not
    retiring, what refusal-reach actually moved? Is the
    classification against row 2 stated honestly?
11. NO REGRESSION: boolean/sweep/topo suites green under the
    gated lane=both point; MATE-2's `Placement` rows and the
    germ-arm pins are near ground — verify none moved.
12. ε POSTURE (issue-1356): the four NUMERIC margins' band story;
    lane=both is the both-compile-modes precedent — is it the
    right ask here and is the band argument in the PR honest?
13. SWEEP HONESTY: genus "a per-face-KIND gate arm whose admission
    posture predates covered declarations" — 10 hits, 3 fixed, 7
    not-this-unit; spot-check two dispositions; are the four
    stated blind spots real?
14. KEYWORD HYGIENE + record: the PR must NOT close issue 968
    (the kissing arm remains) and must say so; no closing keyword
    anywhere before a #-reference.

METHOD AND RULES:
- Own worktree, own default target/; sibling lanes share the
  machine; foreground, one at a time; never end a turn with
  background work active. Work only inside your worktree — the
  shared session scratchpad is OFF-LIMITS.
- Do NOT re-run the full regular suite locally (standing method
  rule): the hosted step-verified gated run is the suite evidence.
  Targeted tests and probes are yours to run.
- Commit probes to branch {PROBES} and push. Do NOT push to
  mate/7a-torus-rest, no PR comments, no PRs.
- ISOLATION: until your report is delivered, do not fetch, read,
  or check out the other review lane's branches or artifacts
  (anything named like {OTHER}), and do not read mate/ab-state or
  any MATE-AB-STATE file. Disclose any accidental glimpse.
- BLINDING: never speculate about the implementing model; no model
  names anywhere, no Co-Authored-By trailers in commits.
- Structural lane: duplication; rows that red only at a chosen
  fixture; invalidated premises; comment truth.

REPORT (final message, ≤150 lines): verdict (MERGEABLE /
MERGEABLE-AFTER-FIXES / NOT-MERGEABLE), findings MAJ/MIN/NOTE each
with demonstration, silent-deviation count, rubric triple
idiom/tests/docs (1–5), claims-to-falsify outcomes one line each,
probe branch contents, isolation/blinding disclosure, approximate
token and wall-clock usage.
