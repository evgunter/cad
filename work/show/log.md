# SHOW — log

## 2026-10-02 — opened

Ev asked in chat for a check of the demos against recent work, then
for this program, with this session as its orchestrator. The audit
(three read-only lanes plus a full tour run on `6cacd42dd`, exit 0)
found four fudges whose gaps are closed (klein's two-elbow loop, the
heat sink's 1/16 embedment, the teapot lid's vent, the letterforms'
1/16 decoupling), two unblocked plans (lanceolate blades; gallery
documents), retired walls (klein 5 and 7, lily 13), and seven
undemoed landings. Ev's answers in chat: (a) and (b) all in, the
partly-unblocked items the orchestrator's call; (c) all in, folded
into existing cells where possible, the snowman its own cell.

Orchestrator calls, logged:
- **Partly-unblocked items deferred** to their owners' existing rows
  (plan, "Not in scope").
- **The twisted loft goes into `lofts`, not over `twisted_duct`**:
  `twisted_duct` already lost its montage cell to `twisted_tube`, and
  `twisted_tube`'s roll is a placement roll, a different fact from a
  correspondence twist. `lofts` is the cell about what a loft reads
  from its author.
- **The bored section goes into `projectbox`** (its bosses become
  bored), **the engraving into `tiltedcut`** (the elliptical face
  first), **joined-rim fillets into the heat sink**, **gauges into
  `bench`**, **keyhole creases into `rocker`**; the helix and the
  split-by-name are fold candidates taken last.
- **Claimed into SHOW**: `klein-scene-should-adopt-the-one-body-loop-sweep`
  (from issues/), its rider from lib/, and both heat-sink rows from
  issues/. `heatsink-union-lives-in-the-demo` closed on arrival (the
  union has been in-document since #1344's first half);
  `heatsink-placedunion-base-union-unfinished` re-scoped to its
  remaining follow-up, the flush declared fins.
- **Filed on CLEAVE**:
  `boolean-operands-with-nurbs-or-spiric-edges-have-no-schedule` —
  three demo joins pin `CurvedEdgeUnsupported` and no row scheduled
  lifting it.
- **Stale demo prose fixed in the opening PR** (Ev: "fix the out of
  date stuff"): the citations the audit found rotted in `impeller.rs`,
  `lily.rs`, `teapot.rs` and `demos/README.md`.

## 2026-10-02 — wave 1 dispatched

Three Opus implementer lanes, each in its own worktree, from the
opening branch (work/show/ is not on main yet): `snowman-cell`
(`show/snowman-cell`), `klein-scene-should-adopt-the-one-body-loop-sweep`
with its rider (`show/klein-one-body-loop`),
`heatsink-placedunion-base-union-unfinished` (`show/heatsink-flush-fins`).
Common brief at a lane-private path; it points at
`docs/prompts/implementer-discipline.md` by path. Review tier for all
three: single Opus review, style lane plus the unit's claims (each
moves an oracle and a frame; none is architectural).

## 2026-10-02 — snowman in review; the render door

`snowman-cell` is PR 3787, green, in single Opus review. Its lane found
that agents cannot dispatch `render.yml` (`403 Resource not accessible
by integration`), so the frames are unrendered. Ev approved a commit-tag
trigger in chat: `render-on-a-commit-tag` is dispatched (single Opus
review; crosses into CIW's workflows, announced there). Scene PRs hold
until their frames are looked at, through that door once it lands.

## 2026-10-02 — snowman reviewed (MERGE WITH FIXES)

Single Opus review of PR 3787. The oracles were re-derived independently: five
configurations, slice quadrature, agreement within 2.5e-15. No MAJOR. The fix pass
is with the lane:
- the off-axis narration overclaims wall 7: an x shift, a z shift and a spin meet
  three different doors;
- the torus check is missing its axis and centre;
- the census and the exactness claim are not pinned;
- the README carries a measured number;
- the contact-free routing has a third copy;
- the filed TQUERY issue's refs, and `naming.seam_edges` as the public route it
  should address.

Class findings with no home of their own, recorded here:
- `demos/tour/src/main.rs`'s crate doc says every scene is generic over the run
  scalar; about 20 scene modules are f64-only (`grep -L '<S:'`). Pre-existing;
  correct the doc in the next SHOW unit that touches `main.rs`.
- `strut/seed-finder-home-reads-only-the-y-station` cites
  `no-public-rim-arc-selector`, which has no file in `work/`. Pre-existing and
  STRUT's; noted on STRUT's log.
- "Exclude co-surface seams" has three spellings: snowman `waist`, `bodies.rs`
  `bud_rim`, and `rim_select.rs` `Seeds::TwoSided`. The TQUERY issue owns the
  sweep.

## 2026-10-02 — snowman fix pass in; heat sink in review; wave 2 begins

- **`snowman-cell` (PR 3787):** the fix pass is in, head `ec09704f9`, green.
  - Every review finding is addressed.
  - The off-axis poses are pinned by a test.
  - The z-shift refusal is `SectionNotPolar` on the lane's build, which disagrees with the reviewer's reading. The PR body says so.
  - It holds only on its frames.
- **`heatsink-placedunion-base-union-unfinished` (PR 3793):** green, in single Opus review.
  - The flush declared fins refuse at the count edit.
  - The post-union fillet refuses on the rectangular fin-foot rings.
  - Both are pinned as walls.
  - The base is now rounded in-document, before the union.
- **Wave 2 opens:** `projectbox-section-cuts-through-bores` is dispatched.

## 2026-10-02 — heat sink reviewed (MERGE WITH FIXES)

Single Opus review of PR 3793. The oracle checks out against the Steiner
rounded-box form: Δ = 0 at 0, 5, 7 and 9 fins. The counters were
mutation-checked. Rulings:
- **Fins stay sunk.** The reviewer showed flush fins build today by deleting and
  re-adding the union (four edits per step, and the union gets a new id). That
  is the parked `doors/a-union-that-becomes-flush-later-...` deficiency. Taking
  it would muddy the scene's one-edit subject, so the scene narrates the door as
  measured and keeps sunk fins.
- **Wall 1 pins designed fail-loud behaviour, not a gap.** It is re-spelled to
  pin the gap, or demoted to narration.
- **The r = 1/16 trimline refusal gets a file.**
- **One rounded-box closed form serves `diefillet` and `heatsink`.**
- **Wall matchers are tightened, and their `contains(...)` looseness is swept
  tour-wide.**

## 2026-10-02 — klein: the one-body loop builds but cannot ship yet

PR 3792 is green and in single Opus review. The one-body loop sweep
builds, but every natural section fails tier 3 or the mesh:

| Section | Tier 3 | Mesh |
|---|---|---|
| `circle` | refuses `QuadratureBudget` | refuses (the known C0 crease) |
| `circle_split(4)` | passes | refuses `CertificateExceeded` |
| `circle_split(8)` | passes | refuses `CertificateExceeded` |

The scene therefore keeps its two elbows. Walls 5 and 8 pin the two
refusals, and the lane filed a QUAD row and a TESS row.

Ruling: the PR lands on review for the walls, the spine bug fix and the
filings. The unit then PARKS on the two filed rows rather than closing,
because its subject (adopt the sweep) is unchanged and simply waits.

Watch item for the review: the klein wall suite grew from 4 s to 59 s
on CI.

## 2026-10-02 — render tag reviewed (MERGE WITH FIXES); main was red

- **Main was briefly red.** Two merges collided in `geom-brep` (`crate::SurfaceKind::of`): #3733 merged after TQUERY moved the kind into geom. Every PR's gate went red. #3799 fixed it first; my duplicate hotfix branch (`show/main-red-surfacekind-of`) was never opened as a PR and is dead.
- **The render tag review (PR 3791) is back.** The loop guard, the tag read and the permissions are confirmed from the run records. The fix pass is with the lane:
  - the re-gate tests the branch tip, not the merge with main (M1);
  - a failed re-gate is invisible (M2);
  - a mention anywhere in the message fires the tag; narrowed to the subject line;
  - cells rendered from an older tree land on a newer tip;
  - `[skip ci]` on bot heads blocks reopen and undraft runs;
  - dead gate-mode inputs;
  - the tag read moves into the filter job.
- **My own mistake:** the commit carrying Ev's approved §3 wording named "[render]" in its message, so it fired an unplanned render.
- **Process point for this log:** never write the literal tag in a commit message that is not meant to render.

## 2026-10-02 — heat sink fixed; tiltedcut dispatched

PR 3793's fix pass is in. Head `a51b3c5c6` is green, and every review item is addressed:
- delete-and-re-add is measured live and narrated;
- wall 1 is demoted to narration;
- the r = 1/16 refusal is filed on HONE;
- one `rounded_box_volume` in the tour's new `oracles.rs`;
- the matchers are tightened, with a sweep listed.

The PR holds only on its frames. `tiltedcut-engraved-face` is dispatched.

## 2026-10-02 — render tag fix pass in; main red on the bounds census

- **PR 3791 fix pass is in** (head `eea82b6`, then the orchestrator's `1407a18cf`). All twelve review items are addressed.
- **Acceptance on the new code:**
  - A subject-tagged commit renders, and its re-gate correctly reports that no lane committed.
  - A tag mentioned only in the commit body does not fire.
  - Still unproven: the merge-ref re-gate (`pr` input). It only fires when a lane commits, so its first real firing will be the snowman's tagged push. That push is its acceptance.
- **Main is red on `geom-core` `bounds_census`** (#3733 added `circle_image_envelope` with no roster line). Reproduced locally on `9074254`. EMIT's open #3794 carries the line; I ported it verbatim into 3791 and commented there.
- **Filed** `ciw/montage-kernel-reset-discards-freecad-rebaseline`, from the lane's reading.
- **Dispatched a delta review** of the fix pass, since the workflow logic changed substantially.

## 2026-10-02 — klein review: BLOCK, the claim was falsified; reworked to adopt

The PR 3792 review found that quarter-arc sections at 17 stations and
v-degree 2 pass tier 3 and mesh at all three ε. The lane had varied
neither the station count nor the v-degree for the split sections.

Ruling:
- Adopt the one-body loop as `circle_split(4)`, gap-commented against the
  TESS lofted-circle row, at a setting inside a stable neighbourhood. If no
  such neighbourhood exists on the merged head, the lane stops and reports
  rather than shipping a lottery.
- The TESS row is rewritten around the station × degree table; the n=16
  sliver is its lead.
- The ε-proxy wall becomes per-row postures.
- The duplicated 30 s suite test goes.
- The scene's mesh grows about 13×, which is reported and, if over budget,
  filed rather than coarsened.

Process point: a negative result ("each spelling fails") was claimed
from one setting of two untested knobs. Future lane briefs that pin a
refusal must name the knobs varied.

§3 wording question: Ev approved "message contains `[render]`". The
review fix made the code subject-only, so the text is now false. Put to
Ev in chat: (a) say "subject line" (recommended), or (b) widen the code
back. A body-only warning goes in either way.

## 2026-10-02 — render tag second pass in; lofts and teapot lid dispatched

PR 3791 head `ecf3ef9` is green, and the body-only warning is shown firing on that run. The PR now waits only on Ev's §3 wording answer.

`lofts-correspondence-twist` and `teapot-lid-unbored` are dispatched to one lane, as two PRs. The lofts PR merges on the orchestrator's read; the teapot lid PR gets a single review.

Brief addition from the klein lesson: a pinned refusal names the knobs that were varied.

## 2026-10-02 — projectbox reviewed (MERGE WITH FIXES); tiltedcut and klein in review

- **projectbox (PR 3811).** Square bosses were an inherited unnatural spelling: round bosses build, split and section identically (probed at two radii). They become round, and their outlines move to closed form.
- **Already-filed gap.** The `plane_section` corners-only gap was filed already (`cleave/plane-section-polygons-drop-their-arcs`). The scene now gap-comments it rather than justifying around it.
- **Main's brief fmt red.** I fixed it in 3811 by a base merge (`dc3f79ff0`).
- **In review:**
  - tiltedcut (PR 3819, engraved before the cut; four walls; a new ZIP row on a SeamOrientation refusal that the kernel calls a bug);
  - klein's rework (PR 3792, delta review).

## 2026-10-02 — klein delta review (MERGE WITH FIXES): the loop rides a float knife edge

The rework is real, but the delta review found three problems.

1. **The loop builds only on a float knife edge.** An exact U-turn spine refuses `PathTangentReversal`: `sweep_places` turns every station from the base tangent. The interpolated spine builds only because its end tilts mirror each other. That puts it on the C6 float knife edge, which `review_m5_pr10` pins as "executed behaviour, not intent". No public door pins end tangents or joins arcs.
2. **The Python audit row is false.** Python cannot author the sweep, so the north-star audit row 15 no longer holds.
3. **The Pappus oracle passes a visibly wrong loop at 1e-9.**

Ruling: the loop stays. The fragility is gap-commented, the sweep-frame and bindings gaps are filed, the oracle gains a mesh-volume band, and the `circle` attempt remains as a live wall. A scene on a knife edge is acceptable only when the edge is named where a reader will find it, and here a change to it fails loudly.

## 2026-10-02 — tiltedcut reviewed (MERGE WITH FIXES): a new P0 for ZIP

Review of PR 3819. The oracles were confirmed independently. Engraving before the cut is a plausible order, and the scene discloses it.

**The new ZIP row is a P0.** A one-arc annular sector refuses `SeamOrientation` on plain planar boxes under every verb tried: pocket, through-cut, and union boss. The kernel's own docs call that error a kernel bug. The sweep threshold depends on the radii.

The fix pass also covers:
- wall 2 is pose-dependent, so it is re-pinned to the glyph that always refuses;
- wall 3's pose is corrected;
- the dangling `pin_frontier` citations;
- the zip-log note.

**Class noted:** a single scene-wide tessellation delta re-meshes the whole body to suit its smallest feature (lily and tiltedcut). The lane cites or files the per-body delta row.

## 2026-10-02 — lofts twist (PR 3816) passed on the orchestrator's read; teapot lid (PR 3824) in review

Lofts read:
- the slice-area derivation is correct;
- the bitwise vertex identity with the prism is asserted;
- the struts c_k→c_(k+1) are asserted;
- the chord-average t is pinned at 4 ulp.

It merges once its frame has been looked at. Teapot lid gets a single review.

The two PRs re-pin the same lines of `tools/tess-lint/tests/baseline_census.rs`. Whichever merges second re-pins them during its merge.

Two findings from the lane, not yet homed:
- **"Chordal, inscribed" is wrong on saddle walls.** The tour's shared per-body line says the mesh is "chordal, inscribed". The twisted loft's saddle walls mesh to a volume ABOVE the exact one, so that sentence is false on saddle walls. It lives in demo-crate narration, so SHOW fixes it in the next unit that touches `main.rs`'s per-body line.
- **Triangle count.** The twisted loft meshes to 42k triangles, against the prism's 8.9k.

## 2026-10-02 — tiltedcut fixed; letterforms dispatched

PR 3819's fix pass is in: head `bed4d1660`, green. ZIP's new row is P0/H with the full case list. Wall 2 is re-pinned to the U, and wall 3's pose is corrected. The lane filed `show/a-tour-scene-meshes-every-body-at-one-delta` (P3/M) on this slate, since the scene-wide `Stop::delta` is the tour's own door.

Declined a follow-up: a lines-only oval nameplate on the section face. It builds for a square and for the T at only one offset. That is a pose lottery, not a door, so the scene keeps the cap.

`letterforms-flush-declared` dispatched.

## 2026-10-02 — projectbox fixed; rocker dispatched; lily waits on klein

- **PR 3811 fix pass in.** Head `c319b3e86`, green.
  - Round bosses (R = 0.1875); V = 4.0701 on the closed form.
  - Pad caps derived from what each check must see.
  - The Python mirror re-measured; the suite runs green locally.
  - Main briefly carried a `prose_census` red, which a base merge cured.
- **Rocker dispatched.** `rocker-keyhole-crease-fillets` is out.
- **Lily held.** `lily-lanceolate-blade-sections` waits until klein's fix pass lands, because that pass deletes lily's duplicate in-bin wall test. Two lanes on `lily.rs` at once would collide.

## 2026-10-02 — klein fixed; lily dispatched

PR 3792's fix pass is in, at head `a93a0db7a`, green. It does six things:
- **Knife edge:** new wall 9 pins the exact spine's refusal. Its sensitivity is measured: trig-computed quarter points, about 2e-16 off, build.
- **Filed:** `carve/a-half-turn-spine-sweeps-only-off-its-exact-tangents` and `lib/the-klein-loop-sweep-has-no-document-spelling`.
- **Audit:** north-star row 15 is now NO, and the tallies are re-derived.
- **Oracle:** a chordal-deficit band of ±1.14%. Planted bodies go red.
- **Live walls:** `circle` stays live as walls 5 (per-ε posture) and 8.
- **Lily cleanup:** lily's duplicate in-bin test is deleted.

`tools/tess-lint/tests/baseline_census.rs` is now re-pinned by THREE open PRs: 3792, 3816 and 3824. Whichever merges second or third re-derives the pins from the merged CSV.

Teapot's and torusvessel's in-bin wall tests share the stale "the walk never runs under cargo test" premise. They go in the next SHOW unit that touches those files.

`lily-lanceolate-blade-sections` is dispatched.

## 2026-10-02 — teapot lid reviewed (MERGE WITH FIXES)

PR 3824: the closed forms re-derive independently, and planted wrong lids go red.

The split latitude itself is ratified design. Ev, 2026-09-25: curved walls stay π-halves for pole valence. The scene's two-name request is therefore the right authored spelling.

The finding is in the GUI. One visible circle is two names. A one-click pick gets half the rim and refuses, and the recourse it offers is kernel-only. That gets a row on the viewer's owner.

Fix pass:
- a solid-lid document-vs-kernel equivalence row;
- a per-band census check;
- the literal `x{SPOUT_STATIONS}` narration bug;
- whether the eps walk runs every scene's stops in CI.

Tess census merge plan: the deltas from 3792, 3816 and 3824 add by scene. Re-run tess-lint on each merge commit rather than trusting arithmetic.

## 2026-10-02 — teapot lid fixed; bench dispatched

PR 3824's fix pass is in, at head `1913437ac`, green.
- **Half-rim pick gap:** filed as `vseam/a-picked-half-rim-blends-half-and-refuses` (P2/M).
- **Equivalence row:** now on the solid lid, pinning the face order.
- **Per-rim census:** asserted.
- **CI coverage:** CI walks every scene's stops at three ε (`eps_regression` spawns the binary), so there is no class finding.

`bench-on-a-gauge` is dispatched. Review tier: FULL (claims plus style), because it is new API surface on PLACE/MSOLVE ground.

## 2026-10-02 — rocker (PR 3834) and letterforms (PR 3836) in review

- **rocker:** a keyhole with two convex creases rounded on the solid, at the closed form. Two refusals were found and filed or evidenced on BAND:
  - a false `RingClearance` at r ≥ 0.32;
  - a side-blind `RadiusHeadroom` at r ≥ 0.5.

  A possible duplicate pair of selector rows goes to review: the wire "sharp edge atom" row and the tquery co-surface row.
- **letterforms:** natural proportions with declared contacts. The 3-way intersect is ORDER-dependent: (H∩T)∩C refuses `JoinDesync` and C∩(H∩T) builds. The same split shows on az.
  - Ruling: the scene uses the order that builds and pins the other as a live wall, filed on JOIN. Intersection is commutative in meaning, and choosing the order is a user's free choice. It is honest because it is disclosed and pinned.
  - New row: `show/projectbox-offsets-by-sixteenths-to-dodge-coincidence` (the projectbox still decouples by 1/16).
- **Merge-order note:** `tools/tess-lint/tests/baseline_census.rs` is now re-pinned by five open PRs (3792, 3816, 3824, 3834, 3836). They merge in sequence, each re-deriving the pins on the merged tree.

## 2026-10-02 — rocker reviewed (MERGE WITH FIXES); census pins precomputed

PR 3834 review results:
- The closed form A(r) checks against a brute-force area integral.
- The `RingClearance` refusal is confirmed false: with a slot of length 2.0, every r up to 0.49 carves. The threshold is 0.3097, not ≈0.32.
- The wire selector row and the tquery co-surface row are the same class but different asks. The wire row becomes the one GS-Q2 Convex/Reflex demand row, naming both call sites.

The fix pass also covers:
- positional loop reads, which are the class the keyhole broke once;
- one crease localiser shared across Rust and Python;
- the closed form ported with its wrap restored and its source cited;
- metering the rendered body.

The review precomputed the `baseline_census.rs` pins after 3792, 3816, 3824 and 3834 all merge. The deltas are per scene and disjoint, so they sum:
- rows 1603
- all_pairs 29,663
- triangles 427,890
- nurbs_triangles 333,508
- sized 92
- sized_scenes 16
- at_bound 13

3836 (letterforms: rows −52, triangles −200, pairs −1398) composes on top. Each merge re-runs tess-lint on its merge commit regardless.

## 2026-10-02 — letterforms reviewed (MERGE WITH FIXES)

PR 3836. The oracles were independently confirmed: by hand, and az by sympy. Planted 1e-3 shifts all go red.

The operand-order split is real and not a regression. Before #3770, 5 of the 6 nestings refused; #3770 narrowed that to 2. This points JOIN at the same mechanism.

The fix pass:
- adds a live wall for az's Z∩A;
- fixes a stale Python T row;
- narrows the projectbox-sixteenths row against #3811;
- asserts the shadow claim.

The reviewer's claim that "walls never run in CI" contradicts the teapot lane's finding that `eps_regression` spawns the tour binary. The lane must settle it with evidence.

Composed census pins once all five overlapping PRs merge: rows 1551, pairs 28,265, triangles 427,690, scenes 78.

## 2026-10-02 — bench (PR 3840) in full review; the disk filled

### bench
- **What the scene now does:**
  - The stand sits on a turntable gauge driven by a `swing` parameter.
  - A crate sits on a nested shelf-top gauge, with a cross-gauge declared rest.
  - Three swing edits: each recomputes 5 nodes and reuses 4.
  - Vertices are checked to 1e-12 at every pose.
- **No refusal was met.**
- **Filed:** `lib/the-turntable-bench-has-no-python-row`.
- **Not filed, noted as awkward:**
  - a rotation about a pivot takes a three-step chain;
  - Rust's `regauge_then_mate` returns edits, while Python's returns an id.

  The review judges whether either is a finding.

### Disk
- **The disk filled during the bench lane.** The root allowance was down to 409 MB.
- **Reclaimed:**
  - the targets of seven idle, finished implementer lanes, checked first for no writes in the last 5 minutes;
  - eight finished reviewer worktrees.

  Free space is back to 16 GB.
- **Lesson:** lane targets run 0.6–7 GB each. Reclaim when each review returns, not when the disk fills (agent-lane-operations already says so; I lagged).
- **From now on:** a lane's target goes when its PR merges, or when its lane is done and no fix pass is pending.

## 2026-10-02 — bench full review (MERGE WITH FIXES)

**Probes on PR 3840.** The reviewer probed it four ways:
- evaluating from the memo and evaluating fresh give bit-identical results, so the mates' reuse is sound;
- a planted wrong swing, and a stale pose, both go red;
- the A5 gate certifies the cross-gauge rest: it refuses a 1e-7 m float;
- split works off the turntable, and `CutHoldsGauge` fires with it.

**Fix pass.**
- File the two unfiled library findings:
  - no placement turns about a point or axis;
  - the regauge-then-mate order hazard is silent at the edit door.
- Stop narrowing the update walk around the crate. A resting crate on a hand-restated shelf-top gauge cannot follow a part edit; pin that as a wall and file it.
- Fix the stale narration and the audit text.

## 2026-10-02 — rocker fixed; split-by-name dispatched

PR 3834 fix pass is in, head `72c166913`, green. What changed:
- **r* derived:** r* = 0.3097 comes from the margin formula, pinned at ±1% by a carve and a wall.
- **GS-Q2 row:** the wire row is now the single GS-Q2 demand row. Its `refs` to the tquery row is added once 3787 merges.
- **Loop handles:** a `PlateLoop` enum replaces the positional reads.
- **Rendered body metered:** the body the scene renders is the one it meters.

`split-node-chords-by-name-has-no-demo` dispatched, with bracket as its home. The other open slate rows wait on files that open PRs hold:
- helix → projectbox (3811)
- gallery → gallery.rs (3793)
- per-body delta → main.rs, which every scene PR touches
- projectbox sixteenths → 3811
