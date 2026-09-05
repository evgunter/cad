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
