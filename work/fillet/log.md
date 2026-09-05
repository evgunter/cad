# FILLET log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/fillet/plan.md`. A/B band 2000–2099
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose FILLET section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `fillet-nonpositive-radius-false-fact-refusal` from `work/issues/`
- `recourse-sentences-owe-followability-pin` from `work/issues/`
- `bare-f64-margin-payload-family` from `work/issues/`
- `concave-closed-rim-has-no-band` from `work/issues/`
- `repaired-pole-rim-serves-no-closed-door` from `work/issues/`
- `extrude-cap-rim-smooth-arm-noop` from `work/issues/`
- `fillet-ruled-spine-arms-no-surgery` from `work/issues/`
- `nocornersidecandidate-has-no-producer` from `work/issues/`
- `fillet-refusal-describes-unbracketed-crossing` from `work/issues/`
- `no-public-rim-arc-selector` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Orchestration opens (2026-09-04)

Picked up on Ev's direction (in-chat, 2026-09-04: "pick up program
`fillet` as the orchestrator"). Single-orchestrator remote box, no
away-channel, no `gh`: GitHub goes through the MCP tools, lanes are
Agent-tool worktrees with private `CARGO_TARGET_DIR`s seeded from one
warm build, at most two heavy lanes at once (four cores). The
orchestrator branch is the session's designated
`claude/fillet-orchestrator-cc9l8o` rather than `fillet/orchestrator`;
unit branches keep the plan's `fillet/` prefix.

**Design work up front — the assessment Ev asked for.** None blocks
the openers or the first three H units: E1–E3 have their shapes
written in their item files, H4 is the closed-rim analogue of the
open-chain convexity fold BLEND-3/BLEND-4 already landed, H5 is a
widening of one door the item itself names, H6 is a reachability
argument. What IS design: the four D rulings (by construction — each
goes out as an `[ev]` PR now, so Ev's answers arrive while the E/H
units run), and H7, whose chain terminations are the run-out question
ARMS3 A3-3 names and OQ6 reserves for Ev — it gets its own `[ev]` PR
early rather than waiting to be reached.

Decisions taken unilaterally at opening:

- **The E openers run outside the A/B experiment**: the charter Ev
  ratified gives them a single style review, v6 is a dual-per-row
  protocol, and S-TCOST's precedent is that non-dual units record no
  row. They draw no ordinal and no block slot; block FILLET-B1 opens
  with H4.
- **Track T's stale park cleared**: `D322`–`D324` were parked on
  #1360, which merged 2026-08-31; they are `open` again and land as
  riders on the first unit that opens `blend/surgery.rs` or
  `blend/naming.rs` (H4 for `D322`/`D325`; `D323`/`D324` together, the
  naming pair, on whichever lane touches `naming.rs` first).
- **Dispatch order**: E1 and E2 in parallel now; E3 after E1 merges
  (both edit `blend/mod.rs`'s error surface). Item-header state
  (`kind`, `status`, `branch`, `pr`) rides each unit's own PR per
  `work/README.md`; this branch carries only the log and the park
  clears.
- **Lane commits carry no model trailer and no model name**, the
  standing lane rule, experiment or not; orchestrator commits keep the
  harness trailer.

The four `[ev]` PRs are open (2026-09-04), one per ruling, each a
question section appended to its item with a firm recommendation:
[#1733](https://github.com/evgunter/cad/pull/1733) `NoCornerSideCandidate`
(keep as a stated defensive arm),
[#1734](https://github.com/evgunter/cad/pull/1734) the arc-carrier refusal
attribution (carry the corner point; report the crossing nearest the
anchors), [#1735](https://github.com/evgunter/cad/pull/1735) the rim
selector (`rim_of(body, edge)` in `topo::query`, SEAT's seam),
[#1736](https://github.com/evgunter/cad/pull/1736) H7's terminations (the
transverse cut-off at perpendicular caps). E1 and E2 dispatched on
`fillet/e1-nonpositive-radius` and `fillet/e2-recourse-followability`.

Ev 👍'd [#1733](https://github.com/evgunter/cad/pull/1733) (option 1:
`NoCornerSideCandidate` stays as a stated defensive arm); executed on
that PR — the doc comment states the invariant, the item is closed.
The H4 spec is `docs/FILLET-H4-SPEC.md`; block FILLET-B1's pre-draw
fields and draw are recorded branch-side on `fillet/b1-block` (slot 0
= H4, slots 1–2 bank for H5, H6). H4 dispatches when a heavy lane
frees (E1 or E2).

Rulings landed (2026-09-04): [#1735](https://github.com/evgunter/cad/pull/1735)
approved — the rim selector is `rim_of(body, edge)` in `topo::query`;
the item is a unit, spec `docs/FILLET-RIM-SPEC.md`, the seam announced
in `work/seat/log.md`, and it takes block FILLET-B1's slot 1 ahead of
H5 (record branch-side). [#1734](https://github.com/evgunter/cad/pull/1734):
Ev asked whether reporting the whole list would be worse; answered
"not worse, it costs shape", approved — every refusing crossing is
reported with its corner point, nearest-to-the-anchors first; the item
is a unit (spec to follow). [#1736](https://github.com/evgunter/cad/pull/1736):
Ev asked whether the transverse cut-off would be contradicted by the
later run-out design and whether the open-chain door's plane–plane
restriction is being extended; answered (different situation from the
mid-curve stop, a rename at most; H7 is the only widening) — awaiting
the 👍. H4 dispatched on `fillet/h4-concave-closed-rim`.

[#1736](https://github.com/evgunter/cad/pull/1736) ruled (Ev: "ok
sounds good"): H7 builds the transverse cut-off at perpendicular caps;
the item is a unit, spec to follow after H4/RIM/H5. All four opening
rulings are now answered; only the cut-off's tag name remains to be
ratified, inside H7's spec. This session is subscribed to its own
open `[ev]` PRs so comments wake it.

**H4 stopped at Phase 1 and was re-scoped, same lane (2026-09-04).** The
lane measured, gate off, that every concave closed rim reaches its door
and the surgery correctly refuses to cut a seam at a foot beyond the
rim: the curved arms and `plane_sphere_blend` fold the supports' sense
bits and never the chain's convexity, so a concave rest is the convex
one mirrored through the rim (waist spine `0.5 − r√2` vs the void-side
`0.5 + r√2`). With the fold applied and the surgery untouched, the
waist, the lily mouth and a `cube ∪ ball` boss (which the boolean
builds and which routes to the LADDER) all carve tier-3 clean at their
Pappus volumes, pad 0. The spec's stop clause routed the arms out; the
orchestrator's re-scope routes them back in — same territory, the
ratified rolling-ball convention, BLEND-4's plane–plane fold the
precedent — as `docs/FILLET-H4-SPEC.md` §"Re-scope at Phase 1". L /
NUMERIC unchanged, re-logged branch-side. PR
[#1752](https://github.com/evgunter/cad/pull/1752) is the unit's PR; the
finding is filed as `concave-rim-arms-rest-ball-on-material-side`,
which the unit closes.

**E1 reviewed (2026-09-04): MERGEABLE-AFTER-FIXES**, single style review
on frozen `acb85399` of PR [#1743](https://github.com/evgunter/cad/pull/1743).
C1/C2/C3/C5 held under executed differentials and mutants; C4 missed one
stale sentence. The review's real yield is a sibling class: the shared
gate reads `lo() > 0` unmetered while the band's zero is 1e-9, so a
positive size under ε still reaches the false-fact refusal at both doors
(fillet: a false headroom sentence; chamfer: `DependentNormals` about an
orthonormal corner) — the blend's is the one unmetered spelling of three
(shell and tube meter positivity against the band). Adjudicated: the fix
pass (implementer-inherited) adopts the reviewer's rows `--no-ff`
(order pin, one-refusal row, the `Interval` bracket rows that make the
interval claim real — the flipped probe is `f64`), fixes the prose and
the duplicated rationale, and files the sub-ε class as its own issue
with a characterization row rather than widening E1; the order change
(size gate before repeated-edge) is disclosed as a change and stands.
The inline-Display-advice class the review surfaced (`NonpositiveSize`,
`RepeatedEdge` map to `Recourse::None` yet advise) is routed to E2's
inventory. E3 dispatched on `fillet/e3-margin-payloads`.

**E2 landed (2026-09-04)** as PR [#1753](https://github.com/evgunter/cad/pull/1753),
head `804aa72f`, green at the drawn point (default, ε 1e-6). Twenty-two
composed rows follow every recourse constant and two inline-advice
sites to their promised outcome: eleven followable, ten pinned
unreachable with the reason at the constant, two wording fixes
(`FILLET3_CHAIN_RECOURSE`, `FILLET3_CORNER_RECOURSE`: a corner left
partly requested runs out wherever it sits). Headline claim, to be
falsified by the review before it gets an issue: all six `profile`
fillet recourse sentences are dead because `EscalationSite::Fillet`
has no producer. Style review dispatched on the frozen head.

**E1 MERGED (2026-09-04)**, PR [#1743](https://github.com/evgunter/cad/pull/1743)
at `fcb2c915`, green 22/22 at the asked-for interval lane, ε 1e-12
drawn. Both blend doors share one size gate; the false-fact refusal at
radius zero is gone; the reviewer's rows adopted `--no-ff` (the gate
order pinned, the `Interval` bracket rows that make the interval claim
real, the no-metering probe row); the sub-ε class filed as
`blend-size-gate-unmetered-under-epsilon` with a characterization row
as its witness. The first FILLET unit closes.

**H4 landed (2026-09-04)** on PR [#1752](https://github.com/evgunter/cad/pull/1752),
head `797e37f9`, green at the asked-for interval lane. The curved arms
fold the chain's convexity; the concave gate is retired; the waist, the
`cube ∪ ball` boss (LADDER), the lily mouth and the snowman waist carve
at their closed-form volumes with pad 0; every convex carve bit-identical
to the merge base by the dump; two mutants red only on concave rows;
`D322` closed as a rider. Ordinal **2000** claimed; the v6 dual is
dispatched concurrently on the frozen head (the draw and method are in
`docs/MODEL-AB-LOG.md`'s claim entry). The implementer's unit entry
waits for the dual.

**E2 reviewed (2026-09-04): MERGEABLE-AFTER-FIXES**, single style review
on frozen `804aa72f` of PR [#1753](https://github.com/evgunter/cad/pull/1753).
Twenty of twenty-two rows hold; two "unreachable" verdicts were
falsified by executed witnesses: `FILLET3_RING_RECOURSE` reaches the
front door off the 9-sample lattice (a 30°-turned prism with a dimple)
and is followable; `FILLET3_GEOMETRY_RECOURSE` reaches it at a
polygonal pocket ring and is DEAD — the class the unit exists to find.
The headline (six `profile` sentences dead, `EscalationSite::Fillet`
never minted) is confirmed and predates the PR; the reviewer filed it,
with the door/validator tangency disagreement on small bends, as
issues. Adjudicated: fix pass implementer-inherited — the two rows flip
to the reviewer's witnesses, the geometry sentence made true, the
fixture copies homed in `test_support` (matching H4's spellings), the
`profile` constants behind a `test-support` feature instead of a root
re-export, one home for the constant list, the "no witness" docs
worded as suite facts not invariants; the under-describing recourses
filed as their own issue.

**E2 MERGED (2026-09-04)**, PR [#1753](https://github.com/evgunter/cad/pull/1753)
at `ecbb4467`, green across three drawn points over the branch's life
(default/1e-12, default/1e-6, interval/1e-6). Every recourse constant
and two inline-advice sites now have a composed followability row or a
suite-scoped "no witness" row whose premise is stated as such; the two
falsified verdicts became witnesses (the ring recourse followable off
the sample lattice; the geometry recourse rewritten to name the ring
and the ORDER that builds — blend first, cut the pocket after — and
composed); `ALL_RECOURSES` is the one home for the constant list;
`profile` gained a `test-support` feature in `sweep`'s shape (and the
façade guard learned to skip cfg-gated code); `waisted`/`spool`/`prism`
homed in `test_support` matching H4's spellings. Filed by the review
and kept: `fillet-escalation-site-has-no-producer` (the six `profile`
fillet sentences are dead), `path-fillet-door-validator-tangency-disagree`
(small bends build at the path door and refuse at validate, window
riding √ε), `blend-recourses-under-describe-their-doors`. All three E
openers are now merged or in review; E3 landed on
[#1763](https://github.com/evgunter/cad/pull/1763) at `93341783` (the
seven margins carry a `ClassifiedMargin`; the NaN hole is unreachable
by construction) and its style review is dispatched.

**E3 reviewed (2026-09-04): MERGEABLE-AFTER-FIXES**, single style review
on frozen `93341783` of PR [#1763](https://github.com/evgunter/cad/pull/1763);
every claim held under a mutant and a merge-base differential. Owed and
adjudicated TAKE: a thin bracket must read as the scalar's own
classifier spells it (`Enclosure{lo==hi}` at Interval, not
`Value(lo)`); the companion fields `gap` and `arm` are measured
enclosures projected and rendered as facts — the item's anticipated
third shape one field over — and now carry the measurement whole (the
request `radius` stays `f64`, said so at the field); the reviewer's
three red-by-design interval rows adopted and turned green by those
fixes; the new NaN finding re-homed to `work/trim/` (TRIM is open and
owns `pcurve_cache.rs` — the brief's premise was the orchestrator's
error); the class one arm over (`support_coaxiality` drops its margin,
`corner_at` folds a definite refusal margin-less, a `headroom: f64`
"classified margin" in `offset_meters.rs`) filed as
`blend-payloads-outside-the-margin-family`. Fix pass
implementer-inherited.

**H4 dual adjudicated (2026-09-04)**, both reviews on frozen `797e37f9`:
R1 MERGEABLE-AFTER-FIXES 2/4/6, R2 MERGEABLE-AFTER-FIXES 0/2/4. Both
re-derived the concave rest by hand (spine `0.5 + r√2`), reproduced
both closed forms by independent quadrature (Green's theorem; Simpson)
to 1e-17, re-ran the bit-dump differential themselves (identical, all
six dumps), and reproduced the mutants; R2 added a third (band sense
forced `true`: exactly the four concave carves red via tier 3). Both
falsified C5 at stale prose — bilateral at `docs/KERNEL-VERBS.md:59`,
each with unilateral sites (R1: `mod.rs`, `surgery.rs`, `battery.rs`
sentences still stating the retired hedge as a REASON; R2:
`docs/DESIGN.md:578` and `chamfer.rs:40`, stale since BLEND-4). **Tally
candidate (the program's first): R1's MAJOR-2 is UNILATERAL and
DEMONSTRATED BY EXECUTION** — the plane–sphere setback sign the unit
introduced survives its mutant across all 1030 rows, and the value
feeds predicate 2's consumption screen, making it more permissive on
concave plane–sphere pairs; R2 never mentions the setback sign. Class:
test-gap on code the unit moved. Bilateral otherwise: naming totality
one-directional (R1 MINOR-1 ≡ R2 NOTE-3, R2's probe passes — the
record is complete, the pin was not), fixture copies (R1 MINOR-2 ≡ R2
Q1), the assembly recourse over-promising at H5's shape (R1 NOTE-5 ≡
R2 NOTE-1), the default lane never run on the landing head (R1 NOTE-6 ≡
R2 MINOR-2). Union fix pass implementer-inherited: both probe branches
adopted, one home for the sign (`Convexity` grows the method; bools
become the type), the sentence-shape re-sweep, `lane=both` on the
landing head. The row and sample number wait for the fix head.

**H4 MERGED (2026-09-04)** at `fc38f753`, PR [#1752](https://github.com/evgunter/cad/pull/1752):
sample **#126** (renumbered from #123 at the sync — three other
programs' duals merged ahead of it), ordinal 2000, the row in
`docs/MODEL-AB-LOG.md`; `docs/FILLET-H4-SPEC.md` deleted into the
ledger. A concave closed rim carves on either material side through
both rim doors; the fold has one home on `Convexity`; `D322` closed as
a rider; the fourth quadrant (sphere pocket, concave chain) is built
and carving. Block FILLET-B1 slot 0 concluded (record branch-side).
**RIM dispatched** into slot 1 on `fillet/rim-selector` under
`docs/FILLET-RIM-SPEC.md`. E3's fix pass is in flight.

**E3 MERGED (2026-09-04)**, PR [#1763](https://github.com/evgunter/cad/pull/1763)
at `503f4ad5`, green at the asked-for interval lane with the k-lint
`dev-probe` row pinned. `BlendError`'s seven margins carry a
`ClassifiedMargin` (reading in the scalar's own spelling — an
enclosure at Interval, thin or not; band; predicate; sign); `gap` and
`arm` carry their measurement whole; the request `radius` fields stay
`f64` and say so; `ConvexitySignFlip` reports the link's own classified
levered margin and its NaN hole is unreachable by construction. The
k-lint probe row caught the first thin-bracket spelling writing
synthetic K telemetry through the recorded classifier path — replaced
by a type-level enclosure test with no classifier call. Filed by the
review and the fix pass: `blend-payloads-outside-the-margin-family`
(FILLET), `fitted-magnitude-nan-schedule-parameter` re-homed to
`work/trim/`, `blamed-mates-lost-its-exhaustive-arm` (VIEW; main did
not compile at `--features interval`, fixed on main independently).
**All three E openers are closed.** RIM (slot 1) is in flight.

**H5 dispatched (2026-09-04)** into block FILLET-B1's slot 2 on
`fillet/h5-hostless-rim` under `docs/FILLET-H5-SPEC.md` (pre-draw M /
STRUCTURAL at the spec): a closed rim whose arcs one plane face hosts
in its outer cycle is the annulus band with hostless crossings — the
host foot minted by the ladder's strut, the mate side the seam-split
walk unchanged; the shape arises both from `merge_coplanar_faces` and
natively (a pole-touching dome on a wider flat top). Runs beside RIM
(slot 1, `fillet/rim-selector`); they touch different files.

**Specs written while slots 1–2 run (2026-09-04):** `docs/FILLET-H6-SPEC.md`
(S / STRUCTURAL: measure whether extrude's cap-rim `Smooth` arm is
reachable; either way the must-carry rule gets one home its three
sibling arms call) and `docs/FILLET-H7-SPEC.md` (L / NUMERIC: the ruled
band with the transverse cut-off, out as an `[ev]` PR because it
proposes `CornerConfig::TransverseCap` / `RunOutPolicy::CutOffAtTransverseCap`
for ratification; `needs_ev` set on the item for that alone). Block
FILLET-B2 opens with ATTR, H6 and H7 once slot 2 concludes.

**H5 stopped at Phase 1 and was re-scoped, same lane (2026-09-04).** The
lane measured that the spec's "native instance" does not exist (a full
revolve splits the whole loop at its seam, so the plane-hosted rim
arises only after `merge_coplanar_faces`), that the repaired boss and
dimple route to the LADDER and refuse on a FALSE ring clearance (a
nested trim circle judged by external separation — filed as
`ring-clearance-refuses-a-nested-trim-circle`, its own numeric unit),
and that the seam-split resolution refuses the shape at its half-band
gate rather than at `wall_seam`. Six repaired fixtures carry the defect
on both material sides (the bowl floor is the concave one). Spec
amended §"Re-scope at Phase 1"; the Phase 2 design stands; PR
[#1824](https://github.com/evgunter/cad/pull/1824) is the unit's PR.
`D323`/`D324` turn out closed by code-quality (PR 1783) — this log's
opening note that they land as FILLET riders is superseded.

**RIM landed (2026-09-04)** on PR [#1821](https://github.com/evgunter/cad/pull/1821),
head `9b9ae75e`, green at the asked-for interval lane. Phase 1 measured
every body class's rim arcs bit-equal on centre, radius and axis (never
negated; `u_ref` differs across chart seams and is not compared), so
the EXACT door is the spec's own outcome: `topo::query::rim_of` with
its four typed refusals, a topological tiling test (shared vertices,
key equality — the arcs' params live in their own frames, so a
parametric test would need a decision), eleven hand-rolled scans
turned into calls including the tour's bud mouth, and
`test_support::rim_arcs_at` now seeding one arc into the door. The PR
carried H4's title from a brief-template slip — corrected at dispatch.
Ordinal **2001** claimed; the v6 dual is dispatched concurrently on the
frozen head. Two other programs' finds routed, not fixed: the tour's
`blend1_r1_wall6_probes.rs:94` selects a rim at a 5e-4 radius
tolerance (the reviewers measure whether that is real slack).

**H7's vocabulary ratified (2026-09-05).** Ev merged the `[ev]` spec PR
[#1819](https://github.com/evgunter/cad/pull/1819) without comment, so
`CornerConfig::TransverseCap` / `RunOutPolicy::CutOffAtTransverseCap` are
the names `docs/FILLET-H7-SPEC.md` builds on; `needs_ev` cleared on
`fillet-ruled-spine-arms-no-surgery`. Nothing else on that item waits on Ev.

**RIM dual delivered; union fix pass sent (2026-09-05).** Both reviews
MERGEABLE-AFTER-FIXES on `9b9ae75e`. Convergent: the rotation claim is
unconditional in the doc and false under the match's negated-axis
admission at ≥3 arcs (both lanes); the interval row exercises point
enclosures only; seven roster lines not six; the wall6 5e-4 disposition
rests on a premise both measured false (the lily rims are bit-exact).
Orchestrator's call in the pass: the match drops axis negation (Phase 1
and both reviews measured none on any corpus rim; spec amended as a
deviation), the tiling contract is restated as a closed chain on shared
vertices with the double cover filed, `circle_param` folds the negative
radius the way `param_near` does, and the seed-finder class is filed not
swept. Adjudication is written into the unit's row at merge, after the
pass lands; ordinal 2001, sample assigned in main's merge order.

**H5 dual dispatched (2026-09-05)** on frozen `e44f1a7fe` (PR
[#1824](https://github.com/evgunter/cad/pull/1824)): ordinal **2002**
claimed on main, parity byte 110 ⇒ R1 OPUS / R2 FABLE, briefs stored with
sha256 before dispatch, concurrent, isolated. Emphasis: the C4
`validate_closed` window after the strut `mev` (the lane measured it is
inherited from the ladder — the reviewers verify the ladder claim and
judge whether C4 is met or owed a deviation), `HostSide` passed not
derived, the `Struts` gate's exact-outer-cycle question, `strut_foot` as
one home, `refresh_annulus_seams` carrying a `Strut`, the ungated
default-ε / 1e-12 points and the unbuilt tour.

**Block FILLET-B2 opened branch-side (2026-09-05)** on `fillet/b2-block`
with every B1 slot dispatched (H4 merged, RIM in its fix pass, H5 under
review): pre-draw fields first (slot 0 H6 S/STRUCTURAL, slot 1 ATTR
M/STRUCTURAL, slot 2 H7 L/NUMERIC), then the v3 draw. **H6 dispatched**
into slot 0 on `fillet/h6-cap-rim-smooth` under `docs/FILLET-H6-SPEC.md`;
ATTR follows when an H5 review lane frees the box (four cores, two
reviewers and the RIM fix pass live), H7 after its brief. The H6 and ATTR
briefs were re-derived from RIM's and had carried H5's PR title — the
same template slip that mis-titled PR 1821 — corrected before dispatch.

**H5 dual delivered; union fix pass sent (2026-09-05).** R1
MERGEABLE-AFTER-FIXES 1/6/4, R2 MERGEABLE 0/5/8; the kernel change held
under both (dump re-taken at both SHAs, closed forms re-derived to 1e-13
and 1e-15, the foot-parameter mutant red on exactly the seven rows, the
tour 62 green locally by both, the four ungated {lane}×{ε} points green).
Convergent: the boss's merged top rim — one plane host carrying a RING —
refuses typed under the rewritten assembly recourse, which the PR made
unconditional (R1's MAJOR, executed; R2 named the same site as an
unstated frontier), three `Struts` gates with no red row, the dump
convex-only against a constraint naming H4's concave rims, the spec's
curved-single-host statement unwritten, the `validate_closed` deviation
disclosed but unlisted (both reviewers instrumented the LADDER's identical
window). Orchestrator's calls: the recourse states its condition and
every `Struts`-routed refusal is audited for the sentence it carries; the
ringed host is filed, not carved; the curved single host is stated and
filed; the concave fixtures join the dump corpus. No unilateral MAJOR —
tally +0 this pair; pair FAIR. Row at merge, ordinal 2002.

**RIM merged (2026-09-05)** at `40d50f272` (PR
[#1821](https://github.com/evgunter/cad/pull/1821)), **sample #131**
(ledger max #130 at merge), ordinal 2001; block FILLET-B1 slot 1 concluded
(line on `fillet/b1-block`). Adjudicated in the row: R2's MAJOR-2 (the
unused-arcs guard with no row, mutant-proven) is a UNILATERAL tally
candidate; R2's MAJOR-1 (the interval fixture's point enclosures) flagged
for the blinded adjudication; v6 tally +1 this pair. The fix pass took
every decision, and its landing run was the FULL matrix: `ci.yml` retired
the `CI-Config` trailer and the k-lint draw on 2026-09-04, so every PR run
now gates every point — the trailer is prose, and the "which point gated"
sentence leaves the briefs. `docs/FILLET-RIM-SPEC.md` deleted, ledgered at
the merge SHA. Filed by the pass: `rim-door-admits-a-double-cover`,
`rim-seed-finders-disagree-on-at-this-radius`.

**H6 landed (2026-09-05)** on PR [#1891](https://github.com/evgunter/cad/pull/1891),
head `f9cfceaef`, full matrix green (run 33935813397; the first head red
on the lane's own probe — absolute-metre radii refused at ε = 1e-6 — re-cut
to `tol.eps()`). Shape A: sixteen cap-rim shapes through the public doors,
none reaches `Smooth`; the spec's "exactly 90°" premise corrected to the
`1/K` obliquity bound (`sin θ ≥ √(1 − 1/K²)`); the must-carry rule homed
as `geom_brep::tangent_second_order` with both siblings calling it
bit-neutrally; the dump corpus gained an extrude/revolve row, identical at
base and head. The lane's "worth a decision" — the two smooth siblings
disagree on the in-band policy and on how much edge they read — filed as
`smooth-arm-siblings-disagree-on-the-in-band-case`. **H6 dual dispatched**
on frozen `f9cfceaef`: ordinal **2003**, parity byte 211 ⇒ R1 FABLE / R2
OPUS, briefs stored with sha256, concurrent, isolated. H7 dispatched into
block B2 slot 2 (the box was quiet) — three implementer lanes and two
reviewers live.

**H6 dual delivered; union fix pass sent (2026-09-05).** Both
MERGEABLE-AFTER-FIXES (R1 1/5/3, R2 3/7/7); the refactor bit-neutral and
the dump identical by both re-takes, no planted `Smooth` verdict able to
reach a consumer mis-described. Bilateral MAJOR: the arm's "no extrusion
reaches this arm" is K-CONDITIONAL — `Tol` admits any K > 1, and at
K = 1.1 the worst admitted vector on the shortest admitted chord reaches
the `Smooth` arm through the public door and `extrude` hands back a body
tier 3 refuses (`SliverDihedral` ×4), while the module doc calls the bound
"K-free" (crossover K ≈ 1.272; the written bound is loose). Convergent
too: the arm's dichotomy false at every K (a definite arm with an in-band
wedge → `SliverRim` at default K), the one-home undercount (a sibling in
the helper's own crate), the `finish.rs` census disposition, the spec left
uncorrected. Unilateral doc-class MAJOR (R2): the crate docs one level up
still state the falsified premise inside the swept set. Orchestrator's
calls: the argument stated K-conditionally with the behaviour made a typed
refusal wherever tier 3 would refuse, no K floor on `Tol` (filed for a
decision), the stale-sentence class swept incl. the spec, `certify.rs` and
`rim_wedge.rs` migrated to the one home, the dump corpus widened. No
unilateral code/test-gap MAJOR — tally +0; pair FAIR. Row at merge,
ordinal 2003.

**ATTR landed (2026-09-05)** on PR [#1895](https://github.com/evgunter/cad/pull/1895),
head `e8813f998`, full matrix green (run 33937888422, twelve test jobs,
five k-lint tiers, python suite). Phase 1 confirmed the premise at the
merge base (arc×arc: 8.2 % of refusals named a corner other than the
anchors' nearest, max 0.792 m; 16 % mixed reasons across crossings;
line×arc 0 % but 50 % mixed). Landed: `NoCornerOfPair { radius, corners }`
with `CornerReason` arms carrying the three retired variants' payloads
verbatim, both channels (arc-carrier resolve and the straight pair)
feeding it, `FilletOffsetLeverTooShort` still aborting alone; Python
`no_corner_of_pair` + `corner_reason_tag` + `PathError.corners`. Reported
not filed by the lane (outside its fence): the same first-wins discard
shape at `editor-core/src/clearance.rs:1237` and `drive.rs:1707`, and
`sugar.rs:612` one level down. **ATTR dual dispatched** on frozen
`e8813f998`: ordinal **2004**, parity byte 43 ⇒ R1 FABLE / R2 OPUS, briefs
stored with sha256, concurrent, isolated.

**H5 merged; block FILLET-B1 concluded (2026-09-05).** PR
[#1824](https://github.com/evgunter/cad/pull/1824) merged at `91e6d4309`,
**sample #132**, ordinal 2002, its fix pass the whole union: the assembly
recourse conditioned on a ring-free host carrying the rim as its whole
outer cycle with every `Struts`-routed refusal audited at its site, the
concave corpus in a bit-identical dump, the curved single host stated and
filed, `validate_closed` amended to validity-at-rest, `waist_fill` folded
(measured not bit-identical, ≤ 2.6e-17, both H4 bars hold), and one red
leg on the way (two intra-doc links — `scripts/doc-gate.sh --pr` joins
every lane's local scope from here). Filed by the pass:
`hostless-rim-on-a-ringed-host-refuses`,
`curved-single-host-rim-refuses-at-the-half-band-gate`. `docs/FILLET-H5-SPEC.md`
deleted, ledgered at the merge SHA. **Block B1 is concluded** — H4 (FABLE,
#126), RIM (OPUS, #131), H5 (OPUS, #132) — and its record reaches main
with this sync. Block B2 runs: H6 in its fix pass, ATTR under review, H7
implementing.

**ATTR dual delivered; union fix pass sent (2026-09-05).** Both
MERGEABLE-AFTER-FIXES (R1 0/3/7, R2 1/5/14); the fence holds by both
diffs (no gate, band, window or margin moved), Phase 1's grid reproduces
exactly at the head, the ORDER rule holds on all 2 512 two-entry envelopes,
the Python surface 493/493 by both. Bilateral: the Display header's "n
derived corners" counts ENTRIES — a false fact about the geometry on
76.5 % of grid-A refusals (adjudicated MAJOR: the program's own class);
whole-pair construction refusals (`AlreadyTangent` and kin) now lose to a
window entry about the OTHER crossing — a silent precedence inversion the
PR body denies, witnessed red by one lane and argued unreachable by the
other; C4's abort pinned at ε = 1e-12 only; exact refusal pins turned
existential in the migration; the in-fence residues (probe headers, the
`sugar.rs:612` candidate discard — instrumented: 232 discards on grid A,
every one with different payload numbers) unscheduled; C1's fence
defence does not follow and the spec is internally inconsistent about
"every corner tried". Orchestrator's calls: whole-pair refusals outrank
the envelope as at the base (no silent discard), the header made true,
the spec's C1 amended to the channel rule with the acceptance row as its
reason, the C4 abort pinned at every ε, exact pins restored, the two
residues filed, the `NoTangentCircle` sub-kinds carried by the tag. No
unilateral MAJOR — tally +0; pair FAIR. Row at merge, ordinal 2004.
Block B1's record reached main (PR
[#1901](https://github.com/evgunter/cad/pull/1901)).

**H7 landed (2026-09-05)** on PR [#1897](https://github.com/evgunter/cad/pull/1897),
head `fc6ca2268`, full matrix green (run 33939777803; the previous head
red on the discipline gate's interval-square `powi(2)` allowlist, fixed in
place). Phase 1: the rod ∖ box and the D-profile extrusion build and their
creases refuse `UnsupportedChain` at `AdmittedOpen::admit`; the
parallel-cylinder union refuses `CurvedPierceUnsupported` (no concave
ruled fixture — pinned as the refusal); the box edge refuses
`UnsupportedRunOut` and stays so (`arm.is_ruled()` gates the cut-off).
Landed: `CornerConfig::TransverseCap` / `RunOutPolicy::CutOffAtTransverseCap`
as ratified, `fillet3_cap_transverse` levered by the link extent with its
trio pin in both lanes, the ruled band as `RuledPlan`/`ruled_phase` in
`surgery.rs` (deviation 1: the compound-bound allowlist names
`blend/{battery,surgery,build}` only — a new module would need Ev's
ratification), the rod row at its prism closed form (`A = 4.6977159e-4`,
`ΔV = 9.3954318e-4` for two creases, 800k-point shoelace to 2e-15), the
oblique cap refusing `RULED_END_NOT_TRANSVERSE` through the front door,
`seam_split_param` generalised to line carriers bit-identically, dump
identical at two bases. **H7 dual dispatched** on frozen `fc6ca2268`:
ordinal **2005**, parity byte 115 ⇒ R1 FABLE / R2 OPUS, briefs stored with
sha256, concurrent, isolated. With this every FILLET unit is landed; what
remains is three fix passes (H6, ATTR, H7 after its dual) and the block
B2 record.

**H7 dual delivered; union fix pass sent (2026-09-05).** Both
MERGEABLE-AFTER-FIXES (R1 0/5/5, R2 2/6/8); the carve correct on every
fixture either lane could build — two of them fixtures the unit said do
not exist — C1 re-taken at the real merge base by both, `A_section`
re-derived three ways. **Unilateral MAJOR, executed (R2): the lever
`corner_at` hands the new predicate is not red-capable** — `arm_len →
1.0` survives all 1101 rows because every fixture's crease extent is
exactly 1.0 and every cap margin exactly 0; the trio row calls the
predicate directly and never observes the call site. Test-gap class on
the unit's own new decision, dedup single — **tally candidate, +1 this
pair** (running: H4 +1, RIM +1 with one flagged, H7 +1). Bilateral: the
governing docs name `blend/ruled.rs`, which does not exist (deviation 1
folded the carve into `surgery.rs` after the prose landed); the mutant
row's disjunction; `TransverseCap` has no producer. Unilateral R1: the
concave ruled band HAS an extrude fixture (a sunk rod, `ΔV = +2AL`) so
deviation 5's reason is false of that door; the `seam_split_param`
line-carrier change carves a revolve rim that refused at base —
undisclosed consequence; a transverse arc described TANGENT passes
certification and tier 3 (the certifier's `TangentParallel` margin,
pre-existing). Unilateral R2: `ROD_L = 1` makes the prism factor
unmeasured; half of the `seam_split_param` change is dead
generalisation. Orchestrator's calls: the lever pinned at the call site
with a length-dependent fixture, the docs fixed and re-swept by
paraphrase, the concave pin adopted, both halves of the `seam_split_param`
change disclosed, four issues filed (the coaxiality recourse arm, the
ladder's possibly-new dead key, the certifier gap, the persisted
`CornerArc` name). Pair FAIR, flagged: R1 glimpsed the other lane's
branch name and SHA and ~20 lines of the ATTR fix pass's output — no
review content. Q8 from both: `surgery.rs` at 4395 lines holds four
surgeries; the split (`blend/open/{planar,ruled}.rs` behind one
ratification of the compound-bound allowlist) goes to Ev as an `[ev]`
issue. Row at merge, ordinal 2005.

**`[ev]` raised: the surgery module (2026-09-05).** Both H7 reviewers'
Q8 finding — `blend/surgery.rs` at ~4 300 lines holds four surgeries, and
H7's carve landed there only because the compound-bound allowlist names
`blend/(battery|build|surgery).rs` — filed as
`surgery-module-holds-four-surgeries` (`needs_ev`) and put to Ev on PR
[#1916](https://github.com/evgunter/cad/pull/1916): may the ratification
be re-scoped for a `blend/open/{planar,ruled}.rs` split, a file move
bit-identical by the dump? Subscribed for comments; nothing moves until
ruled.

**ATTR merged (2026-09-05)** at `aa5384288` (PR
[#1895](https://github.com/evgunter/cad/pull/1895)), **sample #134**,
ordinal 2004; block FILLET-B2 slot 1 concluded (line on `fillet/b2-block`).
The fix pass took every decision: whole-pair refusals outrank the envelope
again (the hairline pair answers `CarriersParallel` as at the base), the
header counts what it counts, the fence justification withdrawn for the
acceptance row's reason with the spec amended in place, C4 pinned at every
band (mutant red at all three ε), exact pins restored through one
`assert_corners`, three no-assert probe rows deleted, the `NoTangentCircle`
sub-kinds carried by the tag. Filed by the pass:
`overrun-attribution-picks-the-first-candidate` (the `sugar.rs:612`
first-candidate discard, 232 discards on grid A with differing payloads).
`docs/FILLET-ATTR-SPEC.md` deleted, ledgered at the merge SHA.

**H6 fix pass landed (2026-09-05)** on PR [#1891](https://github.com/evgunter/cad/pull/1891),
head `8abdf250b`, full matrix green (run 33942855470). The arm's argument
is K-conditional and says so (tight bound `K/√(K²+1)`, crossover
K ≈ 1.272; shape A at the shipped K = 10); the door now REFUSES typed
(`ExtrudeError::SmoothCapRim`, naming the run's K and the crossover)
where it used to hand back a body tier 3 rejects — measured at K = 1.1
(refuses) and K = 3 (builds, tier 3 green); the stale-sentence class swept
at thirteen sites; `certify.rs` and `rim_wedge.rs` migrated to the one
home (bit-identical by the dump, corpus widened to thirteen rows); three
`describe_at_rest` copies became `topo::Body::describe_at_rest` (a public
door, so three `topo` meta-gates gained their entries); filed
`ambiguity-k-below-the-cap-rim-crossover`. One process fact: a push
produced NO workflow run for twenty minutes — not a filter decision;
"CI is green" and "CI ran" are different questions, and the state-sync
verifies the run exists on the head.

**H6 merged (2026-09-05)** at `195460c7a` (PR
[#1891](https://github.com/evgunter/cad/pull/1891)), **sample #135**,
ordinal 2003; block FILLET-B2 slot 0 concluded (line on `fillet/b2-block`
— ATTR, slot 1, reached main first by merge order). `docs/FILLET-H6-SPEC.md`
deleted, ledgered at the merge SHA. Filed by the pass:
`ambiguity-k-below-the-cap-rim-crossover`. Block B2 now waits on H7's fix
pass alone.

## Inherited red on main, from FILLET-ATTR (TOPO relaying, 2026-09-05)

PR 1895 (`fillet/attr-every-crossing`) merged at `aa5384288` and the
CI run on that merge, 33943429161, was CANCELLED, so main has been red
at the code tier since: `crates/geom-core/tests/bounds_census.rs`'s
`every_sole_bracket_bound_door_is_in_the_roster` names
`crates/profile/src/path/arc_fillet.rs::anchor_span` (`:522`) as
unrostered. TOPO's S330 lane hit it on PR 1923 and carries one
`HandedOff` roster line beside the sibling door as the port so lanes
stop failing on it; the disposition is this program's to sharpen
(`Selection`/DL5(b) may be the truer one) and the debt is FILLET-ATTR's.

**Toward the exit (2026-09-05).** With H7's fix pass landed, what stands
between the program and its walk: (1) Track T — `D325`/`D326`, held while
lanes contended `surgery.rs`, land as ONE S/STRUCTURAL unit FILLET-T in
block FILLET-B3 (spec drafted, dispatch after H7 merges); (2) the fourth D
ruling of the plan, `corner-config-tag-all-concave-trihedron`
(code-quality's, issue 1355), still unanswered — put to Ev again on `[ev]`
PR [#1935](https://github.com/evgunter/cad/pull/1935) with its moved
premise (both verbs carve; carved configurations now carry tags):
mint `ThreeConcaveEdges` or close with no tag; CARRIED to code-quality if
unruled at the walk; (3) the residue slate (fifteen open issues) re-homed
before the sweep. The walk is drafted from the plan's criteria verbatim.

**H7 merged; block FILLET-B2 concluded (2026-09-05).** PR
[#1897](https://github.com/evgunter/cad/pull/1897) merged at `235d05241`,
**sample #136**, ordinal 2005; the fix pass took all fourteen decisions —
the lever pinned at its call site through `run_battery` at two lengths
(the `T::one()` mutant red), the concave ruled band pinned through the
extrude door, both halves of the `seam_split_param` change disclosed,
`corner_at` returning the ratified tag so "classifies" is true, one
chord-site core and one `cap_incidence` home, `assert_naming_totality`
generic (and catching a double-recorded fragment in `split_rim` on the
way), four issues filed, an inherited sole-`Bounds` door given its roster
line. `docs/FILLET-H7-SPEC.md` deleted, ledgered at the merge SHA.
**Block B2 is concluded** — H6 (OPUS, #135), ATTR (OPUS, #134), H7
(FABLE, #136) — and its record reaches main with this sync. Every unit
of the plan's seven, plus the two the rulings grew, is merged.
**FILLET-T specced** (`docs/FILLET-T-SPEC.md`, S / STRUCTURAL): Track T's
`D325` + `D326` as one unit now that no lane contends `surgery.rs`; block
FILLET-B3 opens branch-side for it.

**FILLET-T landed (2026-09-05)** on PR [#1943](https://github.com/evgunter/cad/pull/1943),
head `9290f0b21`, full matrix green (run 33948095965). Phase 1: EIGHT
`kef` sites (the spec's six plus H7's two), every one killing a face the
surgery's own `mef` minted — two of them (the rim strut and the annulus
seam-crossing) with no local argument at all before this, the strongest
evidence for `D326`'s premise; the stop clause did not fire. Landed:
`CornerLinks::sorted` seeded (`(first, rest)`, the minimum carried as the
walk runs), the arc-mint body hoisted into one closure, `first_arc` an
`EdgeKey` with its `unreachable!` deleted (24 → 23 in the file);
`kef_minted` the ONE `kef` door in `surgery.rs` (`grep -c 'body.kef('`
= 1), refusing a half whose face is a source face — and refusing an
EMPTY source set, the one way past the door that would leave no trace;
`D323`'s five-sentence argument at `naming::Retired` cut to two; dump
identical over all 9 armed rows / 13 files. Filed:
`ruled-band-has-no-bit-identity-corpus-row` (no dump row reaches
`ruled_phase` — H7's two `kef` sites sit outside every blend PR's C1).
**FILLET-T dual dispatched** on frozen `9290f0b21`: ordinal **2006**,
parity byte 129 ⇒ R1 FABLE / R2 OPUS, briefs stored with sha256,
concurrent, isolated.
