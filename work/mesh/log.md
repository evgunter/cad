# S-MESH log — mesh honesty and budget

Narrative record; the plan is `docs/S-MESH-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-MESH. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-31)

Opened on Ev's direction (in-chat: "pick up S-MESH as the
orchestrator", with S-BOOL taken by the same instruction), by a fresh
orchestrator on a remote container. The plan is a DRAFT design
conversation for its **Rulings sought** section; MESH-1 is
dispatchable pre-ratification as an inherited defect fix whose shape
#303's merged unit established (recorded here as a unilateral
decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `mesh/`** — unit branches
  `mesh/<unit>-<slug>`, orchestrator branch `mesh/orchestrator` (the
  harness-designated session branch `claude/s-mesh-orchestrator-7o6gjc`
  carries the opening PR and is otherwise unused, per the
  S-CERT/S-QA precedent). The remote's dormant `mesh/*` branches are
  pre-program #284-era work, not this program's.
- **A/B ordinal band: S-MESH = 1200–1299**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in this same PR, per that
  entry's rule — after TWO collisions, both resolved by the
  main-is-authority tiebreak before any S-MESH ordinal was assigned:
  the opening claim of 900–999 lost to GAUTH, and the renumber to
  1000–1099 lost to SEAT while this opening PR was held for
  sign-off. The lesson recorded once: a band claim protects nobody
  until it is ON MAIN, so an opening PR merges promptly (its open
  rulings stay open as conversations) rather than holding the claim
  hostage to them. S-BOOL's 1100–1199 was fixed on main earlier by
  its ordinal-1100 claim at BOOL-1's review dispatch. Implementer
  blocks are named `MESH-B1, MESH-B2, …` (`MESH-<n>` are unit
  names). **S-BOOL = 1100–1199 is claimed in the same entry**
  (same orchestrator, `docs/S-BOOL-LOG.md`).
- **This session runs in a remote container** (the S-CERT/S-QA/M10/GUI
  precedent): no persistent `~/.local/share/cad-work`, no script
  monitors (PR watching via MCP subscriptions + scheduled self
  check-ins; away-channel etiquette by hand under the `(S-MESH
  orchestrator)` tag), GitHub through MCP rather than `gh`. Disk ~28 G
  free is the binding constraint: lanes are worktrees sharing one
  object store, own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent lane
  targets shared with S-BOOL, review targets reclaimed at report time.
  The clone arrived shallow; unshallowed with a blob filter at
  opening.

**Sweep at opening** (beyond the charter, what the slate is grounded
in): #320 and #782 are resolved on main and want closing
(orchestrator-direct, after verifying the pins at HEAD — TESS-SPAN
#594 + TESS-SPLIT #951 closed both #320 halves with #950 the
scheduled residual; #782's table re-pinned green and its CI arming
landed, with `docs/VERBS-PLAN.md` already recording "wants closing").
#881 is half-landed at #894 (the named-operations half remains, per
the reopen comment). Inherited from S-CERT by name: #1362 and the
`closing_column` note. `walk.rs` is contended by #1362/#896/#881/#868
— sequenced, never fanned out. S-CERT is live (CERT-6 in flight, then
CERT-8/CERT-10/CERT-M/CERT-N): `props/quad.rs`/`patch_bound.rs`/area
lanes and the tess-budget re-baseline stay its until its slate closes,
so C3/D30/C23 wait on CERT-10 and S26 on CERT-6. Track R table
corrections ride the opening PR (count re-derived after D304's
arrival; C3/D30's discharged #723 gate; D302 deleted with members
relocated — Display landed at `types.rs:271`, consumer half is
#1111's/LIB's, Track U's D47 unblocked for the type).

## MESH-1 merged (2026-08-31) — issue 1362 closed; the walk's loop-area fold is placement-honest

PR 1389 at fix head `014ae4ee` (merged with current main). Gates:
impl head `f0618c8e` green (interval/default drawn); fix head green
(default/1e-12 drawn — the two heads span both compile lanes and two
ε rows; the ε=1e-12 skip-band was checked by hand so the drawn gate
demonstrably executed the defect fixture). The band_u fold anchors at
the loop's own bbox centre; the red-first table shows whole-2π branch
flips at 1e6 and 1e8 under the old spelling; an e2e gate pins the
door-admitted silent failure (1 µm ball at 1 km: watertight, 16
orders wrong pre-fix).

**The dual (ordinal 1200, sample at the row)**: R1 A-W-F 4/3/2 — all
four MAJORs in the sweep/claims layer (the code fix survived both
lanes digit-for-digit); the no-e2e claim falsified by a built
fixture; the sweep completed to 16 read hits at fix. R2 APPROVE
0/2/1 with the donut structural zero confirmed exact. The
arm-classification conflict on the review_m2_pr5 hits settled by
reading in R1's favor. R1's MAJORs are v6 tally candidates. Fix pass
IMPLEMENTER-INHERITED, all 10 items; the direction row closes the
constant-fold blindness; the lifted oracle's contract restated from
its derived closed form.

En route findings homed: issue 1396 (structural-zero oracle class,
+ball-quarter member), 1401 (remaining origin-anchored copies, both
spellings, enumerated), 1402 (azimuth/bbox-anchor idiom homes —
a natural MESH-4 rider). Two walk.rs style classes recorded there
rather than widened into this unit. Slate next: MESH-2 (issue 555,
the sub-floor engineered zeros) interleaved with S-BOOL's BOOL-2;
walk.rs stays sequenced (MESH-3 #896 before MESH-4 #881).

## MESH-2 merged (2026-08-31) — issue 555 closed; sub-floor engineered zeros mesh, in two layers

PR 1421 at fix head `22228e47`. Gates: both heads drew default/1e-12
(the fix head's k-lint budget row gated against the same-day lofts
re-cut — see the seam entry below); three ε rows + tour + clippy
local at both heads. The chart frame writes the far point's
structurally-zero v at the projection site (bit-keyed), and spade's
`mitigate_underflow` floors the insert feed for the ringed/diagonal
residue R2 demonstrated systematic — the adjudicated two-layer
close, each layer red-first under its own mutant. The Klein wall-7
pin retired to banked-case-closed; the refusal lottery re-swept
denser than the issue recorded (four cells, all meshing, counts
pinned).

**The dual (ordinal 1201, sample at the row)**: R1 APPROVE 0/2/0
(5/5/4) with every claim executed; R2 A-W-F 2/4/2 (4/3/3) with two
executed unilateral MAJORs — the siting defense's spade premise
falsified at the source, and the ringed residue shown in-class and
closable (the blanket variant run green across suite and tour).
Both are v6 tally candidates. Fix pass IMPLEMENTER-INHERITED, all
8 items.

**Seam entry — two main repairs orchestrator-direct en route** (the
S-CERT PR-1257 precedent): PR 1428 re-cut the tess-budget baseline
for PR 1351's un-re-cut lofts renames (the landmine detonated on
BOOL-2's draw first), and ported the twopeg dead-const clippy red
the forced k-lint row exposed. Issues filed this cycle: 1434 (the
tour test suite never executes the wall probes — Track X's), and
the ball-quarter member recorded on 1396. Slate next: MESH-3
(issue 896, the undeclared-pole guard) after BOOL-2's cycle
concludes; walk.rs stays sequenced.

## MESH-3 merged (2026-09-01) — issue 896 closed; the undeclared-pole guard lands single-homed

PR 1460 at fix head `9ef9dc88` (+ a main merge before state-sync).
Gates: impl head drew interval/1e-12, fix head default/1e-6 — the
two heads span both compile lanes and two ε rows, with the local
three-ε battery at both. The guard is a D2-row-5 debug_assert
beside #895's: no junction emitted `pole: false` lies within ε of
a chart pole, junctions-only for #895's load-bearing reason, and
the identified side deliberately unasserted (whether an in-band
junction really IS the pole is an intent question). After the fix
pass the pole find has one home (`pole_index`) and the route
argument one holder (`poleguard.rs`) with both firing branches
derived and the K>2π premise guarded by a row against the live Tol.

**The dual (ordinal 1202, sample at the row)**: twin A-W-F (R1
0/3/3, R2 2/4/4) — R1's arm was interrupted by the account's fable
limit and resumed same-arm, so the pair is 3(e)-EXCLUDED from the
tally and the twelve. Both lanes built independent byte instruments
(36/36 and 63/63 identical) and independently measured the boolean
door shut. The bilateral finding: the halfcap_eps7 en-route report
was false at 1e-6 (it tessellates watertight there, pole
identified) — the corrected band shape is pinned in-tree and homed
on issue 881 as MESH-4 substrate, along with R2's premise
falsification (an in-tree vertex 1.0e-9 m inside the 1e-6 band of
an undeclared pole). R1's K=3 experiment is the cycle's best find:
below 2π the span argument voids and the door HOLDS anyway, at the
adoption transversality bar — the site now says so.

Slate next: MESH-5 (#685) dispatches on lane-budget room; MESH-8
inherits the corrected halfcap witness. Issue 881 carries the two
new measurements as MESH-4 substrate.

## MESH-5 merged (2026-09-01) — issue 685 closed by measurement; one strip is right

PR 1507 at fix head `c79609f7`. Gates: impl head drew interval/1e-12,
fix head default/default — both compile lanes spanned. The nu==1
sizing intent is DECIDED, not patched: the two-build δ-sweep showed
the honoured schedule multiplies the cone patch 5–9× at
bitwise-identical deviation (the binding deviation is the rim
chord's azimuthal sagitta — boundary geometry no interior row can
touch), and the post-fix site cites the structural proof (cert_cone's
worst-triangle bound is row-invariant). grid_counts' cone arm
returns (1,1) at a single column and no longer computes the
schedule it discarded. The S29 instance retires at the site.

**The dual (ordinal 1203, sample at the row)**: R2 APPROVE, R1
A-W-F, both arms uninterrupted — the pair COUNTS. The load-bearing
find was bilateral at split severity (no tally candidate): the
removed ResolutionOverflow edge IS publicly reachable (extreme-aspect
frusta / a needle cone through revolve), so the refusal→served
change is now stated, characterized by the true binding parameter
(the aspect, not half-angle), and pinned red-first by an adopted
probe. The sibling class (one-element grid axes dropping the other
axis' schedule — sphere/torus nu==1, the nv==1 mirror,
uniform_candidates) is scheduled as issue 1513.

Slate next: MESH-4 (#881 named-ε ops) or MESH-6/7 per lane budget;
MESH-8 holds its corrected halfcap witness from MESH-3.

## MESH-4 merged (2026-09-01) — issue 881 closed; the ε inventory becomes the methods

PR 1517 at the re-rolled head (fix content 51b03b3c6 + the main
repair below). Gates: impl head lane=both asked/1e-12 drawn; the
fix head's only red was main's own break (below); final head green
at merge. The mesh-local Eps newtype carries the four named ops
and every terminal read; the binding gate — no mesh byte moves —
was proved twice by the unit and twice more by the reviews (a
superset digest including the budget leg; a 242,040-check bitwise
differential), and the fix pass made the evidence TRUE rather than
trimmed: the budget third leg added, pad's execution proven by
panic plant, the instrument's splice artifact fixed. The one-arg
API deviation is disclosed with its byte-preserving reason; the
unearned derives dropped; R1's band-edge parity probe now runs in
CI. #741 coordinated by comment, corrected once (carriers survive
for the weaker reason).

**The dual (ordinal 1204, sample at the row)**: R2 APPROVE, R1
A-W-F, no MAJOR, pair COUNTS. The reviews' real product was
evidence-honesty: everything the PR claimed was TRUE of the code
and slightly false of what the instruments had seen.

**Seam entry — one main repair orchestrator-direct en route** (the
PR-1428 precedent): PR 1516's authored-literals change missed the
probe,interval-gated initializer in editor-core, redding EVERY PR
that merged main on the k-lint gate (first seen here; BOOL-11's
lane hit and locally patched the same break independently).
Repaired at PR 1523 with the file's false "neither hosted lane
builds that pair" coverage sentence corrected — the stale claim
that invited the skip. Issue 1525 filed for the geom-brep
--all-features sibling (VERBS' ground). Reviewer-brief discipline
tightened: lane-private uniquely-prefixed scratch paths (a
same-named-script collision in the shared scratchpad, caught
within a minute, no cross-lane source contact).

Slate next: MESH-6 (#897) and MESH-7 (#727/#726) per lane budget;
MESH-8 holds the corrected halfcap witness. Issue 1513 carries the
sibling class from MESH-5.

## MESH-6 merged (2026-09-02) — issue 897 closed; S65's two cases become censuses

PR 1545 at head 6261f8646 (gate 33595370207 green; lane default,
ε default drawn). Both of issue 897's uncovered S65 cases — the
full-2π seam and cross-face identification — now have a mechanical
`cfg(debug_assertions)` census, priced inside one release binary
(seam +5–12% of `tessellate` on the donut, chord −8…+1%, the pair
+13–15%); `check_mesh` as the cross-face guard was measured and
rejected on the donut rows and footprint, with the sub-millisecond
rows honestly discounted as noise. The `pole_columns` argument was
verified as arithmetic and read per arm: the torus floor is
protective, the cylinder's vacuous. The fix pass covered the
trimmed NURBS arm too, factored the shared rules into `walk`, and
gated the new rows so the crate compiles with debug-assertions off
— finding one pre-existing `walk.rs` row (issue 896's) that fails
in that configuration, reported not touched.

**The dual (ordinal 1205, sample #101 — the row landed in this
docs PR after the merge, the first unit under the new mechanics:
no ledger append on the unit branch)**: R1 mergeable-with-MINORs,
R2 mergeable-after-MINORs with ONE unilateral executed MAJOR (the
`check_mesh` price sentence contradicting the PR's own table) —
a tally candidate. Pair COUNTS.

**Spec-wording correction, recorded.** `docs/MESH-6-SPEC.md`'s
deliverable 3 said the censuses are "compiled out of every shipping
build". That was the spec's phrase, not the measurement's: the
workspace's PRE-PUBLISH `[profile.release] debug-assertions = true`
stanza makes every `cfg(debug_assertions)` guard LIVE in today's
release builds, so both censuses ship at the measured +13–15% on
the donut until that stanza flips. The S65 ruling's intended state
is cfg-conditional ("no unconditional shipped guard"), which is
what landed; the dispatch brief that attributed the phrase to the
PR was the orchestrator's error, symmetric across both reviews.

Slate next: MESH-7 (#727/#726) draws MESH-B3 (MESH-B2 exhausted);
MESH-8 holds the corrected halfcap witness.

## MESH-7 merged (2026-09-02) — issues 727 and 726 closed; explicit iso-rectangle doors

PR 1565 at head 5ea2a87d1 (gate 33621048246 green; lane=both asked,
ε 1e-6 drawn). Under the Q3 ruling `mesh` now cites props' shape
predicate before the walk through a public, flux-free door, and the
spatial check keeps only the walk-consistency question; the 12-row
door census records who leaned on whom and who still does (tier-3
check 7: redundant for `mesh`, load-bearing for import, the editor
checks, pncad and the tour). The oblique-lens qualification at
`walk::iso_side_starts` is closed AS WORDED, and the review made the
unit say exactly that much and no more: the walk's arc premise is
still inherited (issue 1571 — a pole-crossing great-circle arc passes
the door, which certifies carriers, not arcs). D9 held to the byte
across two builds and three ε rows, and the one body class that
meshed on main and refuses now is a props extent defect (issue 1562,
the split-seam donut), pinned as a limitation rather than softened.

**The dual (ordinal 1206, sample #106 — row in this docs PR after the
merge)**: R1 one unilateral EXECUTED MAJOR — the PR's central
"premise established" claim falsified by a constructed body — the
pair's tally candidate; R2 mergeable with MINORs, independently
re-deriving the digest and the finding. Pair COUNTS. The union fix
pass retracted the claim at four sites, factored the torus prologue
the door had copied (C11's own mechanism, caught by R2), replaced a
cross-crate `unreachable!` with a narrowed error type, and
demonstrated the zero-width slit it had only argued.

SMELL §D row C11 retired with this record. Slate grows by two units
from the unit's own findings: MESH-10 (issue 1562, S) and MESH-11
(issue 1571, M), both after MESH-8. Residues on issue 727 at close:
the boolean's frontier door and volume backstop (S-BOOL's), and
import's at-rest promise on tier 3.

Slate next: MESH-8 (#868, the coherence-detector relocation) draws
MESH-B3 slot 1.

## MESH-8 merged (2026-09-02) — issue 868 closed; the detectors become topo's examination

PR 1585 at head 26ddeeaaf (run 33645902263 green; lane default and
ε 1e-12 both asked, so the point the module decides at is CI-gated).
Under the Q2 ruling (option (d), relocation) the three input-quality
`debug_assert!`s in `mesh::walk` are deleted and their conditions
re-derived body-side as `topo::coherence::examine_chart_coherence` —
a non-gating findings report, deterministic in (body, ε), explicitly
outside D9's mesh-byte contract. The door was decided before the
build: `topo`, because only it can see both π-rad witnesses (issue
1571's Euler-door body never touches a STEP file; issue 723's half-cap
arrives through import) with no new crate edge. The walk's chart
closed forms and, after the reviews, its iso classification were
hoisted verbatim into `topo` so the two consumers run one set of
expressions — the D9 digest identical at every step (63/63, 99/99,
162/162; three ε rows) is the receipt. Both witnesses report; the
corpus and forty more bodies stay quiet; two measured negatives are
recorded (rim continuation unreachable through `tessellate` on any
natively constructible body; meridian continuation never fired first).

**The dual (ordinal 1207, sample #110)**: both arms mergeable after
MINORs, no MAJOR, no tally candidate; pair COUNTS. The reviews' real
product: the classification had been COPIED into topo rather than
moved (fixed by extending the hoist), the sub-ε-not-constructible
claim was false (a tilted meridian circle opens any gap), and the
`unexamined` half of the new surface had no red row.

**Stated plainly.** A debug build no longer panics in the walk on the
π-rad bodies; the loudness passes to the issue-897 census and to
`CertificateExceeded`. On merge day the examination has zero
production callers — wiring it into editor-core's checks and
step-import's diagnostics is issue 1587 (a consumer decision on other
programs' ground: recorded as a cross-program seam); the
rim-continuation condition's missing natively-constructible witness is
issue 1588. SMELL S115(d) was already retired from the roll-up when
issue 868 became its schedule; nothing to edit there.

**Seam entry — the container restarted mid-battery.** Every commit had
been pushed; the lane and target survived; the implementer re-ran the
battery in the foreground (~7 min lost). The one red hosted job on the
interim head was rustdoc — two intra-doc links the hoists left
dangling — now part of every head's local battery, not the first
head's only.

Slate next: MESH-10 (issue 1562) dispatched from this merge (MESH-B3
slot 2; the block is exhausted — MESH-11 draws MESH-B4); MESH-11 (issue
1571) after it; MESH-9 stays parked on its trigger.

## MESH-10 merged (2026-09-02) — issue 1562 closed; the torus reads the whole meridian

PR 1595 at head b3314bfca plus this row (run 33674151153 green;
lane interval, ε default, both drawn). `torus_parse` folds the pieces
of a split meridian into the one meridian they carry before the span
is read, keyed on split lineage — `Provenance::SplitEdge` chased to
the root edge and stamped on `LoopEdge` by `topo::props::loop_edges`
— never on value coincidence; every consumer flips through the one
parse with zero ulps and the issue-653 sweep returns to (254, 4).

**The dual (ordinal 1208, sample #112 — the row on the branch as its
last commit, per the reaffirmed rule)**: R1 not mergeable on one
executed MAJOR, R2 mergeable after MINORs; the same substance at split
severity, so no tally candidate; pair COUNTS. The reviews' product:
the fold ASSUMED the pieces partition the parent's parametrisation and
enforced nothing, and three public calls could make it sum a span past
the certification bound into a silent doubled volume where the base
refused. The fix pass makes the fold enforce what it assumed —
contiguity at the exact-order band, one direction, the reconstructed
span re-decided against the per-edge winding invariant — refusing where
the sphere arm clamps, with the class stated at one home.

**Seam entries.** (1) Boolean `combine.rs` copies every provenance
record verbatim across a graft, so lineage chases alias source-arena
keys and `union` on a split-seam operand is order-dependent (typed
refusal one way): issue 1597, found by R2, MEASURED by the fix pass —
forwarding `SplitEdge` across the graft breaks the names lane's
`chase_b`, which needs the verbatim key of an ancestor that died
before the graft; the graft stays verbatim, the row pins the finding,
a dead-ancestor bridge on `GraftMap` is the fix. (2) The class sweep
measured wedge/rounded_prism moving one ulp in area under ANY edge
split — planar summation order, not this unit's; a standing fact now
in the sweep table. (3) `Body::split_root` is typed and its cycle arm
fired on real assembly products through 1597's aliasing.

Slate next: MESH-11 (issue 1571) dispatches from this merge and draws
MESH-B4; MESH-9 stays parked; issues 1587/1588/1597 are the
cross-program seams left open.

## MESH-11 merged (2026-09-02) — issue 1571 closed; the walk's arc premise is verified at the door

PR 1599 at head ce9af2bf2 plus this row (run 33693735802 green; lane
interval and ε 1e-12 both asked). The premise every chart consumer
inherited — each boundary edge traversed on one chart branch — is now
verified at props by a SEPARATE named predicate,
`require_one_chart_branch`, cited by `mesh` in front of the walk and
NOT by the flux lane: CERT-1's four pole rows admit pole-crossing
meridian arcs on purpose (the closed form measures them exactly),
while the walk's one-column-per-edge model cannot read them. Two
predicates, not two answers — the door/walk question the spec called
a possible fork was ruled not one by the orchestrator, and the PR says
so plainly for Ev to overrule. The pole-membership arithmetic has one
home, shared with the CERT-1 fold, one sign apart. The cone turned out
NOT immune (an apex-crossing generator passes the shape door and the
walk mis-reads it) and is covered by the predicate's cone arm. Both
π-rad witnesses refuse typed at every δ; the imported half-cap is
band-shaped (refused only at 1e-12, where its ~1e-9 m overshoot clears
the band); D9 identical.

**The dual (ordinal 1209, sample #113)**: both arms mergeable after
MINORs, no MAJOR, no tally candidate; pair COUNTS, both arms
delivering across three container restarts. The reviews' product:
the retired sentence's citers had not been swept, a doubled doc block
on the hoisted helper, the floor is `escalate` not ε, the asked-for
interval lane had compiled but not executed the new arithmetic, and
R2 found a pre-existing flux-lane defect — a meridian span past 2π
folds SHORT because the saturated clamp's sign still has a zero set
(issue 1601; the door is unaffected).

**Filed forward.** Issue 1598 (the L-shaped complement's volume 0.0 —
equal-and-opposite flux from one parse handing both faces the same
levels; not closable without retracting CERT-1); issue 1601 (the
saturated-span fold, Track R); issue 1602 (whether a props refusal may
carry its measured overshoot — `props/curved.rs` is not on the Bounds
allowlist).

Slate state: the defect cluster MESH-6/7/8/10/11 is cleared. MESH-9
stays parked on its trigger. What remains on S-MESH's slate is the
MESH-R track lanes and the cross-program follow-ons (1587, 1588, 1597,
1598, 1601, 1602) — the next unit is a slate decision, put to Ev.

## Tracker migration (2026-09-03)

The plan and this log moved from `docs/S-MESH-PLAN.md` / `docs/S-MESH-LOG.md`
to `work/mesh/plan.md` / `work/mesh/log.md`. The slate now lives in this
directory's item files and in `work/STATUS.md`; this log stays the
narrative. Items created at migration: MESH-12 (dispatched, PR 1617; its
spec landed at PR 1605), MESH-9 (parked on issue 950's trigger), MESH-R
(open).
