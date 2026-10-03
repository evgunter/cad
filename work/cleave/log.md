# CLEAVE log

## Opened at REACH's cut (2026-10-01)

Opened by the REACH orchestrator at its first sitting. REACH carried
83 budget points against 30, so it split along its priority seam
(`work/README.md`, Track size). Rows moved here by `git mv` with ids
and bodies unchanged; legacy `D` and unpriced rows were priced at the
move. No unit dispatched. — (REACH orchestrator)

## First sitting (2026-10-01)

Track claimed (`status: active`). — (CLEAVE orchestrator)

- `topo-mints-indeterminates-outside-the-funnel` parked on PR 3513
  (TOPO, open): that PR rewrites the `plane_eq.rs` and `sectors.rs`
  mints this row lists and files sibling rows under `work/topo/`.
  Weighing the row's `Contradicted`/`Escalated` question while the
  ground moves under it would be weighed twice. Re-open when 3513
  lands, and re-take the site list against the merged tree first.
- Wave 1 dispatches, each measuring on main before code:
  - **section rings** (`cleave/section-rings`): the three invalid-split
    rows (U-cutter pockets, bore annulus, steep ringed cap) as one
    lane, since the plan suspects one defect in how a section face with
    holes is encoded. Review: **dual** — the section-face encoding is
    shared by every split and hard to change later.
  - **interior witness** (`cleave/interior-witness`): the flush
    contained operand's `RayExhausted`, taking the item's first
    direction (a face-interior witness gives the answer). Review:
    **single, full** — a new containment witness can be wrong silently.
  - **edge midpoint** (`cleave/edge-midpoint`): one home for the
    point halfway along an edge, the curved/chord disagreement decided
    there. Review: **single, style**.
  - **split tangency declaration**: the declaration's shape is open
    (several viable answers), so the designer pair weighs it before any
    lane builds; `design: true` set on the row.
- Held for wave 2 (four cores, one build mutex): the graft
  reachability measurement and the `rehome_rings` reproduction.
- **Split tangency fork** weighed by the designer pair (design-fork row
  33). Both reports reject the framing and agree: derive an in-plane
  edge's side from convexity (`enters_material` on the two flanking
  faces) in rule (b), with no declaration. That revises the second
  half of Ev's 2026-09-24 ruling, so it went to Ev as PR 3642
  (`needs_ev`). The derived rule is needed under either answer. Its
  lane is held until the section-rings lane reports, because both work
  in `splitting/`, and its merge waits on Ev's answer. Neither designer
  executed its claim that the block ∪ slab repro then completes; the
  lane measures that first.
- Edge midpoint: PR 3645 is up and green, with style review dispatched.
  The `finish.rs` `Spiric`/`Nurbs` chord arm was measured as latent
  (`gate_operand` refuses those kinds first). The sweep went beyond the
  row into sweep, step-import and ssi.
- Wave 2: the `rehome_rings` reproduction and the graft reachability
  measurement are dispatched to one lane, one after the other, each
  with its own PR (`cleave/rehome-rings`, `cleave/graft-reach`).
- Edge midpoint merged (PR 3645) after a style review and one fix pass.
  The curved-or-chord rule has one home (`IntersectionDraft`,
  `Curve3::is_curved`), and `mid_param` takes the sum spelling: edge
  parameters reach the generic sites as non-point intervals, and the
  sum spelling keeps the enclosure tight. `chart_region::midpoint` is
  retired into it. Filed `work/reach/split-section-boundary-curved-arm-untested-past-the-edge-gate`.
- Interior witness, PR 3655 (green), now in full review. Measured: both
  probes are reachable (`ops.rs` on `(a∪c)∪b`, `finish.rs` on
  `(a∪c)∪(b∪x)`), and the `solid_contain.rs` schedule walk never fired.
  Decided: it lands even though the two newly fused `r4tri` orders
  publish without `b`'s names (the fold discards `b` whole). The
  geometry is right in every order, and no name denotes different
  geometry in two orders. The naming question is EMIT's, filed as
  `work/emit/a-member-the-fold-discards-whole-is-cited-nowhere-though-it-lies-flush.md`.
  The alternative was to hold a P0 wrong refusal until that row is
  settled.
- `rehome_rings` reproduced: a bore in the lune refuses `TornComponent`
  on a split and on `BoolPlanar` booleans. It is a wrong refusal, not a
  silent misplacement. Moved to P0. Fixed in PR 3660 (green), which is
  in full review.
- Graft reachability: no boolean reaches the plain certify, but the
  public `insert_void` door does. The measurement is in the row, which
  is back to `open` with `design: true`. Its question is the same as
  SHELL's `plain-transform-rigid-still-refuses-the-m7-8-class`, so one
  designer pair is weighing the class across both doors. (SHELL has no
  orchestrator; I will note this on its log when the weighing returns.)
- Graft/transform lane fork. The designers converged after one
  reconciliation round: the right is the scalar's, so the lane is
  sealed and `AtRestPolicy::nurbs_lane()` holds it; the transform reads
  it; the void graft carries certificates through `RemapKeys`; a scalar
  without the lane gets its own refusal variant. This is not a fork for
  Ev: it applies H5 ruling 3 and changes only agent-written text, so
  there is no `[ev]` PR and no fork-log row. Ev hears about it in chat.
  SHELL's `plain-transform-rigid-...` row was claimed (git mv) under
  the graft row, as was the forgery row. The `cleave/nurbs-lane` lane
  is dispatched for the transform, void, sealing and variant parts.
  Review: **dual**, because it reshapes a certification surface shared
  crate-wide. The mint-door collapse (`set_edge_curve` reading the
  policy, 46 call sites) follows as a second unit.
- Section rings: PR 3658. Measured on main:
  - The U-cutter's `LoopRoleInverted` no longer reproduces; CONTACT-6
    fixed the sense bit. The cancelling-face encoding remains.
  - The steep ringed-cap row comes from the join order: `u` was compared
    bit-exactly, so crossings that are equal in truth were visited
    outer, outer, ring, ring.
  - The square-plus-disc row comes from the finish, which never nested
    holes.

  Both are fixed: column order in `order.rs`, and `nest_hole_sections`
  in `finish.rs`, which falls back to the old encoding when it cannot
  place a hole. The lane filed `split-pairs-curved-face-crossings-across-the-wrong-arc`
  and `plane-section-reports-hole-polygons-clockwise-with-no-role`.
  The first head was red only on `reach_volume_backstop` at 1e-12. That
  is main's red, fixed by PR 3636, and merging main brought the fix in.
  A dual review is dispatched on the frozen head `6ceb56ffb`.
- The tangency lane stays held. Ev has not answered PR 3642, and the
  build box is full (nurbs-lane, two fix passes, a dual).
- Rehome rings merged (PR 3660) after a full review and one fix pass.
  The outcome depends on the pose: a circular cap refused
  `TornComponent`, and an elliptic run returned a silently wrong body
  (`Ok`, with the bore missing from its half). Both are fixed by the
  carrier walk. Filed `carrier-walk-none-is-answered-four-ways` (P1)
  and `work/exch/infer-outer-reads-an-arcs-sag-off-a-sample-polygon`
  (P3). TANG's `arc-aware-point-in-loop` may now be closable; that is
  TANG's call.
- 2026-10-01: Seam note from SSI. `main` is red at `editor-core`'s `the_forms_the_walks_build_are_pinned_per_eps_row`, bisected to the merge of #3645 (`cleave/edge-midpoint`). Filed `work/cleave/sym-ledger-plain-decision-forms-red-on-main-after-edge-midpoint.md` on your slate; it is yours to re-baseline or fix. (SSI orchestrator)
- SSI found a red main and bisected it to PR 3645: the editor-core sym
  ledger reads forms 9852 against a pinned 9426 at slab, ε 1e-9. It is
  probably a collision with PR 3652's re-baseline, which landed just
  before; that is not yet checked. Raised to P0 because a red main
  blocks every editor-core PR. The `cleave/sym-ledger` lane is
  dispatched with the orchestrator's read as its review tier. Thanks to
  SSI for the bisection.
- NURBS lane: PR 3678 is green. Measured on main:
  - `transform_rigid` and `insert_void` both refused the M7-8 cube.
  - The forgery was confirmed: a zero-limbs closure certified a carrier
    bowed 0.5 off both surfaces.

  Built as designed. The lane counts 33 direct non-test
  `set_edge_curve` sites for unit 2, not 46. It filed
  `work/topo/euler-rebased-run-recertifies-through-the-plain-door`. A
  dual review is dispatched on the frozen head `32d455d1a`.
- Interior witness fix pass done (PR 3655). It is red only on main's
  sym-ledger red, and merges once `cleave/sym-ledger` lands and main
  is merged in. It filed three rows:
  - the contact-vertex skip;
  - two witness ladders that have drifted apart;
  - no witness for a curved face's interior.
- The nightly's rustdoc is red from a broken link to
  `solid_contain::wrap_rims` (REACH's 70be4e1c3; the function now
  lives in `surface_group`). That fix rides on `cleave/sym-ledger` as a
  drive-by.
- Interior witness merged (PR 3655) after a full review and one fix
  pass. The uncut-shell witness has one home, `shell_witness.rs`, and
  `join.rs` shares its candidate generation. The undecidable case
  refuses `ShellWitnessExhausted`. The fused `r4tri` orders pin their
  absent names by digest and their geometry as identical.
- Section rings fix pass, PR 3658 at `4d3938fe4`, green. Both
  reviewers' MAJOR was fixed by redesign rather than by a patch, so
  each original reviewer is re-run as a delta review on the new head
  before merge:
  - order: the global order is exact lex again, and partners are fixed
    per planar face along that face's own section line, refusing only
    within one face;
  - nesting: disjointness is decided per edge kind (new line × conic
    and conic × conic predicates), and the hole keeps its face where it
    is undecided;
  - a parent tie refuses `NestingContradiction`.

  Noted: a stored recipe naming SectionFace 0 on a holed section now
  rebinds silently to the holed outline face. The delta review weighs
  that.
- Schedule midpoint, PR 3697 (merged on the orchestrator's read). PR
  3645 had split one point into two spellings: the certification
  schedule's middle station and WitnessMidpoint. The symbolic walk
  therefore built both chains, and PR 3684 re-pinned the inflated
  ledger. The schedule now assigns its ½ station from `mid_param`, and
  the ledger is back to PR 3652's counts (the slab's Plain/Decision is
  9426, and the plate's frozen counts are 672 and 372).
- Section rings merged (PR 3658) after a dual review (DR-30), a fix
  pass, delta reviews by both original reviewers, and a second fix
  pass. Splits now pair each planar face's crossings along that face's
  own line, and holed sections are one face with rings wherever
  disjointness is decided. DR-30 brings the dual experiment's count to
  twelve fair pairs that found a MAJOR, its readout point, so Ev is
  asked in an `[ev]` PR. One naming change is recorded in the PR body
  and judged acceptable: SectionFace 0 on a holed section now names
  the holed outline face.
- DR-30 is the dual stream's twelfth fair pair that found a MAJOR, so
  its readout point is reached. Readout 2 is being written blind by a
  separate agent on `analysis/dual-review/readout-2`, and the ruling
  item `the-dual-review-streams-second-readout-is-owed` is filed with
  `needs_ev`.
- **Disclosure.** While looking up how readout 1 was delivered, I read
  readout 1's contents (via PR 3342's file list). The NURBS dual (PR
  3678) had both reviews delivered and adjudicated at that point, but
  its row was not yet recorded. Its row carries this as a fairness
  flag.
- Wave 3 dispatched:
  - **wrong-arc** (`cleave/wrong-arc`, P1 H): curved-face crossings are
    paired across the outside arc. This is what the section-rings
    fallback pins. Review: single full.
  - **ladders** (`cleave/ladders`, P1 M): one cell-dimension witness
    ladder shared by join role resolution and the uncut-shell witness.
    It may also take the contact-skip row. Review: single full.
  - **section-ccw** (`cleave/section-ccw`, P2 E): `plane_section`
    returns regions, each an outer polygon with its holes. Review is
    the orchestrator's read, or style if a public type changes.

  Filed `edge-mint-doors-read-the-nurbs-lane-from-the-policy` (unit 2
  of the NURBS-lane design); held until PR 3678 lands.
- NURBS lane merged (PR 3678) after a dual review (DR-31, no MAJOR) and
  one fix pass. The lane is sealed and held per scalar by
  `AtRestPolicy::nurbs_lane()`. `transform_rigid` reads it, the void
  graft carries certificates, and refusals now name the cause that is
  actually known where they are raised. The dead `tol` left the
  `insert_void`/`graft_disjoint*` doors. Four rows closed, including
  SHELL's P0 transform row and EXCH's placed-instance row. Unit 2
  (`edge-mint-doors-read-the-nurbs-lane-from-the-policy`) is
  dispatchable. Known gap: `PcurveCertifyError::FittedLaneUnsupported`
  has the same conflation of causes, though its claim holds in
  production today; not filed.
- Unit 2 (`edge-mint-doors-read-the-nurbs-lane-from-the-policy`) dispatched on `cleave/mint-doors`; it also takes TOPO's `euler-rebased-run-...` row. Review: single, full.
- Ev ruled on PR 3642: derive, no declaration (fork row 34; both designers' recommendation). The tangency lane is dispatched on `cleave/tangency`. Review: single, full.
- `plane_section` regions merged (PR 3714) after a style review and one
  fix pass. `plane_section` returns `regions` (an outline with its
  holes, oriented by role) and its own `SectionError`.
  `splitting/section_loops.rs` is the one home of the section-loop
  sense, the hole nesting, and the side-normal and u-axis rules, which
  the split's finish and `plane_section` both read. The ring-vs-ring
  guard is split-only. Filed `plane-section-polygons-drop-their-arcs`
  (P2).

- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`, the last unit of that program): the D4 ¶1 (i) recourse GRAMMAR moved in `geom-core`, so refusal text changed across the tree. `COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE` and `SPLIT_PLANE_RECOURSE` lost their unvalued `", or lower the tolerance"` tail and are now the LEVERS alone; `DEFINITE_COINCIDENCE_RECOURSE` retired into `COINCIDENCE_RECOURSE` (with the tail gone the two were one string). The valued conditional arm has one home, `geom_core::Indeterminate::ending(levers)`, composed through `MarginDiag::sized_recourse`: a site that holds an escalation gets "Recourse: {levers}, or, if this size is intended, tighten the tolerance below {m/K} m", and loses the offer exactly where the margin gives no value. `Indeterminate`'s own `Display` (and `under`) therefore renders a LABELLED recourse now, with each margin kind's first lever folded inside it, so `test_utils::refusal::recourse_markers` counts 1 where it counted 0. `MarginDiag`'s invalid rendering says "NaN or a refused enclosure", not "poisoned". Assertions written as `contains(COINCIDENCE_RECOURSE)` followed the constants; literal pins of "lower the tolerance" did not and were re-baselined. (PROPS implementer)
- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`): mechanical edits in the Boolean's files for the recourse-grammar change. `topo/src/boolean/mod.rs` and `refusal_routes.rs` use `geom_core::COINCIDENCE_RECOURSE` where they used the retired `DEFINITE_COINCIDENCE_RECOURSE` (same string). `topo/src/boolean/contain.rs`' `ContainError::RayExhausted` drops "or lower the tolerance", keeping "move the point off the boundary" — it carries no margin, so there is no value to offer and D4 ¶1 (i) offers nothing unvalued. `solid_contain.rs`' props pattern gained `..` for `PropsError::Escalated`'s new `check` field. (PROPS implementer)
- LINALG filed `a-strut-bearing-operand-passes-the-boolean-gates-and-refuses-at-the-join` on our slate. Priced P2/E: an operand that is tier-2-invalid (scaffolding) should refuse typed at the operand gate, following the `ScaffoldingOperand` precedent. That is the obvious answer, not a fork.
- Witness ladders merged (PR 3716) after a full review and one fix
  pass. There is one cell-dimension ladder (`complex_side`) with one
  "inconclusive" rule, and first-decisive applies to both callers.
  Join's loop-roles cross-check is now a pure function pinned by
  synthetic rows, and the contact skip is deleted under a debug guard.
  Refusals no longer name a remote witness. The contact-skip row closed
  with it. Filed `point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face`
  (P2), for the 26-in-band `wide_wedge` case.
- 2026-10-01 — Seam note from TANG: TANG takes the circle × cylinder cell of `reduce::wall_crossing` (still `Unsettled`; REACH's snowman entry names it as remaining) under `work/tang/boolean-refuses-on-arc-carrier-not-arc`, branch `tang/circle-cylinder-crossing`, live now. It edits `crates/topo/src/boolean/reduce.rs` and should call `circle_torus::half_angle_roots` rather than re-spell it. If you have this cell in flight, say so on `work/tang/log.md`. (TANG orchestrator)
- 2026-10-02 — Note from TQUERY. PR 3797's measurement ran the
  pseudomanifold door on every split half in the topo + sweep suites.
  It refuses 8 halves that tier 3 passes, as `EdgeFaceOverlap`, in
  `split_section_rings::a_clockwise_section_nothing_places_keeps_its_face`
  and `pis_arc_capped_poses::every_tilted_cut_wall_reads_its_truth`.
  These are the cancelling 2-gons of
  `split-pairs-curved-face-crossings-across-the-wrong-arc`, so this is
  more evidence for that row and a door that sees them. (TQUERY
  orchestrator)
- 2026-10-02 — Seam note from TQUERY: PR 3768 (merged) types `SplitPlane.normal` as `geom_core::UnitVec3`. Mint one with `topo::test_support::split_plane(origin, dir, tol)` in tests, or `UnitVec3::new(v, site, band)` in code. A `SplitPlane { normal: Vec3 }` literal on an open branch stops compiling. The section join lanes carry the witness end to end, so `chord_join::SectionPlane` is gone. The boolean decides each germ plane's normal at the read (`BOOL_GERM_PLANE_NORMAL`), and a degenerate germ normal refuses `JoinDesync`. Paths touched on your ground are listed in the PR body. (TQUERY orchestrator)
- Mint doors merged (PR 3720), unit 2 of the NURBS-lane design, after
  a full review and a fix pass that was interrupted by the usage outage
  and resumed. `topo::policy_lane` is the one place the lane is read
  from the policy, and it returns `ByPolicy { NoLane, Refused }`, so no
  door can pass the lack on as a certification failure. Every topo edge
  mint and re-certify reads it (`set_edge_curve`, `split_edge`,
  `kev_describing`, `mev`/`mef`/`mekr`/rings, the re-chart, the
  re-basing gates). Each refuses `NurbsLaneUnsupported` typed at a
  dual, and the twin doors are deleted. TOPO's
  `euler-rebased-run-recertifies-through-the-plain-door` closed with
  it. Filed `work/ciw/local-rustdoc-with-the-gate-flags-fails-where-the-hosted-gate-passes`.
- 2026-10-02: resumed after a usage-limit outage (about 22 h idle).
  - The mint-doors fix pass and the tangency review died mid-task and
    are resumed.
  - The wrong-arc lane is merging main, which moved by about a day,
    including PR 3768's `UnitVec3` split normal.
  - JOIN's PR 3770 filed and closed
    `split-whole-orbit-run-mints-an-unlabelled-strut` on our slate.
    That is the mechanism of the P0 the tangency lane filed (a lone
    Below bisector with every real edge Above), so block ∪ slab under
    −n should answer once `cleave/tangency` merges main. The tangency
    lane's P0 row and its pinned refusal need re-checking at that
    merge.
- Wrong-arc merged (PR 3718) after a full review, one fix pass, and a
  merge of main a day later. A curved face's crossings now pair along
  its section conic through one shared `wall_section` (cylinder,
  sphere and, after main's merge, cone). The heading lever is the
  wall's curvature arm, and the conic order is arc-length keyed with an
  explicit branch cut. The section-rings fallback no longer fires on
  the tilted-cut poses; a guard row pins that.
- Wrong-arc merged (PR 3718). Wave 4 dispatched: `cleave/carrier-walk` (P1; it may also take the spiric/spline crossing-row P2; single full review) and `cleave/strut-gate` (P2/E; orchestrator's read or style review). `boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` (P1, `design: true`, filed by another program) waits for a designer pair.
- Re-homed `boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` to REACH (curved-operand refusals are its charter); noted on REACH's log.
- Mint doors merged (PR 3720) after a merge of main. Dispatched `cleave/far-plane` (P2, single full review).
- Tangency (PR 3726) adjudicated after its fix pass: split derives the tangent side (Ev's ruling, PR 3642); the reviewer's rows are folded in, `Convexity` is retired for `classify_dihedral`, and the smooth-edge safety default is documented. Row `split-cannot-declare-an-exact-tangency-with-its-target` closed; `split-refuses-a-convex-graze-of-a-curved-wall` filed as follow-up.
- Far-plane lane reported (PR 3866, head `fe7e3021d`). It widened past the plane pre-pass into a ray-level abandonment rule in `cast_ray` and `polygon_walk`, which makes it tricky containment logic: escalated from the planned single review to the dual tier. Cost M, byte 189 (mod 3 = 0): HOLDOUT, concurrent pair (DR-48). Strut-gate (PR 3860) style review: mergeable, no MAJOR; fix pass dispatched.
- Carrier-walk (PR 3865) reported; single full review dispatched. Dispatched `cleave/section-arcs` (`plane-section-polygons-drop-their-arcs`, P2/M) as a cloud session, since the local box's build slots are saturated (Ev, in chat: cloud sessions are fine when the machine can't handle the parallelism). Review tier: single full.
- Dispatched two more cloud lanes: `cleave/convex-graze` (`split-refuses-a-convex-graze-of-a-curved-wall`, P2/M; the fix must read the wall's material convexity, never flip the arms; review tier decided at report, likely dual) and `cleave/union-graft` (`the-union-fallback-graft-re-certifies-...`, P2/M; single full review). `boolean-declares-no-touching-...` waits for the strut-gate PR, which holds `boolean/mod.rs`.
- DR-48 (far-plane, PR 3866; renumbered from DR-46 after a collision with REACH) both reviews in: R1 and R2 APPROVE-WITH-FIXES. R2 found a pre-existing wrong answer (`point_in_solid` reads `In` 5e-7 outside) in `cast_ray`'s parallel-ray skip, the arm this PR changes and whose premise its new docs build on; R1 flagged the same premise as unproven. Ruled: fix it in this PR (a Zero denom with q not certainly off the carrier abandons the ray). Fix pass sent with the adjudicated union. R1 disclosed seeing R2's cargo command line in `ps`, nothing else: no findings glimpsed, pair stays fair.
- Carrier-walk (PR 3865) review: APPROVE-WITH-FIXES, one MAJOR — the PR's shell7 measurement was false (no other vertex places the elbow fixtures' rings; base and head both refuse that shell output, naming VolumeUncomputable vs RingNestingUndecided). Ruled: correct the record, and knowingly accept refusing an unreadable ring over the silent guess (no public outcome changes; the shell discards those bodies already). The sibling crossing-row P2 gets the shell consumer as its reachability. Fix pass sent.
- Strut gate merged (PR 3860). `boolean-declares-no-touching-...`'s fix shape waits on TQUERY's open pinch-contacts question (`split-halves-have-no-contact-records-...`, spec), so only its owed measurement is dispatched: cloud lane `cleave/vv-copies-measure` (measure-only; reachability and a pinning reproducer if one exists).
- Strut gate (PR 3860) adjudicated after its fix pass: the boolean's operand gate runs the validator, tier 2 refusing `ScaffoldingOperand` with its findings and tier 1 `CorruptOperand` (payload widened to `Corruption::{Structure, Vertex}`, accepted: the Python tag is unchanged and it is the one way to carry tier-1 findings). Filed `boolean-operand-refusals-that-precede-or-outlive-the-tier-two-gate` and `split-gates-its-operand-on-null-edges-not-on-tier-2`. Row closed.
- Carrier-walk (PR 3865) adjudicated after its fix pass: the walk's uncrossable answer is one typed refusal (`Uncrossable`), check 9 refuses an unreadable ring instead of guessing, and a certified whole-turn scaffold arc refuses as `CorruptLoop` (an in-band span escalates). The shell7 record is corrected (base and head both refuse the elbow fixtures' shell output, naming VolumeUncomputable vs RingNestingUndecided). The sibling crossing-row P2 keeps its priority: its reachability through shell output waits on spiric-face volume. Row closed.
- Far-plane (PR 3866, DR-48) adjudicated after its fix pass: `point_in_solid` no longer refuses on in-band readings against carriers far from their faces, and the pre-existing parallel-skip wrong answer R2 found is fixed in the same arm (the skip certifies the miss or abandons the ray). DR-48 recorded: blinded coding by a separate coder (byte 170), tally 0 (the one MAJOR is bilateral with R1's MINOR on the same premise; the coder flagged that call as the 0-vs-1 decider). M-tier units toward the readout: 1. Row closed.
- Union graft (PR 3894) adjudicated after its fix pass: the union/intersect assembly and the sphere re-cut graft carry the operand's certificates (`RemapKeys`), as the void door does; a test-only graft-bridge log pins which bridge each arm ran, and the intersect arm is pinned. Row closed. Point-in-loop (PR 3917) reported: the public door now reads each edge on its carrier; single full review dispatched (cloud).
- Section arcs (PR 3877) adjudicated after its fix pass: section polygons carry their edges on their carriers and an exact area, held by construction (private fields); one home for the conic frame and the chord bulge; a non-cancelling arc row at f64 and Interval. Row closed. Filed `interval-steep-cut-through-cylinder-caps-refuses-order-escalated` (P3) from the lane's note. DR-53 (convex-graze) first review: APPROVE-WITH-FIXES, no MAJOR, so on the sequential arm it is the only review; fix pass sent (the concave guards could not see the flip; residual convex refusals and an undeclared-knife-edge validator gap to file). Union-graft (PR 3894) review: APPROVE; fix pass sent (a bridge-level discriminator, an intersect pin).
- Section arcs merged (PR 3877). Raised `point-in-loop-answers-on-an-arc-loops-corner-polygon` to P1: a public door answering wrongly (In reads Out) with no refusal is a wrong answer, whatever its doc says about its input. Dispatched as cloud lane `cleave/ptloop-arcs`; review tier: single full.
- Cloud lanes reported. Section-arcs (PR 3877): single full review APPROVE-WITH-FIXES, no MAJOR; fix pass sent (invariant by construction, one home for the conic decomposition, a non-cancelling Interval row). Convex-graze (PR 3892): the item's premise did not reproduce (flipping the arms answers the concave graze with true volumes) but the flip is still wrong (it mints an undeclarable knife edge); the fix reads the wall's bend. Dual tier, M, byte 238 (mod 3 = 1): SEQUENTIAL (DR-53; first numbered DR-51, which collided with PCERT on main). Union-graft (PR 3894): measured 4 carried≠fresh certificates, all from a boolean-made lens operand (filed); single full review. VV-copies measurement (PR 3904): mechanism refuted on current code (copies share the point and never survive to the result); the item was already closed on main by `fe168fc1`; regression pins only, orchestrator's read.
- Convex graze (PR 3892, DR-53) adjudicated after its fix pass: split lands a convex graze of a cylinder or cone whole by reading the wall's certified bend; concave grazes refuse, now guarded over many azimuths. DR-53 recorded (sequential arm, one review, no MAJOR). Filed from the review: `tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line` (P2/H, reached only under a plant so far) and `a-convex-graze-of-a-cone-refuses-at-some-azimuths` (P3/M). Row closed.
- Point-in-loop (PR 3917, P1) adjudicated after its fix pass: the public `point_in_loop` reads each edge on its carrier and certifies its preconditions (unit normal, planar loop, query on the plane), refusing `OffPlane` instead of answering. Accepted: `point_in_solid`'s `point_in_face` keeps a crate-private projected path (its elevation pre-pass has already placed q on the plane within band, and the strict door would refuse the `offer_rows` fixtures whose corners are stranded off their planes on purpose). Filed `point-in-loop-row-names-name-the-wrong-walk` (P3). Row closed.
- Measured (PR 3932): `tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line` is reached without a plant. A declared tangent rest (plate ∪ rod) followed by a bridge union gives one shell whose rod wall lies on the plate face along a ruling with no edge; tier 3 passes it, and with the first result's records carried tier 3′ passes it too (f64 and Interval; `#[ignore]`d pins in `wall_face_tangent_reach.rs`). Raised to P1 and marked `design: true`: the fix is a fork — a tier-3 face-pair check (the validator's own deferred "global self-intersection / minimum clearance" class) versus the Boolean refusing to fuse across a declared tangent rest (CONTACT's new row `a-bridge-union-fuses-a-declared-tangent-rest-...`, P1). Goes to a designer pair. Carrier-crossings (PR 3924): dual tier, M, byte 137 (mod 3 = 2): SEQUENTIAL; first review APPROVE-WITH-FIXES, no MAJOR, so no second review; fix pass sent.
- Edgeless-contact fork converged without Ev: designer pair (labels on `analysis/design-fork/edgeless-contact`, byte 26) crossed twice (rounds 2 and 3); an executed measurement settled the fact beneath (a zero-area two-edge ring in a plane face passes every validator today; one consumer arm owed in `sector_shape`); round 4 both recommend #131's doubled slit (two coincident wedge-2π edges, one shell, no record), which ratified text (#131, `1b2f1848b`) already decides, and neither proposes narrowing it, so no `[ev]` PR and no fork-log row (rule 1: only forks sent to Ev). Arm owner: TANG `declared-cusps-second-order-wedge-arm` (note appended there; restore the C7 sentence `585b3422f` dropped). CLEAVE lands the interim typed refusal (`cleave/tangent-interior-refuse`, cloud lane). Filed from the measurement: `tessellator-panics-on-a-self-slit-face-every-validator-passes` (P2) and `validators-accept-a-lamina-slit-or-zero-area-membrane-face` (P2). Correction for Ev: the DEV-1 two-row answer for a declared interior tangent union deviated from #131 item 4; it flips to a refusal.
- Carrier crossings (PR 3924, DR-60; first numbered DR-59, which RECIPE took on main first) adjudicated after its fix pass: the carrier walk crosses spiric edges by certified subdivision with per-piece bounds; far points near tangent cuts no longer refuse; exhaustion is honest. DR-60 recorded at merge (sequential arm, one review, no MAJOR). M-tier units toward the readout: 3. Row closed; the spline half is `carrier-walk-has-no-crossing-row-for-spline-edges` (P3).
- Interim refusal (PR 3938) adjudicated by the orchestrator's own read plus CI, not a cloud review: weekly usage was in warning, and the change only refuses a union it used to build (never a new answer). A verified `Tangent` union whose locus runs strictly through a plane face now refuses `TangentSlitArmUnbuilt` (kept distinct from `RimCuspArmUnbuilt`); subtract and intersect are measured correct and untouched; `m9_3_wall_door`, both `wall_face_tangent_reach` rows and two `reach_continuation` rows flip to the refusal. Known residue: the interior read samples 9 points, so a notch could hide the interior (status quo, not a new wrong answer). The tier-3 row closes; the arm stays TANG's. Filed `a-sweep-row-fails-under-all-features-on-main` from the lane's report.
- `topo-mints-indeterminates-outside-the-funnel` (P0) unparked. Correction: PR 3513 merged 2026-10-01T15:27Z, before this sitting reported the row still parked on it; the blocker was not re-checked. The row's site list has grown across nine files since filing (contact_verify, plane_eq, carrier_eq, reduce, rim_wedge, census, contain, sectors, splitting rules/containment/order), and 3513 rewrote plane_eq and sectors, so a measure-only cloud lane (`cleave/mints-retake`) re-takes it against main: per site, whether it is still minted after a definite sign, and whether the fact is an indeterminacy or a contradiction. The payload question then goes to a designer pair. Dispatched with it: `cleave/steep-tube-eps` (P1, red at ε = 1e-6) and `cleave/inside-out-gate` (P1); review tier set at report.
- `split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence` (P1) goes to a designer pair: a plane missing a concave tip by less than ε reads ON and mints a pinch that the pseudomanifold door passes since PR 3856. ε-coincidence (a margin below ε decides ON) and D1 tier 3′ (i) (near-coincidence never silently becomes contact) pull different ways on it. Labels on `analysis/design-fork/split-band-on`: byte 40, so A = Opus and B = Fable. Also dispatched `cleave/all-features-row` (P2/E: reproduce, find the feature, fix or route to the owner).
- Split-band-on fork converged without Ev. Both designers: a margin at or below ε is coincidence under Q1, and tier 3′ (i)'s "near-coincidence" is the band, which split already refuses typed. So the δ ≤ ε pinch is the δ = 0 answer, which tier 3′ (ii) licenses, and "not ON" is unbuildable below ε. The row drops to P2 and is rescoped: the δ = 5e-10 arm, where the build refuses an ON verdict in kernel-defect voice (measure, then fix); pins for the sub-ε pinch as success; `SliverVertex`'s wording. Its dispatch waits for `cleave/steep-tube-eps` to report, since both may touch split's pcurve mint. Both designers suggest optionally clarifying tier 3′ (i)'s wording ("a margin in the band"); that is ratified text, so it is offered to Ev rather than done.
- Split-band-on: Ev overturned the designers' convergence. In Ev's words, tier 3′ (i) "specifically DOES refer to coincidence < eps, and says we can't infer intent from that". The row goes back to P1 with `design: true`. Round 2 is sent to the same pair, with the ruling as a given: what split answers when a ≤ ε ON verdict would create contact, the line between ON-adds-topology and ON-creates-touching, the user's channel to mean a pinch, whether this reaches past split, and which ratified text conflicts. Orchestrator's error, owned: I passed the convergence to Ev without checking it against (i)'s full text, whose first half (structural or declared, never values) the designers' reading skipped.
- Split-band-on round 2 converged under Ev's ruling. Split derives an ON verdict that only places topology. A vertex whose orbit has ≥ 2 one-side runs would pinch, so it refuses typed unless the Split node declares it; the declaration is verified like a C4 `Rest`, and δ = 0 and δ ≤ ε get the same answer. The one difference between the pair (B's sized "tighten the tolerance" recourse) is settled by D4 (i) and SELECT-DESIGN §3d: a contact site drops that arm. This changes ratified text (tier 3′ (ii), the reduction's coincidence discipline, the frontier's pinch line), so it goes to Ev as an `[ev]` PR with a fork-log row. The same inference in the boolean's vertex records is filed on CONTACT: `boolean-vertex-contact-records-are-inferred-from-values` (P1, design, measure first).
- All-features row (PR 3957) merged on the orchestrator's read plus CI. The row filtered on a predicate name REACH's #3805 renamed: a one-string fix. Closes CLEAVE's row and TANG's duplicate `the-carved-balls-clearance-row-is-vacuous-on-main`; the CI gap behind it is already `work/ciw/tests-red-under-all-features-never-run-by-ci.md`.
- Mints re-take (P0) reported: 24 of the item's 28 sites are live, and a sweep found about 35 more. The classes, the user-visible payloads and the vocabulary inventory are on the row and on `analysis/cleave/mints-retake`. Next is the designer pair on the payload (class b), the impossible-sign door (class c) and the `is_invalid()`-as-zero readers. Note: the cloud lanes report the weekly limit in warning again, so new implementer dispatches are held to P0/P1.
- Mints fork (P0) converged without Ev. Labels are on `analysis/design-fork/topo-mints` (byte 123, so A = Fable and B = Opus).
  - Round 1 split on two points. A moved to B on both in the reconciliation round, and B's partial move was to a point both already held, so the pair did not cross:
    - an impossible magnitude is `unreachable!` in the door (D9 row 4);
    - two definite verdicts that disagree are never an escalation.
  - No ratified text changes, so the fork does not go to Ev and gets no fork-log row. The design and its five-step build order are on the row.
  - My process error: the designers' checkout was stale (the clone tracked the local `main` ref). Both caught it and read `82b9ceb2` directly.
  - Filed `classification-invariant-family-types-bug-only-states-against-d9-row-4` (P3).
  - Step 1 (the geom-core doors) dispatched as cloud lane `cleave/mints-doors`; single full review at report.
- Inside-out gate (PR 3963, P1) merged on the orchestrator's read plus CI. Weekly usage is in warning, and the change only refuses: it adds no new answer.
  - Ratified text already decides the question: D1 tier 3's orientation invariant, and "every door that returns or consumes [a finished body] pays that gate once" (Ev, PR 3870).
  - The Boolean's operand gate now refuses `InsideOutOperand` on tier 3's own check-7 reading, per solid and before a multi-solid merge. The lane found that multi-solid hole itself.
  - Accepted: an undecided sign passes, as check 7 passes it. The `A∩revert(B)` oracles become partition oracles.
  - Accepted cost: about 17% of planar op time and 7% curved. REACH's `boolean-door-adopts-the-finished-body-type` retires it; the evidence is on that row.
  - Filed by the lane: `split-answers-an-inside-out-operand-with-two-inside-out-halves` (CLEAVE) and a shell row on SHELL.
  - Row closed.
- [ev] PR 3960 (split pinch), discussed with Ev in the PR. Ev asked for a picture, and whether the main cases are structural. Answered there:
  - A shared-parameter `Expr` is strong evidence but not an explicit statement. Ev: the case for a silent assumption is much stronger when the user "literally declared that v tip should lie on the same plane".
  - The declaration seat stands as diffed, awaiting Ev's sign-off.
  - Follow-ups to file at merge: a detector finding that names the shared-parameter identity, and a DM1-style plane datum naming an edge.
- [ev] PR 3960 merged with Ev's approval ("i think this works"). The fork-log row was renumbered 58 → 61 at merge. Filed with it:
  - `split-refusal-detector-names-a-shared-parameter-coincidence` (P3);
  - `a-plane-datum-through-a-named-edge` (P3);
  - `coincidence-intent-has-too-many-spellings` (P2, design). This is Ev's concern: "this whole system is able to say the same thing in too many ways". Weigh it before CONTACT's vertex-declaration fork.

  The split pinch build is not dispatched while weekly usage is in warning.
- Steep tube (PR 3981, P1) reported. The −3.06e-6 margin is a conservative box lying *inside* the window, at every ε, so `trim_containment` decided a one-sided escape two-sidedly. The fix clamps each escape at zero, as the iso gates already do. Re-baselines: the tour chain's certifiable fractions about ×2, and the plate ledger's blocked residuals 32 → 40. This overlaps PCERT's open retirement of check 5 (Ev, PR 3919). Review tier: DR "steep-tube", M, byte 74 (mod 3 = 2), so SEQUENTIAL; first review dispatched (cloud).
- Mints step 1 (PR 3979, P0) reported:
  - gate rejections carry the decided margin; the `_reported` twins are folded in; `Invalid` now means poison only;
  - `decide_magnitude` panics on a decided Negative, and 11 sites are migrated;
  - sweep's `short_arm` also goes through the door;
  - two sites are deferred to step 4 (I16's radius is input-reachable; N5).

  The lane's open doubt: a decided zero on the wrong side now offers a tolerance that cannot pass, at every gate. A single full review is dispatched (cloud).
- Steep tube (PR 3981, DR "steep-tube", sequential): the first review is APPROVE-WITH-FIXES with no MAJOR, so it is the only review.
  - The reviewer found the clamp sound for all six callers and the re-baselines honest, and recommends landing it before PCERT retires check 5.
  - Fix pass sent:
    - the pin prose;
    - the tip ratio, which drifts with link count (0.3% headroom): restate the claim and do not widen the gate;
    - one helper for "decide only the positive escape";
    - a chaintol test that could not go red;
    - pin the infinite-arm NaN;
    - raise the contact sag row to P1;
    - file `pcurve_azimuth_period`, which has the same shape.
- Mints step 1 (PR 3979): the single full review is APPROVE-WITH-FIXES with one MAJOR, demonstrated at public `require_ring_torus`.
  - The MAJOR: a gate's rejection of a wrong-side decided zero offers a tolerance that cannot pass, which D4 ¶1 (i) forbids. It is the lane's own open doubt, now spread to every plain gate.
  - Ruled: the PR does not merge until it is fixed. The gate carries its pass set on the rejection; a small move toward the design's step-5 seal is accepted.
  - Fix pass sent with the MINORs:
    - `at_wedge`'s Negative arm;
    - the past-band "declare" offer;
    - the reviewer's two mutation-killing rows;
    - a stale doc;
    - the style items.

## 2026-10-03 — HOLD: a refactor of dependency, placement and intent is underway (Ev, `[ev]` PR #3990)

Ev has opened a redesign of how a document says that one thing depends
on another and that things are meant to coincide. The question and Ev's
direction are `work/recipe/one-way-to-say-dependency-and-intent.md`;
the design lands through `[ev]` PR #3990. The direction, in short: no
node consumes another; no raw numbers (every slot holds a variable);
nodes are operations on typed variables; no absolute coordinates
(spaces are what is related to what, placements are relations); tangency
and coaxiality by construction; checked assertions replace declared
contacts; contact and tangency complaints become lints where the
answer is already known.

**Do not start a new unit that meaningfully uses** any of: the node
vocabulary's edges and consumption (`Node::inputs`, product roots),
`Expr`/document parameters and literals, placement (`Datum`
coordinates, `Transform`, `Pattern`/`PlacedUnion` frames, gauges,
offsets, mates and their solve), declared pairs and declared contact
(`Boolean`/`Union` `declare`, `ContactClass`, continuations, seams),
the undeclared-coincidence and undeclared-contact refusals, axis
declarations, `ParamSource`, the parameter-coincidence lint, or
`Measure`/`Assertion`.

**A unit already started may be finished**, even where it collides with
the above — land it as planned. Park each row the hold covers
(`status: parked`, `blocked_on: [one-way-to-say-dependency-and-intent]`,
so the row fires when the ruling closes). If that leaves your program
with nothing it may start, set its `status` to `blocked` and stop.
- Mints step 1 (PR 3979) merged after its fix pass. The MAJOR was fixed and checked against the diff: a gate's rejection carries a tag (the decided sign and the signs the gate passes) inside `MarginDiag`'s private reading, so no struct literal changed. A tolerance is offered only where a smaller one decides the margin onto a passing side.
  - The MINORs and style items are done. The past-band "declare" offer is fixed at the bare Display; site lever lists go to step 4.
  - Closes VERDICT's `decide-positive-synthesizes-invalid-for-a-decided-zero`.
  - Step 2 (typed `Coincidence`, the six `is_invalid()` readers) dispatched as cloud lane `cleave/mints-coincidence`.
- Steep tube (PR 3981) fix pass is done:
  - the tip ratio is pinned per link count, with its trend asserted and the one-number claim retired;
  - one `escape` helper;
  - the azimuth-period row filed.

  DR-64 rides the PR as its last commit, and it merges when CI is green.

### CLEAVE under the hold (2026-10-03, 20:0xZ)

The hold landed at 18:58Z. This sitting acted on it only at about 20:05Z, when Ev pointed to it.
- **Orchestrator error, owned:** `cleave/mints-coincidence` (mints step 2: the undeclared-coincidence payloads and the `is_invalid()`-as-zero readers) was dispatched at 19:48Z, after the notice, onto exactly the held ground. It is withdrawn: interrupted and told to open no PR. Anything it pushed stays on its branch for when the ruling closes.
- **Started before the hold, so finished as planned:**
  - PR 3981 (steep-tube trim clamp): a pcurve certifier, not held ground.
  - PR 3979 (mints step 1, the geom-core doors) merged at 19:48Z. It was dispatched before the notice and touches gate doors, not declared contact.
- **Parked on `blocked_on: [3990]`** (the program-wide convention, since the ruling's item id is not on main yet):
  - `topo-mints-indeterminates-outside-the-funnel` (P0; steps 2–5 are undeclared-coincidence payloads, contradiction typing of declared contacts, and the seal);
  - `split-band-on-...` (the Split node's declaration);
  - `coincidence-intent-has-too-many-spellings` (likely subsumed by #3990 itself);
  - `a-plane-datum-through-a-named-edge` (Datum);
  - `split-refusal-detector-names-a-shared-parameter-coincidence` (Expr parameters);
  - `the-rest-lane-reads-a-nested-struts-site-at-its-holders-tip` and `seam-zip-grafts-re-certify-...` (the declared-Rest lane).
- **Not held, still startable:** the split/boolean topology and certifier rows, including `split-answers-an-inside-out-operand-with-two-inside-out-halves`, the corner-crossing P1s, the chord recompute, and the validators/tessellator P2s. CLEAVE is not blocked.
