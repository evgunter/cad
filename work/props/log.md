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
