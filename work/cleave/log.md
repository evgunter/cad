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

## 2026-10-03 — the intent refactor's hold now waits on the build, not the ruling (Ev ratified #3990)

Ev ratified DESIGN.md D10 on PR #3990, and the ruling
`one-way-to-say-dependency-and-intent` is closed. The hold announced in
the entry before this one CONTINUES until D10 is built: it now waits on
`work/recipe/d10-one-way-to-say-intent-is-unbuilt.md`. Every row that
was parked on the ruling or on #3990 has been re-pointed there, so
nothing fires at this merge. Park any further held row with
`blocked_on: [d10-one-way-to-say-intent-is-unbuilt]`. Units already
started may still finish. Read D10 before resuming work on this ground:
coincidence is now a margined verdict (no declarations), checked by the
`unproven-coincidence` lint.
- Steep tube (PR 3981, DR-65, renumbered from 64 at merge) merged after its fix pass, with main merged in after PR 3979. Split's pcurve check 5 decides only an escape's positive part, through one `escape` helper. The tour's tip ratio is now pinned per link count. Row closed. M-tier units toward the readout: 4. Mints step 2's withdrawn lane is told to push its uncommitted edits to `cleave/mints-coincidence` as held, unreviewed WIP, with no PR.

## 2026-10-06 — a new CLEAVE orchestrator picks the track up

The previous orchestrator has exited; this session holds the track (`status: active` stands).
- **The P0 on two vertices at one point moves to JOIN.** Ev's PR 4057 ruling says that row is
  settled by JOIN's `a-pinch-no-kept-face-can-cross-refuses` (its bodies are right; its check
  becomes "every corner is a slice of its own face"). It is moved by `git mv`, keeping its id,
  parked on and riding with that unit. A note is on JOIN's log.
- **`three-corners-alternating-round-a-corner-refuse-at-the-join` is parked** on the same JOIN
  unit. On main its three ops refuse `SharedVertexCrossings` at the shared corner (FUSE's
  interleave arm) or `PinchUncrossed`, and JOIN's draft PR 4074 rebuilds exactly that ground: a
  pinch becomes one vertex per cone and `PinchUncrossed`, the crossing pre-pass and the pinch welds
  retire. Working the join under it now would be built twice. Alternative not taken: dispatch it
  against today's zips and accept the rework.
- **Dispatched** (review tier for each: single FULL — both change what a door accepts or how a
  section arc is decided, so believing them takes more than reading them):
  - `cleave/split-operand-gate`: `split-answers-an-inside-out-operand-with-two-inside-out-halves`,
    with `split-gates-its-operand-on-null-edges-not-on-tier-2` riding along (priced P1/E). The
    split door takes the Boolean's finished-operand shape (REACH PR 3987).
  - `cleave/split-segment-curve`: `split-lane-second-chord-recomputes-the-first-chords-arc`.
- **Design weighing**: `closest-crossing-and-graze-abandon-have-three-homes` (`design: true`) went
  to the designer pair, blinding record on `analysis/design-fork/cleave-ray-walk-protocol`. It goes
  to Ev only if the reports leave a fork that is Ev's.

Signed (CLEAVE orchestrator).
- **The ray-walk design question is decided without Ev** (`closest-crossing-and-graze-abandon-have-three-homes`).
  Both designers' first reports agree on the final state, and neither touches ratified text:
  - one walk driver (the retry ladder: graze → next ray, a ray-level in-band reading abandons and the
    first is kept, refuse, exhausted → typed) and one closest-crossing fold, in `topo::ray_walk`, grown
    from `ray_parity`;
  - the per-ray reading stays two kinds: parity (planar, chart) and closest crossing (solid, sphere);
  - each geometry keeps its schedule, crossings, no-hit verdict, K rows and error enum.

  Both corrected the row's premise: the planar walk is parity, and there are five driver copies in
  `topo` plus `profile`'s, not three. Their differences are of detail (a set-aside kind for confined
  limits; a sphere boundary pre-pass; geom-core against topo). The orchestrator reconciled them: `topo`;
  one precedence rule written once in the driver, decided on semantics; the sphere pre-pass measured
  first. Re-priced H, `design` cleared. Dispatched as `cleave/ray-walk`. Review tier: DUAL (concurrent,
  H) — a protocol consolidated across five readers, with refusal semantics that are hard to change later.
  It is not a design-fork row: nothing went to Ev. The blinding record stays on its analysis branch.
- **PR 4081 merges** (`split-lane-second-chord-recomputes-the-first-chords-arc`, closed). The split's
  segment curve and its chord plan are each decided once (`ChordJoiner::segment_curve`, `JoinPlan`),
  shared with the Boolean lane. The row's `NoChartedRun` premise had been dead on main since
  `5ec92edc58`. What the double decision cost was measured on a new fixture, a pocketed drum split
  across its ring: the two halves' wall arcs could sit a period apart, and half volumes failed to sum by
  up to 5.9e-9. Goldens re-baselined: `tilted_below` and editor-core `cut_cylinder`.
  - Review: single FULL, then the orchestrator's read of the fix pass's delta. The delta plans the
    second chord before the first chord's surgery; the invariant that makes this exact is documented on
    `JoinPlan`, and topo and sweep are green.
  - Filed: `split-halves-volumes-sum-to-the-whole-only-within-their-pads` (P3, `design`). Its misses
    track unequal quadrature pads, so it may belong to FLUX's quadrature ground; it stays here until
    measured further.
  - Class noted from the review: "decided once" has to cover the plan as well as the value. A
    computed value that is shared while the plan choosing it is re-derived is the shape to grep for.
- **Re-homed off CLEAVE's ground**: the lamina validator row went to RESTFRONT and the self-slit
  tessellator panic to TESS, each with a note on its owner's log.
- **Dispatched** (single FULL review each: both are refusal paths whose cause is untraced):
  - `cleave/concave-graze`: `a-concave-graze-of-a-curved-wall-refuses-for-reasons-other-than-its-knife-edge`;
  - `cleave/pierce-strut-after`: `a-pierce-strut-after-half-may-be-moved-by-the-other-run` (prove,
    or refuse typed).
- **PR 4084 merges** (the split's operand gate; `split-answers-an-inside-out-operand-with-two-inside-out-halves`
  and its rider `split-gates-its-operand-on-null-edges-not-on-tier-2` are closed).
  - What it does:
    - The split's doors take `AtRestBody`, as the Boolean's do (REACH PR 3987).
    - An operand that carries no verdict (a dual) goes through one shared gate: tier 2, check 7 per
      solid, then check 10's winding per shell.
    - The split's hand-written null-edge arms are now invariants.
  - Review tier: single FULL, then the orchestrator's read of the fix pass's delta.
  - Class noted from the review: **a total hides a sign one level down.** The gate read orientation
    per solid, and an inside-out shell under a positive solid slipped through. A check over a sum
    has to read each term at the finest grain the invariant names.
  - Capability that regressed, disclosed: the lune piece no longer finishes at ε = 1e-6
    (`VolumeUncomputable`, quadrature convergence), so a second split of it now refuses there. The
    cause is on QUAD's row.
  - Filed elsewhere:
    - BAND: `blend-doors-answer-an-inside-out-operand-with-an-inside-out-body` (P1).
    - EXCH: `exchange-writers-take-a-body-no-at-rest-gate-read`.
    - Evidence on SHELL's and QUAD's existing rows.
  - Filed here: `a-stray-inside-out-shell-beside-two-outer-shells-passes-the-no-verdict-gate` (P3),
    plus the lane's three result-gate and sides rows.
- **PR 4096 merges** (`a-pierce-strut-after-half-may-be-moved-by-the-other-run`, closed). The row's
  witness cannot occur: the strut's `after` is the chord of the next sector, which no other run's
  mint moves. The invariant is checked at both twins (vtxfac, `splitting/insert.rs`), and each check
  goes red under a fan-overreach mutant.
  - Review: single FULL, then the orchestrator's read of the fix pass's delta.
  - Class noted: **a sweep scoped to one directory is not a sweep of the shape.** The first pass
    covered `boolean/` only and missed the splitting twin. Name the scope as the blind spot.
  - Filed: `run-loops-of-split-and-pierce-are-twins` (P3). Evidence added to
    `classification-invariant-family-types-bug-only-states-against-d9-row-4`.
- **Six unpriced rows priced** (the bands are per `work/README.md`):
  - P1, two implementations of one logic: `which-fragment-of-a-divided-face-holds-a-segment-is-spelled-three-ways`
    (M) and `topo-smooth-arms-decide-descriptions-outside-the-must-carry-rule` (M; its boolean arm
    folds an in-band station into the conventional description, against `must_carry_over_edge`'s contract).
  - P2, error propagation: `recl-flanker-representative-normalizes-an-undecided-residual` (M) and
    `lily-walls-curved-clearance-crowds-the-band-under-k-lint` (M).
  - P3: `boolean-operand-refusals-that-precede-or-outlive-the-tier-two-gate` (M).
  - P4, tooling: `site-census-attributes-a-parents-decisions-to-its-nested-fn` (E).
- **Dispatched** `cleave/fragment-lineage` (`which-fragment-…`; single FULL review). The smooth-arms
  P1 waits for PR 4098 to merge, because both edit `splitting/finish.rs`.
- **Filed a P0**: `a-cylinder-split-through-its-seam-ruling-refuses-rechart-undescribed`. PR 4098's delta
  review found that a solid cylinder split through its seam ruling refuses
  `RechartUndescribed{seam}`, on main as well. Its lane is dispatched once PR 4098 merges, because
  both edit the same re-chart site in `splitting/finish.rs`.
- **PR 4131 merges** (`which-fragment-…`, closed): one lineage home (`boolean::fragments`), one
  half-edge face reader, and the join plan's face read once. No body moves.
  - Ruled: a new refusal must be more causal than what it replaces. The lane's plan-time two-face
    `SectionInvariant` hid 34 deliberate tangent-frontier texts and misnamed the face in 14 cases, so
    it was dropped and today's refusals stand.
  - Filed on JOIN: the half-selection defect.
- **PR 4098 merges** (`a-concave-graze-of-a-curved-wall-refuses-for-reasons-other-than-its-knife-edge`, closed).
  Of 90 concave-graze poses measured on main, only 10 named the knife edge. Now each refuses
  `Reduce(KnifeEdge)` at rule (a), naming the wall and the contact site.
  - Review: single FULL, then a DELTA review of a fix pass that widened rule (a). It ran about 1,330
    poses × 3 ε plus the topo, sweep, editor-core and verbs suites; no split main built refuses.
  - Ruled and built: one decision, one payload. The knife edge was spelled
    `ConcaveGraze`/`SectionCusp` at two phases; the two phase variants now carry one `KnifeEdge` type.
  - `SectionSpur` is retired to a panic (`assert_no_spur`) whose invariant names its three fences.
  - Filed: `an-in-band-concave-graze-refuses-at-certification-on-one-side-of-tangency` (P2) and
    `certified-line-carrier-is-read-in-five-places` (P4).
  - Class noted: **a fix pass that widens a rule earns a delta review.** Rule (a) now reads every
    tangent wall; the first review never saw that code.
- **PR 4120 merges.** The P0 `a-revolved-tube-split-across-its-axis-refuses-a-degenerate-section` is closed:
  a pipe cut across its axis now splits.
  - Cause: every curved self-loop chord got a placeholder circle. Its area was zero (`DegenerateSection`)
    or made up (`Finish(Corrupt)`, which blamed the body).
  - Review: single FULL, then the orchestrator's read of the fix pass. The placeholder's type is filed
    on TOPO.
  - Class noted: a stand-in value that rides into a reader as if it were real (the placeholder circle)
    needs a type, not a convention. Two readers disagreed on how to recognise it.
- 2026-10-06 — Note from SHELL: four split rows (three in `split_across_a_revolve_seam.rs` and `m5_pr6_pcurves::a_seam_closed_tube_split_mints_clean_halves`) fail at ε = 1e-6 on main, identically with the merge-base's shell code. Every sweep PR's 1e-6 row is red on them. Filed as `split-seam-closed-form-rows-fail-at-eps-1e-6` (P0, on your slate). The rows came in with `bf06e9a0` (PR 4120). (SHELL orchestrator)
- **PR 4083 merges** (`closest-crossing-and-graze-abandon-have-three-homes`, closed). `topo::ray_walk`
  is the one ray-walk driver, ranking and closest-crossing fold. The sphere region reads distance to
  an ARC, and `Blocked` means only an ε-independent limit.
  - Review: DUAL (concurrent, H). The fix pass from the union was checked by an independent verifier:
    every mutant red, 0 wrong over 194,580 queries per side.
  - The dual row is recorded on this PR.
  - Tidied: three JOIN rows for one 1e-6 red; two are closed and one kept, with a note on JOIN's log.
  - Class noted: **a pre-pass must read the bounded thing, not its carrier.** The sphere arc's circle
    refused far from the face, and the curved pre-passes still read the carrier band first (evidence
    added to `point-in-solid-curved-arms-read-the-band-before-the-face`).
- **PR 4157 merges** (`topo-smooth-arms-…`, closed). The split's smooth arm decides through the
  must-carry rule, and a refuted smooth join refuses typed rather than folding to conventional.
  Filed or advanced:
  - TANG: the tangent-ruling chord, which mints a tangency without the rule;
  - RESTFRONT: whether a derived description short of intrinsic must refuse at rest (D2/C7 against
    `validate.rs`);
  - PRED: two more hand-rolled fold sites.
- **PR 4158 merges** (`a-cylinder-split-through-its-seam-ruling-…`, P0, closed). The re-chart reads
  the door's own stranded-edge query (`Body::stranded_by`) instead of a second spelling of it. What
  moved is disclosed in the PR body: edges whose descriptions were already incoherent are no longer
  restated early, so curve keys are permuted in 28 tests, with the final descriptions unchanged.
- **PR 4083 merged** (the ray walk, DR-90).
  - Its tail ported the Klein tour tripwire's retirement and re-baselined the tolerance stop, both
    after PATHS PR 3774, which turned main's tour red unseen. The tolerance study's own answer moved,
    noted on PATHS's log.
  - It also closed the 1e-6 reds of PR 4120's seam rows, which SHELL and another lane had filed here
    twice.
- **Fixed by hand:** the tube P0 row's closure had been dropped from PR 4120's state-sync commit (the
  row file was absent at that branch's head, so the status flip failed silently). Closed here.
- **Dispatched**, single FULL review each: `cleave/recl-flanker` (P2, the undecided Gram–Schmidt
  residual) and `cleave/inband-graze` (P2, one story for the in-band arms of the graze decision).
- **PR 4181 merged** (the frustum apex P1; single FULL review, fix pass done). Its row is closed.
- **PR 4158 merged** (the seam-ruling P0, closed). **Dispatched** `cleave/frustum-apex`
  (`a-frustum-split-through-a-ruling-off-its-seam-refuses-a-degenerate-section`, P1: a plane through a
  cone's apex pairs the wall's crossings top↔top). Review tier: single FULL.

## 2026-10-06 — reviews of PRs 4177, 4179 and 4181 (CLEAVE orchestrator)

- **PR 4181** (frustum apex), single FULL review: APPROVE-WITH-FIXES, no code defect.
  - The near-tangent gate left out the azimuth (5.5) where the 1→½ frustum's tier 3 escalates, and
    the flared frustum had no gate. Ruled: no gate leaves out a pose because it goes red there. The
    gate covers both frustums over the full reported set: a wrong number is red, and a typed tier-3
    refusal is the open sides row's debt.
  - Counts corrected (78 of 240; 8 poses).
  - Fix pass also covers: the leftover-on-a-ruling question, a line-typed `SectionCase::Straight`,
    and the dispatch, doc and naming cleanups.
- **PR 4177** (recl flanker), single FULL review: APPROVE-WITH-FIXES, no MAJOR.
  - Suites and a public-door probe show no answer moved. The in-band flanker is refused upstream as
    `Coincide::Sectors`, so the user gets one story.
  - Ruled:
    - `bool_flank_axis`, which only a bug reaches, panics with its values (D9);
    - the flank decisions move after the declared-Tangent short-circuit;
    - the in-band test row pins its margin;
    - `bool_wedge_reflex` gets its decision words;
    - a row is filed for `UnitVec3::levered`'s margin-less `Degenerate`, which forces three
      hand-spelled sites.
- **PR 4179** (in-band graze), single FULL review: APPROVE-WITH-FIXES, no MAJOR.
  - Confirmed:
    - the cause, instrumented;
    - the root, against bisection on five conics;
    - "whole" is the true answer under the belly Zero decision;
    - the sweep-crate Boolean graze roots moved onto the contact with no answer moved.
  - Ruled:
    - the in-band arms are three decisions' arms, so the test and row stop calling them one story;
    - the graze root takes the decided sign directly (exact under `Interval`);
    - one row names the three copies of the residue root solve;
    - tests are tightened (side, landing, tiers).
- **PR 4177 merged** (recl flanker; single FULL review, fix pass done). Its row is closed, and the
  `UnitVec3::levered` margin gap is filed on FLUX.
- **PR 4179 merged** (in-band graze; single FULL review, fix pass done). Its row is closed. The
  residue-root class is filed on HONE, and the convex-graze row is narrowed to the apex pose.
- **Main reds owned and fixed:**
  - PR 4199: `carrier_touch::ball_off_face` reads the boundary before placing the foot. This fixed
    the `rest_zip_admission` 1e-6 red that #4128 introduced. Orchestrator read; merged.
  - PR 4205: `one_segment_loop` held a quadrature midpoint to a fixed tolerance, the 1e-6 red from
    #4169. It is merged, and the latent instances of the class are filed on QUAD.
  - #4208, a REACH bounds-census roster line, was ported into PRs 4177 and 4181 ahead of its merge.
- **Re-dispatched after the merges:**
  - `revolve-seam-split-volumes-miss-their-closed-forms-at-eps-1e-6`, raised to P1 because it is a
    nightly red; it is probably the #4205 class. Lane `cleave/revseam-1e6`.
  - `lily-walls-curved-clearance-crowds-the-band-under-k-lint`, P2. Lane `cleave/lily-clearance`.
- **PR 4214 merged** (lily-walls k-lint; tracker only). Main had already fixed it via #3817, and the
  row is closed. The sphere-region root-slack sibling is filed on HONE.
- **PR 4213 merged** (revolve-seam 1e-6 volumes; tracker only). Main had already fixed it in PR
  4083's `6ac174bf1a`, and the row is closed. Two `props_quad_converged` witnesses were added to
  QUAD's convergence row.
- **Dispatched** two P3 measure-first lanes: `a-rim-touching-split-escalates-on-the-side-of-plane-band`
  (`cleave/rim-touch`) and `interval-steep-cut-through-cylinder-caps-refuses-order-escalated`
  (`cleave/interval-join-order`). Review tier for each: single FULL if code moves, orchestrator read
  if not.
- **PR 4223 merged** (rim-touch; test and tracker only, orchestrator read). PR 4179 had already
  fixed it, and the witnesses are pinned. Filed: a plane touching a bore rim splits into a pinched
  side or refuses (P3; its answer is D10 coincidence work, so it waits on the hold).
- **PR 4224** (interval join order; single FULL review): APPROVE-WITH-FIXES.
  - Confirmed: the cause (the old frame's `u` was perpendicular to y, so cap chords tied); no
    golden moved; the tests go red under mutation.
  - MAJOR: the oblique frame still ties on one great circle of normals per face direction, and the
    cylinder at n = (2,1,±1) went from answering to refusing. Accepted as a large net narrowing
    (cylinder 42→53, box 2→48 of 62 normals). The docs and body must state the residual exactly,
    and it is filed with candidate shapes (a SCHEDULE ladder or an intrinsic tie-break, priced in
    f64/Interval lane agreement).
  - Fix pass also covers:
    - the frame-axis refusal decides only what it needs and names its cause;
    - a dimensionless value no longer goes through the length door;
    - stored `SectionFace` indices re-pointing on tilted multi-region splits is disclosed and filed
      on the naming owner.
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. Your ground: `work/cleave/split-dihedral-readers-drop-the-arm-rung.md` covers `seam_refusal` and the split readers, which still drop the verdict; `sectors.rs:534`/`:750` `BooleanError::of_lever` callers too. Its "near line 874" cite is now `finish.rs:932`. (ENCL orchestrator)
- 2026-10-08 — Filed from ENCL (`encl/offset-cert-coefficient-norms`): `ellipse-torus-graze-certifies-six-roots-where-the-true-distance-crosses-four` (P1). A fuzz counterexample in `ellipse_torus`'s certified-answers row, reproduced on main; replay line in the row. (ENCL implementer)
- 2026-10-09 — Seam note from ENCL (PR 4366, merged): `geom_brep::enters::LeverEscalation`'s `rung` and `diag` are private; read them with `rung()`/`diag()`, re-quote only through `with_diag`, which keeps the gate's verdict. `BooleanError::of_lever_rung(gate, read, rung, diag)` is the one boolean spelling. The dihedral lever-arm decision is told in one shape ("long enough, for how its faces curve, to measure their angle"), with the lever "clearly longer and no face curves tightly there"; pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` (it reads source literals: a natural "long enough … angle … face" wording elsewhere trips it). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4401, merged): `LeverEscalation::with_diag` is gone. The one re-quote door is `quoting_reading`; ask `re_quotes()` first. A decided arm keeps its verdict only onto a reading of the same rejected sign, and otherwise is returned unchanged. The dihedral arm clause is `geom_brep::DIHEDRAL_ARM_CLAUSE` / `dihedral_arm_clause!()`; compose it, never re-spell it. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4416, merged): `geom_core::lever_recourse(lever, note)` is the one spelling of a lever-alone ending, and `Indeterminate::undecided(subject, ending)` the one "{subject} is undecided: {payload}. {ending}"; compose them, do not re-spell. In `topo::boolean::refusal_routes`, `Ending::Lever` is now `Lever(&str)` (`LeverPass` is gone). Rendered texts are unchanged. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: dispatching `encl/rim-wedge-one-walk` (P3, `rim-wedge-walks-the-second-order-stations-by-hand`). It touches `topo::boolean::rim_wedge::classify_shared_rim`'s station loop, which will read through `geom_brep::second_order_walk` with a shared material hook. No answer may move. A unit on `rim_wedge.rs` should wait for it or coordinate. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4433, `encl/rim-wedge-one-walk`, merged at `0bdb2baeae`): `geom_brep::second_order_walk` now takes its stations (`impl IntoIterator<Item = (Point3, Vec3)>`) instead of `carrier, t0, t1`. `topo::validate::MaterialStations` (`pub(crate)`, `new`, `outcome`) is the one home of the per-station material reads, shared by tier 3's check 4 and `boolean::rim_wedge::classify_shared_rim`. The rim's own station loop and its cusp-side Zero mint are gone; the Zero arm is now `MaterialStations::after_positive`. Tier 3 reads `interior_stations` once for checks 4 and 5. A source scan (`tier3_tests::check_4_and_the_rim_route_…`) reds on a second walk in either file's production code. No answer or text moved. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4461, merged at `a71723601f`): `topo::ShellRead::of` now returns `Result<Self, ShellClassifyError>`, with no `Option`: the unreadable selection was unreachable. `SolidFaces::of_shell` returns `Self`, and `ChartGroups::of_live` groups faces already read off the live arena (`of_body` uses it). `PieceSortError::RoleUnread` carries the role read's `ShellClassifyError` and renders it. (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL: ENCL's scoping of `hand-minted-invalid-gates-in-topo` overlaps your parked `topo-mints-indeterminates-outside-the-funnel` at `splitting/rules:197` (a face extent decided Zero, reported as INVALID), `splitting/classify:608` (`split_conic_graze_side` decided Zero), `rules:319` and `census:2583`. ENCL's `topo-poisoned-escalations-offer-unfollowable-endings` will touch only the endings at `rules:319`/`:488`/`:500` and `census:2583`, not the mints. (ENCL orchestrator)
- 2026-10-10 — Seam note from PIPE (PR 4486, `pipe/census-arms-and-witness-caps`): `boolean::boxes::edge_box_rule` now takes the certified `Option<&geom_brep::EdgeCurve<T>>`, and `EdgeBoxRule::ConicAmplitude` carries `params: (T, T)`. `AxialCarrier::Conic::params` is `(T, T)`, not `Option`, and the full-turn `None` arms (unreachable by construction) are gone. No box moved. (PIPE implementer, D291 lane)
- 2026-10-10 — Seam note from ENCL (PR 4475, merged at `61977302ae`): a poisoned margin's note now depends on the door that reads it. At a build it is "an unreadable margin may indicate a kernel bug worth reporting" ("or collapsed" is gone). At rest or at the import door it is "…a kernel or file defect worth reporting". `geom_brep::recourse::unreadable_margin_note(reading)` picks it. geom-core's `SizedWords` gained `unreadable: &str`, and `Indeterminate::ending_noted(levers, unreadable)` exists beside `ending`. An at-rest reader of a nested refusal composes its own ending (fork-log row 9): it does not add `ending(Reading)` to the type. `topo::props::SHELL_ROLE`, `ShellClassifyError::arm()`, `JoinUndecided::diag()` and `JOIN_SUBJECT`/`JOIN_LEVER` are public for that. (ENCL orchestrator)
- 2026-10-10 — Seam note from PIPE S350 (PR 4482): comment-only edit in `crates/topo/src/boolean/boxes.rs`'s `every_door_that_reads_a_box_is_inventoried` prose — the two census `face_box(` calls are now described as test-module rows pinning a described net carrying poison (one of them gates). The pinned count (`census.rs`, 7) is unchanged. (PIPE S350 lane)
- 2026-10-10 — Seam note from ENCL: dispatching `encl/topo-poisoned-endings`. It touches only the endings of poisoned or contradicted arms on `PointInSolidError::Escalated` (solid_contain), `SplitReduceError::SliverSector` (splitting/rules :319/:488/:500), `CensusEscalated` (census :2583) and possibly ChartRegion/Region/Section/Contain escalations. No mint moves; the readers compose their own endings, per fork-log row 9. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4474, merged): the material pairing is a decision. `geom_brep::MATERIAL_PAIRING` and `MATERIAL_PAIRING_CLAUSE` end it at tier 3 (`WedgeCheck::MaterialPairing`) and split finish (`SplitFinishError::DescribeSideEscalated`), with a true tolerance offer. Where the wedge would re-decide below m/K, `pairing_at_wedge` quotes the wedge and offers w/K, as `at_wedge` does. `MaterialStations`' `Break` is `MaterialStop { check, cause }`. The cusp-side Zero reads via `decide_nonzero`. New k_stats predicates appear on the refusal path only: "material_pairing_offer_wedge" and "material_pairing_wedge". (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4497, merged at `4c7fb9b0fc`). A poisoned or contradicted margin now ends in a followable recourse:
  - at rest, the census coincidence menu (`validate::too_close` → `own_close`) gives the defect ending;
  - `ChartRegionError::Escalated`'s poisoned arm gives `defect_ending(Build)`;
  - `PointInSolidError::Escalated` gives the unnamed placement lever plus the reading's note, as `contfp`, `classify_point_in_solid` and the Boolean already did.

  `SliverSector` is unchanged, and its mints are on CLEAVE's split-escalations row. (ENCL orchestrator)

## 2026-10-10 — resumed after the usage pause (CLEAVE orchestrator)

- The PR 4224 fix-pass lane was cut off by the limit before it pushed anything. It has been resumed
  and will merge three days of main.
- **Dispatched** (single FULL review each when code moves):
  - `ellipse-torus-graze-certifies-six-roots-where-the-true-distance-crosses-four` (P1): a possibly
    unsound root certificate.
  - `a-split-through-a-vertex-whose-above-runs-nest-crosses-its-section-face` (P1): a shipped
    section face that crosses itself at an apex.
  - `wedge-classes-reads-a-corner-flat-at-its-short-bounds-as-convex` (P1): an `On` bound votes
    convex.
  - The P0 `topo-mints-indeterminates-outside-the-funnel`, steps 3–5. It was released by INTENT
    stage 4 E on 2026-10-09. First a scoping lane re-takes the census, because ENCL, CONTACT, PRED
    and the D10 hold have since taken parts of this ground. It files CLEAVE's remaining units as
    rows.
- Still open, waiting for a slot: the P1s `a-built-sliver-is-not-a-legal-operand`,
  `near-tangent-pierce-poses-reach-three-classification-invariants`,
  `continuation-scan-box-fallback-reads-a-world-axis-run` and
  `split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence`.
- **PR 4546 merged** (the P0 mints re-scope; tracker only).
  - The census found 69 arms: 24 CLEAVE, 35 D10-held, and the rest PRED, CHART, TOPO and
    RESTFRONT.
  - Step 3 of the 10-03 design is superseded. Main's vocabulary already does the job, and its
    contradictions go with D10 stage 4.
  - Seven units are filed: four are dispatchable P1s; the stage-4 unit and the seal are parked; the
    ratchet gate is P3.
  - The umbrella is re-prioritised P0 → P1 and `design: true` moves to the stage-4 unit.
- **PR 4552 merged** (the ellipse-torus graze P1; the oracle was wrong, not the kernel; test and
  oracle change only, orchestrator read). Filed P3: the ray × torus probe oracles cannot see a graze
  pair inside one cell.
- **PR 4224 merged** (the interval join-order frame; single FULL review, fix pass done). The
  oblique-frame residual and the axis-plane residual are P3 rows. The frame-building triplication is
  filed on HONE, and the `SectionFace` renumbering is evidence on EMIT's ordinal row.
- **Dispatched:**
  - mints unit 1, `split-sector-rules-mint-no-invalid`;
  - mints unit 2, `split-straddles-and-disagreements-carry-their-reading`, on disjoint files;
  - `near-tangent-pierce-poses-reach-three-classification-invariants` (P1).
- **PR 4552 merged** earlier today: the ellipse-torus "6 vs 4 roots" was the test oracle's
  unsearched end cells. The certificate is sound, and the row is closed.
- **PR 4224 merged**: the interval join-order frame, after its fix pass.
- **PR 4565** (apex section corners; single FULL review): NOT-MERGEABLE-AS-IS.
  - It fixes the nested-run crossings and more: the 4- and 7-legged combs, the monkey-saddle crown,
    and base's W-base `unreachable!`.
  - It regresses two classes that base splits soundly:
    - in-plane valley edges at the apex, where it refuses the "unreachable"
      `SectionGermsUnordered`;
    - an edge decided On but off the plane by more than ε/2, which escalates.
  - Ruled: order by topology (orbit plus alternation) where it decides, and by geometry only where
    it doesn't, with no regression against the reviewer's 31-family probe. Also: the verbs mirror
    prose, refusal payloads, the PR-body provenance (e16309aa7a), and the germ-order duplication
    against the Boolean's.
  - The PR 3960 "mirror lane stays" clause belonged to the SplitDeclarations design that D10
    replaced, so it no longer binds.
- **PR 4571** (mints unit 1; single FULL review): APPROVE-WITH-FIXES. Ruled:
  - one geom-brep door for a direction's sense along a normal, with its precondition typed;
  - C4 is a disagreement of two decided readings, not a straddle, and carries both margins;
  - unreachable `Collapsed` panics with its values (D9 row 4);
  - the K-dependence of `GrazeAgainstNeighbours` and the C5 offer at K < 2 are filed.
- **PR 4564** (`wedge_classes` On; single FULL review): in review. The lane pushed an empty commit
  to kick CI, against the rules; noted.
- **PR 4564** (`wedge_classes` On; single FULL review): APPROVE-WITH-FIXES.
  - Confirmed: 0 wrong across 9 fuzz seeds; 4,649 refusals now answer, each matching the exact
    oracle.
  - Ruled: disclose and test the two-face arm; give `one_plane` its own predicate name; route the
    met in-band row like the hollow one; narrow the `Err(_)` catch; move the pin to the fast set.
- **PR 4581** (near-tangent invariants; single FULL review): NOT-MERGEABLE-AS-IS. The fix is right.
  At 1e-12, 4 runs that refused `ClassificationInvariant` now ship bodies failing tier 3′. That is
  FUSE's stale-contact / undeclared-contact class, and the door runs no census over its result.
  - **Orchestrator ruling: trade accepted, made explicit.**
    - The refusal it replaces was a wrong, translation-dependent reading.
    - Main already ships identical findings at the neighbouring pose, and a sub-ulp pose change
      puts 25 main runs into this class.
    - The four runs are pinned at their current findings, pointing to FUSE's row and to
      `boolean-door-runs-the-census-over-its-result`.
    - Reported to Ev.
  - Fix pass also covers:
    - the sweep pin checks U/I/S, closed forms and tier 3′ (it passed under a germ-sign mutation);
    - `ExtentBall` radii that enclose their region;
    - one germ-sign method;
    - the probe fixture made bit-identical again.
- **PR 4571**: re-running CI after the merge of main; it merges when green. The seal's fired `blocked_on` entry is pruned.
- **PR 4571 merged** (mints unit 1; single FULL review, fix pass done). It announces two seams:
  `geom_brep::normal_alignment` / `ParallelNormal`, and `geom_core::k_stats::decide_nonzero_reported`
  on SCALAR's ground. Unit 2 (PR 4578) must reconcile its own `k_stats` doors against these.
- **PR 4571 merged** after a green re-run on the main merge.
- **PR 4578** (mints unit 2; single FULL review): APPROVE-WITH-FIXES. Ruled:
  - the unreachable `(Some, Some)` arm panics, and its one `at` is made structural;
  - "nearest to passing" gets a pin;
  - the straddle and gate-rejection doors take `Decided` bounds (D9 row 0);
  - one `k_stats` gate-rejection path, reconciled with 4571's additions;
  - `lift` and `clearance` endings traced.
- **Dispatched:** mints unit 3, `point-in-solid-mints-outside-its-period-and-nappe-rows`.
- **PR 4564 merged** (`wedge_classes` On; single FULL review, fix pass done). The coplanarity
  triplication is filed on HONE.
- **PR 4578 merged** (mints unit 2; single FULL review, fix pass done). `k_stats` has one
  gate-rejection path, reconciled with 4571.
- **PR 4605 merged** (tracker only). The sliver-operand row is parked on FLUXTAIL's new
  `a-sphere-faces-flux-cancels-r-times-its-area-against-its-vector-area` (P1). The re-pose is the
  fix under the ratified Q1 premise, not a result-gate refusal, which PR 4335's honesty condition
  forbids. Option (B) is noted on TALLY's typing row.
- **PR 4578 merged** after a green re-run on the main merge.
- **PR 4565** (apex section) re-review: APPROVE-WITH-FIXES. Across about 63k fuzz cells head ships 0
  crossed corners, against base's about 1,500.
  - About 0.5% of cells now refuse an existing `pcurve_map_residual` certification class that base
    escaped only through the retired mirror rerun. Ruled: accept the trade, disclose it, and file
    the class on its owner.
  - `ArcSides` becomes `unreachable!`.
  - File the pre-existing crown `join.rs` panic as a CLEAVE P1.
- **Dispatched:**
  - mints unit 4, `boolean-gate-mints-route-through-the-funnel`;
  - `split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence`, its post-D10 residue
    (kernel-defect voice at δ = 5e-10, and `SliverVertex` text beyond ε).
- **PR 4565 merged** (apex section corners: review, re-review and two fix passes). Filed:
  - TOPO's recharted-residual row (P2);
  - CLEAVE's crown spur panic (P1);
  - HONE's two-homes germ-order row.
- **PR 4581 merged** (near-tangent invariants; single FULL review, fix pass done). The 1e-12 trade is
  pinned as `SHIPS_A_FINDING` and points at FUSE's row and the door-census row.
- **PR 4581 merged** after a green re-run on its main merge.
- **PR 4565 merged** after a fresh run on a real main merge, which cleared GERM's random
  `pn_apex_point_reach` fuzz red (its second seed is on GERM's row; the 403 stopped a re-run).
- **PR 4607** (mints unit 3; single FULL review): APPROVE-WITH-FIXES, one MAJOR.
  - C17's cause was inverted. The certified quartic count is wrong (rung 1's Δ rounding is
    uncharged), and the construction is right.
  - At R = 1e6, 10 rays get a certified, silent wrong `Miss`. Ruled: correct the texts, keep the
    kernel-defect ending, and file the rung-1 defect on INSIDE (P1).
- **PR 4609** (band-on residue): approved on an orchestrator read. It waits on a main merge and a
  digest re-bless.
- **Dispatched:**
  - `a-crown-touching-the-plane-along-one-edge-panics-at-the-section-spur-check` (P1, a D9 panic on
    exact input);
  - `single-on-edge-crossing-without-a-flanking-record-at-1e-12` (P1).
- **PR 4609 merged** (band-on pinch residue; orchestrator read). The row is closed.
- **PR 4609 merged** (band-on residue).
- **PR 4607 merged** (mints unit 3; single FULL review, fix pass done). INSIDE's P1
  `ray-torus-count-certifies-on-an-uncharged-discriminant` carries 7 silent wrong `Miss` rays at
  R = 1e6.
- 2026-10-11 — PR 4613 (mints unit 4) review: APPROVE-WITH-FIXES, no MAJOR. Fix pass sent: C10's at-rest defect arm is unreachable at the census (tier 3 certifies endpoint pinning first), so it goes to D9 `unreachable!` or is pinned through a real caller, with the K ≤ 3 lever move pinned on geometry; C11's hand-built `Decided` carries vtxfac's margin or is pinned and listed on the ratchet row; the `wall_crossing`/`ball_off_face` moves get pinned; `rim_wedge::contains`' lift is typed by cause (in-band → Escalated); style sweep (stale coarser-band text, Display order, the definite finding out of `CensusUnsupported`, the editor-core attribution row, unit 3's stale `Decided` prescription).
- 2026-10-11 — `continuation-scan-box-fallback-reads-a-world-axis-run` → PR 4626 (tests and tracker only). The scan was deleted by glue-on-Zero (PR 4496), and the row's premise was false even against the deleted code (an ellipse never reached the fallback; a poison box read "shares a curve" at every pose). Pinned with a 13-pose rotation family (apart and plugged) at all three ε, with the `glue::touched` mutation red. Orchestrator read: accepted. Filed P2 `a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims` (its open point: whether the one-carrier arms' `LinesAndCircles` reach is still D10-held now that they serve value-decided pairs). Merges once the 4609 P123 red on main is fixed.
- 2026-10-11 — Note from ENCL: main is red on `topo` `split_through_an_apex::a_recharted_residual_in_the_band_refuses_typed`. Main pushes skip the test job, and any PR that runs topo goes red. ENCL bisected it: the test (PR 4565, `bb4f54837c`) and PR 4609's fix (`758162892d`, `chord_u_ref` charted in the plane) conflict semantically. 4609's branch predates 4565, so its CI never ran the row. Measured: before 4609 this witness refused at 1.077 ε; after it, all 20 pose/normal/side cells split, with the largest re-charted residual 0.9000002 ε (the On vertex's own 0.9 ε offset, under ε). Every side passes tier 3 and `pyramid_splits_sound`. So 4609 is an improvement and the row's premise is stale. The suggested re-baseline: turn the row into a sound-split row (`pyramid_splits_sound` on this base, with residual ≤ ε). Re-run the class's fuzz (seeds `0x9E3779B97F4A7C15` and `1234567891011`) before closing `work/topo/a-split-recharts-an-edge-at-a-vertex-read-on-off-the-plane-into-the-band.md`. ENCL has not touched your file. (ENCL orchestrator)
- 2026-10-11 — PR 4631 merged (main red fix): P123 builds after 4609's `chord_u_ref` projection. The cause was confirmed by reverting the projection. The pin now holds P123 sound against the closed form in every cell, and the TOPO re-chart row is closed (every fuzz re-chart refusal now splits sound). Ruling: the in-band tier-3′ census escalation on these bodies does not count against "sound". It is an Escalated reading, not a falsification, and it matches the file's existing bar; RESTREAD P2 tracks it. Closed the P0 `main-reds-on-a-recharted-residual-in-the-band-since-4609`. Merged on a green head: main's moves since were disjoint from the split code (two comment-only topo lines).
- 2026-10-11 — PR 4607: merged main with the 4631 fix. That brought in GERM's `in_band_sweep` fuzz (PR 4600), whose match named `CircleRoots::CountDisagrees`, which this PR deletes. Dropped the arm in the merge; topo clippy and the affected lib rows are green locally. The GERM half-angle row (closed on main) keeps both its Closed section and the CLEAVE ray-lane note; the note now says INSIDE owns the fix.
- 2026-10-11 — PR 4626 merged (continuation box: does not reproduce; pinned across 13 poses). Main's moves since its green head were disjoint (NURBS knot algebra, ENCL offset fit).
- 2026-10-11 — PR 4613 re-review of the fix pass: FIX-FIRST. MAJOR: the census `unreachable!` on an off-corner arc end can be reached at K near 1. Contain reads ends through `ConicArc::point` and certify through `Curve3::eval`, and the two part by rounding wider than `(K−1)·zero` (witness at ε 1e-9, K 1.001, r ≈ 1e5). Second fix pass: one evaluation story, plus a typed defect ending instead of the panic. The lane's rim_wedge deviation (`contains → Ok(true)`) is not accepted: through `departures` it ships a decided `SeamFaceRunsOn` contradiction from an in-band reading. Restore base's escalation, pin it through the Boolean, and file the pre-existing OnEdge conflation. `ball_off_face` and vtxfac `unreachable!`s hold; `enters_material_reported` is a pure refactor; the recl pin goes red under mutation.
- 2026-10-11 — PR 4626 state-sync: `continuation-scan-box-fallback-reads-a-world-axis-run` closed (does not reproduce; pinned across 13 poses). Merged main with the 4631 fix for a green gate.
- 2026-10-11 — PR 4607's `test` went red on GERM's `in_band_sweep::no_pose_inside_the_band_is_answered` (random seed 0x71b4eb0ade4c1576). It reproduces on main at 50537331ae, so it is not 4607's. Filed GERM P1 `the-in-band-sweep-fuzz-finds-a-miss-answered-inside-the-escalation-gap`. PR 4622 (crown spur) is in for a full review.
- 2026-10-11 — All four P1 lanes are in: PR 4622 (crown spur), PR 4625 (folded-graze JoinDesync), PR 4630 (single on-edge at 1e-12) and PR 4626 (merged); each of 4622, 4625 and 4630 is in for a full review. 4625's sibling variant `EdgeUndescribed` (instead of a payload on `JoinDesync`'s ~100 sites) is accepted, subject to review. Dispatched two P2 rows whose code no open PR touches: `bool-pair-gap-is-eps-coupled-…` (`separating.rs`, k-lint roster) and `section-cert-axis-pose-…` (`section_cert.rs`). Held: `a-split-through-a-wrap-edges-vertex-…`, which overlaps 4622's `finish.rs` and 4625's `describe_edges`; `a-tilted-plug-…`, whose D10 reach question comes first.
- 2026-10-11 — PR 4607 merged (mints unit 3: point_in_solid's C13–C18 mint no INVALID) on a green head. The re-run after the GERM fuzz red came back clean; main's moves since were disjoint. Its row was closed and the seal's `blocked_on` pruned in the PR.
- 2026-10-11 — PR 4630 review: APPROVE-WITH-FIXES. Over 280k runs at all three ε, including off-grid and sub-band shifts, the new mint arm gave no wrong answer. But the sweep missed two reachable siblings of the lookup-refuses-if-absent class, both pre-existing. `resolve_bisector_graze` never retries the reference's other twin: `ClassificationInvariant` on 126 runs at 1e-6, and this PR moves 18 runs into it. `find_ref_sector`'s refusal is reachable at 1e-12 and is operand-order asymmetric (vee300 e0 a11.75). Fix pass sent: twin retry in the bisector graze; root-cause the reference-sector refusal and fix or file it; a 1e-6 witness for the off-grid firing; style. The lane is warned that 4613 edits the same `recl.rs` neighbourhood.
- 2026-10-11 — `bool-pair-gap-is-eps-coupled-…` came back as a design fork with no code change. The recorder is right: the margin is `gap − 2·sweep_pad`, the margin the decision decides on, ε-coupled by construction at `2(K+2)·ε`. Rostering it needs a ruling on k-lint rule (4) (one shared floor at 150·ε vs per-family floors, plus a new committed era), which is INSTR's territory (`tools/k-lint`, `docs/K-REPORT.md`). Re-homed the row to `work/instr/` with the measurement, `design: true`.
- 2026-10-11 — PR 4625 review: APPROVE-WITH-FIXES. `Escalated` is honest: exact rationals put the input on the band edge, the move is sub-ulp, and the lever builds both ways. Fix pass sent:
  - document the edge-orientation dependence at d = ε, and file the endpoint-sample re-pose;
  - `WitnessSurfaceResidual` and fitted carriers keep their defect or fit endings, and only exact carriers' `SurfaceResidual` takes `OffFace`;
  - the merge door's definite failure takes the defect ending;
  - `check_pose` uses `offer_key`'s defect rule;
  - text and skip style.

  The shared disk briefly hit 100% during the reviews. Running lanes are warned to rerun anything that died with ENOSPC, and the orchestrator's target dir and main worktree are freed (19G free).
- 2026-10-11 — PR 4622 review: CHANGES. The split fix is right: the reviewer could not break it at any ε, both sides pass tier 3′, and killing a strut changes no point set. But public `plane_section` reads the join's loops before the kill, so for this class it now returns an outline with a zero-width spike, where main panicked. That is a wrong answer, and the pin checks area only. Fix pass sent: strip the strut where both consumers share it and pin outline simplicity; the unpaired slit becomes D9 `unreachable!`; widen the pin to ±0.9ε, add a boolean-built trench and a negative test for the helper; strut-definition and doc style.
- 2026-10-11 — Note from FLUSH: evidence added to `a-built-sliver-is-not-a-legal-operand`. A slab and a closed hollow 1.5e-8 thick, at rest, refuse `ShellWitnessExhausted` as operands of a union with a cube 5 m away (ε 1e-9); a fin of the same thickness on a thick part builds. A designer lane's hypothesis: `point_in_solid`'s margin reads half the wall's thickness, not the test point's distance to the boundary. FLUSH's thin-hollow design fork (fork log row 113) depends on the answer. (FLUSH orchestrator)
- 2026-10-11 — Note from FLUSH: filed `a-missed-ray-reads-the-solids-volume-sign-for-its-side` (P1) on this slate. `solid_contain.rs` `at_infinity_side` takes a missed ray's side from the solid's V/A, so a definite 1.5e-8 slab refuses `ShellWitnessExhausted` as a disjoint operand. It supersedes the hypothesis FLUSH added to `a-built-sliver-is-not-a-legal-operand` earlier today. (FLUSH orchestrator)
- 2026-10-11 — Main is red on sweep `operand_gate_support_plates` (touching operands built as disjoint, a wrong answer) and on the `mass_props` thread-count golden. Both block 4625 and 4630. The golden is NURBS's (PR 4684 re-cuts it). The support-plates row was filed on GERM, and also on CONTACTHOLD via the unmerged PR 4681, with no bisect yet, so CLEAVE dispatched `cleave-plates` on `cleave/support-plates-main-red`. 4625's fix pass is pushed (c57abcea66); its only reds are these two from main.
- 2026-10-11 — PR 4657 review: APPROVE-WITH-FIXES. The swing and `turn` bounds hold numerically, and no built verdict moved to refused at any ε. Fix pass sent:
  - the cylinder-pair coincidence swing scales with the wall radius, so walls 20–200 bands apart now refuse — bound it by the axial station;
  - pins that a halved swing or a dropped `turn` would turn red (a constructible cone-apex `turn` witness);
  - converge the copies of `decide_across`, `line_tilt` and `ExtentBall`;
  - name the straddle refusal.
- 2026-10-11 — PR 4622 re-review of the fix pass: FIX-FIRST, small. The straight case is sound everywhere probed: crowns, trenches, apex pyramids, regions meeting at a vertex or along an edge, real notches kept. Both pins go red under the mutants. But a curved slit (a revolved V-groove arc keel) still spikes `plane_section` with the normal down, which is pre-existing on main but contradicts the new doc. Second pass sent: read curved slits, or refuse typed and file; a tighter `section_is`; the `strut_turn` self-loop doc; style.
- 2026-10-11 — PR 4630 re-review of the fix pass: no wrong answer anywhere. The twin retry and `first_read` only fill gaps and never pick a different record, and edge-edge through `place` is identical. Ruling: one pose (`w345 e1 a3.8 d1e-7`, 1e-6, `pc ∪`) moves from a `ClassificationInvariant` refusal on main to shipping a body with a definite tier-3′ `EdgeEdgeCross`. I accept it as I did for 4581: main's answer was itself a kernel-defect refusal on valid input, and the finding is FUSE's tracked census-on-result class. It is pinned in `SHIPS_A_FINDING` with evidence added to FUSE's row, and reported to Ev. Small final pass sent: pin the held-lookup `twins` and the 1e-9 vee300 instance, give the `records` vs `first_read` reason at each call site, style.
- 2026-10-11 — Main's support-plates red, bisected with fresh compiles: #4604 (`08ba4968fa`, CONTACTHOLD) retired `CurvedRestUnrecorded` on purpose, so the frustum rim~plate point touch now ships as two solids plus an `EdgeFace` record. That is D1's ratified answer, with 0 of 16,320 sampled points wrong at every ε. The test's ∪ exclusion is stale. Ruling: re-baseline (`excluded = cut_touch || Frustum`; an excluded ∪ with no record is still wrong), with no revert of #4604. Lane `cleave-plates` is opening the PR. TALLY's P2 `a-plate-touching-a-cut-torus-at-its-cap-rim-builds-an-assembly` gets the D1/#4604 answer as a note.
- 2026-10-11 — PR 4707 merged: main's support-plates red is fixed by re-baselining the row's frustum exclusion to D1's two-solids-plus-record answer (#4604). The GERM P0 is closed. The remaining slow-set red on main is the `mass_props` golden (NURBS PR 4684). 4707's own gate was red on that test only; I merged it on the grounds that it is identical on main and owned elsewhere.
- 2026-10-11 — PR 4630 state-sync: final pass accepted. Merged main (with 4707's support-plates fix) and closed `single-on-edge-crossing-without-a-flanking-record-at-1e-12`.
- 2026-10-11 — PR 4622 state-sync: second fix pass accepted by orchestrator read (a curved slit is read when two conic arcs agree in band at the ends and at ¼, ½, ¾; `strut_turn` excludes closed edges). Merged main (with 4707) and closed the crown spur row.
- 2026-10-11 — PR 4625 state-sync: the fix pass was accepted by orchestrator read (`of_attachment` routes only an exact carrier's `SurfaceResidual` to `OffFace`; the witness and fitted carriers keep their own endings). Merged main (kept both payload-rung dispositions), closed `a-minted-edge-along-a-folded-graze-refuses-join-desync` and `joindesync-swallows-the-certification-payload`.
- 2026-10-11 — PR 4613 state-sync: second fix pass accepted on the lane's report. M1 is root-caused (arc ends read through `Curve3::eval`), and off-corner is the typed defect `ArcEndOffCorner`, not a panic. The witness pin reads in band at every ε. The rim_wedge deviation is accepted: base had no in-band margin to escalate on, and the arm now ends in the seam cover's typed lane-limit refusal rather than a decided contradiction. `invalid_margin` was restored for main's new plane×torus caller and listed on the ratchet row. Merged main (no conflicts), closed the row, and pruned the seal's `blocked_on`.
- 2026-10-11 — PR 4625 merged (folded-graze JoinDesync → `Escalated EdgeOnFaces` / `EdgeUndescribed`). The gate was red only on the `mass_props` golden, identical on main and owned by NURBS PR 4684, and main's moves since were disjoint from its files. Its rows were closed in the PR.
- 2026-10-11 — PR 4657 merged (section_cert axis pose: one margin across the reach). The re-review said MERGE: no verdict moved at any ε, the bounds hold against dense samples, and the mutations go red apart from two pinning-impractical second-order terms. Merged on its green head (main's only overlap was a two-line message change in `intersect.rs`), and the row is closed here. Style residue left in the PR's review (S1 `Reach` beside `ExtentBall`, S2 `line_tilt` hosted in `carrier_eq`).
- 2026-10-11 — Main red on the hand-written impl census since SHELL's #4677 (`ssi/plane.rs`'s `Debug` was not listed). Fixed by PR 4713 (CLEAVE): the impl already destructures, so only the roster needed the line.
- 2026-10-11 — PR 4613 merged (mints unit 4). Its last run was red only on the census (fixed by 4713) and GERM's random fuzz. Its row was closed and the seal's `blocked_on` pruned in the PR.
- 2026-10-11 — PR 4622 merged (crown spur). It was red only on the `mass_props` golden (NURBS 4684), and its row was closed in the PR. 4630's main merge conflicts with 4625 in `near_tangent_pierce_poses.rs`, and the lane is resolving it.
- 2026-10-11 — Dispatched P1 `a-missed-ray-reads-the-solids-volume-sign-for-its-side` (FLUSH's thin-hollow fork finding; `solid_contain.rs` `at_infinity_side`) → lane `cleave-missedray`, branch `cleave/missed-ray-side`.
- 2026-10-11 — Dispatched P2 `a-split-through-a-wrap-edges-vertex-cuts-the-wall-along-it` (its overlaps with 4622/4625 are gone) → lane `cleave-wrapedge`, branch `cleave/wrap-edge-split`.
- 2026-10-11 — 4613 and #4650 merged green on their own branches but did not compile together on main (an exhaustive `piece_holds` match without `ArcEndClearOfCorners`). CONTACTHOLD fixed it in #4716, and 4630 carried the same line. Lesson: merging a PR whose last CI predates other merges to its files needs a `cargo check` against the CURRENT main, which I ran for 4613 against a main that #4650 had not yet reached.
- 2026-10-11 — PR 4630 merged (single on-edge 1e-12). It was red only on the `mass_props` golden. Its merge resolution with 4625 kept both sides' poses and tests, and four of 4581's 1e-12 shipped findings no longer ship on main, so they are now held to a clean tier 3′ (noted on FUSE's row). Its row was closed in the PR.
- 2026-10-11 — Dispatched P2 `a-chords-end-sample-reads-carrier-t1-not-its-vertex` → `cleave/chord-end-sample`, and P2 `seam-cover-reads-a-boundary-point-as-the-face-interior` → `cleave/seam-cover-boundary`.
- 2026-10-11 — PR 4722 merged (wrap-edge split: premise false, tests and rows only). Its gate was red only on the `mass_props` golden; resolved a `nextest.toml` conflict keeping both sides' slow-set entries; row closed. Filed: SECT P2 (a plane tangent to a rim at a wrap vertex has no join arm) and two CLEAVE P3s.
- 2026-10-11 — PR 4734 merged (seam cover boundary point; orchestrator read, small). The lane corrected the brief: no public door returned the wrong fact, because `glue::tangency` reads the seam check with `.is_ok()`. Only the check's own answer was wrong. The lane's choice to add no defect arm is accepted. Row closed.
- 2026-10-11 — PR 4736 merged (main red, CLEAVE's own pin): the shipped-finding pin carries the census's newly recorded margin.
- 2026-10-11 — PR 4732 merged (chord end sample; tests and docs only). Ruling: the certificate bounds the carrier (D4 ¶2, Q1), so the orientation dependence at the band edge is admissible. Its gate was red only on the `mass_props` golden. Row closed; filed RESTFRONT P3 `tier-3s-planar-arm-never-reads-an-edges-carrier-ends`. PR 4725 (missed-ray side) is in full review.
- 2026-10-11 — PR 4725 review: APPROVE-WITH-FIXES, and no wrong side found by a dense oracle at any ε. Blocking fix: an inside-out at-rest operand (D1's volume exemption) now gives silent wrong Booleans where main refused, so it must refuse typed as an operand defect. RESTFRONT's row is raised to P1 with that evidence. Also: refuse with the aimed rays' own in-band reading rather than `VolumeUncertified`; aimed-ray faults are ray-local; share `stands`' witness ladder; stronger pins; file `differential::outcome`'s `is_ok()`-only operand check as a class.
- 2026-10-11 — PR 4725 re-review: FIX-FIRST. Two silent-wrong routes remain, both refused on main: the gate's single probe point reads in band and passes (a thin reverted tube at 1e-12), and dual operands skip the gate. Second pass sent: refuse a decided far-side `In` on a user operand at the reading itself, for every scalar; promote only genuine in-band aimed readings, not the period guard's structural INVALID; fix the operand refusal's DEFECT ending; style.
- 2026-10-12 — PR 4725 (missed-ray side) closed after a final re-review said MERGE. Its style residue is filed as P3 `inside-out-operand-reading-has-two-homes-and-four-spellings`, and RESTFRONT holds the at-rest inside-out row (P1) and the ending row.
- 2026-10-12 — Seam note from ENCL (PR 4745, merged at `512ef4b6c7`): topo's too-close-to-call sentences now take the undecided shape. Split endings: finish.rs ×3 and `SliverVertex` share `splitting::split_lever_ending` (lever alone; the unreadable-margin note only on poison); `SectionError` refusals share `SECTION_PLANE_RECOURSE`; `SectionWindingUndecided` uses one wording. Join endings: `SplitJoinError::Escalated` ends `diag.ending(JOIN_RECOURSE)` (the same ending geom-brep gives the decision); `OrderEscalated` picks by predicate (`order::FRAME_AXIS`/`FRAME_ARM` take the lever alone, order_u/v are sized); `RingHoming` leads 're-homing a hole loop: ' and ends `ContainDecision::Loop(..).lever_ending` (no tolerance; follows refusal_routes' Placement ruling). `UnderBoolean` is removed and `BooleanError::Join` prints the join's own message. Left open on `split-escalations-end-a-poisoned-margin-in-the-plane-lever`: `CrossingEscalated`, contradiction-minted poison, the `pc_axis_plane_parallel_straddle` 'subdivide' vs lever-alone straddle clash, and `chord_arc_leave_germ`/`_section` (unsized, but offer the tolerance). (ENCL orchestrator)
- 2026-10-12 — PR 4725 merged (missed-ray side, with the inside-out operand refusal). Its gate was red only on INTENT's 1e-12 loft-section row, which is also red on main; the row was closed in the PR. Dispatched P2 `an-in-band-certification-of-a-minted-edge-other-than-its-face-residual-reads-as-a-defect` → `cleave/minted-edge-inband-cert`.
- 2026-10-12 — Design fork opened for `a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims`: whether the one-carrier arms' boundary reach (`BoundaryReads::LinesAndCircles`, held by D10) still holds now that glue-on-Zero (PR 4496) sends undeclared, value-decided pairs through them. Two designers dispatched concurrently with one problem statement and no candidates. The blinding record is on `analysis/design-fork/cleave-tilted-plug-reach`.
- 2026-10-12 — Seam note from ENCL (PR 4756, merged at `730e342025`): `JoinUndecided`'s own Display (read only at the Boolean) now asks one question per reading through `Indeterminate::undecided`. Regularity uses `JOIN_SUBJECT` with `diag.ending(COINCIDENCE_RECOURSE)`; the chart class uses its own `ISO_FAMILY_CLAUSE` question and ends lever-only 'move the geometry' (a form selection, per recourse.rs's Unsized). The `Image` arm still ends with no recourse; it is pinned as `a_chart_image_that_did_not_derive_ends_without_a_recourse_today` and filed on HONE as `the-join-lever-names-a-pole-for-a-chart-class-reading` (P3), with `JOIN_LEVER` on the chart class and the hand-spelled shapes in `JoinRefusal` and `JoinUndecidedAtRest`. (ENCL orchestrator)
- 2026-10-12 — Tilted-plug reach fork: both designers agree it is NOT a fork. The "D10 hold leaves their reach" sentence was agent-written (PR 4174), and Ev's hold covered units using declared pairs, which stage 4 F retired, so these are D10's own glue arms. Recommendation: delete `BoundaryReads` and read ellipses in both roles (the swept metre and `meetings` arms plus the boundary). Measured on prototypes, ∩ and both ∖ then build at closed form and nothing else moves. No ratified text changes, so there is no `[ev]` PR and no fork-log row. Dispatched the implementation (`cleave/one-carrier-reads-ellipses`). Filed P2 `a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters` for ∪'s remaining frontier.
- 2026-10-12 — PR 4773 review: APPROVE-WITH-FIXES. All 21 pinned in-band residuals are the input's: Line carriers, the vertex at the band edge, the carrier one ulp past. But a dense cylinder census finds amplified ellipse rounding in band at 1e-12, refused upstream by the join today. Fix pass sent: map only Line carriers, at both the split and the Boolean (4625's `of_attachment`), from one shared home; give the lever angular clearance; correct the join row's framing; style.
- 2026-10-12 — PR 4782 (one-carrier reads ellipses) went up and is in full review. Its CI showed an unowned main red at 1e-12, `editor-core coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven` (from FLUSH's PR 4728), which reproduces on clean main. Filed it as FLUSH P1 `a-ring-serving-two-pierces-fails-to-evaluate-at-1e-12-on-main`.
- 2026-10-12 — PR 4773 merged (minted-edge in-band certification). The fix pass, narrowed to Line chords with one rule home for both doors, was accepted on the lane's report; CI was green. Row closed.
- 2026-10-12 — Design fork opened for `a-keel-in-the-plane-wrapped-round-at-both-ends-refuses-a-degenerate-section`: how to represent a section whose trace is a closed zero-area loop (Above's slit island), and whether D1's tiers admit it. Two designers dispatched concurrently with one problem statement. The blinding record is on `analysis/design-fork/cleave-keel-wrapped-both-ends`.
- 2026-10-12 — Keel fork: both designers recommend the slit form, with Above's keel as a zero-area ring of two coincident distinct edges. They also find that the boolean builds the same body in the rider form, a reflex edge lying in the face, and that a wall-to-wall trench returns one solid against D1 / PR 3901. `[ev]` PR 4795 generalises D1's #131 sentence: two wedges on one line are coincident distinct edges, no face passes through an edge's wedge, and the ring is stated. It also adds fork-log row 121, and the keel row is `needs_ev`. Filed, parked on 4795: `a-boolean-builds-a-reflex-edge-lying-inside-a-face` (P1, cleave) and restread's `census-passes-a-face-inside-an-edges-material-wedge` (P2). Filed open: `a-split-slit-self-touch-has-no-record-and-no-3-prime-pin` (P3).
- 2026-10-12 — Dispatched P2 `a-section-curves-end-read-in-band-of-its-vertex-refuses-as-a-kernel-defect-at-the-join` (lane `cleave/section-end-inband-join`). The job is to separate the input's in-band endpoint pin, which gets typed `Escalated` with its lever, from the conic's amplified rounding, which stays a Q1 kernel defect. The rule home is reused from 4773's `edge_on_faces_reading`. If no reading the input owns can tell the two apart, it is a design fork.
- 2026-10-12 — PR 4782 full review (head 2a44beda13): APPROVE-WITH-FIXES with one MAJOR. The ellipse×ellipse arm answers an in-band "parallel" pair as meeting nowhere without checking `off_plane`. On a cylinder the plane→section map stretches by a/b, so two sections whose planes read parallel in band can lie up to (a/b)·zero apart and still cross. Witness: z = 100x against z = 100(1+d)x, swept `Clear` across a real crossing, masked downstream by a refusal. The root is style S1: the hand-copied mirror arms have drifted. Fix pass sent: mirrors delegate by swapping arguments; distinct planes are decided by `off_plane`, and a pair in one plane is decided as one curve or not; the metre and lever get pins that a swap or a 2× overstatement turns red; the speed list goes to one home. To file: the origin-anchored `common_line`, the rate-pair rule against the local metre, and ellipses at `split_other_at_point`. Mutation table and dense oracle are otherwise clean; no regressions against main at any ε.
- 2026-10-12 — PR 4810 (section-end in-band join) went up and is in full review. The join re-reads an escalated endpoint pin with the vertex moved onto the conic's plane, using `split_vertex_side`'s formula. A Zero read types the refusal `SectionEndEscalated { OnPlane | SquareToAxis }`; anything else stays the gate's Q1 defect. The rule home is `edge_on_faces_reading`, gaining an endpoint arm. Filed: cleave `a-section-read-square-at-its-band-edge-refuses-downstream-at-1e-12` and flux `a-slab-face-nearly-along-a-cylinder-ruling-exhausts-the-volume-quadrature`. Its CI also showed a new, unowned main red: `mass_props_are_thread_count_invariant` sym `frozen` counts 1065/1181 against the golden's 1060/1177, with both NURBS digest rows closed. A main-red lane is tracing it (`cleave/main-red-sym-frozen-digest`).
- 2026-10-12 — The main-red sym `frozen` digest is resolved by NURBS's PR 4814 (`5e46b73af5`), which re-cut it while the lane was bisecting. The breaking merge was #4803, the speed-meter memo. The cache is filled at build time, outside any session, so the session freezes 5 / 4 more nodes. That is a correct consequence, so the re-cut is right. Filed P3 on nurbs: `the-speed-meter-memo-is-visible-to-a-sym-session` (the "invisible" claim is not true in Sym). PR 4782's fix pass is done (`d88f865dd1`) and in narrow re-review.
- 2026-10-12 — PR 4810 review: CHANGES with one MAJOR. The re-read decides Zero on the carrier's residual against the moved vertex, and that residual contains the near-axial ellipse's own rounding (1–7ε at 1e-12). So an amplified kernel error is typed `Escalated` at both doors whenever a vertex sits near the band edge. The typed/untyped verdict flips with the sign and size of the vertex offset, and the lever does not build. This breaks PR 4335's rule. Fix pass sent: separate the two populations by a bound on the carrier's own error from its construction, not by its residual; otherwise stop and report a design fork. Also sent: the square arm's plane, the remaining rim population at every ε, the SurfaceResidual second check, the test premises that exclude the failing mode, and filing for the disclosed residue.
- 2026-10-12 — PR 4810's fix pass stopped at a design fork, which is correct. No reading of the vertex or the plane alone separates the input's band-edge offset from the near-axial ellipse's amplified rounding; only the carrier's own error differs. Any sound rule needs a bound on the section conic's construction and evaluation error, and every way to get one is a choice. The same quantity serves GERM's `conic-section-just-past-its-band-fails-certification-downstream`. Two designers were dispatched on the problem statement, with no candidate solutions given; the blinding record is on `analysis/design-fork/cleave-section-end-carrier-bound`. PR 4810 holds (head 416ed72fe3; M1 live, do not merge).
- 2026-10-12 — Section-end fork, first reports. Both separate the populations at the section table, where the curve is minted, not at the join. Once a curve is admitted, every in-band join pin is the input's and is typed without a re-read; PR 4810's re-read goes. They differ on five points:
  - A re-poses the ellipse (seam at the nearest principal vertex, signed span). Its prototype builds the review's coin-flip family and all through-axis planes at 1e-12. B refuses those at the arm and calls re-posing an unproven alternative.
  - A admits a resolution ≤ ε; B admits ≤ ε/K.
  - A types the residual refusal as a kernel limit; B gives it a tilt lever.
  - B proposes a derived `Curve3::resolution(at)` as the rule's one home.
  - B puts the split's δ/sin ψ vertex reading in the final state; A lists it as a separate row.
  Reconciliation round 1 is sent: each designer is shown the other's report.
- 2026-10-12 — PR 4782 narrow re-review: MERGE. The one-curve margin read Zero on 0 of about 800 pairs per ε that were more than 1.5z apart in a dense Hausdorff fuzz. The m = 100 witness no longer reads `Clear` across a crossing. The mirror delegation keeps generation complete, every mutation that matters turns red, and there are no regressions against main. Its notes are filed as P3 `carrier-cross-one-curve-reads-tilt-apart-and-conic-helper-copies-conic-of`: the tilt is composed from a separate Zero, and the fix minted one fresh `Conic::of` copy. I merged main into the PR (conflict in nextest.toml; clippy on topo and sweep is clean on the merged tree) and it is waiting on CI.
- 2026-10-12 — Section-end fork, round 1, B: moved to A's re-pose; B's own replication shows the parameter term goes to 0 down to θ = 1e-6. B merged the error terms: what remains is the far centre, u·|centre − face|. B moved to frontier typing under D9 row 2 with D4 ¶1(i)'s ending, and no tilt lever. B holds ε/K over ε, on the join's typing promise. B merged the δ/sin ψ vertex reading as its own row, named in the final state. Waiting on A's round 1.
- 2026-10-12 — The section-end fork converged after round 1, with no ratified text changed, so it does not go to Ev and gets no fork-log row. The converged design:
  - re-pose the section ellipse (seam at the nearest principal vertex, signed span), which removes the parameter term;
  - admit at the section table on `Curve3::resolution` ≤ ε/K, and refuse the far-centre term as a kernel limit (D9 row 2, D4 ¶1(i), no lever);
  - at the join, every escalated attachment pin on an admitted curve is the input's, typed by provenance with no re-read;
  - the split reads a vertex against its section (δ/sin ψ), not against the plane.

  Filed: units `a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex` (P2), `the-section-table-admits-a-curve-on-its-resolution` (P2, parked on the re-pose) and `the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane` (P2), plus germ's P3 `circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band`. A note went on GERM's conic row (FLUX's quadrature row is still on 4810's branch; the re-pose unit re-measures it), and seam notes to CURVED, GERM and PCERT. PR 4810 is reworked on top of the first two units.
- 2026-10-12 — Dispatched the re-pose unit to the 4810 lane (`cleave/section-ellipse-re-pose`; then admission, then the 4810 rework, in order). Dispatched the vertex-against-section unit to a new lane (`cleave/vertex-against-section`).
- 2026-10-12 — PR 4782 merged at `8790d3db44`, after the narrow re-review said MERGE. Its last run was red only on FLUSH's known 1e-12 `coincidence_door` row; the PR comment says why. Main had moved only in nextest.toml since the merge-in, and the merged tree passes clippy on topo and sweep. Rows closed: `a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims` and TANG's `an-ellipse-lying-on-a-wall-keeps-the-door` (the PR built it). Dispatched P2 `crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync` (`cleave/crossing-cylinders-join`) and P2 `a-rod-rim-across-a-tube-ellipse-union-is-undescribed-in-one-order` (`cleave/rod-rim-union-describe`). `a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters` waits for the ellipse re-pose, because both touch `chord_join`'s section copy. Note for FLUSH: four rows now cover the one `coincidence_door` 1e-12 red.
- 2026-10-12 — PR 4856 (rod rim across a tube's ellipse) went up and is in full review. `describe_edges`' stale-curved lane handled circles only. In `t ∪ r` the union keeps the tube's wall whole, which leaves the ellipse arc stale, so that order refused `NoConventionalLane`. The fix restates every kind as a derived image in `s1`'s chart, carrying its authority unchanged; the old circle and line lanes had stamped a fabricated `Declared`. Its CI is red only on FLUSH's 1e-12 `coincidence_door` row.
- 2026-10-12 — Seam note from ENCL (PR 4855, merged at `5903a8c5a2`): `geom_brep::recourse::SizedDecision::recourse`'s Undecided arm takes the decision's `at_zero` note when the cause carries a gate-rejected Zero, sharing the Zero arm's body. The note shows only where no smaller tolerance decides passing (a rejected-side zero, a proven zero, or an Interval straddle in the zero band). Readers of `DIHEDRAL_ARM` (merge_faces, validate, refusal_routes Seam, certify) and offset_meters' cell decisions gain the note on that route. (ENCL orchestrator)
- 2026-10-12 — PR 4863 (crossing cylinders) went up green and is in full review. `kind_arm` now reads the cylinder pair's axes through the frame dispatch's own decision, `cylinder_pair_axes`. A definite crossing goes to the edge-plane lane, and an in-band one is `Escalated`. The scene then refuses one step on, at `chord_join::along_edge_spec` ('only lines and circles are copied'). That is filed as `the-edge-plane-lane-copies-no-ellipse-edge` (P2) on the PR's branch. It waits for the ellipse re-pose, which holds `chord_join`. Lane clones from merged work were removed to free disk (about 8 GB free).
- 2026-10-12 — PR 4858 (vertex against section) went up green and is in full review. `vertex_section::decide_vertex_on_plane` is now the one home, used by the split and the Boolean's three sites. An offset read Zero is read again at δ/sin ψ under `vertex_section_side`. Three rows were filed. The review is asked to look hard at three things: a CI-driven exclusion (a face beside a conic edge that reads parallel), the 'shallowest face' rule, and the δ < 0 definite side, which now refuses `UnpairedLooseEnds` at the join.
- 2026-10-12 — PR 4856 review: APPROVE-WITH-FIXES, no MAJOR. Every stale ellipse certifies against `s1`'s chart; the image is held to the pcurve map residual. Declared → Derived is an improvement: the old record was made up, and `EdgeAuthority`'s docs count kernel images as `Derived`. It does change `replace_face` outcomes (a half rod ∪ rod offset now builds), and no test can see the difference. Fix pass sent: pins for that consequence and for the stale-line path; one home for the 'restate in chart' idiom the fix copied; a correction to DESIGN's `TransverseNotIntrinsic` exemption, which an agent editing pass (99cc678bfd) had inverted against Ev's 2026-07-19 text and the code; doc fixes; and rows for the unreachable spiric/NURBS lanes and the scaffold `Declared` path.
- 2026-10-13 — PR 4863 review: APPROVE-WITH-FIXES, no MAJOR. Outside the new fixture, every kind-arm reading in the topo and sweep ci profiles at the default ε and 1e-12 is parallel; the fixture's 24 runs are the only APART readings, so no outcome moved. The routing is right: a crossing EF germ's edge must be an ellipse. Fix pass sent for the gaps the mutations found: the join's in-band arm and its one-reading lever are unpinned (M3a and M4 stay green); the fixture pin accepts a kernel-invariant ending with no ratchet; and there are stale docs, a double destructure, and FF germs reading their axes twice. Usage limit hit 2026-10-12 about 20:00; all four lanes were resumed on 10-13. A diagram of the keel geometry is posted on [ev] PR 4795 at Ev's request (branch `analysis/diagrams/cleave-keel`).
- 2026-10-13 — PR 4858 review: CHANGES. The core fix is right and the two readings never disagree in sign. But the section reading divides by sin ψ with no bound on its own rounding, so at small ψ it offers the kernel's amplified rounding as a `SliverVertex` lever. That breaks PR 4335 and is DESIGN's Q1 class again: 15 of 25 runs at 1e-9 with ψ = 1e-7, and 13 or more at 1e-12. It also reads a face whose section is never minted, so a supporting plane touching only at the vertex refuses where main builds. Fix pass sent:
  - bound the reading's rounding, admit it as the input's only at ε/K, and end the rest as a kernel limit with no lever, in one home that part 2 reuses;
  - take the face set from where the join decides it;
  - add pins the mutations showed missing (min vs max ψ, the exclusion, exact Off endings);
  - correct the mirrored row, which is a live definite defect: raise it to P2;
  - fix the docs, and type or justify the silent fallbacks.
- 2026-10-13 — PR 4863 fix pass done (`de9a48d229`, green). Added an EF door pin that reaches the in-band arm and turns M3a and M4 red. The fixture now asserts each op's exact ending, with the row id. `segment_arm` is computed for every segment before surgery, at the frame's own lever. Filed P3 `an-in-band-axis-parallelism-offers-a-proximity-lever`. Narrow re-review resumed.
- 2026-10-12 — Seam note from ENCL: PR 4874 merged (`encl/fitted-escalated-keeps-decision`).
  - `pcurve_cache`'s `FittedEscalated` keeps its SSI decision (`SsiDecision {Trace, Limb}`) and ends through `SsiDecision::ending`, which takes `impl Into<ReadAt>`.
  - Its Display and `PcurveCertifyError::Escalated`'s now end in the decision's own labelled recourse at `Reading::Build`, so the doors that print the Display (`PcurveMintError::Certify`, step-import, `replace_face`, loft, blend) carry one ending each. Both texts are pinned literally.
  - `PlaneNurbsRefusal::ending` ends `Escalated` through `SsiDecision::Limb`.
  - Rows filed:
    - PCERT `a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect`;
    - CLEAVE `chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid`;
    - `work/issues/describe-maps-every-chart-pcurve-escalation-to-chart-image`;
    - ENCL `unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word`.
  - Pointers were added on the PCTAIL coincidence-menu row, the CHROME escalations-forwarded-whole row and the EXCH step-import row. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4925 merged. ε now has a floor: geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km and `EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m, per D4 ¶4 (Ev, PR 4836).
  - Every door that sets a run's ε refuses below it. `tolerance::EpsFloor` is the one wording of the refusal.
  - `Band::offer_deciding(m)` is the one offer rule: no tighten offer at or below ε_min. Tests that predict an offer should call it rather than restate `|m|/K`.
  - At CI's 1e-12 row, every zero-band offer now withholds.
  - A run below the floor (for example `CAD_TOLERANCE_EPS=1e-14`) is refused. Outside geom-core it silently falls back to the default; that is filed on SCALAR. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4943 merged. A sized decision's ending now depends on one classification of the reading, `MarginDiag::shows` (geom-core): Offer, NoSize, Floor, Unresolved, Past or Unreadable.
  - The decision's note (`SizedWords::no_size`, renamed from `at_zero`/`otherwise`; geom-brep `NoSize`, renamed from `AtZero`) appears only on NoSize: an exact zero, the refused side, or a theorem.
  - A positive reading below the floor gets the one `floor_note` sentence.
  - A reading that touches or straddles zero gets the lever alone.
  - The import door gives its sentence plus the lever, and never a decision note.
  - PR 4855's rejected-sign filter is gone.
  - New sized decisions take `no_size:`.
  - Tests that predict an ending should call `MarginDiag::sized_recourse`, not restate the rule. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4766 has merged. It covers the sweep slice of the undecided-wording sweep (sweep crate endings by decision kind), and geom-core now has `Indeterminate::lever_ending` as the one lever-only-at-build ending. Lever-only decisions (angles, cos/sin, winding, form selection, gauges, walks) end through it. Do not compose `lever_recourse` + the unreadable note by hand. (ENCL orchestrator)
- 2026-10-13 — PR 4863 narrow re-review: MERGE. Mutations M1–M5 go red where they should. The hoisted read was instrumented over about 16k reads per ε row: it never differed from the old post-surgery read, and no outcome moved. Filed P3 `boolean-join-resolves-germ-face-surfaces-in-three-closures`, the copy the fix minted. Merged main (192 commits) into the PR. Main's new `BooleanJoin` lift broke the pin's pattern, so I adapted it through `.error()`. Clippy on topo and sweep is clean and the pins pass; waiting on CI. PR 4856's fix pass is done (`d8dbca1190`, green) and in narrow re-review. It includes the DESIGN.md `TransverseNotIntrinsic` sentence restored to Ev's 2026-07-20 ratified text (b846df1f10); the inverting sentence came only from agent edit 99cc678bfd.
- 2026-10-13 — PR 4858 fix pass done (`302460facd`, green).
  - M1: δ/sin ψ now carries a running bound ρ (`Rounded::quotient`, moved to `geom_core::running`). It is the input's only at ρ ≤ ε/K (`geom_brep::recourse::UnsupportedConditioning::admit`); otherwise it is the kernel-limit `SectionUnresolved`, with no lever. Part 2's row is told that this is the shared home.
  - m1: faces entered come from the walk's own `entry_side`/`bisector_side`; the parallel-edge exclusion is gone.
  - Pins and docs fixed; the fallbacks are typed or justified.
  - Residue: the Boolean reads entry through the split's `entry_side` beside its own `side_code`, two spellings of one question. To be filed after the re-review shapes it.
  
  Narrow re-review resumed.
- 2026-10-13 — PR 4863 merged at `5d15bdc231` on a green head. Clippy on topo and sweep was clean against current main before merging. Row `crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync` closed. Its residue, `the-edge-plane-lane-copies-no-ellipse-edge` (P2), waits for the ellipse re-pose, which holds `chord_join`.
- 2026-10-13 — PR 4861 (ellipse re-pose, unit 1) is up, green at `a2e3c1614b`, and in full review. The lane merged main three times; main's spiric chord arm (4927) takes an oval as it stands. A main test caught a near-circle whose minor axis rounds one ulp above its major, which is fixed with `Curve3::section_frame(axis)`. A second sweep found two more ordered readers (blend `end_face`, `ring_path`). Re-baselined: seat7 `cut_cylinder`/`boss_union`, `cut_cylinder.expect`, and the thread-count digest's `tilted_cut_upper` pads (narrower). Part 2 (admission) is next on that lane once the review settles.
- 2026-10-13 — PR 4856 re-review: FIX-FIRST, narrowly. The mutants discriminate, `restated_in_chart` is honestly the one spelling, the `finish.rs` merge resolution is right, and nothing regressed. But the restored DESIGN.md sentence says more than b846df1f10: read as exhaustive, it would let a derived chart on a transverse edge pass, which is restfront's open question. It must be reworded so it decides nothing new. Also asked for: a stale line asserted derived (the old chord mutant stays green), and doc fixes. Narrow fix pass sent.
- 2026-10-13 — Correction. Ev decided GERM's conic-past-band fork on PR 4812 on 2026-10-12, in parallel with our section-end fork, and a GERM seam note on our part-2 row recorded it. I missed that note, and two lanes built against the superseded admission shape. The decision: the anchored conic, with the centre derived, as the end state; no refusal at the arm; in the interim, a miss within a carrier's own resolution ends as a kernel limitation and nothing new refuses; the refusing check is deferred to D4 ¶4. Re-scoped accordingly:
  - part 2 becomes the anchored conic (H, still after the re-pose);
  - part 3 (PR 4810) carries the interim relabel at the join;
  - part 4 (PR 4858) must refuse nothing new.
  PR 4858's re-review is FIX-FIRST on exactly this. Its `SectionUnresolved` refuses poses main builds tier-3-valid: 6 of 7 at ψ = 3e-4, and all at ψ = 1e-4 and 1e-5, at 1e-9. Fix pass sent. Lesson: before building a design, re-read the target rows on current main for seam notes.
- 2026-10-14 — PR 4856 merged at `a1b214d8d7`. Its last run was red only on three main reds, all filed: FLUX's thread-count golden, and NURBS's saddle-wall fit and lily tessellation table. Main had moved only in `certify.rs`, and the merged tree passes clippy on geom-brep, topo and sweep. The DESIGN.md `TransverseNotIntrinsic` sentence is restored to the meaning of b846df1f10 without claiming to be exhaustive; the derived-chart question stays on restfront's row. Row `a-rod-rim-across-a-tube-ellipse-union-is-undescribed-in-one-order` closed.
- 2026-10-14 — Dispatched P3 cleanups that stay clear of the ellipse and vertex fences:
  - `boolean-join-resolves-germ-face-surfaces-in-three-closures` and `an-in-band-axis-parallelism-offers-a-proximity-lever`, together on `cleave/join-face-surface-one-home`;
  - `carrier-cross-one-curve-reads-tilt-apart-and-conic-helper-copies-conic-of` on `cleave/carrier-cross-one-curve`.
- 2026-10-14 — Ev asked on [ev] PR 4795: 'how is the top of a convex wedge represented?', then 'what happens if we glue a plane to its tip?'. Answered: a convex top is one edge, and a tip touching a face is one edge in edge-on-face contact (two solids by D1/3901 unless joined elsewhere). The second question showed the D1 sentence was too broad, because a face's own 180° side is no wedge. Narrowed it in 7e245eea9b to 'two material wedges, each bounded by two faces at an angle', adding that 'an edge touching a face from outside its wedge stays one edge, in contact with that face'. PR 4858's second fix pass (`efa3830bc7`) brings it in line with Ev's 4812 ruling: an unresolved reading keeps the offset's verdict, so nothing new refuses. Its CI is red only on the two filed main reds. Short re-review resumed. Filed: P1 `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body` (a pre-existing invalid Ok body) and P3 `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`.
- 2026-10-14 — PR 4861 review: CHANGES. The design holds: the parameter miss falls about 1000×, no old-convention reader survives in the kernel, and the re-baselines are justified. But at 1e-12 the at-rest gate refuses 64 of the 162 claimed-building poses with `UnrepresentableCurveDatum`. Tier 3's frame margin reads ε − δ·A, and the unnormalized turned seam worsens it, while the pins only assert that split returned Ok. The public `Pose::u_ref` readback now returns the minor direction for re-seamed chords, and the FLUX pose credit is wrong (it already builds on main). Fix pass sent:
  - normalize the seam;
  - validate the pins at rest;
  - derive the major direction at readback;
  - give 'larger semi-axis' and the quarter turn one home at the curve;
  - rename the remaining `major`/`minor` names;
  - pin the tie-break;
  - correct the PR body;
  - close restfront's minor-over-major row;
  - file the tier-3 frame-margin residue on its owner (Ev's 4812 interim).
- 2026-10-14 — The container restarted.
  - Resumed: the two P3 lanes (join.rs, and carrier_cross).
  - Lost: the 4858 short re-review and the 4861 fix-pass lane.
    - The 4858 re-review was re-created.
    - The 4861 lane was re-created from the recovered fix-pass brief. The old lane's clone was mid-merge of main, with no fix-pass code yet; the new lane chooses whether to finish or abort that merge.
  - No new Ev comment on 4795.
  - Noted the new CLAUDE.md rule: a blocking question about an open PR goes on that PR, with `[ev]` added to its title and the question at the top of its body.
- 2026-10-14 — PR 4858 third review: MERGE. Head's output is byte-identical to main's on 222 witness rows across three ε. The one newly refusing row in the full ψ×δ ladder sits exactly on the zero band's edge, and is the intended in-band refusal with a lever. No lever is offered on an unadmitted reading, `plane_offset_rounded` is bit-identical at every door, every mutation goes red, and nothing regressed. Its MINORs are filed as P2 `vertex-section-quotient-has-no-room-precondition-and-four-offset-spellings`: the quotient has no room>0 precondition, the sine charge is asserted rather than derived, and offset/side is spelled several ways. I merged main into the PR (197 behind, splitting files touched): clippy is clean on geom-core, geom-brep, topo and sweep, the 22 pins pass, and the merge is pushed (`d938ebc42c`) and waiting on CI.
- 2026-10-14 — PR 4858 merged at `0adcbf0ea0` on a green head; main had moved 35 commits, none in its files. Part 4 (`the-split-reads-a-vertex-against-the-section-it-ends-not-the-plane`) is closed. Now on main from it: P1 `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body` (a pre-existing invalid Ok, also on main), P2 `vertex-section-quotient-...` (on the tracker branch), and P3 `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways`.
- 2026-10-14 — Dispatched P1 `a-split-along-a-seam-at-a-vanishing-tilt-returns-an-invalid-body` (`cleave/seam-split-invalid-ok`). The split's result gate passes halves its own `validate_geometric` refuses. Two separate questions: why the gate accepts them, and whether the near-axial ellipse's amplified error should end under Ev's 4812 interim kernel-limitation ending. The lane measures on main and on 4861's branch.
- 2026-10-14 — PR 5058 (one-curve margin) went up and is in full review. The margin is now `|Δc| + ‖L_e − L_f·Q‖_op`, with Q an up-sign flip plus a Procrustes turn. It bounds Hausdorff outright, and measured margin/Hausdorff is 1.00–1.54 (the old margin under-read tilt to 0.24). The speed arms read once through `Kind::of`/`Conic::of`. There is a new body pin for the transverse ellipse×ellipse arm. Its CI is red only on the three filed main reds.
- 2026-10-14 — Keel PR 4795, Ev's questions continued. Ev saw the keel and the glued tip as one picture flipped, with material and void swapped. Answered: four sheets meet at the line, and the material forces their pairing. Ev then asked whether the other pairing could be used instead. Answered: it is forced given an edge's meaning, and the real alternative is a non-manifold radial edge. Ev asked for radial edges to be weighed 'for completeness'. A designer pair has been dispatched on the same problem statement; the blinding record is on `analysis/design-fork/cleave-radial-edges`. Results go into 4795's body as A and B, with a new fork-log row.
- 2026-10-14 — PR 5061 (join.rs: one germ-face resolver, and an angle lever for axes read parallel in band) went up and is in full review. Its CI is red only on NURBS's lily table. Filed: P3 `the-germ-frames-in-band-angle-rungs-offer-a-proximity-lever`.
- 2026-10-14 — Radial-edges fork, both reports in. A and B agree: keep the manifold model with the forced pairing, restate D1 as one principle (the material decides the edges, the sectors are disjoint, the complement re-pairs, tier 3′ reads sides), and do not adopt radial edges. Their reasons: the asymmetry is honest (manifold bounds the material, radial the surface; PR 3901 chose the material), radial still needs the ring, and radial means replacing the topology kernel. No reconciliation was needed. Each raised one extra text change: A, qualify 'legal together or not at all'; B, the agent-written 'genuine non-manifoldness is unrepresentable'. Both are put to Ev separately in 4795's body. Rows 141 and 142 are added on the ev branch after merging main (row 121 collided, renumbered to 141). My 'radial needs no ring' was wrong, and it is corrected on the PR. Filed P1 on topo, `reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier` (reachable via ∖ with 4622's slit). Notes added to restread's census row and the keel row.
- 2026-10-14 — TOPO's P1 `reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier` dispatched from CLEAVE (TOPO has no orchestrator, and the slit form is CLEAVE's). Branch `cleave/revert-repairs-coincident-edges`; measure first.
- 2026-10-14 — PR 4861 fix pass pushed (`014d0a4177`). Its CI is red only on NURBS's lily table, which main's push CI skips (demos). Changes:
  - the seam is normalized, and the pins validate at rest;
  - at 1e-12: coin-flip 66/66 valid; through-axis 64/96 valid and 32 refusing on tier 3's frame margin; off-axis 38 refusing the same way;
  - one regression remains (`axial w0.7 φ3.9 θ1e-4`);
  - the public `edge_pose` u_ref is read through `major_frame`;
  - `semi_axes_by_size`/`quarter_turned_where` are the one home; the `Conic` fields are renamed; seat7/8 history is cut.
  Filed: RESTFRONT P2 `tier-3s-frame-check-reads-a-section-ellipses-rounding-amplified-by-its-semi-axis`; CLEAVE P3 `a-split-leaves-an-in-band-sliver-wall-face-that-refuses-only-at-rest` and `a-joined-section-arc-runs-its-kept-carrier-without-re-seaming`. Closed RESTFRONT's minor-over-major row. Narrow re-review resumed. The render lane's `render_gui.py` int-parse failure is main's, and CIW has it filed twice.
- 2026-10-14 — PR 5061 review: APPROVE-WITH-FIXES, no MAJOR. Across 240 probe runs the 'clearly not parallel' half of the new lever never builds, so the lever is to name only the parallel side (the house convention: `RIM_LEVER` names both sides because both pass, `WallRoots(AxisParallel)` only its passing side). The two new loud refusals are unreachable; nothing that built now refuses. Fix pass sent:
  - one-site segments still re-read their frame after surgery, contradicting the module doc;
  - the `carrier_cyl_axis_parallel` subject is a near-twin wording of the new decision;
  - the visible-tilt endings carry ε-sized margins (real coincidence or amplified error?);
  - a fresh surface closure in `choose_roles`, out of a class of about 19 inline chains across boolean/;
  - unify the FrameError variants;
  - docs.
- 2026-10-14 — PR 5058 review: APPROVE-WITH-FIXES. The new one-curve margin is proved a Hausdorff upper bound (adversarial worst H/margin 0.999999), and the k<1 re-baseline is right. Found a sibling of the same defect class: `parallel_circles` decides one circle from three separate Zeros and never reads the radius, so a great circle against a section tilted 0.9ε certifies Clear across an in-band crossing (arithmetic, not yet run). Fix pass sent: fix the sibling in this PR with the shared displacement margin; pin the body rows exactly; correct the filed row's backwards fix (Zero is definite, so the refusal is a frontier, not a lever); remove the fresh double match in `Kind::of`; one home for the semi-axis frame.
- 2026-10-14 — PR 5100 (seam split invalid Ok) went up and is in full review. The split never ran tier 3: it gated with `validate_closed` only. It now runs the Boolean's `AtRestPolicy` per side and refuses with `SplitFinishError::ResultNotAtRest`, carrying the validator's findings verbatim. The witness family's invalid Oks become refusals at 1e-9 and 1e-12; 1e-6 is unchanged (all valid). The invalidity is the far-centre ellipse's chart image, which only Ev's 4812 anchored conic fixes. Filed on CARVE: the sweep doors gate only in debug builds. The review checks that nothing that builds valid today refuses, and what tier 3 costs per split.
- 2026-10-14 — PR 4861 narrow re-review: MERGE.
  - M1 closed honestly. The at-rest tables match an independent probe exactly at 1e-12 (coin-flip 66/66 valid; through-axis 64/96), and the 1e-6 and 1e-9 tables pass on main.
  - The remaining regression, `axial w0.7 φ3.9 θ1e-4`, is a far-centre coin flip of limit (b): main builds it with a 9.1e-13 miss, under ε by a hair. Off-axis builds rose overall (58→81 at w0.3, 60→69 at w0.7), and Ev's 4812 anchored conic removes the term.
  - Every mutation goes red. The public `u_ref` readback is deterministic.
  - Filed P3 `semi-axes-by-size-misses-seven-inline-sites`: a half-fix of the class, since the sweep missed locals named a/b.
  - The re-review also found FLUX's `main-red-the-mass-props-thread-golden-since-4938` stale at the merge base: #4952 re-cut it.
  - Merging main (236 behind) into the PR; a workspace clippy is running on the merged tree because of the field rename.
- 2026-10-14 — PR 5058 fix pass is green (`e86336714b`). `parallel_circles` now decides one circle with the shared `one_curve_gap`, and the witness is pinned at unit level only, since no door reaches it. Pins assert exact endings and bitwise symmetry. `Kind::of` matches once, and `meetings` reads every conic arm from `Conic`. Filed: flux `frame-read-off-ellipse-fields-in-curved-props`, and cleave `a-ball-cut-by-a-plane-turned-epsilon-escalates-arc-ahead`. Narrow re-review resumed. PR 4861: I merged main (236 behind). Main's new `flat_cone_tilted_ellipse.rs` still used the old fields, so I adapted it to `semi_u`/`semi_v`. Workspace and demos clippy are clean, the pins pass, and it is pushed (`acd896a622`), waiting on CI. 4861 and 5058 overlap on `Conic`, and the second to land reconciles.
- 2026-10-14 — PR 5061 fix pass is green (`3fb534e707`). The lever names only the passing side; the frontier is pinned exactly at 0.2 rad across three ε; frames are read before surgery; `FrameError` has one `Escalated`; and join.rs has one face-to-surface walk, enforced by a census test. Filed (P3): `boolean-walks-face-to-surface-inline-at-two-dozen-sites` and `tipped-seams-pose-reads-eps-scaled-margins-at-visible-tilts`. Narrow re-review resumed. Main's lily red is fixed by #5102. PR 4861: main moved in 19 of its files after CI went green, so I merged main again; clippy is clean, and it is pushed (`3d9c62584b`).
- 2026-10-14 — PR 5100 full review: APPROVE-WITH-FIXES. Nothing valid changes outcome (8940 tests per ε; the only hits are the pin and one lever-only grazing test), and the witness family reproduces exactly. Cost: median 0.35 ms per side, but up to 5 s on the pin's quadrature poses; editor `finished_operand` re-gates every split side. Fixes sent to the implementer lane:
  - pin the exact variant;
  - one Display and one python tag shared with the Boolean;
  - route through the Boolean's in-band typing (D4), or stop and report;
  - a `ResultNotAtRest` arm in 4861's seam table if 4861 lands first;
  - stale docs;
  - raise `split-sides-are-not-finished-bodies` to P2 for the double pay;
  - correct the CARVE row's two false claims and add its measured extrude witness (R 200–400 m arc, 4/20 at 1e-12);
  - make sure the RESTFRONT row reaches main;
  - probe `transform_rigid`.
- 2026-10-14 — PR 4861 (the re-pose) is merged (`1d6c939490`), green on `3d9c62584b` after a second merge of main. Its row is closed, and the anchored-conic unit (`the-section-table-admits-a-curve-on-its-resolution`, P2/H) is unparked to open. The part 3 rework of PR 4810 is dispatched: an escalated section-end pin is read against its carrier's per-kind resolution, so a miss the bound explains ends as a kernel limitation with no lever, a miss it can't explain stays the input's with its lever, and nothing new refuses. 5058 and 5100 are told that 4861 is in.
- 2026-10-14 — The topo P1 `reverting-a-slit-body-…` is up as PR 5106 (`a2948a17dc`, gate ok). `revert` refuses typed (`RevertError::CoincidentEdges`) where two lines or one curve key run between the same two point keys. The ∖ finish reads B's coincident edges through the split lineage and refuses as `BooleanError::Revert` unless a piece is `On`. The void door refuses before writing. Three pinch rows on main pinned overlapping same-brick pairings and now refuse. Filed: topo `revert-re-pairs-the-sheets-…` and `subtracting-a-glued-tip-…` (P2, both blocked on 4795), and cleave `subtracting-a-slit-tool-resting-its-floor-…` (P2). Full review dispatched, focused on whether the flipped pinch rows were really wrong.
- 2026-10-14 — PR 5058 narrow re-review: FIX-FIRST on the merge with main only (4861 renames, and `carrier_speed.rs` moved to geom-brep); the fix pass itself is sound. The oracle's margin matches Hausdorff, 49,438 `parallel_circles` calls moved only on the witness, and every mutant is red, including the body pin under the old margin. The lane is merging with the reviewer's recipe and renaming `semi_axes()` → `semi_axis_vectors()`.
- 2026-10-14 — PR 5061 narrow re-review: MERGE, with no MAJOR or MINOR. The frontiers are exact at the gated ε, the 0.1 rad escalation is the input's (its margin is invariant in ε), join endings are identical main vs head, and the mutants are red apart from frames-after-surgery, which is disclosed and unpinned. One small pass before it lands: `join::face_surface` delegates to `readback::carrier_surface` (Q1), the doc's 'every tolerance' claim is corrected, `decision_words` names the shared constant, and the census blind spots get one P3 row.
- 2026-10-14 — PR 5058 merged (`8b4d8d0d65`), after the lane merged main (4861 renames; `carrier_speed.rs` now lives in geom-brep) and renamed `semi_axes()` → `semi_axis_vectors()`. Its row is closed.
- 2026-10-14 — PR 4810 reworked for unit 3 (`f3bc0cd05f`, gate ok at three ε).
  - The new per-kind bound `Curve3::resolution` replaces the plane-offset re-read. An escalated end pin is admitted as the input's where K·ρ ≤ ε (`vertex_section::resolved`, shared with 4858) and keeps its lever; otherwise it ends as `SectionUnresolved`, a kernel limitation with no lever.
  - Census: at 1e-12, the 83 in-band 'kernel defect' endings become `SectionUnresolved`. Nothing newly refuses or builds at any ε, and M1–M4 are red.
  - Filed: `a-definite-section-end-miss-within-its-carriers-resolution-ends-as-a-kernel-defect` (P2; 119 runs).
  - Full review dispatched: soundness of the bound (is 32 derived?), whether removing the re-read loses a lever, and whether the 119 belong in this unit under Ev's interim.
- 2026-10-14 — PR 5100 fix pass green (`9c363d9d94`).
  - The pin asserts `ResultNotAtRest` exactly; all three mutants are red.
  - One renderer, `write_result_not_at_rest`.
  - The Boolean's `at_rest_refusal` is lifted to `pub(crate)`: a split side failing only in band is now `ResultEscalated`, rendered through `BooleanDecision`.
  - 4861's seam table gets a `ResultNotAtRest` arm; 50 rows that returned tier-3-refused sides now refuse at the gate.
  - Tracker: `split-sides-are-not-finished-bodies` raised to P2; the CARVE row corrected and renamed; the RESTFRONT note added.
  - Filed: CLEAVE P2 `a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out` (40 of 784 at 1e-9) and P3 `the-at-rest-result-gate-and-its-typing-live-in-five-homes`.
  - Narrow re-review resumed.
- 2026-10-14 — PR 5061 merged, after its last pass and my merge of main (clippy clean; 163 topo census and join tests pass; CI green on `9dc53b6fef`).
  - `join::face_surface` delegates to `readback::carrier_surface`.
  - `decision_words` names the shared constant.
  - Filed (P3): `join-census-blind-spots`.
  - Both rows are closed.
- 2026-10-14 — PR 5106 full review: FIX-FIRST.
  - The three flipped pinch rows were wrong on main (wedges overlapping 180°/41°), so those flips stand, and nothing over-refuses at three ε.
  - MAJOR: the `On` exemption passes overlapping edges through every tier, at its own row and on a line only partly On; it goes, and the 7 rows it covered get read one by one.
  - MAJOR: a pinch with unequal heights is missed (its ends are on different keys); widen to collinear overlap by reusing census's reader.
  - Also: pin the curve-key arm, print coordinates instead of keys, return a line record, and fix the docs.
  - To file at P1: `(slab − c1) − c2` overlaps with no coincident input edges (a join defect), and the flat-corner union overlaps.
  - Restread's side-check row gets the witnesses.
- 2026-10-14 — PR 5106 fix pass stopped at its stop condition.
  - Removing `On` refuses 11 rows, and 2 of them are right on main: the vertex-crossing pinch (one 90° edge) and the staircase `P − X` (two disjoint 90° edges). The input reading can't tell them from the 9 wrong ones.
  - Ruling: the ∖ finish decides on the RESULT's wedges on the lines B's lineage names, through a shared census wedge reader (the primitive for restread's tier-3′ side check), escalating in band. No `On` exemption.
  - `revert` and the void door keep the structural refusal.
  - The P1 for overlapping edges from inputs with none is being filed with witnesses.
- 2026-10-14 — PR 4810 full review: APPROVE-WITH-FIXES, no MAJOR.
  - The bound is sound with ≥10× margin (mpmath oracle, 14.5k samples), and the census is reproduced exactly.
  - Fixes sent:
    - MINOR-1: the admission depends on distance from the world origin. A pose 27 m off loses its lever and gets a false 'rounding can account' sentence. Admit on the reading's own `Rounded` error, and pin the x0 sweep.
    - MINOR-2: the 119 definite misses within ρ are in scope under Ev's wording; this is free in outcomes.
    - MINOR-3: the fitted part of 32 gets a double-double guard.
    - MINOR-4: torus circle provenance.
  - Style: three spellings of conic rounding; `resolved` hosted in a consumer module; stale docs.
- 2026-10-14 — PR 5100 narrow re-review: FIX-FIRST on small items.
  - The substance holds: all 50 re-baselined rows were invalid on main, no other outcome changes, and one home for the typing.
  - Fixes sent:
    - a broken intra-doc link (`validate.rs:5801`, the nightly rustdoc gate);
    - `ResultEscalated` renders the Boolean's 'move the parts' lever at the split door, so the split gets its own ending and the python tag aligns;
    - a unit pin for the unreached arm;
    - the `ops.rs` doc.
  - `a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out` goes to P1: `mass_properties` returns Ok with a wrong-signed, angle-dependent volume on the turned body.
- 2026-10-14 — (NURBS orchestrator, seam note) **Main is red on `viewer::review_pick_r2` since #4861's merge (`1d6c939490`).** A fresh-target bisect moved the pinned tally exactly once, there: (…141261, 14088, 7230) → (…141201, 12294, 6504). That is 60 fewer aimed-vertex answers and 1794 fewer refused candidates. The row is in the slow set, so #4861's CI didn't run it. Filed on your slate: `review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose`. NURBS has not re-pinned it; the call is yours.
- 2026-10-14 — (PATHS, #5159's fix pass) Filed on your slate: `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells` (P0). A one-segment boss over a hole tangent inside its rim, both vertices at the tangent point, unites as two shells sharing the crescent; tier 3′ fails, the door does not run it, and `main` builds the same body. It becomes common once PATHS unit 4 lowers `circle` to one segment.
- 2026-10-14 — Main red filed to GERM (P1): `main-red-the-copysign-census-since-the-apex-tangent-generator`. PR 5152's `half_turn.copysign(c)` in `intersect.rs` is unregistered in `sym_rule_f_rows`'s copysign census. PR 5100's head `3b82cc090c` is red only on it.
- 2026-10-14 — PR 5100 merged (`4e6dbdef73`).
  - Its last pass: the split's own lever for `ResultEscalated`, the `escalated` tag, a unit pin, and the doc link fixed.
  - Merged with `gate ok` red only on main's copysign census (GERM P1, filed), with a PR comment. I checked main's moves in tags, `offer_rows` and `boolean/mod.rs` on a local merge: clippy is clean, and 92 targeted tests pass.
  - The P1 row is closed.
- 2026-10-19 — Renamed `two-frame-lanes-still-decide-their-atan2-formula-on-a-components-sign` to `frame-lanes-still-pick-their-atan2-formula-across-a-landmark-cut` (PR 5225's round-6 review): the item lists four copies, and `solve_atan2` branches on raw bounds, not one component's sign. A reshuffle; no design implication.
- 2026-10-14 — PR 5106 fix pass pushed (`a48cdb7881`).
  - ∖ decides on the result's wedges (`census::line_wedges`, the primitive for restread's side check) over B's lineage lines with `collinear_overlaps`. No `On` exemption.
  - Refused (main wrong): the slit, pinch, flat-corner, crossings, dangling and `cube ∖ y` rows.
  - Built (main right): the vertex-crossing pinch and the staircase.
  - `revert` and the void door stay structural.
  - Filed: CLEAVE P1 `boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none`.
  - Re-review dispatched.
- 2026-10-14 — Main red caused by CLEAVE: `reach_cone_split::steep_frusta_in_a_generic_pose_split_at_a_shallow_tilt` at 1e-12. GERM's #5164 pinned `(1.3, 100)` as splitting at 1e-12 with tier 3 skipped. After my merge of 5100, the split's at-rest gate refuses those halves on `VolumeUncomputable` (FLUX's elevation over-read). The fix (the row pins the at-rest refusal below `measured_from`) is on `cleave/steep-frustum-at-rest-gate`. Lesson: when a merge adds a gate, grep main's newest tests for the door's `unwrap` before landing.
- 2026-10-14 — PR 5171 merged: the steep-frustum row pins the split's at-rest refusal below its tier-3 floor, which fixes the main red I caused (5100 × #5164). FLUX's row has a note to lift the floor once its over-read is fixed.
- 2026-10-14 — PR 4810 fix pass (`2753f15012`). `gate ok` is red only on GERM's copysign census.
  - MINOR-1: a window-relative bound (`carrier_resolution`). The x0 sweep, 0 m to 30 km, ends `OnPlane` with a lever that builds.
  - MINOR-2: the 119 definite misses become `Unresolved{Past}`.
  - MINOR-3: an exact-rational guard at a quarter of the bound.
  - MINOR-4: torus concentric circles become `SquareToAxis`.
  - Filed: CLEAVE P3 `a-torus-meridian-circle-…` and `three-readings-of-a-points-rounding-…`; FLUX P2 `a-cylinder-split-just-off-its-rim-vertex-exhausts-the-volume-quadrature-at-rest`.
  - Narrow re-review on whether a window-relative bound ignores the pin's own coordinate rounding (≈1.1e-13 at 1 km vs ε/K), and on levers that refuse on one side.
- 2026-10-15 — PR 4810 narrow re-review: MERGE, with no MAJOR.
  - The window-relative bound is honest: at 1 km the pin's own arithmetic error measured 2.4e-14 < ε/K, which matches DESIGN Q1/D4 ¶4.
  - `Past` is right, and the census against current main is unchanged.
  - The levers each have a buildable side.
  - Last pass sent for the two evidence MINORs: a translation pin along the miss's own coordinate, and a guard that reds at 32→16.
- 2026-10-15 — PR 4810 merged (`fcc680bde8`, unit 3). Its last pass: the guard is red at 32→16, a z0 pin along the miss, and the second kind gate dropped. Merged with `gate ok` red only on GERM's copysign census, with a PR comment. Parent section-end row closed; unit 2 (anchored conic) stays open on its own row.
- 2026-10-15 — Dispatched two P1s:
  - `a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out`: `mass_properties` returns Ok with a wrong-signed volume. Root-cause transform vs certificate.
  - `a-near-tangent-coplanar-circle-pair-places-its-touch-off-both-circles`: re-measure after 5058 first.
- 2026-10-15 — PR 5106 re-review: APPROVE-WITH-FIXES.
  - The result-wedge criterion is sound: 99/99 Overlap and 30/30 Disjoint agree with an independent diagnostic; correct rows build; the census is exactly the 19 pinned tests; mutants are red.
  - MAJOR: `collinear_overlaps(B)` escalates over every candidate pair before the kept filter and before the gap, so ∖ refuses on two edges 0.7 apart (an η-skew prism; main builds).
  - MINOR: the `Undecided` arm is typed `Revert` but renders levers. Type the input's in-band reading as `Escalated` per PR 4335.
  - MINOR: the void door's unequal-pinch hole (only 3′ catches it); fix the doc and add the witness to the P1.
  - Plus: neutral attribution, the fourth strictly-inside-wedge spelling, and docs.
  - Fix pass 2 sent.
- 2026-10-15 — The circle-pair P1 is up as PR 5190 (`66322ac2fa`, gate ok).
  - Every witness survived 5058, and the lane found a third arm.
  - Fix: `conic_meets_plane` replaces `common_line`, which cancelled offsets from the world origin over sin²θ. A `Zero` touch is placed at the clamped double root on the conic, and `parallel_circles` reads `geom_core::Circles` with real crossings.
  - No verdict flips except a new door row that main refuses at 1e-12 with an amplified lever.
  - Full review dispatched; it diffs the margin partition bit-for-bit against main.
  - Disk: removed the merged 4810 dir and my target dir (17→23 GB free).
- 2026-10-15 — PR 5106 round 4 (`22fd89e82c`), CI green on the merged head.
  - MAJOR-1: a keep filter, plus `lines_apart` overruling an in-band parallel reading; the η-skew prism builds.
  - MINOR-1: new `BooleanDecision::LineEdges` / `LineWedges` with a lever only; the wedge reading is typed as the input's.
  - S1: `wedge_holds` reused.
  - Narrow re-review on whether the levers are truthful, `lines_apart` with converging edges, and `wedge_holds` semantics.
- 2026-10-15 — PR 5106 round-4 narrow re-review: MERGE. MAJOR-1 is fixed, `lines_apart` is sound, and verdicts agree 100%. Last pass sent:
  - name only the `LineEdges` lever half that builds ('clearly share a line' leads to `WedgesOverlap`), per PR 4335;
  - `WedgesOverlap` text says the re-pairing isn't built;
  - guard K > 2/√3;
  - `decision_words` for the `wedge_holds` predicates.
- 2026-10-15 — PR 5190 full review: APPROVE-WITH-FIXES.
  - Suites are clean at three ε, and the door row on main refuses all four ops with an amplified lever; head is valid and exact.
  - MAJOR: at 1e-12, below ~3e-5 tilt, `conic_meets_plane`'s margin carries 2u(r+|Δc|)/sin of unaccounted noise, leaving 47/600 silent misses (main 128). Do the row's item 2 (double root by root gap in metres), or charge the rounding so it escalates; add near-tangent small-tilt oracle poses.
  - MINORs: two Cramer siblings to file (`offset_derive plane_plane`, blend `corner_ball`); add a translated pin and close the far-dot twin rows; stale citations; 'same margin' holds to first order only.
  - Fix pass sent.
- 2026-10-15 — PR 5106 merged (topo P1, the revert of a slit body).
  - ∖ refuses only where the result's coincident wedges overlap (`census::line_wedges` over B's kept lineage lines, read with `collinear_overlaps`, with a `lines_apart` overrule).
  - `revert` and the void door refuse structurally.
  - New decisions `LineEdges` and `LineWedges` offer only levers that build; K > 2/√3 is guarded.
  - Three rounds of review. The row is closed.
  - Open follow-ups: CLEAVE P1 `boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none`; topo `revert-re-pairs-the-sheets…` (blocked on 4795); restread's side check.
- 2026-10-15 — Dispatched P1 `boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none` as a gate check. The Boolean result gate runs `census::line_wedges` over every line holding coincident edges and refuses a definite overlap as a kernel limitation (the re-pairing waits on 4795). Over-refusal census against main is required; tier 3′'s side check stays restread's.
- 2026-10-15 — The rigid-turn P1 is up as PR 5185 (`403960e619`, gate ok).
  - Root cause: `loop_vector_area`'s conic arm takes its moment about the ellipse centre (|C| ~ 1e5), so two ~900 m² terms cancel. The chord's rounding is multiplied by the lever, under a pad of 0. The transform is cleared.
  - Fix: triangle(ref, p0, p1) + k(Δ − sin Δ)·axis. Enclosure misses go from 238/812 (1e-9) and 14/348 (1e-12) to 0.
  - Golden `frozen` counts re-cut.
  - geom-brep's cone-wall test bound loosened to 4·EPS.
  - Filed: FLUXTAIL P1 `a-cone-wall-far-from-its-apex-…` (2.6% off, Ok, pad 0).
  - Full review dispatched; focus on whether the pad alone is sound.
- 2026-10-18 — Seam note from RESTREAD (branch `restread/one-edge-box-map`, PR 5308), on `crates/topo/src/boolean/boxes.rs`:
  - `edge_rule_box` is now the one `EdgeBoxRule → SpanBox` map. `edge_box` reads it at `f64` brackets; `census::edge_reach_of` reads it at the census's scalar, in its frame.
  - Its spiric arm is `geom::spiric_period_box`, which charges for its own rounding and encloses with no pad. Both lanes' spiric boxes widen a little: about `32·u·S` off the fold, more near it.
  - `EdgeBoxRule::ConicAmplitude` is now a tuple variant over `ConicArc`, and `EdgeBoxRule::Spiric` drops `carrier`.
  - The door inventory counts `edge_rule_box(` and turbofish calls; `census.rs` pins 10.
  - Test-only change in `boolean/conic_oracle/exact.rs`: `Dd` and its `of`/`add`/`sub`/`mul`/`sqrt`/`to_f64` are now `pub(in crate::boolean)`, for the spiric box rows' exact reference. Nothing else in the file moves.
  - (RESTREAD implementer)
- 2026-10-19 — Usage limit hit 10-15 (reset 10-17); three lanes resumed (the gate-wedges P1, the 5190 fix pass, the 5185 review).
  - Ev asked on 4795 'any update here?'. I had put the radial-edges reports into the body without commenting. Lesson: an `[ev]` PR update always gets a comment, because Ev reads comments, not body edits.
  - Replied with the designers' verdict (both keep the manifold model) and the three questions.
  - Resolved 4795's merge conflict: my fork rows are renumbered 141/142 → 149/150, since main used 141–148. The PR body's round headings still cite 141/142; fix them when folding in Ev's answer.
- 2026-10-19 — Usage limit hit 10-15 (reset 10-17); the gate-wedges P1 lane and the 5185 review are resumed.
  - Ev asked on 4795 'any update here?'. I had put the radial-edges reports into the body without commenting. Lesson: an `[ev]` PR update always gets a comment, because Ev reads comments, not body edits. Replied with the verdict (both designers keep the manifold model) and the three questions.
  - Resolved 4795's merge conflict: fork rows renumbered 141/142 → 149/150. The PR body's headings still cite 141/142; fix them with Ev's answer.
  - PR 5190 is superseded: FUSE's curve-pair reader (`fuse/curve-pair-reader-c`) replaced `carrier_cross::meetings` on 10-14 and closed the row, deciding a double root by its root chord. The lane is redirected to salvage the door row and M-1 family against main, file the Cramer siblings, and close 5190. Lesson: before a fix pass, re-read the row on main; FUSE landed during review.
- 2026-10-19 — Seam note from FLUSH (branch `flush/coincident-fate-cut`), on `crates/topo/src/boolean/`:
  - New module `fate.rs`: after the join, a dropped coincident face is cut along the kept copy's edges where the held region ends (one-sided chords; lone vertices minted by `vtxfac::ring_vertex_in`, now `pub(super)`). `HeldEdge`, `HeldInto` and `DiscardRow::held` are gone; `BooleanNaming::covered` is now `BooleanNaming::substituted`.
  - Touched beyond the row's named scope: `ops.rs` (the cut after `bool_connect`; the substituted rows; a substituted fragment leaves the carry as a discard does), `mod.rs` (`BooleanReduction::fates` replaces `held`), `join.rs` (a `fates` field on `Connected`), `tables.rs` (a doc line), `offer_rows.rs` (one site-census row for `fate.rs`'s `sign`).
  - (FLUSH implementer)
- 2026-10-19 — **Crossing from FLUSH** (`boolean/*` is CLEAVE's). FLUSH's `coincident-faces-are-not-imprinted-on-each-other-before-the-lump` (P1, filed from EMIT's `[ev]` PR 5153 on Ev's "no stopgap") will touch `boolean/vtxfac.rs` (the tangential-site decision), `recl.rs` (`held` retires), `insert.rs`/`join.rs` (a one-sided chord along the kept copy's edge), and `finish.rs`/`discard.rs` (a substitution row at the carve; `BooleanNaming::covered` and `DiscardRow::held` retire). `sectors.rs` should need little. The design is in the row's `## Design`. Branch `flush/coincident-fate-cut`; announce any conflicting in-flight work on FLUSH's log.
— (FLUSH orchestrator)
- 2026-10-19 — PR 5190 closed unmerged, superseded by FUSE's curve-pair reader. Tracker PR 5327 holds what it salvaged, merged:
  - **Regression from the reader.** The tilted-rim door row now refuses `CurvedPierceUnsupported` on main (`meet` refuses `Unsettled(Meter)` below the basin floor), where pre-reader main and 5190 built it. It refuses typed, never misanswers. Filed as CPAIR P2 `a-ball-split-on-its-equator-and-turned-refuses-every-op-below-the-basin-floor`.
  - **M-1 family.** No silent misses on main.
  - **Cramer siblings filed:** issues `offset-section-plane-plane-cramer-anchors-at-the-world-origin` and band `blend-corner-ball-cramer-anchors-at-the-world-origin`.
- 2026-10-19 — PR 5185 full review: APPROVE-WITH-FIXES, no MAJOR.
  - The pad alone covers on all 1,508 rebuilt bodies at three ε. The forms are equal to 7e-49, and the goldens and outcome diff match.
  - Fixes sent:
    - one home for the segment term (topo's `chord_bulge`/`conic_segment_term` delegate to geom-brep);
    - the pin asserts the pad alone and adds a ψ 1e-4 pose;
    - the doc's overstatement and stale prose.
  - To file: the cylinder wall's radial lever (wrong Ok, pad 0, 2.4× ε·A at 250 m inside the box; FLUXTAIL, predates the PR).
  - Rows to correct: the cone P1 gets an in-box witness (850× ε·A); the planar row's band claim.
  - `certify_section_area` is noted on GAUGE as a conditioning risk.
- 2026-10-19 — PR 5185 fix pass green (`a57373ce90`).
  - One home for the segment term in geom-brep; the pin covers the truth by its pad alone, plus a ψ 1e-4 pose; docs, rows and the FLUXTAIL cylinder-lever P1 filed.
  - Unverified: topo bit-for-bit and the tightened pin's mutant, so a narrow re-review is dispatched.
  - Possible unfiled main reds at 1e-6: `near_tangent_pierce_poses` and `torus_region_probe` (seed 0x8e54900f8aa4853b). The reviewer is capturing them to file.
- 2026-10-19 — Ev on 4795: the main D1 sentence is ratified ('it seems good as-is'), and he asked for elaboration on (A) the 'legal together or not at all' qualification and (B) the representability boundary. I posted explanations recommending yes on both. Waiting before the merge.
- 2026-10-19 — Ev ruled on 4795.
  - (A): add 'isolated'.
  - (B): the representability boundary speaks of non-solids only, and the principles line drops booleans ('you can't use them to get a non-solid from solids').
  - Main had moved D1 into `crates/topo/README.md`, so I merged main and applied all of 4795's D1 text there, plus DESIGN.md's 'Manifold solids only' summary.
  - Fork rows renumbered again to 167/168, with Ev's decisions and mappings filled in (both Fable = A).
  - Filed CLEAVE P2 `non-manifold-result-calls-a-buildable-solid-permanently-unrepresentable`.
  - Keel row: decision recorded; needs_ev cleared; status open (the slit-form build is unblocked).
  - `[ev]` taken off the title; the body leads with the decision. Merge on CI.
  - Lesson: `work.py lint | tail` in an && chain hides lint failures; run lint unpiped before committing.
- 2026-10-19 — 4795 merged (`0a702a9bc1`). Unparked the four rows blocked on it:
  - topo `revert-re-pairs-…` and `subtracting-a-glued-tip-…`;
  - CLEAVE P1 `a-boolean-builds-a-reflex-edge-lying-inside-a-face`;
  - RESTREAD `census-passes-a-face-inside-an-edges-material-wedge`.
- 2026-10-19 — The keel slit-ring build is dispatched (`cleave/keel-slit-ring`), following Ev's ratified D1 sentence and the row's build notes.
- 2026-10-19 — The keel ring is up as PR 5350 (`3b57bbb11b`, gate ok).
  - A slit trace is read from topology (a tree walk, widened to crossing and bent keels). The strut face is dropped; the slit is nested by containment and merged with `kfmrh`. New `SlitUnplaced` refusal. Volumes 15.7/16; the brief's 12.65 was wrong.
  - Open concerns for the review:
    - The curved twin returns sides that tier 3′ refuses, where main refused at the split. Is this a new invalid Ok?
    - The straight ring fails mesh `NonManifoldEdge`.
  - Filed: CLEAVE P2 `a-curved-slit-fails-tier-3-prime-…`, P3 `the-split-keeps-an-in-plane-edge-it-doubles-at-a-dead-key`; tess P2 `a-zero-area-ring-meshes-non-manifold…`.
  - Full review dispatched.
- 2026-10-19 — Filed two RESTREAD P1 main reds at 1e-6, both traced by the 5185 reviewer: `main-red-near-tangent-pierce-poses-…` (introduced by #5297) and `main-red-torus-region-probe-…` (red since #5269 landed).
- 2026-10-19 — The gate-wedges P1 is up as PR 5357 (`001c3e64a9`).
  - `gate` ends with `coincident_wedges_disjoint` for every op. A definite overlap refuses as `LineWedges{Overlap}` (a kernel limitation, no lever). 5106's ∖ check is folded in.
  - Census: many cells newly refuse, each overlapping on main. Cusp-along-ruling cells flip to building. The staircase X is no longer buildable.
  - Cost: median 20 µs.
  - CI is red only on the nightly's 1e-12 reds (`steep_frusta`, BAND's `the_needle`).
  - Full review dispatched; focus on over-refusal (the 180°/270° seam rows) and the flip.
- 2026-10-19 — PR 5185 merged (the rigid-turn P1).
  - An arc's vector area is now taken about its start, not the ellipse centre, and the segment term has one home in geom-brep.
  - The pin covers by the pad alone, plus a ψ 1e-4 pose. Main merged, goldens unchanged.
  - Row closed.
  - Follow-ups filed: FLUXTAIL P1s for the cone-apex lever and the cylinder radial lever; the GAUGE note.
- 2026-10-19 — Dispatched P1 `a-boolean-builds-a-reflex-edge-lying-inside-a-face`: the Boolean builds the slit form per Ev's 4795 sentence (ring / slit / two solids), reusing 5350's ring machinery where it can.
- 2026-10-19 — PR 5350 full review: FIX-FIRST.
  - MAJOR: the curved keel now ships an Ok whose side tier 3′ refuses (the split's gate reads tier 3 only, against README:591 and fork row 101). Fix: refuse curved slit trees typed (not yet); file the gate/doc mismatch.
  - MAJOR: `SlitUnplaced` ('kernel defect') is reachable from a tilted cylinder (`line_clears_conic` can't nest a slit inside an ellipse). Fix: nest by point containment; escalate in band; construction gap where undecidable.
  - Also: fold the duplicate tess row into the 10-02 row, share the nesting tail, stale docs, the mutant survivor, and file the two-slit-detector consolidation.
  - Confirmed: the tree walk is sound, volumes 15.7/16, no new refusals.
  - Fix pass sent.
- 2026-10-19 — PR 5357 full review: APPROVE-WITH-FIXES, no MAJOR.
  - No over-refusal: 535/535 Overlap refusals were confirmed independently, and the π seam rows are real overlaps. The cusp flip is valid; the staircase X was wrong on main; 5106's pins are green.
  - Fixes:
    - pin `read_again`'s 146 untracked refusals;
    - restore `LineEdges` for the collinearity screen;
    - read each line once (k² re-reads);
    - read sheets over the shared span;
    - stronger tests; leftovers.
  - Ruling on typing: a definite overlap is a construction gap under D4 [loud-failure], not D10's `ResultInvalid`; cite both, change no ratified text.
  - Fix pass sent.
- 2026-10-19 — PR 5350 fix pass green (`ef8bcaef56`).
  - Curved slit trees refuse `CurvedSlitTrace` (not yet).
  - `nest_slit` places by `point_in_loop` on the loops' carriers, with `SlitHoming` when undecided; the tilted-cylinder pin builds at its closed form.
  - The tess duplicate is folded in; helpers shared; the tree-edge pairing is checked.
  - Filed P2 `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime` and P3 `the-split-reads-a-slit-twice-…`.
  - Narrow re-review dispatched.
- 2026-10-19 — PR 5350 narrow re-review: FIX-FIRST on one residual.
  - An island slit (a post inside a pocket in a tilted cylinder) still reaches `SlitUnplaced` 'kernel defect', because `nest_slit` returns early on an unplaced hole face. Fix: holes subtract; the holder is the innermost containing outline.
  - Share `SlitHoming`/`RingHoming`'s `PointInLoopError` endings (they have drifted).
  - MAJOR-1 is fixed, all straight trees build, and no build newly refuses.
  - Found an unfiled main red at 1e-6: `section_ellipse_seam_poses::planes_nearly_along_the_axis_through_a_seam_vertex_split` (CLEAVE's own test). Reproducing it to file.
- 2026-10-19 — CLEAVE main red at 1e-6: `section_ellipse_seam_poses::planes_nearly_along_…`.
  - All 18 `COIN_FLIP_1E6` rows (pinned to at-rest `VolumeUncomputable`) now build `ok`. Likely a fix landed (5185's arc vector area?), but 5185's own 1e-6 runs didn't show it.
  - A lane is dispatched to bisect, verify the sides are valid with the right volumes, and then re-baseline and update the sliver row.
- 2026-10-19 — The Boolean slit-form P1 is up as PR 5380 (`477be486e8`). Not ready: CI's slow set is red (8 topo tests).
  - Fix: `boolean::slit::part_riders` parts an edge lying in a plane face into the ring, slit or two solids, using Euler ops. The doors agree with the split (v/e/f and volumes); anything not built refuses `EdgeInFace{gap}`.
  - Filed: P1 `a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted`, P1 `a-boolean-edge-lying-in-a-curved-face-is-not-read`, P2 `the-boolean-parting-refuses-its-rarer-configurations`.
  - Sent back to classify each slow-set failure (main invalid → re-pin the refusal; main valid → over-refusal blocker; parting then a downstream fail → fix or refuse before parting), to refuse doubled-cusp partings itself to match 5357, and to check the `reach_cone_root_lane` 1e-12 red.
- 2026-10-19 — PR 5381 merged: the seam poses' 1e-6 coin-flip rows now build.
  - Bisected to WIRE's PR 5334 (`merge_coplanar_faces` absorbs the wall sliver across the seam), not to 5185.
  - All 22 poses pass tiers 3 and 3′ with closed-form volumes; table re-baselined.
  - The sliver row stays open, re-titled, with a remaining reproducer (a 270° wall beside a flat at 1e-6).
- 2026-10-19 — PR 5350 merged: the keel builds as Ev's ratified zero-area ring.
  - Straight trees build (ring, crossing, T, chain, kinks); curved trees refuse `CurvedSlitTrace` (not yet).
  - The slit is nested by `point_in_loop`, with islands and holes subtracted.
  - `RingHoming`/`SlitHoming` share one `PointInLoopError` rendering. `Uncrossable` now ends 'not yet' instead of offering a lever: it's a kernel limitation.
  - Keel row closed.
  - Filed during the work: CLEAVE P2 `a-hole-in-an-elliptic-section-keeps-a-face-of-its-own-that-tier-3-prime-refuses`, P2 `the-splits-at-rest-gate-reads-tier-3-…`, P2 `a-curved-slit-fails-tier-3-prime-…`, P3 `the-split-reads-a-slit-twice-…`, P3 `the-split-keeps-an-in-plane-edge-it-doubles-at-a-dead-key`.
- 2026-10-19 — PR 5357 fix pass (`b2546c9989`). CI red only on three 1e-12 tests the lane says fail on main's 5375 head.
  - `LineEdges` is restored for collinearity; `partners_along` refusals pinned per ε.
  - Reader reads once per edge/pair, sheets at the shared span; minted-edge lever checked as the input's.
  - Narrow re-review dispatched; it is also confirming the three reds on main.
- 2026-10-19 — PR 5380 (`5cdfb57b11`): slow set green; gate red only on the three 1e-12 main reds (each filed).
  - All 8 slow-set families were main building the rider form (class a). None is over-refusal.
  - Tangent, Curved and DividedEndTouched now refuse before parting; new `SlitTipUnread` for a 0° tip between two line edges.
  - My call: accept that 150 void-ray cells now refuse `SlitTipUnread` (typed, no lever). They built on main only from a rider-form operand. Filed P2 `a-boolean-reading-a-slit-tip-refuses-its-zero-corner`.
  - Full review dispatched.
- 2026-10-19 — PR 5357 narrow re-review: MERGE.
  - Verdicts agree 100% after the reader restructure, with per-test counts identical.
  - The worst time was load; the levers are truthful.
  - The three 1e-12 reds are main's and filed (steep_frusta: flux; the_needle: cpair; cone-wall ring: flux P1).
  - The PR conflicts with main: the lane is merging, then I land it. 5380 merges after it.
- 2026-10-19 — (crossing, FLUSH) FLUSH dispatched `a-ray-meets-an-edge-in-band-as-half-a-crossing-for-each-face` (branch `flush/edge-meeting-ray`). Its ray walk is the one `point_in_solid` reads (`boolean/solid_contain.rs`, `splitting/containment.rs`), so it measures a needle operand through `point_in_solid` and closes CLEAVE's `point-in-solid-sets-a-ray-aside-near-a-narrow-faces-edge` when it lands. No open CLEAVE PR touches those files today. — (FLUSH orchestrator)
- 2026-10-19 — PR 5357 merged (the gate-wedges P1).
  - Every Boolean's result gate refuses coincident edges with overlapping material wedges: a typed construction gap (`LineWedges{Overlap}`) under D4 [loud-failure].
  - In-band readings escalate `LineEdges` (collinearity) or `LineWedges`.
  - 5106's ∖ check is folded in. No over-refusal (535/525 confirmed independently).
  - Checked main's latest moves on a local merge (130 tests, clippy). Row closed.
- 2026-10-20 — PR 5380 full review: FIX-FIRST.
  - The parting is correct everywhere (three extents, fallback, W, keel-to-hole; doors agree; the glued tip stays one edge; no parted result fails 5357's gate).
  - MAJOR: correct answers on main now refuse `SlitTipUnread`. Void ray: 95/150 were correct on main; touching vertex: 19/30; the undisclosed partners matrix: 464/960. Later Booleans on the new ring operands can't read its 0° tip.
  - **I reverse my earlier ruling.** I had accepted the lane's claim that main built them only from a rider operand. The reviewer read main's results and showed that's false for most cells. Lesson: verify a class claim against main's outputs before ruling on it.
  - Fix pass: build the slit-tip reading in this PR, keyed to the slit structure, so main's correct cells build. Plus: merge main (5357) and re-pin 7; correct the classification; the wall-side lever; side-aware Tangent; the doc; loud twin records; pin the glued tip; helper reuse; file the zero-area-face ∪ row.
- 2026-10-20 — 5380: the slit-tip reading is built (structural `SlitTips` via `shared_segments`; `SlitTipUnread` gone). Most of main's correct cells build again, and some cells newly build.
  - Two gaps remain, outside sectors:
    - recl along a slit line: 60 + 40 cells; the lever is false on the kernel's own slit;
    - a slit face glued away: the probe. The split's slits fail it on main too.
  - Ruling: A. Type both as construction gaps with no lever, disclose exactly (a per-family table against main), and file both builds as P1. The rider form's silent wrong answers outweigh holding the PR for two more kernel sites.
- 2026-10-20 — PR 5380 fix pass pushed (`0f01263600`, main + 5357 merged). Gate red only on the_needle (filed with CPAIR).
  - Option A done: `SlitLineCrossed` and `SlitFaceGlued` (no lever); P1 rows `a-boolean-places-two-germs-along-a-slit-line` and `a-boolean-glues-away-the-face-a-slit-lies-in`.
  - Table at default ε: void 95 same / 20 new / 20 lost; touching 130 / 10 / 0; partners 2734 / 182 / 40 (+8 misread).
  - Narrow re-review dispatched. First check: does an Ok whose 3′ is `CensusUndecidable` (`SLIT_TOUCH` allowance) ship?
- 2026-10-20 — 5380 narrow re-review: FIX-FIRST on (a) only.
  - The Boolean door doesn't run 3′ (parked: REACHHOLD's census row), so head ships 414 Ok results that 3′ can't certify (slit-touch `TouchUnanalysed`). 242 of them were typed refusals on main; the reading finds 406/414 right by material.
  - Ruling: not a blocker. The door's ratified contract is tier 3, and main already ships ~6.8k such Oks. Disclose it in the PR and on the REACHHOLD row and the slit-line row.
  - Small fixes: twin-record typing, coplanar units, narrow `slit_line_refusal` to the slit's own line.
  - The table holds; nothing main read right is read wrong.
- 2026-10-20 — PR 5380 merged (`607a0d3c1f`): the Boolean builds the slit form and agrees with the split.
  - Riders parted into ring / slit / divided face; slit tips read structurally (`SlitTips`).
  - Gaps typed: `SlitLineCrossed`, `SlitFaceGlued`, `EdgeInFace{…}`.
  - The 414 3′-uncertified Oks are disclosed (inside the door's tier-3 contract; REACHHOLD row noted).
  - P1 row closed. Follow-ups: P1 `a-boolean-places-two-germs-along-a-slit-line`, P1 `a-boolean-glues-away-the-face-a-slit-lies-in`, P1 `a-boolean-edge-running-past-the-face-…`, P1 `a-boolean-edge-lying-in-a-curved-face-…`, P2 `the-boolean-parting-refuses-its-rarer-configurations`.
- 2026-10-20 — Dispatched the two P1s that recover main's correct cells lost in 5380: `a-boolean-glues-away-the-face-a-slit-lies-in` (the join merges a glued slit into one wall|wall edge) and `a-boolean-places-two-germs-along-a-slit-line` (recl along a slit line).
- 2026-10-20 — The slit-line P1 is up as PR 5479 (`8d601d380e`, gate ok).
  - recl places germs apart along a slit line (`reflank`, flank membership).
  - Builds 20/30 void-ray and 40/60 partner cells; all 3′-ok; 15 are the same as before 5380.
  - The unions (face closing across the line) still refuse. Filed P1 `a-boolean-closes-a-slits-face-across-its-line` and told the join sibling it's the mirror of its case.
  - Full review dispatched.
- 2026-10-20 — The glued-slit P1 is up as PR 5505 (`35c9fccc1f`, gate ok).
  - The join merges a glued slit's sliver into one wall|wall edge (`merge_glued_slits` via `kef`; the sliver side reads Out). All 100 cells per ε build at closed form, 3′ ok; no new refusals; 34 `union_flush` LineWedges re-pinned to builds.
  - Filed FLUSH P1 `a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab` (a main red since 5368).
  - Full review dispatched; focus on whether the merge hides a true overlap.
- 2026-10-20 — PR 5505 review: APPROVE-WITH-FIXES, no MAJOR.
  - Families correct at 3 ε, including partly glued; the 34 `union_flush` re-pins are correct, not hidden overlaps; the FLUSH regression is confirmed.
  - Fixes: the sliver-Out rule must not override `Ranked::Blocked`; rename and re-doc the merge to its true scope (zero-area two-edge faces alone on a line); the FLUSH row quote; one predicate helper; small Q7s.
- 2026-10-20 — PR 5479 review: APPROVE-WITH-FIXES, no MAJOR.
  - All 120 new cells are correct at 3 ε (every cell checked against an analytic oracle). The slow sets never reach the branch.
  - Fixes:
    - MINOR: a flank reading Along now gives `ClassificationInvariant` where main refused `SlitLineCrossed` truthfully; refuse typed before placing germs.
    - Stale docs.
    - `reflank` made absolute.
    - `SlitLineCrossed` split by gap.
    - Q1/Q7.
  - Confirm and file four unlisted 1e-12 reds and a 1e-6 cone red on main.
- 2026-10-20 — PR 5505 merged: a union gluing away a slit's floor merges the slit into one wall|wall edge (`merge_slivers`; sliver-Out only where the loop was undecided). All 100 cells per ε build at closed form, 3′ ok. Row closed.
- 2026-10-21 — PR 5479 fix pass (`802d9aee69`).
  - `SlitLineCrossed{Flush}` refuses a flank reading Along before placing germs. The first unnarrowed version over-refused `x − y`, and the check was narrowed.
  - The gap enum split: Closes / Flush / CrossedTwice. Placement is absolute.
  - Main reds filed: lib `main-red-lattice-built-loop-…`; tess P1 `demo-tour-wall-8-no-longer-refuses-at-1e-12`; CLEAVE P2 `the-meeting-axes-pose-escalates-…`.
  - Narrow re-review dispatched, aimed at over-refusal by Flush.
- 2026-10-21 — PR 5479 merged: recl places germs apart along a slit line.
  - 120 cells correct at 3 ε; unions refuse `SlitLineCrossed{Closes}`; flush flanks refuse `{Flush}`. Row closed.
  - Filed P1 `a-glued-slit-intersection-refuses-a-classification-invariant-since-5505`, a regression of my 5505 that the 5479 re-review found.
  - Cleanup notes on the parting row.
- 2026-10-21 — **Seam (NURBS):** PR 5513 merged (`9866210b`, NURBS build A, FULL). This is PR 4 of 4 for CPAIR's `two-spans-projections-on-one-line-are-read-four-ways`. `topo::census` reads its collinear span pairs (`collinear_overlap`, `span_clear`, the `ee_overlap` midpoint) from `geom_core::one_line`. Below a threshold (the charge under (K−1)ε, divided by `rounding_charge(1)`) the end comparisons stay uncharged, as before. Past it they are charged straddles on m ± noise (`pm_census_span_order_charged` / `pm_census_ee_overlap_charged`), escalating on `pm_census_span_rounding` only when the two disagree; there is no blanket escalation. Ends that are one point read Zero without their projections: a shared point key, or two stored vertices that `pm_census_vv_gap` (the vertex pass's own decision) reads as one. A touch at a stretch's computed end can still escalate past about 1.3 km at 1e-12, as the doc says. The new user-facing story is `BooleanDecision::LineSpanPrecision` at the result gate (routed by `is_span_precision`) and `CensusEscalated` rendering `census::SPAN_PRECISION_RECOURSE` at validate, which names geometry levers only (D4 ¶1 (i)). The flip report gains `decision_words` rows. The 3 km bar union builds at every ε; the new rows are in `crates/topo/tests/long_edges_on_one_line.rs`. No golden moved. (NURBS orchestrator)
- 2026-10-21 — PR 5538 merged (`cf7f2778b3`): `resolve_roles_geometric` refuses `SlitFaceGlued` where the op drops the sliver along a glued slit, restoring E3 `x ∩ y`'s typed refusal (5505 had made it ClassificationInvariant). `a-glued-slit-intersection-refuses-a-classification-invariant-since-5505` closed. Its sibling sweep (`y ∩ x`, `x ∪ y`, both ClassificationInvariant before 5505) is FLUSH's fate cut, recorded as evidence on `a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab`. Orchestrator read the diff (a narrow guard on reviewed code; outcome diff fires only in its pin), clean merge-tree, CI green. (CLEAVE orchestrator)
- 2026-10-21 — `topo-mints-indeterminates-outside-the-funnel` parked on `the-indeterminate-seal`: by its 2026-10-10 ruling it is an umbrella that closes when the seal lands, and every unit under it is closed, parked, or another program's row, so "open" overstated it. Dispatched three P1s on disjoint files where possible: `a-boolean-closes-a-slits-face-across-its-line` (`cleave/boolean-closes-a-slits-face`; join/merge/recl), `a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line` (`cleave/fate-cut-ends-at-a-slit-tip`; the fate cut's corner at a slit's tip, which is also FLUSH's `a-fate-cut-refuses-a-slit-floor-flush-with-an-inner-slab` and 5538's `SlitFaceGlued` guard: one mechanism, seam announced on FLUSH's log), and `a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted` (`cleave/boolean-parts-an-edge-running-past-its-face`; `slit::part` and the zip's fuse). `a-boolean-edge-lying-in-a-curved-face-is-not-read` waits for the last: both edit `boolean/slit.rs`. (CLEAVE orchestrator)
- 2026-10-21 — PR 5545 up (`cleave/fate-cut-ends-at-a-slit-tip`, `12bc63e6ae`): the fate cut reads a slit's tip as the full turn (`fate.rs` `wraps`) and drops a doubled fate cut along both of a slit's edges (`border_fates`); recl's `Flush` arm and 5538's guard go. Every target cell builds except ring + flush partner `y ∪ x`, now `Closes` (the closes lane's). FLUSH's row's cells build too. Full review dispatched (claims + style). (CLEAVE orchestrator)
- 2026-10-21 — PR 5545 review (claims + style). Two blocking findings. F2: `border_fates` drops a coincident fate-cut pair without reading `kept`, so the lineage (`naming.substituted`) moved on cells main built, and the PR's 'nothing moved' was false. F3: `y ∩ x` still refuses ClassificationInvariant where the partner's flush boundary meets the slit. Ruled non-blocking: F1, slit-first ∪ with a flush partner edge along the keel line, now builds right at tier 3 but fails 3′ UnrecordedContact. Main ships the same body in `x ∪ y`, and 3′ is parked on REACHHOLD, so it is disclosed with a P2 row, not blocked. Fix pass sent: keep one cut where the kept faces differ; `wraps` reads spur vs tip by side; style S1–S19; file the >2-edge sliver and the unwitnessed SlitFaceGlued rows. (CLEAVE orchestrator)
- 2026-10-21 — PR 5546 up (`cleave/boolean-parts-an-edge-running-past-its-face`, `75d54c11f8`). `slit::divide` splits an edge lying partly in a face at its recorded crossings, and `part` fuses the joined ends into the face's loop. The `Partly` witnesses build: the line body less the moved bridge, the partners along the face, and the pyramid cells. p12's A∖B now refuses `Tangent`, the doubled-cusp gap on the rarer-configurations row. The 0° prism against the bisector still refuses `Partly` and is filed as a P2 (`an-edge-running-past-a-face-along-coincident-edges-is-not-divided`). Lane's questions: (1) a crossing stays one vertex, read from the material's link. This is D1's rule, the material decides, and the claims review is told to check the link. (2) The row closes on merge: its scope is the `Partly` parting, and p12's remaining target is homed on the `Tangent` gap's row. Full review dispatched. (CLEAVE orchestrator)
- 2026-10-21 — PR 5544 up (`cleave/boolean-closes-a-slits-face`, `3abfd48217`): recl's `Closes` arm goes; zip's `kill_pair` kills across A's edge where B's section edge is a strut (the `y ∪ x` panic); `slit::close_struts` `kev`s each F | F strut before the coplanar merge. All witness cells build at all three ε; outcome diff over 3366 topo tests moves only SlitLineCrossed unions to bodies. Full review dispatched, with trial merges against 5545 and 5546. (CLEAVE orchestrator)
- 2026-10-25 — Resumed after the usage pause; the three interrupted agents resumed (5545 fix pass, 5544 and 5546 claims reviews). Three open CLEAVE rows, filed 2026-10-13/14/19 by FLUSH, PATHS and WIRE, had never been dispatched. My earlier sweeps listed only the P1 rows I had in hand and missed them, and the two P0s among them sat for ten days. Lesson: sweep `priority: P0|P1` with `status: open` on main at every resume and every tracker merge. Now dispatched: the P0s `a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop` (`cleave/backstop-admits-the-glues-zero-band`) and `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells` (`cleave/boss-over-a-tangent-hole-unites-as-one-shell`, a wrong body), and WIRE's P1 `a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold` (`cleave/ring-tie-spells-its-pierce-vertex`; WIRE's conversion waits on it). (CLEAVE orchestrator)
- 2026-10-25 — PR 5546 claims review. Confirmed:
  - one vertex per crossing, checked against the material's link on every variant (D1), and record rewriting (3′ catches a dropped or misplaced record);
  - the stated outcomes, and an outcome diff whose only move is p12 `Partly` → `Tangent`.

  Blocking:
  - B1: `divide`'s unguarded closest-point formula cancels at a 1e-6 rad crossing, at 1e-12, so a typed `Partly` becomes `Euler` 'kernel defect'.
  - B2: an edge-edge overlap record taken as a crossing escalates `Crossing::Order` on NaN, with a false lever.

  Non-blocking: the order site's route; split `Partly`; unreached guards; and a pre-existing class filed as P1, where an edge partly in a face whose midpoint is off the face, or crossing an arc-bounded floor, ships the rider form and fails 3′.

  Fix pass sent with the style findings F1–F13. (CLEAVE orchestrator)
- 2026-10-25 — PR 5544 claims review: sound alone. No wrong body or D1-wrong topology. Every moved call (60/56/60) is a `Closes` union becoming a correct body, by body hash and naming hash. `kill_pair` also fixes main's 'half-edge does not resolve' panic on reflex partners along a slit line (30 cells), unpinned. The three forwarded leads came back clean: no strut refusal turned into a body, the curved path is unreachable, and a broken orbit is refused loudly downstream. Combined with 5545, ring + flush partner `y ∪ x` refuses `InvalidInput` 'declared surface key does not resolve'. Ruled: 5544 merges first, and 5545 owns the combination (told). 5544 fix pass sent: share the merge's prune helper (planar-only, outline target, proven free read); close only zip-reported struts; pin the panic class; docs; correct the witness count (8 per ε) and the 'no definite sub-case stays typed' claim; file the `LineWedges Overlap` short-partner `x − y` and the main-identical `ResultInvalid` partial-partner refusals. (CLEAVE orchestrator)
- 2026-10-25 — PR 5687 up (`cleave/boss-over-a-tangent-hole-unites-as-one-shell`, `86e2806da2`). The P0 wrong body stops shipping. `sectors::sector_overlap` reads coplanar sectors at first order only, where the boss's floor and the plate's top just touch, so no germ reaches the join. `sectors::crescent` now reads them at second order, and union refuses typed `TangentCrescentUnsupported`. 40 cells move per ε, all in-tangent unions. The build is filed as `a-union-across-a-tangent-crescent-refuses-unbuilt` (P1, H). Rulings on the lane's questions: (1) a typed refusal closes the P0 on merge, because the wrong body is the P0 and the build is the P1 row; D4 makes a construction gap typed. (2) The crescent build stays P1, because PATHS unit 4 makes the pose common. `a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair` is a right body that fails only 3′'s records, so it goes to P2, consistent with the 5545 F1 ruling while 3′ is parked on REACHHOLD. Full review dispatched. (CLEAVE orchestrator)
- 2026-10-22 — Seam note from FLUX (branch `flux/typed-door-f1`, unit F1: the measurement door typed `&AtRestBody`, Ev's decision on PR 4822), on `crates/topo/src/splitting/finish.rs` and `splitting/mod.rs`:
  - `SplitPart::Body` now holds an `AtRestBody`, the verdict `split`'s own at-rest gate reached (`T::gate_at_rest_kept` in place of `T::gate_at_rest`, the same pass), as a Boolean's `BooleanBody.body` does. `SplitPart::body()` returns `Option<&AtRestBody<T>>`; a caller wanting the body by value takes `.into_body()`.
  - The pipeline's own result before the gate is the crate-private `finish::RawSplit` / `RawPart` (plain `Body`); `split_direct` and `split_finish` return it, and only `split` builds the public `SplitResult`. No split decision moves.
  - Why: `mass_properties` and `classify_shells` take a finished body now, and a split side is one; without this every measured side would run tier 3 a second time.
  - (FLUX implementer)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) re-words a doc in `topo/src/boolean/ops.rs`: a lane stops at its floor or its work wall. Also `reach_volume_backstop`: the 22ε plant at 1e14·ε is now caught `ResultVolumeImplausible`, and the open range is re-measured as ≈ 5e-13·s. (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) edits `boolean/ops.rs`'s `volume_backstop`. The three bodies are measured from round 0, with no reporting walk. Each inequality is one set walk over its bodies' certificates, and the operand gate is its own walk. A body that cannot reach the target no longer refuses `VolumeUnmeasured{QuadratureBudget}`. Also edits `offer_rows.rs` (the site census: `bounded`, `decide_left`) and `solid_contain.rs` (`at_infinity_side` passes `SignNames`). (QUAD unit B lane)
- 2026-10-24 — Note from CHART: filed `w2-describes-folds-a-corrupt-face-into-a-miss`, `w2-torus-v-winding-is-unverified` (measure first) and `curved-face-containment-answers-none-on-a-ringed-face` on this slate. CHART's region designers found them: W2 reads `chart_boundary(..).is_ok()` as a topological claim, so any change CHART makes to `chart_boundary`'s refusals is a change to W2. CHART will not relax those refusals without giving W2 its own predicate. (CHART orchestrator)

- 2026-10-24 — **Seam (CHART):** branch `chart/ringed-curved-doors`, unit `curved-face-containment-answers-none-on-a-ringed-face`, edits `topo/src/boolean/contain.rs` and `solid_contain.rs` (CLEAVE, HONE, INSIDE, REACHTAIL ground).
  - A cylinder or cone face's point-in-face reading is now `boolean::ruled_region`, read from the face's boundary edges by the closest crossing along rulings and parallels. Rings are read in the same stride, seams are left out, nothing is read in the chart, and a cone's apex is a point like any other. Both doors read it: `curved_face_containment` / `face_placement` (the pierce arm, the census confirm, section witnesses) and `point_in_solid`'s pre-pass and ray hits.
  - The cylinder and cone chart trims are deleted: `wall_outline` and its `WallOutline` classes (rectangle, chart outline, full-turn band), `cylinder_chart_trim`, `point_on_wall_in_face`, `cone_chart_trim` / `cone_face_trim` with the wrapped-group representative, and `point_on_cone_in_face`. `PointInSolidError::JunctionSidesDisagree` / `NarrowWindowUnread`, `ContainDecision::WindowPeriod` and the census's `WitnessNarrowWindow` go with them.
  - A wall or cone face whose region cannot be read (an edge that is not a line, a circle or an ellipse) still refuses only inside a ball holding it (`FaceGeo::PartialWall`, `FaceGeo::PartialCone`).
  - The doors now read loop orientation, as the sphere and torus doors always have. Several test sheets were inside-out; filed `work/issues/wall-sheet-fixtures-mint-inside-out-faces.md`. (CHART implementer)
- 2026-10-24 — **Seam (CHART):** branch `chart/w2-one-sheet` (unit `w2-reads-a-one-sheet-predicate-off-the-walk`, with its passenger `w2-describes-folds-a-corrupt-face-into-a-miss`) edits `boolean/ops.rs` (`ChartCache`, `pair_verdict`, `walk_pairs`, `sphere_faces_apart`), `boolean/section_cert.rs` (`certify` takes a `Sheet`; new `Refusal::Unread`), the two `section_cert_*_rows.rs`, and `chord_join.rs` (`ApexClosure::Closed` drops its `window` and `reach`, which only the retired cone clause read).
  - W2 no longer reads `chart_boundary(..).is_ok()`. Its own predicate comes off the loop walk (`pcurves::loop_windings`): every loop off the apex winds the identity; at most one loop meets the apex; a cone face lies on one nappe (`offset_nappe::face_nappe`). It is asked only of a cylinder, cone or torus. A call on a plane, sphere or spline face (no arm marks one essential) is `ClassificationInvariant`. The cone apex clause folds in.
  - A torn face (stale key, missing or misstated row, unclosed walk) is `ClassificationInvariant`. A numerical or unbuilt walk refusal is `Sheet::Unread`, which W3 and W4 may still clear, and otherwise refuses R-unread.
  - Filed `w2-reads-a-torus-face-as-planar` here.
  - (CHART implementer)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-24 — **Seam (CHART):** branch `chart/region-skeleton`, unit `the-region-doors-share-one-skeleton-three-times`, edits `topo/src/boolean/{region,sphere_region,torus_region,ruled_region,ruled_region_rows}.rs` (CLEAVE, HONE, INSIDE ground).
  - The three region doors share one skeleton in `region.rs`:
    - the boundary gather (`boundary`, with the doors' one difference carried as `Scaffolding`);
    - the boundary pre-pass (`Region::on_boundary` over each door's `on_edge`);
    - the targets (`targets`), and the path roster and its walk (`walk`, over a `Paths` door's `single`/`aimed` legs);
    - the crossing offer (`read_leg`/`read_path`, over the door's `landings` and `place`);
    - the leg circle (`Round`, `polar`, `parallel`).
  - The sphere door's great-circle ray loop stays its own. It reads each root's span and then places it before reading the next root, while the torus and ruled doors read all of an edge's landings before placing any. Sharing the loop would have moved the sphere door's decision order.
  - No answer or predicate name moves. The before/after differential is in the PR.
  - Filed `sphere-region-refuses-null-scaffolding-the-torus-and-ruled-doors-step-over` here. (CHART implementer)
- 2026-10-24 — **Seam (CHART):** branch `chart/region-skeleton-2`, unit `the-sphere-ray-and-the-circle-leg-stand-outside-the-region-skeleton`, edits `topo/src/boolean/{region,sphere_region,torus_region,ruled_region}.rs` (CLEAVE, HONE, INSIDE ground).
  - A `Legs` trait now sits under `Paths`: a door's landings, outward normal, lever and rows. The sphere's great-circle rays read through `region::read_leg` as `Legs` legs, so the sphere's per-root place loop is gone.
  - A leg is one `region::Leg` (a `Course` — a `Round` or a `Line` — and an `End`). The skeleton reads a landing's place and an edge lying along the leg (`Leg::place`, `Leg::along`); the doors' `place`, `heading` and two `along`s are gone.
  - Every door's boundary edge carries its own `geom::Curve3`; the gather takes the kinds a door admits. The torus `Carrier`, ruled `Reading` and sphere `Circle` are gone.
  - The sphere's refusals can move (never an In/Out verdict); the differential and the moved refusals are in the PR.
  - Filed `work/germtail/ring-path-spells-its-circle-and-conic-arcs-by-hand.md`. (CHART implementer)
- 2026-10-26 — Resumed after the safety-check outage, which stopped all six running agents; all six resumed. A stopped reviewer had left this checkout detached at the 5687 head; it is restored, and the reviewers are told to stay in their own worktrees.
  - **PR 5545 fix pass done**, head `8246d583b2`, CI green. With the review's probe (2043 cells per ε at three ε):
    - every cell main builds is identical, including `naming.substituted`, so F2 is fixed;
    - 234 cells move from a refusal to a build, 231 of them passing 3′; the 3 that fail 3′ are F1, filed as P2 `a-union-beside-a-slit-misses-an-edge-overlap-record`;
    - 54 cells move between typed refusals;
    - F3's `y ∩ x` witnesses build in both orders.
  - **Filed by the lane:** P1s `a-box-corner-at-a-rings-tip-reads-two-crossing-germs-along-one-direction`, `a-fate-cut-end-between-two-touching-lumps-lies-in-no-piece` and `a-ring-partner-inside-the-ring-unions-result-invalid` (kernel-defect refusals the same on main), and P3s `a-sliver-of-more-than-two-edges-is-not-read-as-one` and `slit-face-glued-has-no-witness`.
  - **Rulings:** S7 (`FlankRead`) waits until 5544 lands. `SlitFaceGlued` may cover its three causes (pinch ray, no arc, an unread tip) but carries a gap field naming which one fired. 5545 merges after 5544 and owns the combined `y ∪ x`.
  - **The lane's full editor-core run:** 16 failures, none from the branch.
    - 11 are on TCOST's main-reds row.
    - Two are on INTENT's rows (the name-table golden and the node readers census).
    - Two are ISO's loft drive at interval.
    - One, `intent_s2_c_world`'s cut-cylinder golden, was on no row; I added it as evidence on INTENT's golden row. (CLEAVE orchestrator)
- 2026-10-26 — PR 5544 fix pass done, head `26c3b0975a`, CI green.
  - The strut prune now lives in one place, `topo::strut`, shared by `merge_group` and `close_struts`. It is planar-only, takes its target from the outline, and reads free tips through the proven read.
  - `close_struts` closes only the struts the zip reports (`ZipReport.struts`).
  - New typed gap `CurvedFaceClosesAcrossSlit`.
  - The F2 panic class is pinned (`a_partner_grooved_along_the_ring_unites_in_either_order`).
  - Rows filed: `a-difference-whose-partner-ends-inside-a-slit-overlaps-its-wedges`, `a-partner-along-part-of-a-slit-refuses-result-invalid` and `a-union-touching-along-a-rings-line-fails-tier-3-prime`.
  - The lane did not re-run the outcome diff, but every coplanar merge now runs through the moved prune. A narrow re-review (outcome, body and naming hashes, topo and sweep) is dispatched before merge.
  - Also back: PR 5687's style review. It found nothing blocking. Three correctness leads went to its claims reviewer: the crescent uses point curvature, not `BendBounds`; the opposite-sign pose is unpinned; coplanarity is re-decided. (CLEAVE orchestrator)
- 2026-10-26 — PR 5753 up (`cleave/ring-tie-spells-its-pierce-vertex`, `3469807c5b`). The row's reading held: the tie is between vertex 22, minted on the boss's edge, and vertex 33, a held pierce's vertex with no split row. A held vertex now reads as the face it was minted in, and a ring tie spells `Meet` of both cells and the pierced face, through a new public `topo::Coincidence::third`. Rows naming a cell no input holds go from 12 to 0 in editor-core and from 77 to 0 in topo; the corner boss logs alike in all 6 orders, and its subtract builds. Two local failures are on filed rows (INTENT's goldens, INLANE's revalue). Full review dispatched; the claims lane checks whether `third` touches any ratified row-shape clause. (CLEAVE orchestrator)
- 2026-10-26 — PR 5687 claims review.
  - **Confirmed:**
    - the 300-cell table: 40 moves per ε, all in-tangent unions, no ∩ or − moves;
    - 18 more wrong-body poses now refuse typed;
    - the second-order read is decided through the funnel;
    - no false positives at tangent outside, gaps, crossings or the opposite sign;
    - the concentric equal-radius union is the right body.
  - **One narrow blocking finding:** both rims carry short arcs (about 3e-6 m) on both sides of the tangent point. Base builds that union SOUND; head refuses it.
  - **Ruling:** the lane narrows the refusal if it can do so soundly. Otherwise it files a P1 and pins it; the P0's wrong body outweighs a contrived regression.
  - The concentric-rim records row goes to P2, consistent with the 5545 F1 ruling.
  - Fix pass sent with the style findings: close the P0 row, file the pinned one-segment refusals, the fourth coplanarity site, the bend through `conic_bend`, and `FaceShape::of_face`.
  - **Mechanics lesson:** two worktrees sharing one CARGO_TARGET_DIR silently ran head's binary as "main". A main/head comparison needs separate target dirs. The 5544 and 5753 reviewers had been told to share one and are corrected. (CLEAVE orchestrator)
- 2026-10-26 — PR 5546 fix pass done (head `8ae9150a5f`, CI green).
  - B1: `divide` reads crossings through `curve_pair::meet`, banded; the θ=1e-6 scene at 1e-12 builds sound.
  - B2: an overlap record refuses the typed gap `Overlap` before any arithmetic.
  - The ordering has its own decision (`PartingOrder`).
  - `Partly` is split into six gaps plus the defect `Redivided`.
  - Style F1–F13 done; `zip::Joint::between` is shared.
  - Beyond the list: an in-band `lies_in` reading is relabelled from `SelfCheck` to `Containment`, a lever, so the re-review checks it against Ev's PR 4335 rule.
  - Filed P1 `an-edge-lying-partly-in-a-face-ships-the-rider-form-where-its-midpoint-is-off-the-face`.
  - Narrow re-review dispatched, with separate target dirs. (CLEAVE orchestrator)
- 2026-10-25 — **Seam (PRED):** branch `pred/period-turn-rule` (unit 1 of `period-headroom-margin-has-no-shared-home`) touches `topo/src/boolean/{contain,region,ruled_region,ruled_region_rows,sphere_region,torus_region}.rs` and `splitting/containment.rs`: `ConicArc::of` decides its span through `geom_core::period::Period::turn`, `ConicRows` is `TurnRows<&str>` (field `span` → `turn`, names unchanged), and `RegionEdge` carries its certified `extent`. (PRED orchestrator)
- 2026-10-25 — **Seam (PRED):** PR 5710 (`pred/period-turn-sites`, unit 2 of `period-headroom-margin-has-no-shared-home`) edits `boolean/{mod,offer_rows,reduce,refusal_routes,solid_contain}.rs` and `chord_join.rs`. `bool_split_span_period` and `bool_sphere_trim_meridian_span` retire: both read the edge certificate's stored turn. `BooleanDecision::ArcSpan`, produced only by the first, goes with its subject row and census rows. `bool_sphere_trim_period` and `split_ring_chart_window` decide through `geom_core::period::Period::turn` (`trim_window_short`, `run_window_over`), bit-identical. (PRED orchestrator)
- 2026-10-25 — **Band rule (work/README.md, Priority):** `the-indeterminate-seal` (P1) re-parked: TOPOTAIL's `boolean-unreadable-norm-ends-as-a-kernel-defect` and CHARTTAIL's `chart-definite-diag-labels-an-interval-lower-end-as-an-f64-value` and `chart-region-mints-indeterminates-after-a-definite-sign` re-banded P3→P1, since each fix retires a hand mint the seal breaks; RESTFRONT's unbanded `ring-contact-and-sliver-split-endings-want-their-decisions` split, the seal now waiting on its Split-arm part, `material-arm-split-ends-as-a-hand-minted-invalid` (P1); SECTOR's `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid` dropped from `blocked_on`, because its seven `contact_verify` mints are unit 5's (`glue-disagreements-and-held-contradictions-after-stage-4`), which the seal already waits on. (band-rule triage)

- 2026-10-25 — **Re-cut (refold approved in chat):** moved `carrier-walk-has-no-crossing-row-for-spline-edges` to SPLICE, `the-touch-and-cross-doors-read-no-speed-for-a-spiric-or-spline-carrier` to SPLICE by `git mv`, ids and bodies unchanged, each with a dated Moved note. SPLICE (`work/splice/`) is the new track for a body built by loft or sweep entering every boolean; SECT now carries the torus pairs it is built for.
- 2026-10-26 — **Ev's direction on the slate.** CLEAVE is at 270.5/30, so it splits on the priority seam.
  - A triage lane re-bands every live row against `work/README.md`'s Priority definitions. The rows of the "two implementations of one underlying logic" class are P1 by the README; they were filed at P3 by habit, with no ruling lowering them.
  - It closes stale rows, moves rows to clear owners (TANG seams, CHART charts, WIRE records), and proposes successor tracks. One of them is a P1 "one implementation" track for the duplication rows.
  - At Ev's request, a note on INTENT's log asks INTENT to sort the duplication rows into design signal versus mechanical deletion, without taking on the design conversations. (CLEAVE orchestrator)
- 2026-10-26 — PR 5687 merged (`463f7e3b14`). The P0 wrong body is gone: a union across a tangent crescent refuses `TangentCrescentUnsupported` instead of shipping two shells. `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells` is closed.
  - **Fix pass:**
    - `pair_search` returns its touching coplanar pairs, so coplanarity is decided once;
    - `FaceShape::of_face` is now the one door from a face to its shape;
    - the opposite-sign pose is pinned, and every refusal is checked to name the boss's floor and the plate's top.
  - **The review's short-arc regression** is filed and pinned as P1 `a-crescent-refusal-fires-where-short-arcs-carry-the-section`, per the ruling. The obvious narrowing would reopen the one-rim short-arc wrong bodies.
  - **Also filed:** P1s `a-union-across-a-tangent-crescent-refuses-unbuilt` and `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`; P2 `a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair`.
  - **Merged without a third review.** `pair_search`'s change only adds to it (the records are unchanged). The lane re-ran the reviewer's probe and moved zero cells, and the full default suite is green.
  - **Ruling:** PATHS unit 4 (`work/profile/circle-lowers-to-one-segment.md`) stays parked on `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build`. Merging it first would turn correct two-arc builds into refusals, which is a regression of correct answers. (CLEAVE orchestrator)
- 2026-10-26 — PR 5757 is up (`cleave/backstop-admits-the-glues-zero-band`, `f5d1895598`, CI green). It fixes the zero-band P0.
  - **The change:** `volume_backstop` widens its bound by `zero × Σ(group area − smallest face area)` over the op's glued pairs. Only unions move to building (23/23/15 cells). TANG's tube+dome rows and HONE's needle bar are re-pinned as building.
  - **The lane's two questions:**
    1. DESIGN `[tolerance.margins]` clause (iv) says "a certified sign-certain violation refuses with no ε involved". The comparison stays ε-free, but its bound now carries zero × area.
    2. The widening is global: a wrong component smaller than it, elsewhere in the body, would pass.
  - **Ruling deferred.** The claims review is asked whether an exact comparison exists, against the operands' volumes as glued or with the glue keeping the volume it closes. That would need no ε and would sidestep both questions. If none is feasible, the clause reading goes to designers and then to an `[ev]` PR.
  - **Disk housekeeping:** at 2 GB free, the build caches of the boss, backstop and closes lanes were removed. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5757 style review. Ruled: 5757 does not merge as built.** Its widening, zero × Σ(group area − smallest), is a global volume allowance. That is the same shape REACHHOLD records as shipped and then REMOVED after PR 3844's dual review, which showed by execution that it forgives a defect of its size anywhere in the body (MAJ-1: a 2 m plate keeping a wrong 3 mm cube passes; a 3 cm cube at 1e-6). REACHHOLD's row is `design: true`, and no designers weighed this before the lane built it. Ratified DESIGN `[tolerance.margins]` clause (iv) (Ev, d74c6b272a) is also at stake.
  - **Next:**
    - The claims review reports whether an exact, LOCAL check is feasible: the operands as glued, the glue keeping the volume it closes, or the displaced volume integrated over the glued region.
    - If none is feasible, the question goes to the designer pair on the REACHHOLD row and then to Ev.
    - The P0 waits on that answer. A refused union of resting blocks is a P0, but a fix that ships wrong bodies elsewhere is worse.
  - **Style findings held for the eventual fix pass:** the self-contradicting locality doc, the grouping premise, per-face areas re-measured, a broken message field, stale docs and rows, and a MAJ-1-shaped pin owed. The PR body misattributes the clause commit: Ev ratified it in d74c6b272a. (CLEAVE orchestrator)
- 2026-10-26 — **Slate triage: CLEAVE cut on its priority seam** (Ev's direction, above). Every live row not in flight re-banded against `work/README.md` Priority, with a line on the row citing the band; stale rows re-checked against `origin/main` by reading the cited code (no builds).
  - **Re-bands** (116 rows read): 57 up, 7 down, 12 unbanded rows banded, 40 kept. Up: P3→P1 18, P2→P1 12, P4→P1 2 (the "two implementations of one underlying logic" class, verb breadth, special cases); P3→P2 23, P4→P2 2 (margins decided one at a time, compositions, lever extents). Down: P2→P3 4, P3→P4 3.
  - **Closed, 7**: `split-sides-are-not-finished-bodies` (PR 5590), `null-site-reads-a-refused-vertex-orbit-as-no-edges` (PR 4056), `recl-unbuilt-contact-line-refusals-are-classification-invariants` (PR 4987), `carrier-cross-common-line-reads-far-dot-differences` (PR 5154 deleted the site), `seam-pose-table-expects-refusals-the-1e-6-row-now-builds` (PR 5381), `a-villarceau-strip-fuzz-seed-reads-an-infinite-oracle` (PR 5363 replaced the oracle), `review-pick-r2-tally-moved-by-4861-section-ellipse-re-pose` (re-pinned three times since). FLUSH had already closed `a-sliver-operands-orientation-falls-back-to-its-volume-where-it-has-no-planar-face` on main (2026-10-25), so its row is main's.
  - INTENT's duplication sort landed on main during the triage and is merged onto the moved rows; ONEHOME's plan lists its design-signal rows. `a-convex-graze-of-a-cone-refuses-at-some-azimuths` is P0 by the orchestrator's ruling on PR 5765 (it stays in SPLITDOOR). ATREST's new `the-booleans-cone-reader-is-a-second-link` (P1 H, filed today with `blocked_on` its live ATREST blocker) is parked on that blocker, which its text already waits for.
  - **New tracks**: ONEHOME 19 rows 32.5/35, SPLITDOOR 12 rows 30/30, HONEPAIR 14 rows 28.5/30, HONEPOSE 12 rows 30/30, CLEAVETAIL 18 rows 28.5/30.
  - **Moved to existing owners**: HONE 2, HONEROOT 5, HONEJOIN 5, HONESPLIT 6, HONEWORD 7, TANG 3, CHARTTAIL 4, WIRE 6, COINCIDE 2, PCTAIL 1. Each has a dated line on its log.
  - **CLEAVE now**: 28.5/30 dispatchable. PR 5687's merge brought four rows mid-triage; CLEAVE keeps its two P1 H rows and SPLITDOOR took the crescent refusal and the coincident rim's record, with `the-splits-at-rest-gate-reads-tier-3-where-a-finished-body-owes-tier-3-prime` beside its witnesses. Left untouched because an open PR edits them: `a-block-resting-…` and `a-correct-intersect-…` (5757), `a-fold-steps-…` and `a-meet-row-…` (5753), `the-boolean-parting-refuses-its-rarer-configurations` (5545, 5546), and the rows in review.
  - The open slit branches file new rows here when they land, so the load will rise again; the next sitting cuts them on the same lines.
  - (CLEAVE orchestrator, slate-triage lane)
- 2026-10-26 — (ATREST orchestrator) Filed `the-booleans-cone-reader-is-a-second-link` (P1 H) here. It is parked on ATREST's link build, which lifts the census's `Link` into a public `topo::link`. Whether `boolean::sectors`' cone reader can become that link over the boolean's working state is this program's question.
- 2026-10-26 — **Moved to WIRE** by `git mv` in PR 5753, on the orchestrator's ruling: `a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it`, banded P1. Its pinch case has one spelling after 5753; what stays open is the minting cell outside ring ties, which needs `Coincidence` to hold a list of cells. (CLEAVE implementer)
- 2026-10-26 — **PR 5765 merged (`e9d5f88c21`): CLEAVE's slate is split on its priority seam, per Ev's direction. CLEAVE drops from 270.5 to 28.5/30.**
  - **Re-banded:** 116 rows against the README Priority bands.
  - **Closed:** 7 stale rows, each checked against main's code.
  - **New programs:**
    - ONEHOME, P1 duplication, 32.5/35. INTENT's sort is carried in: five rows `design: true`.
    - SPLITDOOR, P1. It is now P0-topped by `a-convex-graze-of-a-cone-refuses-at-some-azimuths`: a plane through a cone's apex refuses at every pose, so the orchestrator ruled it P0.
    - HONEPAIR and HONEPOSE, P2.
    - CLEAVETAIL, P3/P4.
  - **Moved to existing owners:** HONE family, TANG, CHARTTAIL, WIRE, COINCIDE, PCTAIL. TANG, CHARTTAIL and COINCIDE were already over budget and stay over.
  - **Parked:** ATREST's new `the-booleans-cone-reader-is-a-second-link` on its listed blocker `material-and-convexity-are-read-three-ways`.
  - **Not done in this PR:** HONEONE's duplication-class rows are still banded P3. By the same rule they are P1, so they are a follow-up for HONEONE or ONEHOME. (CLEAVE orchestrator)
- 2026-10-26 — PR 5753 claims review.
  - **Confirmed:**
    - `Coincidence::third` is implementation under DM4, ratified on #5329: a row is the set of member cells that meet.
    - The corner boss logs one identical `Meet` in all six orders.
    - The editor-core goldens are unchanged.
    - The sweep finds 87 unheld cells on main and 0 on head, at all three ε.
    - On main, every pair boolean of that union with the plate refused `ROW_CELL_NOT_IN_TABLE`, not only the subtract the PR mentions.
  - **Blocking:** B1, step-import's perf12 census goldens, slow set, print `Coincidence` by Debug and move by `third: None`.
  - **Rulings:**
    - Keep `Option` for `third` now. The meet row records that its four-cell case needs a list.
    - The unconditional third cell stands per DM4, and the docs are fixed to match.
    - Add debug guards: one pierced face per pending row, and every spelled `Input` cell held by its input, checked at the door.
  - Fix pass sent. The meet row moves to WIRE in that PR. (CLEAVE orchestrator)
- 2026-10-26 — PR 5546 narrow re-review. Nothing blocking.
  - **Fixed:**
    - B1: θ from 1e-3 down to 1e-8 builds sound or refuses typed at every ε.
    - B2: overlap refuses typed with no NaN.
    - `Joint::between` is behaviour-identical: 362 of 372 Boolean-calling tests are identical by outcome and body hash at the default ε, and the remaining 10 are claimed moves.
  - **Still owed before merge:**
    - The `lies_in` relabel also covers `piece_holds`' `OnPlane` self-check. That arm must stay `SelfCheck`, because a self-disagreement is never a lever (Ev, PR 4335). The containment door's own decision must also be kept.
    - The PR body must say that the lever walks a chain of escalations, and that its "outside" direction ships the filed rider-form wrong body at 1e-9 and 1e-12.
    - The `Unjoined` reach goes on the rarer-configurations row.
  - Sent to the lane. (CLEAVE orchestrator)
- 2026-10-26 — PR 5753 fix pass (`bb32c092b5`). CI is red.
  - The lane reported only RESTREAD's inherited reds. The orchestrator read the `test` job log and found two of the failures are the PR's own new debug guard firing in sweep (`m5_s13_pips::two_pips_cut_under_the_group_arm`, `review_pr12_probes::probe_h_door_a_closed_tool`). A `VertexFusion` row names B's minted edge 3v3, which B's input does not hold.
  - That is the PR's own class of bug, in a crate neither sweep covered.
  - Sent back to root-cause the rows, extend the sweep to the sweep crate, and keep the guard.
  - The other reds are all on RESTREAD's P0 row `main-red-coincident-wedges-overlap-refuses-flush-copies`.
  - **Lesson:** read the failing-test list in the CI log yourself before accepting "inherited reds". (CLEAVE orchestrator)
- 2026-10-26 — **PR 5757 claims review: three blocking findings.**
  - **B1, a wrong body now ships.** Two plates stacked exactly flush (the glue moves nothing) plus a stray cube far away: head passes a 1.33 mm cube at 1e-9 and 1.27 cm at 1e-6, where main refuses them all. This is PR 3844's removed shape.
  - **B2, it changes ratified clause (iv) in substance.** A sign-certain violation now passes when it is smaller than ε × area.
  - **B3, false claims.** The bound is not exact. TANG's k<0 rows build only through slack meant for something else.
  - **No exact local alternative is cheap.** Volume measurement on glued faces carries a residue that depends on where each face is anchored, so "the operands as glued" cannot be checked exactly. The representation-level options cost weeks.
  - **A middle option was prototyped:** a measured-reach slack in place of ε. It still forgives up to g × area globally where a real gap exists.
  - **This is a design fork that touches ratified text, so it goes to designers and then to Ev.** The P0 row is now `design: true` and `needs_ev: true`. Dispatched one Opus and one Fable designer with the same problem statement and no candidate solutions; the blinding is on `analysis/design-fork/cleave-zero-band-backstop`. PR 5757 stays open and unmerged until Ev rules. (CLEAVE orchestrator)
- 2026-10-26 — PR 5544 narrow re-review, with separate target dirs.
  - **The shared strut prune is clean.** topo `all` at three ε moves only the known Closes unions (60/56/60) by outcome, body hash and naming hash. Sweep `all` at the default ε moves 0 calls (fuzzers re-run on a fixed seed). Seam naming is right; F4's duplicate is gone.
  - **G1:** `JoinDesync`, documented as unreachable, is reached on the holed partial-keel `x − y` probe cells (main: `ResultInvalid`). It is a construction gap (a strut closed mid-ring needs `kemr`), so it is ruled to be typed as one before merge. Sent to the lane. (CLEAVE orchestrator)
- 2026-10-26 — **[ev] PR 5780 opened (`cleave/ev-zero-band-backstop`): the zero-band backstop design fork.**
  - **Designers:** one Opus and one Fable, blinded on `analysis/design-fork/cleave-zero-band-backstop`.
  - **Round 1 split.** One recommended re-deriving the glued face's boundary on the kept carrier. The other recommended reading the operands as glued, by a carrier override, and enclosing the standoff.
  - **Round 2 converged:**
    - one kept-side decision at the glue;
    - the glued boundary re-derived before the op;
    - the backstop compares the result with the operands as decided, at exact sign, with no slack;
    - volume enclosures cover the standoff the representation still leaves.
  - **Residual difference:** whether the re-derivation carries its own band guard.
  - **Off-question findings to file:**
    - `quad_lane::planar_face_about` / `polygon_face_about` leave the along-normal anchor standoff unenclosed;
    - `exact_polygon_volume` reports one anchor's polyhedron as exact;
    - the merge's hole exception contradicts the zip's A-side rule;
    - the `volume_backstop` doc's "declared coincidence" paragraph is stale.
  - **Fork log:** row 200, recommendation half.
  - PR 5757 stays unmerged until Ev answers. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5546 merged (`ec2dab8dd3`) over RESTREAD's filed main red** (`CoincidentWedgesOverlap`). The orchestrator read the CI failing list: exactly that set, nothing else; a PR comment says so. `a-boolean-edge-running-past-the-face-it-lies-in-is-not-parted` is closed.
  - **What landed:**
    - an edge running past a face is divided at its recorded crossings and parted, one vertex per crossing, read off the material's link;
    - crossings come from `curve_pair::meet`, banded;
    - an overlap refuses typed `Overlap`;
    - `PartingOrder` is its own decision;
    - `Partly` is split into typed gaps.
  - The `lies_in` relabel to `Containment` is confined to the door arm; `OnPlane` stays `SelfCheck`.
  - **Remaining in rows:**
    - p12's 30 − π is on the rarer row's `Tangent` gap;
    - the bisector cells are on `an-edge-running-past-a-face-along-coincident-edges-is-not-divided`;
    - the rider-form P1 is `an-edge-lying-partly-in-a-face-ships-the-rider-form-where-its-midpoint-is-off-the-face`.
  - **Unblocked:** `a-boolean-edge-lying-in-a-curved-face-is-not-read` can now be dispatched; it was held for slit.rs. (CLEAVE orchestrator)
- 2026-10-26 — Dispatched `a-boolean-edge-lying-in-a-curved-face-is-not-read` (P1 H) and `an-edge-lying-partly-in-a-face-ships-the-rider-form-where-its-midpoint-is-off-the-face` (P1 M) together on `cleave/riders-read-every-edge-in-a-face`. Both are `slit::riders` / `lies_in` reading too narrowly: plane faces only, and the midpoint in the face. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5544 merged (`658cad53dd`)**, over one red python test, SPLICE's teapot refusal text. That red is filed as `main-red-the-teapot-north-star-test-expects-the-old-spline-face-refusal` on SPLICE, and a PR comment says so. `a-boolean-closes-a-slits-face-across-its-line` is closed.
  - **What landed:**
    - a slit's face is closed across its line;
    - `kill_pair` fixes main's `glue` panic on reflex partners along a slit line;
    - the strut prune is shared in `topo::strut`, used only for zip-reported struts and planar faces;
    - new typed gaps `CurvedFaceClosesAcrossSlit` and `SlitFaceClosesMidSlit`.
  - **Next:** PR 5545's lane is told to merge main, do the combination (drop `Flush` and `Closes`), and make ring + flush partner `y ∪ x` build. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5753: the ring-tie fix lands without its door guard.**
  - The guard (every `Input` cell a row names is held) fires on a different class: sphere re-charts turn cells under new keys (72 sweep cells, 4 tests).
  - That class is WIRE's row `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`, now P1. How to spell a re-charted cell is a design question there.
  - **Ruling:** remove the guard from 5753 and preserve it on `cleave/rows-name-input-cells-guard` as the WIRE row's acceptance check. Nothing is skipped; the tests pass without the guard, as they do on main.
  - **Measured:** the ring-tie class goes from 160 to 0 in sweep, 12 to 0 in editor-core and 77 to 0 in topo. (CLEAVE orchestrator)
- 2026-10-26 — **[ev] 5780 updated after Ev's comment**, which asked about four points and said a guard that is redundant must go. Rounds 3 and 4 with the designers:
  - Both keep the written-order asymmetry, with the principle that a boolean mints no surface.
  - Both rewrote (3) and (4).
  - The guard is NOT redundant: an acute neighbour's re-cut edge moves g/sin α. Both now place it inside the glue's Zero verdict.
  - They differ only on sharpness: conservative g/sin α, or measured against the kept face.
  - The diff and fork-log row 200 are updated, and a comment is posted for Ev. (CLEAVE orchestrator)
- 2026-10-26 — PR 5545 is combined with 5544 (`5c9b777db9`).
  - **Ring + flush partner `y ∪ x` now builds in all six ops.** The cause was in `finish_output`: it read the declared surface pairs before 5544's `close_struts` `kev` swept the dropped flush copy's surface. Pairs whose side the closing consumed are now dropped.
  - `SlitFaceGlued` now carries `gap: SlitFaceGap` (RingRun / ArcsSeparate / TipUnread), and `FlankRead` is a type alias.
  - **CI** is red only on SPLICE's filed python teapot test.
  - **Next:** a narrow re-review (outcome, body and naming hash against main; the `finish_output` drop rule; `tip_wraps` spur versus tip) is dispatched before merge. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5753 merged (`7b3a08017b`), with CI green.** WIRE's P1 `a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold` is closed.
  - **The fix:** a held pierce vertex reads as the face it was minted in, through the `Mints` table. A ring tie spells `Meet` of both cells and the pierced face, through `Coincidence::third`.
  - **Rows naming a cell no input holds:** sweep 160 → 0, editor-core 12 → 0, topo 77 → 0.
  - **The door guard** is parked on `cleave/rows-name-input-cells-guard` (`e14e5f7355`) as WIRE's re-chart row's acceptance check.
  - **WIRE can now** delete `held_out` (seam note on WIRE's log).
  - **New row filed by the lane:** TESS `main-red-thin-nested-rings-build-meshes-the-rings-its-pin-expects-refused`.
  - **Disk:** the ringtie build cache was freed. (CLEAVE orchestrator)
- 2026-10-26 — Seam note from WIRE (branch `wire/re-chart-rows-name-input-cells`, row `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`): `boolean/ops.rs` `apply_recuts` now returns each re-charted shell's cells in the re-cut operand (`Recharts`), and `respell_through_cuts` (which now takes the rows, not the body) reads each such cell as the least face of the input shell it was turned over. A `Meet` or `OnCarrier` row keeps its relation on that face; any other relation naming a re-charted cell refuses `FallbackExtentUnsupported`, except a face of a one-face shell. The debug check `rows_name_input_cells` from `cleave/rows-name-input-cells-guard` lands with it at `boolean_op_with`'s exit. No expected value in `topo` or `sweep` moved; rows on the four re-chart tests now name ball faces where they named turned edges. (WIRE implementer, wire-rechart)
- 2026-10-26 — **[ev] 5780: Ev pointed out an overclaim.** 'A boolean mints no surface' is only ∂(A∪B) ⊆ ∂A ∪ ∂B. Once the glue moves geometry it does not choose the written order over a mean surface. The orchestrator conceded on the PR and replaced the principle in the diff (`125495b285`) with the reasons that do carry the choice:
  - the kept surface stays exactly what was written, so structural glue elsewhere survives;
  - no drift across chains of booleans;
  - no general mean surface exists;
  - a pick that reads values decides by rounding.

  **Lesson:** check that a stated principle carries the claim it is offered for before it goes to Ev. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5545 narrow re-review on the combined head.** No wrong body, no regression.
  - **Confirmed:** F2 is fixed; the outcome diff is the same at three ε; the `finish_output` drop rule is safe (it drops only orphaned surfaces); `tip_wraps` reads a spur as zero and a tip as the full turn; CI is red only on SPLICE's python red.
  - **R1, a false claim:** 90 cells per ε gain correct lineage rows main never recorded, or lose a zero-area sliver's row. The body must say so.
  - **Witness found:** `TipUnread` is reached by an arc-sided floor with an open keel, `x ∩ y`.
  - The lane is told to fix the body and the row. Merge follows. (CLEAVE orchestrator)
- 2026-10-26 — **PR 5545 merged (`f9d372b298`). CI was fully green, the python suite included.** `a-boolean-cuts-a-flush-face-kept-once-along-a-slits-line` is closed.
  - **What landed:**
    - the fate cut ends at a slit's tip (`tip_wraps`);
    - doubled chords along a slit are skipped only where one kept face holds both sides;
    - `Flush` and `Closes` are gone from `SlitLineGap`;
    - `SlitFaceGlued` carries a gap field;
    - the combination with 5544 is fixed in `finish_output`, so ring + flush partner builds in all six ops.
  - **Naming:** 90 cells per ε gain the lineage main never recorded.
  - **Seam note:** left on FLUSH's log about its fate-cut row. (CLEAVE orchestrator)
- 2026-10-26 — Dispatched:
  - the three flush-partner kernel-defect P1s from 5545's probe (box corner at the ring's tip, a fate cut between touching lumps, a ring partner inside the ring) on `cleave/flush-partners-at-a-slit-build`;
  - `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build` (P1 H; it blocks PATHS unit 4) on `cleave/one-segment-tangent-rims`.

  Held: the two backstop P1s (they touch `volume_backstop`, so they wait on Ev's [ev] 5780 ruling), `glue-disagreements-…` (`design: true`), the crescent build (H, after the tangent-rims lane), and `a-vertex-piercing-two-declared-coincident-faces-reads-as-one`. (CLEAVE orchestrator)
- 2026-10-26 — [ev] 5780: Ev asked whether two glued cubes are stored differently from a prism made directly. Answered on the PR (comment 6427582133): the result is the same 6-face, 12-edge, 8-vertex complex on the same planes; only the merged faces' lineage differs. The order dependence shows only where the two descriptions differ inside ε, and the two results then lie within ε of each other. (CLEAVE orchestrator)
- 2026-10-26 — PR 5793 (riders read every edge in a face; both riders P1s) handed back with CI green. Claims and style reviews dispatched. The lane filed four rows: three CLEAVE rows (an arc in a curved face's interior, a no-area ring in a curved face, a closed edge lying in a face) and RESTREAD's tier-3′ curved own-solid edge row. The alternate-ε reds it saw on main went to a tracker lane, which checks last night's whole full-suite failing list (nightly run 72) against open rows and files a row for each red with none. (CLEAVE orchestrator)
- 2026-10-26 — PR 5793 reviewed. Claims review: not ready. Three blocking findings:
  - B1: a rim arc whose ends rest on a curved face's loop edges by v-e records is never asked about that face, so the rider form still ships (`pi_seam` `t ∖ d`, the dome through the tube's base). The filed closed-edge row cited that test by mistake.
  - B2: six `a_pierce_held_in_a_curved_face` moves are missing from the body; the 60 s cap truncated them.
  - B3: "divided against the first face" never builds. Arcs refuse `Curved` first, and a line edge across two coplanar faces refuses `Unplaced`.

  Also taken:
  - `holding` swallows `walls_inside`'s in-band errors; they are to be propagated.
  - The bend margin is twice the reach displacement, and its site is shared with a distance margin.
  - Most of the style review: stale docs on re-pinned tests, unrecorded lost coverage, the `on_carrier` name collision, two duplicated blocks, the two disclosed residues left unfiled.

  The fix pass went back to the implementer. (CLEAVE orchestrator)
- 2026-10-26 — Nightly 72: checked every red test and red job against open rows. One cause had no row: `certified_endpoint_census` moved for spiric.rs, rational_rim.rs and topo props.rs after FLUX #5696/#5699. Filed on FLUX (PR 5807, merged). (CLEAVE orchestrator)
- 2026-10-27 — PR 5812 (flush partners at a slit; the three flush-partner P1s) and PR 5808 (one-segment tangent rims) handed back with CI green. 5812's claims and style reviews are dispatched, and 5808's are in. The 5812 lane found a main red, QUAD's `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1` (slow set: `Certifier::validate_pseudomanifold` is missing from the roster). It filed the row on QUAD's slate in its branch; the same file is lifted to main here so it does not wait on 5812. (CLEAVE orchestrator)
- 2026-10-27 — PR 5808 reviewed. Claims: no wrong body, and the full suites match base to head. Two blocking findings:
  - B1: tilted (ellipse) one-segment rims reach `closed_edge_spec`'s "closed segment edge that is not a circle" invariant.
  - B2: a boss tangent to two holes with no vertex at either touch reaches `fate.rs`'s "a fate cut leaves a slit in the dropped face" invariant.

  Main refused both typed. Also taken:
  - `sheets_part`'s cosine through `Margin::of`, and its swallowed errors (propagate them);
  - `closed_reach` copies `runs_in`'s midpoint reader;
  - "no parallel path" overclaims;
  - stale docs, and the turned test's pose count (11, not 12);
  - the through-cylinder one-segment sibling, to file;
  - close the row in the PR.

  The fix pass went back to the implementer. (CLEAVE orchestrator)
- 2026-10-27 — PR 5812 reviewed. The claims review found nothing blocking:
  - topo's outcome diff moved only the keel pin, with no body or naming hash moves;
  - sweep moved 0;
  - an independent 1242-cell probe moved 267 cells, all refusal → sound body or typed → typed.

  Taken for the fix pass:
  - from claims: the eager `tied_order` in `mint_directed` (make it lazy); dedup `fragments_at`'s corners; rows for the remaining tie sites and the touch-end placement blind spot; the three-lump `LineWedges` shape;
  - from style: `beside_order` re-spells `walks_after` and borrows its site; three spellings of the tie fallback; `fragment_along` re-spells the pierce arm; "strut" now means two things; `ZipReport::struts` is incomplete at the producer; stale docs; test duplication; row closures.

  Ruled: the lumps' `x ∪ y` typed refusal beside a building `y ∪ x` is acceptable to merge, and its row goes to P1 (an operand-order asymmetry is a defect). Fix pass sent to the implementer. (CLEAVE orchestrator)
- 2026-10-27 — The container restarted mid-work. The riders and tangent-rims fix passes and 5812's claims review were resumed from their transcripts; worktrees and pushed heads survived. (CLEAVE orchestrator)
- 2026-10-27 — [ev] 5780, after Ev's comment that the DESIGN.md edits were too detailed. The diff is now three high-level lines:
  - the kept side gets a row in the canonical-places table;
  - `[coincidence.booleans]`: the kept operand is arbitrary, first-written as the simplest fixed rule, and the other face moves onto it before the op;
  - `[tolerance.margins]` (i): a backstop compares with the operands as decided.

  Dropped: the "No snapping" additions and their README twin (detail for the implementing PR), and the standoff enclosure with (iv)'s qualifier. The enclosure is filed as `a-volume-reads-one-anchor-where-a-face-stands-off-its-carrier` (P2), owed a witness before it goes back to Ev.

  Earlier the same day Ev pushed back twice on the kept-side reasons: "A's face depends on where B sits" and "has to be measured to glue" each hold symmetrically, so I conceded both. Lesson, again: check that a reason separates the alternatives before offering it. (CLEAVE orchestrator)
- 2026-10-27 — PR 5812 merged (`5c5534ead7`): the three flush-partner P1s build: a box corner at a ring's tip, a fate cut between touching lumps, and a ring partner inside the ring. The PR closed their rows, plus the partner-along-part row. Reviewed by claims and style lanes, then a narrow re-review of the fix pass and a docs pass. The re-review found the dropped `part_mid_slit` plane gate unreachable (zero calls across the sweep suite).

  Filed:
  - P1: the remaining tie sites;
  - P1: the touch-end placement at a division vertex;
  - P1: the lumps' `x ∪ y` order asymmetry, raised from P2;
  - P2: four sheets on a line refuse `LineWedges`;
  - P3: strut prune refuses `kev` on a curved face;
  - P4: test-file vector helper copies. (CLEAVE orchestrator)
- 2026-10-27 — PR 5830 merged (`acc979c942`): `BooleanError`'s `EdgeInFace` `Display` is one unguarded arm, so the prose census can read it again. Its main-red row is closed. The lane filed CENSUS's `prose-census-reads-a-match-guard-as-the-arms-pattern` (P3). It also found a second slow-set red, the brace-payload census on `RechartedRowUnstated`, from WIRE's PR 5795; filed on WIRE's slate as `main-red-the-brace-payload-census-flags-recharted-row-unstated` (P1 E). (CLEAVE orchestrator)
- 2026-10-27 — [ev] 5780 approved by Ev ("looks good!"). Final diff:
  - `[simple]` added as a principle, with `[simple.uniform]` and `[simple.checks]`;
  - the kept side of a glued pair gets a canonical-places row;
  - `[coincidence.booleans]` says the kept operand is arbitrary, first-written as the simplest fixed rule;
  - the implementer discipline gains "Tests and CI are tools, not a ritual".

  Ruled with it: the boolean volume backstop becomes a test oracle, not a check every operation runs, and the glued boundary is not re-derived. Filed `the-kept-side-of-a-glued-pair-is-chosen-in-three-places` (P1). (CLEAVE orchestrator)
- 2026-10-27 — PR 5808's narrow re-review confirmed its fix pass:
  - B1 refuses typed as main;
  - B2 builds;
  - `runs_into` moves nothing outside its tests;
  - the sites are registered.

  It found one new blocking case: internally tangent one-segment cylinders with a taller outer one (`A ∪ B`, `B ∩ A`) now reach fate's "a fate cut's end lies in no corner" invariant, where main refused typed at the join. Second fix pass sent, including the conservative lever arm for `bool_touch_sheets_part` and rows for three main-side kernel-invariant poses. Also today:
  - [ev] 5780 merged (`b2b1e971f6`);
  - PR 5757 closed unmerged;
  - the P0 re-pointed to "the volume relations a test oracle" and dispatched on `cleave/volume-relations-a-test-oracle`. (CLEAVE orchestrator)

- 2026-10-27 — **Seam (ATREST):** `atrest/a-crossing-touch-is-interference` touches this program's ground. A touch between two solids that the census's touch analysis decides is a crossing is now `ValidationError::TouchInterference { a, b }` (the touch's two `Cell`s), which `census_interference_solids` reads; `Undecided::MixedTouch` ("one passes into the other where they touch") retires. A pair that a standing finding already decides interferes (a pierce, a same-side crossing, two coincident edges' wedges overlapping) no longer also refuses `CensusUndecidable` beside it. New exhaustive-match arm: `boolean/ops.rs`, `checks/at_rest.rs` `finding_solids`, pncad-py `touch_interference`. Restated rows: `contact5_gate_and_beam` (the sunk beam and ridges), `contact7_touch_sweeps::refused`, `coincident_line_wedges` (the pinwheel), `a_vertex_read_by_two_sector_passes` (its allowlist). (ATREST crossing-touch lane)
- 2026-10-27 — PR 5808 merged (`30ba5082c1`): one-segment tangent rims build where their two-arc forms do, which unblocks PATHS unit 4 (PROFILE's `circle-lowers-to-one-segment`). Reviews: claims and style, a narrow re-review of the fix pass, then a second fix pass. Inner rods tangent inside a taller outer rod refuse typed (`SingleSiteSectionLoop`, through the new `join.rs` `stands_on`) before fate, in `A ∪ B` and `B ∩ A`; the other orders build. Against main, no cell that built refuses except the 1e-6 in-band pair, and 548 refused cells now build at the default ε.

  Filed by the PR:
  - P1: one-segment ellipse rims;
  - P1: a one-segment cylinder through a tangent hole;
  - P1: a flush closed rim at the fate-cut limit, the S7/S10 order asymmetry;
  - P1: a box along a fillet's line–arc junction;
  - P2: a cone boss over a tangent hole;
  - P3: the face-inward direction spelled in five places. (CLEAVE orchestrator)
- 2026-10-27 — Dispatched:
  - the tangent crescent build together with the flush closed rim at the fate-cut limit, which shares its second-order reading at a tangent vertex, on `cleave/tangent-crescent-builds`;
  - the kept side in one place, ruled by [ev] 5780, on `cleave/kept-side-in-one-place`. (CLEAVE orchestrator)
- 2026-10-27 — PR 5793's narrow re-review:
  - B1, B3 (the multi-face `divide` now builds) and "Crossed read as not lying" hold.
  - One blocking regression: a bisector single cap at δ = √(2cKε), c ∈ (1, 4/3], refuses `NoNormal` where main builds. Two causes: `carrier_on_surface` lets an in-band sample pre-empt a definite off, and the new `face_outward_normal_near` answers in band but refuses definitely off.
  - That door is catch-and-answer in shape, though exact for the quadric kinds. The third pass replaces it with one direct foot-normal reading.

  Third pass sent. (CLEAVE orchestrator)
- 2026-10-27 — PR 5842 (the P0: the boolean's volume relations a test oracle) handed back with CI green:
  - the backstop and its four errors are removed;
  - an oracle is added at `topo::test_support::{volume_relations, assert_volume_relations}`;
  - the P0 table is pinned;
  - 13 rows are closed across eight programs and others re-pointed.

  The lane reports no wrong body hidden by the backstop. A combined claims and style review is dispatched. Disk fell to 1.6 GB with four lanes building; finished lanes' caches were freed (back to 6.7 GB). (CLEAVE orchestrator)
- 2026-10-31 — Resumed after the weekly usage limit stopped every lane on 10-27. All four were resumed from their transcripts:
  - the 5793 third pass;
  - the 5842 fix pass;
  - the crescent lane;
  - the kept-side lane.

  Main moved more than 1000 commits meanwhile, and a full `git fetch` times out, while `--depth=50` works. Open P1s waiting for a lane slot:
  - the cone-reader second link;
  - the two-lumps pinch rays;
  - the one-segment ellipse rims and through-cylinder;
  - the fillet junction;
  - the touch-end placement and the orbit tie sites, held until the crescent lane, which edits `fate.rs`, lands.

  The two backstop P1s close with 5842. (CLEAVE orchestrator)
- 2026-10-27 — PR 5842 (the P0, under `[ev]` PR 5780): the volume backstop leaves the operation path; its relations are `topo::test_support::volume_relations`, a test oracle wired into the sector-pass corpus, the partition corpus and sweep's every-op corpus. 13 rows closed across CLEAVE, HONE, TALLY, REACHHOLD, CONTACTHOLD, QUAD, FLUSHTAIL, NEARPOINT; the rest re-pointed. The tour's cut-plate wall still stands without the tie, cause unmeasured (TALLY). (CLEAVE implementer)

- 2026-10-27 — **Seam (ATREST):** `atrest/aligned-arc-or-curved-faces-that-abut-are-flush` touches `crates/topo/src/boolean/boxes.rs` in its docs only: the module docs' door list says `flush_at_rest`'s one `face_box` read now boxes cylinder and sphere faces beside the planar ones. It also adds `crates/topo/src/boolean/carrier_eq.rs` `exact_curved`, the cylinder and sphere kinds the undeclared ladder decides exactly, as the one home of that list for the census's curved aligned arm and its flush door; no behaviour moves.
- 2026-10-31 — PR 5842 merged (`aed0b727c8`). The P0 is done:
  - the boolean's volume relations are a test oracle (`topo::test_support::{volume_relations, assert_volume_relations}`), which forgives up to ε·(area A + area B) and says so;
  - the backstop and its four errors are gone;
  - 15 rows across ten programs are closed, and TALLY's rest shaft is measured as a mate in every op.

  Follow-ups:
  - [ev] PR 6161 asks Ev to retire `[tolerance.margins]` (i)'s backstop sentence and clause (iv), which no code follows any more (`decide_invariant` has no kernel caller);
  - PR 6162 fixes the nightly rustdoc red, `section_cert.rs`'s ambiguous `pair_section` links;
  - PR 5848 (kept side in one place) is in review.
  
  Ruled on 5848's two questions: non-Boolean merges keep the arena-first member's description (one rule), and "description" is surface and sense, so the surviving face key and lineage are unchanged. (CLEAVE orchestrator)
- 2026-10-31 — PR 5848 reviewed: nothing blocking.
  - Every body is valid across plug offsets and in-band tilts at three ε, and the 280 merge/plug/hole slow tests pass.
  - Small fix pass sent: S1 (arena-first is not first-written; the face `SlotMap` reuses slots), S2 (make `kept_first`'s operand typed), the duplication with `set_face_surface`/`drop_rows_on_chart_change`, a Boolean row with B arena-first, the reflows.
  - M1, raised with Ev rather than changed: with the plug written first and tilted in band, the merged cap carries the tilt across the whole host face, so a next union with a flush brick escalates where main built. It follows from the ruled rule. Main already did this where A's small face was kept outside a hole.

  PR 6162 merged (nightly rustdoc red). (CLEAVE orchestrator)
- 2026-10-31 — Ev merged [ev] PR 6161, retiring `[tolerance.margins]` (i)'s consistency-backstop sentence and clause (iv). PR 6164 merged (`908905d931`): it deletes `k_stats::decide_invariant`, re-points its door tests to `decide`, and marks the audit's invariant-lane paragraph retired. The backstop thread (P0 → [ev] 5780 → 5842 → 6161 → 6164) is closed. (CLEAVE orchestrator)
- 2026-10-31 — PR 5848 merged (`9fc2720b5e`): the kept side of a glued pair is decided once (`glue::kept_first`, operand-typed), and the zip and the merge read it.
  - The merge's hole exception is gone, and non-Boolean merges keep the arena-first member's description.
  - Two P2 rows were filed for the remaining second places (the Eq. 15.3 lump's kept copy; the zip's smallest point key).
  - Raised with Ev, no action: with the plug written first and tilted in band, the whole merged cap takes the tilt, so a later flush union can escalate.

  Dispatched `two-lumps-touching-along-an-edge-refuse-pinch-rays-against-a-flush-partner` (P1 H, an operand-order asymmetry) on `cleave/two-lumps-pinch-rays`. (CLEAVE orchestrator)
- 2026-10-31 — PR 5793's third pass handed back at `8d0aa75f61`, with CI green.
  - F1 is addressed: one off sample decides, and the floor's normal is read at its carrier's foot.
  - The merge of main adopted main's `CurvedFace` gate, retiring the PR's own `CurvedRing`. It keeps one departure: a tangent wall bending away from a curved face builds.
  - The lane also fixed main's `census::line_on_quadric_carrier` to the same one-off rule.

  A narrow re-review was dispatched over the third pass and the reconciliation. (CLEAVE orchestrator)
- 2026-10-31 — The crescent lane handed back PR 5852. It now merges main, and main's OFFSET reach lever replaced half of the original fix; what the PR still adds is the one-order-down bend reading (`bool_fate_corner_bend`).
  - Against main: S10's remaining fate-cut `ClassificationInvariant` cells turn sound (16/8/20 at the three ε), with no regressions.
  - It closes `a-flush-closed-rim-tangent-at-its-vertex-reaches-the-fate-cut-limit` and files a P1: a seam ruling on the crescent's tangent line ships a `B ∖ A` that tier 3′ refuses (main's behaviour).
  - The union-across-a-crescent row goes back to open with `design: true`. It needs a second-order vertex classifier, which is larger than one lane.
  - The PR conflicts with main, so it went back to the lane for a merge before review. (CLEAVE orchestrator)
- 2026-10-31 — PR 5852 reviewed (claims and style): nothing blocking.
  - On the final head against current main, 16/8/20 cells move from `ClassificationInvariant` to built bodies that pass every check, with the closed-form volume. The BAD sets are identical, and the bend is load-bearing.
  - The fix pass sent to the lane covers:
    - the second-order reading straddled through `decide_agreed` (a two-sided ellipse reach read Along where it should refuse);
    - one home for the `(κ − κ_b)·û·l²/2` margin that `fate`, `runs_into` and `crescent` each spell;
    - `bend_at` folded into `leaving`;
    - the filed seam-ruling row's footprint corrected, with its S7 `CensusUndecidable` sibling split out;
    - merge leftovers in the closed-conic test;
    - a P2 row for `NoCorner` speaking as a kernel invariant. (CLEAVE orchestrator)
- 2026-10-31 — The two-lumps lane opened PR 6170 (`a10fec9142`), with CI green. The cause was in the join, not the ring order: the first ring cycle closed was the zero-area sliver between the two lumps' copies of the line.
  - The tie's own order now winds that sliver (`join::tie_winding`, read only where the first-order winding reads Zero).
  - A plug of several faces now nests whole in `outermost_survivor`.
  - Its 216-cell probe: main refuses 54, and head builds 216/216 at three ε. Claims and style reviews dispatched. (CLEAVE orchestrator)
- 2026-10-31 — The PR 5793 fourth pass handed back at `7f54013726`, with CI green and 0 cells moved against the third pass.
  - What it did:
    - F1a's witness now goes red: there is one sampling reading, `census::points_on_surface`, with unit rows behind it.
    - The foot door reads the band on every degenerate locus and refuses torus/spline/Approx as `Unread`.
    - The tangent-wall arms are folded into one second-order reading.
    - The attribution is corrected, and the P2 rows filed.
  - Last push asked for: register `bool_foot_off_locus` in `decision_words`, and close both rows in the PR. Merge on green.

  Dispatched `a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site` (P1 M) on `cleave/one-segment-cylinder-through-a-holed-plate`. (CLEAVE orchestrator)
- 2026-10-31 — The PR 5852 fix pass handed back at `6abaee6075`, with CI green.
  - The narrow re-review verified:
    - `decide_levered` is unchanged, and `runs_into`/`crescent` are token-identical through `sectors::bend_parting`;
    - the straddle row goes red under either one-ended reading;
    - the secant is unsound at an exact tie;
    - S10 is identical to before.
  - One should-fix: the tie refusal covers `out` only, so a tie at `back` still reads the secant. A last narrow push was asked for (S1, a test of the refusal, the nits). (CLEAVE orchestrator)
- 2026-10-31 — PR 5793 merged (`367f1b3c3d`).
  - The parting reads an edge lying in a face by its crossings, divided against every face holding a piece of it, and on curved faces too. It refuses typed (`CurvedFace` / `Curved`) where the parted form isn't meshed.
  - A tangent wall bending away from a curved face builds (one second-order reading, `bool_rider_wall_bend_across`).
  - One sampling reading: `census::points_on_surface`.
  - The foot door is band-guarded (`foot_off_locus`).
  - Both riders rows are closed in the PR. (CLEAVE orchestrator)
- 2026-10-31 — PR 6170 reviewed (claims and style): nothing blocking.
  - The claims lane widened the family to about 3,100 cells: 207 `PinchRaysInAFace` and 21 `Merge` cells move to sound builds, and nothing else moves.
  - `tie_winding` is needed: `best_arc` alone refuses 6 steep wedge-beside cells where capture ranks both arcs Clean.
  - The fix pass sent covers:
    - no silent pick when the two ends' tie readings disagree;
    - an enforced adjacency in `germ_order`;
    - witnesses for both fixes;
    - both union orders in the keel test;
    - one row owning the remaining `PinchRaysInAFace` arms;
    - the stale honeone row, and style. (CLEAVE orchestrator)
- 2026-11-01 — The cylinder-hole lane opened PR 6177 (`a7e28bae92`), with CI green. It found three causes:
  1. `closes_at_its_site` had no `(Wrap, Other planar)` arm.
  2. `germ_locus` read a closed rim's germ from the site record.
  3. `chord_join`'s strut ran its second chord backward, which put the disc inside out.

  - Probe `rev5808r`, main vs head: 206/73/163 cells move from refused to SOUND, with nothing moving the other way. 8 S10 cells now refuse in the fate cut's kernel-bug voice, which PR 5852 addresses.
  - Two P2 rows filed. Claims and style reviews dispatched. (CLEAVE orchestrator)
- 2026-11-01 — PR 5852 merged (`ce374d70b4`).
  - What it adds:
    - A fate cut reads a kept edge leaving along a corner's bound one order down: the bend, straddled where the reach is two-sided, through one home `sectors::bend_parting`.
    - A tie at either bound refuses `NoCorner` rather than reading the secant.
    - Against main, 16/8/20 S10 cells go to SOUND.
  - Merging main brought in PR 5793:
    - The 24 seam-ruling cells now refuse `CurvedFace` typed, so that row is closed (refs 5793).
    - 4 S7 one-segment unions went from refusing `EdgeInFace` to building with tier 3′ `CensusUndecidable`; this is noted on restread's P0. I asked the 6177 claims reviewer to check whether those bodies are right. (CLEAVE orchestrator)
- 2026-11-01 — Dispatched two P1 M rows now that `fate.rs` is free:
  - `a-fate-cut-touch-end-at-a-division-vertex-lies-in-no-piece`, on `cleave/fate-touch-end-at-a-division-vertex`;
  - `the-other-orbit-readings-refuse-a-tie-the-walk-now-reads`, on `cleave/orbit-readings-read-the-beside-tie`. This one is asked to name which of the two second-order side readers its sites should ask. (CLEAVE orchestrator)
- 2026-11-01 — PR 6170 merged (`12c6418450`). Two lumps touching along an edge now build against a flush partner in every op, in either order. What it does:
  - The join winds a zero-area tie sliver by the ring's tie order (`tie_winding`). Disagreeing ends desync.
  - `germ_order` refuses a non-adjacent tie.
  - A multi-face plug nests whole in `outermost_survivor`, across the mergeable adjacency.
  - Witnesses cover the steep wedge-beside cells, which need `tie_winding` over `best_arc`, and the diagonal `Merge` cells, in both union orders at three ε.
  - It filed a P3 row for the two remaining `PinchRaysInAFace` arms, and a dated note on honeone's germ-order row. (CLEAVE orchestrator)
- 2026-11-01 — PR 6177 reviewed (claims and style). Changes requested.
  - The claims lane verified all three fixes are load-bearing, and the 206 → SOUND diff holds (214 merged with main, including 5852's 8 cells). Nothing ships invalid.
  - The 4 S7 unions 5793 moved are right bodies (closed-form volume, 400/400 samples), which the census can't certify (`CensusUndecidable{MayOverlap}`). They are the same on main.
  - Blocking:
    - B1: the slow pin `a_boss_over_a_hole_by_pose` goes red. The flush coplanar bottoms now refuse `SectionLoopUndecided`.
    - B2: conflicts with main.
  - Should-fix:
    - The diff hid 16 + 32 refusal-kind changes.
    - `closed_on_partner` is a second, one-midpoint reader beside `slit::carrier_on_surface`, and disagrees with `closed_reach` on `Outside`.
    - Two fallbacks still use the old site-record reading.
    - `place_strut` copies `join_lone_ring`'s promotion.
  - Fix pass sent. (CLEAVE orchestrator)

- 2026-11-01: 6184 (orbit readings read the beside tie) and 6185 (fate touch-end at a division vertex) handed back green.
  - 6184: one `walks_before` reads `beside_order` at a first-order tie, and every orbit reader asks it. No outcome moves on the lane's grid. The lane skipped the mutation check, so the claims lane runs it first.
  - 6185: `AtCorners` ends the chord at the division's vertex. 70 cells per ε go from refused to valid. The lane did not re-diff after the 6170 merge. A new P1 row covers a cut end on an edge of the division.
  - Full reviews dispatched for both (claims lane and style lane, briefs in `briefs/review-618{4,5}.md`). Rows set to review. (CLEAVE orchestrator)

- 2026-11-01: 6186 (tracker) merged. 6184 and 6185 reviews adjudicated, nothing blocking in either.
  - 6185: the claims lane re-ran the outcome diff after the 6170 merge (70 cells per ε go from refused to valid, nothing else moves). Wider three-piece and near-along families build too, and every mutant refuses loudly.
    - Should-fix: the pin misses the `last` mutant and none of it runs in the gate; the new row's op mapping is wrong for the full partner; the `Nowhere` refusal's words now also describe `AtCorners`; docs still say "no surgery".
    - A dump at (1,1,1) shows the `End::Touch` end has a missing partner record upstream: 7v1 is neither vv'd to nor in the ring of 15v1. A row goes in for that. Fix pass sent.
  - 6184: the claims lane ran the skipped mutation check. A sign flip at the new sites turns both pins, three existing slit tests and the unit test red. `walks_before` is an order (0 violations over 2352 pairs), and outcomes are identical over 135 more poses.
    - Should-fix: "no ring pose reaches these sites" is false (three slit tests reach `held_cut` with a decided tie); the tie arm is spelled twice (`walks_before` and `walk_faces_first`'s closure); `tied_held`'s far-ends test changed meaning unpinned; `strut_anchor` accepts a case argued unreachable.
    - Three readings of which side a tied cut is on are to be measured for agreement and recorded on the honeone two-homes row. Fix pass sent. (CLEAVE orchestrator)

- 2026-11-01: **Split on semantic seams** (Ev, in chat: "cleave has grown very large… split it into smaller programs so that you have a reasonable single session's worth of work… parallelize a lot with cloud sessions"). CLEAVE measured 114 budget points against 30.
  - New programs, rows moved by `git mv` with ids and bodies kept and a dated `## Slate triage` note on each:
    - TANGRIM: tangent rims and crescents, 7 rows, 29/30.
    - PARTING: edges lying in faces and spline cuts, 7 rows, 17/30.
    - TOUCHREC: a result's records of its touches, 7 rows, 21/30.
    - ONEREAD: one question with two readers, 5 rows, 14/30.
    - SEAL: the Indeterminate seal and the declared-contact rows, 4 rows, 6/30.
  - Also moved: `a-built-sliver-is-not-a-legal-operand` to HELDRECORDS and the main-red corner-boss row to BORDERS.
  - CLEAVE keeps lines, slits, rings, fate cuts and orbit ties, and every row in flight: 24/30.
  - Seams considered and set aside:
    - census-wall-tangent sits with ONEREAD (one question, two answers), not TANGRIM, to keep TANGRIM under 30.
    - The ONEHOME, TANG and CONTACTHOLD charters overlap ONEREAD and SEAL, but all three are over budget, so new tracks were opened rather than growing them. (CLEAVE orchestrator)

- 2026-11-01: 6198 (the split) merged. Cloud orchestrator sessions started for TANGRIM, PARTING, ONEREAD and TOUCHREC, each with a kickoff naming its reading, its claim (`status: active` on `<id>/orchestrator`), the rows due to arrive from CLEAVE's in-flight PRs, and the shared-ground and file-overlap notes.
  - SEAL is left `ready` with no orchestrator. Its dispatchable work is one design row plus the vertex-piercing row held by D10, so it is not yet a session's work. (CLEAVE orchestrator)
