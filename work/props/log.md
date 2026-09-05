# PROPS log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/props/plan.md`. A/B band 2400–2499
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose PROPS section is the
charter this plan restates. Opens at S-CERT's exit. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `contribution-bounds-via-dual-interval` from `work/m10/`
- `k-stats-escalation-channel-and-redo` from `work/m10/`
- `three-per-node-verdict-shapes` from `work/m10/`
- `certified-lane-non-real-contract-audit` from `work/m10/`
- `m6-sense-gate-recorded-residuals` from `work/issues/`
- `span-carries-its-knot-vector` from `work/issues/`
- `lily-authoring-needs-shadow-vector-algebra` from `work/issues/`
- `interval-orthonormal-basis-sign-hull` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened for work early (2026-09-05)

New orchestrator. **The inheritance gate has NOT fired**: S-CERT is live
with CERT-M3 ([#1877](https://github.com/evgunter/cad/pull/1877), CI red
on one eps=1e-6 job, dual review in flight), CERT-N3
([#1879](https://github.com/evgunter/cad/pull/1879), green, in review)
and the ChartRegionLane `[ev]` ruling
([#1878](https://github.com/evgunter/cad/pull/1878), on hold by its own
orchestrator's comment); no exit walk exists; twenty-seven S-CERT issues
still sit in `work/cert/` (one, `nurbs-net-point-map-helper`, is a stale
`review` row — PR 1742 merged — and is S-CERT's to close). A prior
claim that "the work props waits on from cert is done" was wrong on
these facts and is recorded here so it is not re-derived.

Ev's direction (in-chat, 2026-09-05): start the work that does not
overlap those PRs' files, and watch them. The file-disjointness argument
and the resulting lane split are in `plan.md` §Early lanes. PR
subscription from this box failed through both tools; the S-CERT PRs
are watched by scheduled check-ins instead.

**This box.** Single-orchestrator remote container: GitHub through the
MCP tools, no `gh`, no away-channel monitor, lanes are Agent-tool
worktrees under `/home/user/lanes/<lane>` with private
`CARGO_TARGET_DIR`s at `/home/user/<lane>-target`. The orchestrator
branch is the session's designated `claude/props-orchestrator-review-x1voda`
rather than `props/orchestrator` (FILLET's precedent); unit branches
keep the `props/` prefix.

Decisions taken unilaterally at opening:

- **The ninth item folded into the plan**: `sphere-flux-arm-refuses-partial-bands`
  arrived from VERBS' sweep after the opening entry's list of eight;
  it joins the sphere polar lane on the same `fn sphere`.
- **PROPS-1 bundles two respells** (`mirror_across_plane`,
  `reject_from`) because both move `f64` bits and both owe the same
  golden / k-lint / render accounting — one re-baseline pass, not two.
  `lerp` is decided and LEFT (its endpoint asymmetry is documented and
  deliberate; the `Interval` cost gets a sentence at the site). Member
  5 (`rotation_about`'s diagonal floor) is filed as its own item,
  `rotation-about-diagonal-width-floor`, because it respells every
  rotation for a measured sixth of the residue.
- **The Span ruling goes out now** as an `[ev]` PR with recommendation
  A; its sweep waits for CERT-N3's `spline/` edits.
- **Block PROPS-B1 is drawn** and recorded branch-side on
  `props/b1-block` (a block record naming unstarted slots is a
  reviewer-visible leak); pre-draw difficulty for PROPS-1: M. Ordinal
  2400 claims at review dispatch on main.
- **Territory notice** to the S-CERT orchestrator on #1879: the linalg
  lane runs on `geom-core/src/linalg/` files no S-CERT PR touches; the
  request to hand `normalize-overflow-yields-zero-axis` over early.

PROPS-1 dispatched on `props/1-linalg-lost-correlation`; spec
`docs/PROPS-1-SPEC.md` on this branch.

**Verdict shapes decided and dispatched (2026-09-05).** A read-only
census (three shapes, one substrate; M10-8 touches none of them; no
test pins the strict-vs-population claim) settled the D half:
consolidate the modules and the outcome enums, keep the split, add the
pin. Unit `three-per-node-verdict-shapes` (converted in place, FILLET's
convention), spec `docs/PROPS-VERDICT-SHAPES-SPEC.md`, branch
`props/verdict-shapes`, dispatched now rather than after M10-8 since the
overlap measured nil on the shapes themselves. An E rider — single style
review, outside the experiment — so it draws no block slot; the
`drive.rs` seam is announced in `work/m10/log.md`. Ruling PR for Span:
[#1906](https://github.com/evgunter/cad/pull/1906). Subscription works
for PRs this session opened and fails for others.

**Span ruling: A (2026-09-05).** Ev, in-chat: "A and B both sound ok,
so if you recommend A then that works." [#1906](https://github.com/evgunter/cad/pull/1906)
merged with the item converted to a unit and parked on CERT-N3 (#1879);
the spec is written at that merge against the post-N3 `spline/` layer.
The three E/S specs queued for build slots: `PROPS-ONB-MEASURE-SPEC`,
`PROPS-LILY-VEC3-SPEC`; verdict-shapes is dispatched.

**Verdict shapes delivered (2026-09-05)** as
[#1920](https://github.com/evgunter/cad/pull/1920), head `0dcec746`,
green on the full matrix (run 33943451208). No golden key moved (no
fixture carries an absent row); the two pins passed against the
unchanged tree first; `certifying` stays in `drive.rs` as the gate's
policy. Style review dispatched on the frozen head with seven claims
(C1 the absent-row behaviour change hidden by "no key moved"; C2/C3 the
pins' non-vacuity and the spec's own pin (b) misstatement; C4 the
re-export lanes; C5 the split inherent impl; C6/C7 docs and tracker).

**S-CERT's two track lanes merged (2026-09-05).** CERT-M3
[#1877](https://github.com/evgunter/cad/pull/1877) at 03:33 and CERT-N3
[#1879](https://github.com/evgunter/cad/pull/1879) at 04:08, both after
a dual and a fix pass. What that unblocks here: the Span sweep
(`span-carries-its-knot-vector`, ruled A) is un-parked to `spec` and cut
against the post-N3 `spline/` layer; `validate.rs` is free of live PRs,
so `m6-sense-gate-recorded-residuals` (check 6's arc-bounded arm) is
dispatchable when a slot frees. Still S-CERT's: the ChartRegionLane
ruling (#1878, restated on the corrected three-consumer premise, awaiting
Ev), the tally adjudication, and the exit walk — no walk exists on main
yet, `normalize-overflow-yields-zero-axis` still sits in `work/cert/`,
and the stale `nurbs-net-point-map-helper` review row is unanswered; no
reply yet to the territory notice. PROPS-1's implementer opened
[#1918](https://github.com/evgunter/cad/pull/1918) (in flight); this
session is subscribed to #1918 and #1920. Dispatch order for the next
free slots: ONB-measure → Span sweep (block slot 1) → lily-vec3 →
M6 sense gate.

**Verdict shapes reviewed (2026-09-05).** Style review on `0dcec746`:
APPROVE-WITH-FIXES, MAJOR 0 / MINOR 3 / NOTE 3, rubric idiom 4 / tests 4 /
docs 3; deviations 3 reported, 0 silent. The MINORs are doc honesty
(the tag-byte sentence asserted an invariant the code does not have;
two history clauses; "no key moved" given its measured reason where the
structural one — the driver never cancels and refuses a non-`Ok`
witness — is the guard). Taken whole, plus the reviewer's structural
point: `certifying` becomes a free function in `drive` rather than a
split inherent impl on `vdiff`'s type, and the pin file gains a positive
control. **Spec correction, recorded here**: the spec's pin (b) ("a sign
exchange between two predicates cancels in the populations") was wrong
— populations are per predicate, so it yields two flips; the documented
blind spot is two instances of ONE predicate trading signs, which is the
row the lane shipped. Fix pass dispatched implementer-inherited.

**PROPS-1 delivered (2026-09-05)** as
[#1918](https://github.com/evgunter/cad/pull/1918), head `acc0719ad`,
green on the full matrix (run 33943922769; the python suite skipped by
the seed filter and run locally, 493 OK). Both respells landed: the
exact-axis mirror translation `[4e-9]³ → [0, 0, 4e-9]`, the axis
rejection `[2e-9, 2e-9, 4e-9] → [2e-9, 2e-9, 0]`; f64 drift ≤ 2 ulp on
oblique inputs only; no committed expectation moved. One claim
re-derived and weakened (`project + reject = self`, exact → ≤ 4 ulp),
flagged for review. Sweep: one new hit (`orthonormal_basis`'s double
mention of `s`, a rider on the sign-hull item); `svd.rs`/`lsq.rs` read
and confirmed concrete `f64`. Out of fence: `profile`'s `anchor_span`
lacks a `bounds_census` roster line (the lane reports main red at the
interval lane); ported as `HandedOff` and filed on FILLET's slate
(`anchor-span-sole-bracket-door-missing-roster-line`). Dual review
dispatched on the frozen head; ordinal 2400 claimed
(`docs/MODEL-AB-LOG.md`, the PROPS section). Lane target reclaimed
(14 GB) — with a ~38 GB allowance this box carries one implementer plus
one dual at a time, so the queued specs dispatch one at a time.

**Main was red, fixed (2026-09-05).** `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
failed in both lanes on `main` after FILLET's `anchor_span` landed
without its roster line — seen on #1918's merge and on #1920's fix-pass
head. [#1931](https://github.com/evgunter/cad/pull/1931) carries the
one-hunk roster line (ported from #1918, `HandedOff` to Track V),
merged green on the full matrix; the wording stays FILLET's to own
(`work/fillet/anchor-span-sole-bracket-door-missing-roster-line.md`).
R2's class finding on the mechanism: a closure-tier run seeded from
`profile` excludes `geom-core`, so a whole-tree census living in a leaf
crate's test binary cannot see a door arrive from any other seed — a
CIW-shaped gap, to be filed.

**PROPS-1 dual adjudicated (2026-09-05).** Both arms on frozen
`acc0719ad`: R1 A-W-F 2/10/8, R2 A-W-F 0/8/5 (rubric R1 idiom 4 / tests
2 / docs 2; R2 4 / 3 / 2). **Soundness upheld by both by execution**
(R1: 21 600 mirror + 3 200 rejection rows against a formula-free
geometric characterisation; R2: 2 400 × 16 and 3 000 × 16 containment
sweeps, both lanes) and every headline number reproduced. **The two
MAJORs are CONVERGED substances with a severity divergence** — R2 found
both at MINOR: (1) `reject_from`'s new form amplifies a WIDE `onto`
through two cross products (up to 34×, worst 1022× on a zero-straddling
component) and the corpus has no `onto`-width dimension, so no pin could
see it; (2) the new totality clause ("bands unchanged") is false — the
numerator scales as `|onto|²·|self|`. No unilateral MAJOR; the severity
split is calibration data. Convergent MINORs: the wide-normal "1.0–1.12×"
and the oblique factors do not reproduce; the soundness pins compare the
shipped formula with itself (a sign-flipped rejection passes); "never
wider" is vacuous on exact-anchor rows and pin (a) does not hold per
component there (a silent spec deviation); history clauses; "nine pins".
Unilateral MINORs adopted: R1's `replace_face`→`translate_curve` stored-
curve path missing from the drift accounting, the `4 ulps` metric
mismatch and unguarded numbers, the grouping rule broken at the mirror
site; R2's negative-axis signed-zero row, the x-only geom test, the
rider's missing counter-argument. **Ruling on the regression**: keep the
shipped form — every in-tree `onto` is an exact stored axis and `self`
the computed vector — and make the contract honest with an `ONTO_RADII`
corpus dimension and a pin of the measured bound. Fix pass
implementer-inherited, 10 items A–J, all taken; the reviewers' probe
rows named for adoption. Row and sample number at merge.
**Verdict shapes MERGED (2026-09-05)** —
[#1920](https://github.com/evgunter/cad/pull/1920), head `9f52d8df`,
green on the full matrix (run 33945846151) after the fix pass took all
eight adjudicated items: the tag-byte invariant stated at the match,
the structural reason no key moved (`drive` never cancels and refuses a
non-`Ok` witness) in the tree at `certifying_vector`, which replaces the
split inherent impl as a free function, one sign ladder, the positive-
control pin, the history clauses gone. One public path moved
(`VerdictVector::certifying` → `drive::certifying_vector`), recorded as
the unit's deviation 3. The item is closed; the spec leaves `docs/` with
this merge (ledgered). Next in the lane:
`k-stats-escalation-channel-and-redo` (D→H, L; a dual unit).

**S-CERT's exit walk is PROPOSED (2026-09-05, check-in 05:26).**
[#1924](https://github.com/evgunter/cad/pull/1924), `[ev]`, ratified by
merging; the closing sweep then re-homes S-CERT's 24 open items and
deletes `work/cert/`. Its handoffs ledger diverges from the ratified
PROPS charter: the offset_fit lane (three items), `refine-dir-hairline`,
the QUAD2 dial, `normalize-overflow` and `orthonormal-basis-poisons`
are proposed for `work/issues/`, `k-report-baseline-fold` (Track K's)
for `work/props/`, `pole-branch-pick` for `work/bool/`, and the
territory globs PROPS' header claims at the exit for no owner. Re-points
posted on #1924 per the charter (the taker says it takes them); Ev
decides at ratification. #1878 (ChartRegionLane) is restated and
unanswered; the walk lists it among three questions ratification does
not answer. PROPS-1's fix-pass head `a7aef51d` is green (lane reporting).
Slots: two lanes live (PROPS-1 fix pass, ONB-measure); the Span sweep
dispatches when PROPS-1 lands.

**PROPS-1 MERGED (2026-09-05)** —
[#1918](https://github.com/evgunter/cad/pull/1918) at `93baf9ce0`, fix
run 33946767666 green on the full matrix. Row recorded at merge: ordinal
2400, sample **#137** (renumbered at the sync — #136 fell to FILLET-H7 by
merge order); no tally candidate, the pair FAIR. The two respells stand
with an honest contract: the wide-`onto` regression disclosed and
pinned, the totality bands real, the soundness rows formula-free (both
reviewers' probes adopted). The spec left `docs/` with the merge
(ledgered); the item is closed; the audit item keeps its remaining
members. Block PROPS-B1 slot 0 concluded; **slot 1 = the Span sweep**,
dispatched on `props/span-knot-vector` (L / STRUCTURAL, pre-draw at the
spec). Lanes live: Span, ONB-measure.

**ONB-measure MERGED (2026-09-05)** —
[#1939](https://github.com/evgunter/cad/pull/1939), evidence only, green
on the full matrix. The numbers: the backend (in-repo
`interval-transcendentals`, not inari) drops the sign of zero in `*`
and `/` with `normalize` on every plane-minting path, so option (c) is
unsound as the tree stands; 815 planar faces censused, exactly 12 walls
with `n.z = −0.0`, all boolean-reversed faces in `die`/`kiss_assembly`;
under (c′) all 12 move (8 STEP `DIRECTION` records, 4 half-turn `u_ref`
flips), no `FaceFrame` anywhere in any corpus; on M10-5's prism 6 of 12
walls narrow from width 2 to ≤ 7.4e-15, 2 stay hulled on a genuinely
wide `n.z`. The item's "cannot converge" premise was false (the cost is
a 2× enclosure; `refines` is true today) and is corrected. **Ruling for
Ev**: `[ev]` PR opening now with recommendation (c′) — canonicalise the
zero at f64 inside `orthonormal_basis` — and its one cost named (a
`FaceFrame` on a boolean-reversed wall would rotate silently; none
exist, no migration channel). Backend findings filed
(`interval-backend-signed-zero-conventions`); the tour row of table 2
is the one acceptance gap (the tour's scenes are private modules). Slot
freed: lily-vec3 dispatches beside the Span sweep.

**Span sweep delivered (2026-09-05)** as
[#1952](https://github.com/evgunter/cad/pull/1952), head `2eccccead`,
green on the full matrix (run 33949970914; python suite closure-skipped
— filed on CIW). The mutation-hold check found no site: the spline layer
has no `&mut self` method, so a refinement is a new binding and the
constraint is pinned by an E0506 `compile_fail` row rather than
observed. 37 files, +1344 −1453; the census's site count was high
because `SurfaceWindow` collapsed rows. One argued deviation: the three
surface doors moved ONTO `SurfaceWindow<T>` because a lifetime does not
brand — `b.eval_in_span(a.window_at(..))` typechecks with the spec's
shape and would silently answer for A. Residue filed:
`coefficients-carry-their-knot-vector`. Dual review dispatched on the
frozen head; ordinal 2401 claimed (#1961 carries it to main). Lanes
live: two Span reviewers, lily-vec3.

**The exit walk executes its homes in-PR (2026-09-05).** After Ev's
"as long as all residuals are filed appropriately", #1924 now moves the
24 items itself: eight to `work/props/` (the two sphere-pole items, the
two hygiene items, `quad-face-extent`, `purchasable-area-valve`, plus
`k-report-baseline-fold` and `edge-chord-len` which the charter names
as Track K's and S-BOOL's), seven to `work/issues/` including the six
the charter assigns here (the offset_fit lane, `refine-dir-hairline`,
the QUAD2 dial, `normalize-overflow`) and `orthonormal-basis-poisons`.
My re-points were not taken in the PR. Plan: after ratification, CLAIM
the six from `work/issues/` by `git mv` (the README's claim rule) and
re-home the two non-charter arrivals to `work/issues/` for their owners
to claim; then the plan's lanes are complete.

**lily-vec3 delivered (2026-09-05)** as
[#1954](https://github.com/evgunter/cad/pull/1954), head `a700a6c3`,
green on the full matrix incl. the tour's own row and all three render
lanes ("matches this render"). Every tuple helper gone; the lift spelled
once per boundary through `map`; 9 of 15 lily bodies byte-identical,
six moved by ≤ 4.2e-15 with the cause MEASURED (reverting only the two
`reject_from` calls restores all 145 artefacts byte-for-byte — the
respell from PROPS-1, not the rewrite). `D79`(b) deleted member by
member. Single style review dispatches when a Span reviewer frees a
slot. Filed: `vec3-point3-const-and-conversion-doors` (props), the
tour-wide layer-rule sweep and the stale tour `Cargo.lock`
(`work/issues/`). Lane hazard recorded: the session scratchpad under
`/tmp/claude-0/…/scratchpad/` is shared between lanes on this box — two
lanes overwrote each other's file; briefs from here on name a private
`/home/user/<lane>-tmp/`.

**Span dual interrupted and resumed (2026-09-05, 06:49–08:01Z).** Both
reviewers died on the account's session limit mid-review (R1 while
writing a probe; R2 earlier) and were resumed from their transcripts on
the same arms after the 07:40Z reset — a method note applying to BOTH
arms equally (the CERT-M3/N3 precedent), not a relaxation; the usage
counters record post-resume segments only and the row will say so.
The orchestrator session itself was resumed cold from its transcript
by the same limit; every worktree and target survived. Lily's style
review still waits for a slot behind the two reviewers.

**Span dual adjudicated (2026-09-05).** R1 NOT-MERGEABLE-AS-IS 4/7/4
(idiom 3 / tests 3 / docs 2); R2 A-W-F 1/2/8 (4 / 4 / 4). Held under
both: the borrow, the surface half, bit identity (R2 re-derived it at
the merge base over 11 151 rows), the `compile_fail` codes, `pncad-py`
compiles. **The curve half is not closed, and the spec's own curve-door
paragraph is why**: the retired `admits` at the curve doors was
load-bearing — a span from a longer vector now PANICS where it poisoned
(R1's M1, executed; R2's probe used equal control counts and saw only
finite wrong answers — a MINOR). Converged: the `quad.rs` `(kv, span)`
door left standing while the body said it was converted (both MAJOR —
bilateral). Unilateral: R1's M1 (the panic, executed) — **one v6 tally
candidate, R1/OPUS, flagged for the blinded adjudication**; R1's M2/M3
(the dead private guard; the surface argument indicting the curve half)
are reading/design, not executed. **Ruling**: close the curve half the
way the surface half was closed — a curve-held window with the doors on
it — with the poison-guard restoration as the stated fallback and a
filed follow-up if the fence cannot carry it. Fix pass
implementer-inherited, items A–K, all taken. The M6 sense-gate item
re-read: a tracker, not a unit — residual 4 is CURVED's on TOPO's
territory, residuals 1–2's single-face halves closed by the wedge arm,
the remainder design-shaped; citations repaired. Next spec written:
`docs/PROPS-KSTATS-SPEC.md` (the verdict bracket with a stack + the
escalation channel; L; block slot 2) — dispatches when a slot frees.

**lily-vec3 reviewed (2026-09-05).** Style review on `a700a6c3`:
APPROVE-WITH-FIXES, MAJOR 0 / MINOR 3 / NOTE 2 / style 8; rubric 4 / 4 / 4.
Every executable claim reproduced (the artefact table number for number;
the two-call revert restores 143/143). **The dispatcher's census missed
a door**: `pncad::authoring::{p3, v3}` already exist beside the `p2` the
file uses — so the lift has one spelling per dimension and `map` is only
for computed values; taken as the first fix item. The other fixes are
the header's over-claim ("exactly once" — two `Affine3<f64>` doors cross
unlifted), the body's sweep claim (`skinned.rs`, `cutaway.rs` carry the
shape — the tour-wide sweep is filed), and the probe half's last
duplicated spellings. A second library finding filed on the doors item:
no `Affine3::from_frame` and no `SketchPlane` lift. Fix pass
implementer-inherited.
**lily-vec3 MERGED (2026-09-05)** —
[#1954](https://github.com/evgunter/cad/pull/1954), head `1111cf0f5`,
green on the full matrix with all three render lanes at "matches this
render". The lily is authored in the kernel's own vector types with one
lift spelling per dimension (`pncad::authoring::{p2, v2, p3, v3}` for
literals at the door, `map` for composed values), the tuple algebra gone,
the header stating the layer rule and its two `Affine3<f64>` exceptions;
`D79`(b) closed member by member. The six moved bodies are the
`reject_from` respell's ulps (PROPS-1), measured by the two-call revert.
The item is closed; the spec is ledgered in the per-merge form — the
form the ledger's recent entries use, and the right one: a closed
sweep's list carries its own count and recovery SHA. The two earlier
PROPS lines appended to sweep 6's list (verdict-shapes, ONB-measure)
move to that form at the next orchestrator sync.

**Span sweep MERGED (2026-09-05)** —
[#1952](https://github.com/evgunter/cad/pull/1952), head `e13d5df0d`,
merge `46020d6b9`, green on the full matrix (37 jobs; the first fix head
was red on the doc gate alone — a broken intra-doc link — and the lane
now runs `scripts/doc-gate.sh` locally). The fix pass took the
STRUCTURAL close: evaluation in a span moved onto `CurveWindow{2,3}`
borrowing its curve, so the panic R1 executed no longer compiles; both
bit-identity digests unchanged after the doors changed receivers twice.
A/B row recorded (ordinal 2401, **sample #140**, one tally candidate
R1/OPUS — the executed panic; both arms interrupted and resumed equally);
block PROPS-B1 slot 1 concluded on `props/b1-block`. **Orchestrator
error disclosed**: the ordinal-2401 claim paragraph was committed at
dispatch on the block branch instead of this one, so #1961 did not carry
it to main as its body said — carried now, with the note in the ledger.
The lane's three out-of-fence notes: (1) `ssi/enclose.rs:418,456` is a
subject edit on cert's ground (net read through `win.surface()`),
disclosed in the PR body — S-CERT is at its exit walk, so it is recorded
here rather than in a tracker about to be deleted; (2) `benches`,
`demos/tour` and `demos/wild` lockfiles were stale on main (a `profile`
edge on the `verbs` entry) and dirtied every lane that ran the doc
gate — regenerated with cargo in this sync, closing the stale-lockfile
half of `work/issues/tour-scenes-lift-componentwise-not-through-map`;
(3) the python-suite closure skip is the CIW finding already filed
(`closure-tier-skips-python-suite-on-geom-core-changes`, also carried
to main only now). Lane reclaimed. Live: the k-stats bracket (slot 2).

**k-stats bracket delivered (2026-09-05)** as
[#1969](https://github.com/evgunter/cad/pull/1969), head `e832a1fc2`,
green on the full matrix (run 33958408008, 37 jobs; the python suite
RAN this time — the closure tier reached it through `editor-core`).
The returned-value alternative measured and declined in writing (530
`decide*` call sites, 261 enclosing functions, 104 public signatures);
the bracket with a stack, `!Send`, `Drop`-popped; escalations recorded
beside verdicts on both `NodeValue` and `NodeError` (deviation 1) and
read first by `drive::classify_replay`; both red-first rows quoted
(0-vs-724 nesting on the instantiate fixture; `Budget`→`SliverTerminal`
on `slab(20ε, 40ε)`); witness-vector keys byte-identical, the accounting
goldens re-blessed as the acceptance's own move. Five argued deviations
including one hunk in `eval/parts.rs` outside the fence (the part-cache
miss path shielded by its own discarded frame) and a bless arm on
M10-7's tier-off differential. Dual review dispatched on the frozen head
at 09:57Z: ordinal 2402 claimed (#1972 carries it — committed on THIS
branch), byte 200 ⇒ R1 opus, R2 fable; briefs stored on `props/b1-block`
with sha256 (`kstats-review-brief-{r1,r2}.md`). The reviewers' first
targets are the spec's: the nesting fix under memo reuse and
cancellation; the channel's completeness. Four findings the lane left
for filing wait on the dual (the pre-bracket decisions of the profile
pre-pass and the mate solve; a part's per-node logs dropped with its
nested evaluation; the coincidence zone priced `Budget` at the floor —
M10's; `refuse_non_finite`'s out-of-funnel `sign_within`).

**k-stats dual adjudicated (2026-09-05).** R1 A-W-F 1/7/7 (idiom 4 /
tests 3 / docs 3); R2 A-W-F 0/6/8 (4 / 4 / 3). Held under both by
execution: the bracket-with-a-stack, the nesting fix under memo reuse,
cancellation, the parallel schedule (×8 and ×12 runs), part-in-part
(922/922); the 530 / 261 / 104 measurement reproduced to the digit by
both (the ruling's support is honest); the accounting move is entirely
`classify`'s (R2: main's `drive.rs` on this head's eval reproduces
main's bytes); no witness-vector key moved. **Converged, with a
severity divergence**: the escalation channel misses the family of
eight shipped sites that mint their own `Indeterminate` after a
definite sign, while `drive.rs` and the body say "none is known" (R1
MAJOR, executed on `enters_material`; R2 MINOR) — the same substance,
calibration data, NOT a tally candidate. Also converged: the
mis-nesting D2 row is wrong in every profile (R2: the repo's release
profile keeps debug assertions, so the discard arm is dead and the
assert leaks the outer frame; R1: with assertions off a stale guard at a
reused index STEALS — executed); the escalation-first read preempts the
box-independent terminal classes; territory 30/31 not 23; the spec's
"file it" not carried out; the m10_7 row's name now false; no
parallel-schedule row; `seqgen.rs:641` a second raw `sign_within`.
Unilateral, all MINOR/NOTE: R1's unpinned `!Send` doctest (E0277
measured by both); R2's missing "not persisted" sentence on
`NodeError`; R2's mate-solve escalations on no node's log. **No
unilateral MAJOR — no tally candidate.** Silent deviations: R1 counts 2
(two tcost test paths; the un-filed sweep), R2 0 silent / 5 body
inaccuracies. Ruling for the fix pass (items A–K, implementer-inherited):
frame ids make mis-nesting defined and non-stealing in every profile
with no assert; the box-independent classes read before the channel;
the completeness claim restated and guarded by R1's probe as a row, the
family filed (not routed — the spec's separate unit); `compile_fail,E0277`
with a twin; six issues filed (the arms sweep, the bracket's scope, the
part's dropped logs, M10's coincidence zone, the op-minted family, the
bare `compile_fail` class); the schedule rows adopted with literal
counts. **Orchestrator error disclosed**: reclaiming the reviewer lanes,
my copy clobbered R2's editor-core probe file (same basename as its
geom-core one); the rows are rebuilt from R2's report, R1's schedule
probe covering three of the six. Reviewer lanes reclaimed; the
implementer runs the fix pass.

**Two E riders dispatched (2026-09-05)** while the k-stats fix pass runs
and the sphere lane waits on #1924 and MESH-12 (#1617, still open on
`props/curved.rs`; CERT-M3 #1877 and CERT-N3 #1879 both merged this
morning, so the exit walk is the inheritance's only gate).
`docs/PROPS-VEC3-DOORS-SPEC.md`: `const fn new` on the four vector
types, `Affine3::from_frame` as the one home (the `SketchPlane` body
moved down, bit-identical), `SketchPlane::map`, `skinned.rs` on the
door; the `Vec → Point` conversion REFUSED as a ruling written at the
type (a point is not a vector); `lily.rs` left to the tour-wide sweep.
`docs/PROPS-ROTATION-FLOOR-SPEC.md`: ruled NO RESPELL on the item's own
numbers (≤ 17 % at a start sample, 0 % at full period; the floor is the
backend's `cos` at exact angles) — a present-tense paragraph at
`rotation_about` with the pair re-verified, the composition rider filed
at its own home, the item closed with the ruling. One lane, two PRs in
sequence, single style reviews, outside the experiment. Seam posted to
BOOL for `SketchPlane`.

**vec3-doors LANDED (2026-09-05)** —
[#1977](https://github.com/evgunter/cad/pull/1977). `const fn new` on
the four vector types (generic; the bound costs nothing), one home for
the frame constructor (`Affine3::from_frame`, `SketchPlane::from_frame`
delegating bit-identically), `SketchPlane::map` with the two lift
spellings written at the door, `skinned.rs` reading the door; the
`Vec → Point` conversion refused as a ruling at `Point3`'s type doc and
in the item's `## Closed`. `lily.rs` untouched (the tour-wide sweep's
sites); `teapot.rs`'s three struct-literal constants named in the PR's
sweep for placing. Item closed; spec deleted and ledgered in the
per-merge form.
**k-stats fix pass (2026-09-05)** — on
[#1969](https://github.com/evgunter/cad/pull/1969), the dual's eleven
items A–K all taken (one refuted by measurement inside C: 8 bare
`compile_fail` fences on this head, not 16, all in
`quantity/src/units.rs`). The mis-nesting rule was wrong in every
profile the repo builds (`debug-assertions = true` everywhere; the
assert fired before the truncate and leaked a frame; with assertions
off a stale guard stole a later bracket's decision) — frames now carry
per-thread ids and an out-of-order close is defined and pinned in the
default profile. The channel's completeness claim was false and is
restated where it was made: the log carries the funnel's escalations;
eight op-minted `Indeterminate`s, two raw `sign_within` calls and the
unbracketed mate solve reach a consumer through the error enums, whose
arms in `classify_replay` are load-bearing; `classify_replay` now reads
a definite box-independent refusal first, the log second, the arms
third. Rows adopted: r1's steal probe, escalation-channel probe and
parts/schedule rows; r2's release and geom-core rows and, rebuilt from
description, the outside-bracket counts (0 assembly / 75 part) and the
mate-solve escalation in an outer frame. Six issues filed
(`escalation-channel-misses-op-minted-indeterminates`,
`indeterminate-error-arms-sweep`,
`part-per-node-logs-dropped-with-nested-evaluation` here;
`bracket-scope-is-run-op-not-the-node`,
`compile-fail-blocks-without-error-codes` in `work/issues/`;
`coincidence-zone-priced-budget-at-the-floor` on M10's slate). The
item is closed; the spec deleted and ledgered.

**k-stats bracket MERGED (2026-09-05)** —
[#1969](https://github.com/evgunter/cad/pull/1969), head `692141c5d`,
merge `3f8a91ff3`, green on the full matrix (run 33962252657, python
suite included). Fix pass A–K all taken; one sub-claim refuted by
measurement (the bare `compile_fail` class is eight fences, all in
`quantity/src/units.rs`, not sixteen — filed with the measured list).
Frames carry ids, so mis-nesting is defined and non-stealing in every
profile with no assert; the channel's gap is stated at three sites and
guarded by R1's probe as a row; the box-independent classes read before
the log; `Escalation.predicate` reads through `source`; the schedule,
part-in-part, memo, cancel, pre-pass and mate-solve rows adopted with
literal counts (mid-run cancel not adopted — not deterministically
reachable, stated). Territory measured at 35 paths; seams posted to
DOCM (`eval/parts.rs`) and TCOST (the new test files) beside M10 and
BOOL. A/B row recorded: ordinal 2402, **sample #142**, no tally
candidate. **Block PROPS-B1 concluded** — its record and the six stored
briefs merged with #1978; the next kernel unit draws PROPS-B2. Lane
reclaimed. Live: the linalg riders lane.

**rotation-floor LANDED (2026-09-05)** —
[#1980](https://github.com/evgunter/cad/pull/1980). Ruled NO RESPELL on
the item's own numbers, re-taken at the head by the `cert3_evidence`
rows and unmoved (`t` alone 100 % / 133 %; `t` and `c` 83 % at the
start sample, 100 % at full period): a present-tense paragraph at
`Mat3::rotation_about` states the floor as the sum of the two
enclosures, what each respell recovers, why `identity_minus_rotation_about`
differs, and that the floor is the backend's. The composition rider is
its own file in `work/issues/` (`mapped.rs` is in no program's paths),
with the composition-side fix named: compose in the parameter, keep one
placement. Item closed; spec deleted and ledgered in the per-merge form.
**vec3-doors reviewed (2026-09-05).** Style review on `1e168c607`
(#1977): APPROVE-WITH-FIXES, MAJOR 0 / MINOR 2 / NOTE 6 / style 6;
rubric 4 / 4 / 4. Every executable claim held — bit identity of the
frame over 2535 frames at `f64` and `Interval` (sign patterns, NaN,
±inf, non-orthogonal pairs), the `const` doctest red on main with
`E0015` ×4, the `map` doc's normal widths measured as written. Fixes:
the deferral's carrier issue does not cover `lily.rs`'s sites as
written (extend it); the profile TEST file is a fence deviation to
disclose (BOOL's seam amended); `Mat3::from_cols` and
`Affine3::from_parts` are literal bodies and go `const` too; two doc
overstatements ("transcribed", "not the same plane"); the signed-zero
corpus widened to its prose; the duplicated test helpers given one
home; the obligation rule one home; `Point2` points at the ruling. A
non-tour consumer of the lift found by the review
(`eval/wire.rs:1132` through `anchor::embed_affine`) is filed, not
fixed — editor-core is outside the fence. Fix pass implementer-inherited,
sequenced after the rotation-floor unit's PR opens.

**Check-in (2026-09-05, 11:54Z).** #1944 (sign-hull ruling): no answer
yet. #1924 (S-CERT exit walk): still open, but its 10:40Z update takes
every PROPS re-point — the offset_fit, rational-quad and linalg-lane
items (`budgetexhausted…`, `offset-fit-mignitude…`, `patch-bound…`,
`refine-dir…`, `quad2-rational…`, `normalize-overflow…`,
`orthonormal-basis…`, `pole-branch…`) now move INTO `work/props/` in
that PR, nothing goes to `work/issues/`, and PROPS' territory paths
(`geom-brep/src/props/*`, `offset_fit.rs`, `patch_bound.rs`,
`geom-core/src/*`, `geom/src/*`) join `program.md` there. So the
post-ratification plan is no longer a `git mv` sweep: retire the
header's successor clause, update the plan's §Opening condition and
§Early lanes, and dispatch the offset_fit lane (E→H→D) and the sphere
lane (still behind MESH-12, #1617). Riders: #1977 (vec3-doors) reviewed,
fix pass queued behind #1980 (rotation-floor), whose head is on CI.

**rotation-floor reviewed (2026-09-05).** Style review on `f02fce570`
(#1980): MERGEABLE, MAJOR 0 / MINOR 3 / NOTE 4; rubric 4 / 5 / 4. The
instrument reproduced line for line and the decomposition checked by
hand (the axis entry's width is EXACTLY the sum of the two enclosures;
`t` from the half angle is exact at θ = 0, what remains is `c`). Fixes,
all prose: the paragraph omits `t`-alone's 133 % regression at full
period; one cite off by six lines in the re-homed issue; the body's
territory sentence contradicts the tool (`DOC-LEDGER.md` is META's);
a test-file sentence this unit rotted (`revolved_point_anchor.rs:95`);
the "different reason" wording reconciled across the paragraph, the
neighbour's bullet and the item; the three homes of the decomposition's
digits pointed at the one paragraph. Fix pass implementer-inherited,
sequenced after the vec3-doors fix pass in the same lane.

**rotation-floor MERGED (2026-09-05)** —
[#1980](https://github.com/evgunter/cad/pull/1980), head `2c42a8bff`,
merge `5a483b12a`, green on the full matrix (run 33966034494). The
ruling stands as landed: no respell; the diagonal's floor documented
once at `rotation_about` with the re-verified pair and the 133 %
regression; the two operators' reasons reconciled; the law row and the
audit item point at the one home; the composition rider filed as
`work/issues/mapped-curve-restrict-composes-placements-per-split`
(`mapped.rs` is in no program's paths). Fix pass all taken. One
disclosed fence deviation (a doc block in `revolved_point_anchor.rs`,
tcost's — rot this unit created). E rider: no A/B row.

**vec3-doors MERGED (2026-09-05)** —
[#1977](https://github.com/evgunter/cad/pull/1977), head `cfd272ce7`,
merge `db07a1641`, green on the full matrix (run 33966064457). Landed:
`const fn` on the four vector constructors and on `Mat3::from_cols` /
`Affine3::from_parts` (a `const` placement reads through them);
`Affine3::from_frame` as the one home with the 2535-frame bit-identity
corpus beside it; `SketchPlane::map` with the conditional two-spellings
doc; `skinned.rs` on the door; the `Vec → Point` conversion refused as a
ruling at `Point3` (`Point2` points there). Fix pass all taken; the
lift's non-tour consumer (`eval/wire.rs` through
`anchor::embed_affine`) filed as
`work/issues/affine-lift-has-a-second-home-in-anchor-embed-affine`;
`lily.rs`'s sites and constants named on the tour-wide sweep's carrier.
E rider: no A/B row. Both riders landed; the lane is reclaimed.

**coeffs-window dispatched (2026-09-05)** — the Span sweep's residue,
`coefficients-carry-their-knot-vector`, as the first kernel unit of
**block PROPS-B2** (drawn: byte 87 ⇒ fable at slot 0; record branch-side
on `props/b2-block`). Ruling: option (a) in the structural form the
curve half took — `SplineCoeffs<'a, E>` minted only by
`KnotVector::coeffs*` (the length check once, at the mint),
`CoeffWindow` carrying the pair, every free `hull` door a method on one
of the two so no free function takes a coefficient array; the three
residue shapes become `compile_fail` rows with twins; a bit-identity
digest captured at the merge base through the retired spellings.
Spec `docs/PROPS-COEFFS-SPEC.md`; L / STRUCTURAL; seams posted to TRIM
(`ssi.rs`, `ssi/certify.rs`) and MESH (`chords.rs`). The riders lane's
one finding for CIW filed
(`no-ci-run-on-a-conflicting-pr`); `teapot.rs`'s three struct-literal
constants (const-convertible, the tour's) stay with the tour-wide
sweep's carrier.

**coeffs-window landed** — PR
[#1985](https://github.com/evgunter/cad/pull/1985), branch
`props/coeffs-window`. `SplineCoeffs<'a, E>` borrows the `KnotVector`
its array was fitted against (weights optional), minted only by
`KnotVector::{coeffs, coeffs_rational}` with the count checked once;
`CoeffWindow<'a, E>` is the pair beside a `Span` of its own vector;
every `hull` door is a method on one of the two and no free function
in `hull.rs` takes a coefficient array. The three residue shapes are
`compile_fail` rows with twins; the 960/480-row coefficient digest and
`geom`'s 1001/11,151-row span digests are unchanged across both lanes.
Consumers in `ssi.rs`, `ssi/certify.rs`, `props/quad.rs`,
`spline/net.rs`, `curves/nurbs.rs` and `mesh/chords.rs` on the pair,
none mis-paired. Sweep residue (the `quad.rs` evaluators, `TensorNet`
and `to_bezier_spans` carrying the same shape) reported in the PR body
for placing. Item closed; spec deleted and ledgered.

**coeffs-window LANDED BEFORE ITS REVIEW (2026-09-05) — orchestrator
brief defect, disclosed.** The implementer lane opened
[#1985](https://github.com/evgunter/cad/pull/1985), polled it green
(run 33969355105, full matrix; one red on the interval-cfg gate,
root-caused and fixed by the lane) and MERGED it (`55d541ae5`) before
the dual was dispatched. The cause is mine: the spec's §Landing said
"the item `status: closed` … the spec deleted at merge", the landing
wording of a rider, where the k-stats and Span specs said `status:
review` and left the close to the fix pass. The unit stays merged
(merge-only); the dual runs on the merged head `4521bd658` as if the PR
were open — ordinal 2403 claimed, byte 25 ⇒ R1 fable, R2 opus, briefs
stored on `props/b2-block` — and its findings land as a fix-pass PR.
The A/B row will carry the deviation. Sample #144 (CURVED's C5ARMS took
#143 minutes earlier). Every future kernel spec's §Landing says `status:
review` and "do not merge; the orchestrator lands after the dual" in
those words. The lane's three findings for placement (the
`bspline_eval_ring*` evaluators, the tensor grids, `to_bezier_spans`)
wait on the dual.

**coeffs-window fix pass (2026-09-05)** — branch `props/coeffs-fixpass`,
the dual's APPROVE-WITH-FIXES (0/4/4 and 1/5/4) on the merged head
`4521bd658`. Taken in full: the pair split into `SplineCoeffs` /
`RationalCoeffs` with the rational and nonrational doors partitioned by
type (both directions D2 row 0, `compile_fail` rows (d)/(e)); the
triplicated differencing helper folded to `KnotVector::difference_coeffs`
and `quad.rs`'s two range-hull spellings to one; the mints renamed
`with_coeffs` / `with_rational_coeffs`, the pair accessor `pair()`, four
silent accessors deleted; doc rot at `spline/mod.rs`, `hull.rs`,
`spline_hull.rs`, `span_window_pairing.rs` fixed; the three dead
refusal arms annotated; the sweep residue filed
(`coefficient-vector-pairing-survivors`, with the reviewers' additions
and the blind spot); the dual's 3,403-row corpus and type rows adopted.
All four digests unchanged. The territory tool's full 40-path output is
in the PR body (the unit's body listed three). Item stays closed; no
ledger line.
