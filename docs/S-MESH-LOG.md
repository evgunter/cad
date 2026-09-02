# S-MESH log — mesh honesty and budget

Narrative record; the plan is `docs/S-MESH-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-MESH. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-31)

Opened on Evan's direction (in-chat: "pick up S-MESH as the
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
