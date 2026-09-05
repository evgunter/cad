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
