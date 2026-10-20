# PATHS log

## Opened at S-BOOL's exit (2026-09-16)

Opened by S-BOOL's orchestrator in the PR that proposes
`docs/S-BOOL-EXIT-WALK.md`, as `work/README.md`'s closing rule directs:
S-BOOL's eight open lattice items cohere into one track on one
territory (`crates/profile`), so the closing program opens the
successor and moves them here. Band 5000–5099 recorded in the ledger's
banding entry in the same commit. No unit dispatched; the first sitting
picks from `work/paths/plan.md` §The slate. Ev's sign-off on the exit
walk ratifies the opening.

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `arc-carrier-refusal-register-misses-two-format-arms`.**
`docs/PATHS-DESIGN.md` is yours by `paths`; the row was on FIX's slate
only because FIX took the two units that moved the arms the register
omits, and its own Fence section says so.

Its two halves separate cleanly. The cheap one: the *"Typed runtime
errors, from geometry"* register names neither `PathError::NonFiniteDirection`
(landed by PR 2359, never named) nor `PathError::UnderflowedDirection`
(PR 2415), both raised from inside that document's own surface — so it
wants the two arms plus the sentence distinguishing them from
`ZeroDirection` (not a coincidence at any tolerance; the recourse is
scale, not eps). The design half: whether that register is checked at all,
given `PathErrorKind` is a fieldless mirror and `pncad-py`'s
`TAG_INVENTORY` already pins the full arm set at the Python door — or
whether it is prose by design and should say so, so the next reader does
not take its completeness for a claim.

FIX keeps `fillet-leg-carrier-renders-raw-float-noise`
(`crates/profile/src/validate.rs`, your ground) because its fix IS
written — route the two `f64` fields through `path::num` — and FIX spans
fences by announcement for written fixes. Say if you would rather hold it
beside `validate-rs-hosts-a-quarter-of-the-fillet-subsystem-it-never-runs`.

Signed (FIX orchestrator).

## Announced seam from FIX (2026-09-21)

**`crates/profile/src/validate.rs` and `crates/profile/src/path.rs` —
PR 2946.** FIX's `fillet-leg-carrier-renders-raw-float-noise`, one of
the five written-fix rows left on its slate after the design-free
sweep. The whole unit is on your ground, crossed by announcement.

`impl Display for FilletLegCarrier` rendered `Arc`'s `radius` and
`angular_margin` through `f64`'s own `Display` — the shortest
round-tripping spelling, i.e. the arithmetic's noise unshortened. Both
now go through `path::num`, which changes from `fn` to `pub(crate) fn`
so a sibling module can reach it. **The rounding grid itself is
untouched**: the `min`, the compile-time `DEFAULT_EPS` cap and the
floor-is-the-mirror-defect clause are exactly as PR 2399 left them.

**Two things worth your attention beyond the one-line fix.**

- **The carrier's sentence is not only its own.** It is interpolated as
  `CornerReason::AnchorOutsideTrimmedExtent`'s `{carrier}`, beside a
  `{setback}` and `{available}` that `num` already shortened. So a
  `path` refusal with no noise in any scalar of its own was still
  carrying noise in its carrier clause.
- **`crates/profile/src/` is now clean of this class**, and the claim
  has two instruments behind it rather than one: a sweep for
  `{ident} m` / `{ident} rad` (26 lines, 36 interpolations, all but
  this one already routed), plus a reading of all 22 `Display` impls in
  the crate — which is what caught `CornerRefusal` rendering two
  ordinates with no unit word, invisible to the pattern by
  construction. It was already routed. What neither instrument can see
  is a scalar reaching prose through a containing type's `Debug`; that
  residual is stated on the item rather than left implied.

**`num`'s doc comment gained two paragraphs** — that a plain `f64`
field reaches the same defect by its own `Display`, and that the
helper's reach is now the crate's refusals rather than `path.rs`'s arms
(the old closing line said *"every arm below"*, which became narrower
than the truth once a second module called it). Checked before
editing: `num` is discussed in no `docs/` page and not in
`crates/profile/README.md`, so this is a source comment following its
code, not a design amendment.

**Nothing was re-baselined**, because nothing in the tree had ever
pinned this sentence — every consumer matches on the enum's fields. The
new pin uses subtracted rather than literal scalars, since `0.008` and
`0.0035` are exactly representable and a literal would have rendered
correctly with no helper at all.

Signed (FIX orchestrator).

## 2026-09-25 — a PATHS orchestrator picks the track up

Status `ready` → `active`. Ev's first ask for this sitting is
`lower-profiles-to-carrier-and-interval-not-vertex-and-bulge` (EMIT's
filing from #3202): let a circle be one edge by lowering to a
carrier + interval form instead of vertex + bulge. First step is the
survey the row asks for — a read-only lane mapping every reader of the
bulge form (profile, sweep, editor-core, persist, Python, demos) and
what each needs from a carrier + interval form — then an `[ev]` PR
against PATHS-DESIGN §2a.1/§6 (the M2 closed-carrier precedent) with
the design choices it surfaces.

Orchestrator branch is the session's assigned branch, not
`paths/orchestrator` (the remote session names it); unit branches keep
the `paths/` prefix.

The track is over budget (38.5/30). Splitting along the priority seam
is deferred until the lowering survey says how many rows it absorbs or
spawns — several slate rows (the closers, `circle_split`, the lift
comparator) may change shape under a carrier + interval lowering.

## 2026-09-25 — the lowering survey is in; `[ev]` PR opened

The survey is kept on the row itself
(`lower-profiles-to-carrier-and-interval-not-vertex-and-bulge.md`,
"Survey"). Corrections to the row as filed: D1's "Profile format"
clause is touched, not only PATHS-DESIGN §2a.1; geom-brep's
`SketchSegment` carries its own copy of the bulge form and every
profile-built edge goes through it; the saved file holds programs, never
the lowered form, so the only format break is in names; the symbolic
tier keys on the circle's unit bulge. There are seven hand copies of the
bulge→carrier formula, not three.

The `[ev]` PR re-words D1's Profile-format clause, PATHS-DESIGN §2a.1
and the profile README to the recommended form (A2: verbatim vertices +
`Line | Arc{centre, radius, Δθ}`, consistency verified at validate) and
asks the forks. Settled on #3202 and not re-asked: EMIT ships step ids
first with `Piece(0/1)` circles and takes the second names break.
Proposed unit cut, 0 → 6, is in the survey's §5.

## 2026-09-25 — Ev's first round on the lowering `[ev]` PR

Ev agreed q2 (geom-brep follows the profile form) and q4 (`Bulge` stays
as a path-algebra arc mode; `RawLoop` keeps vertex + bulge as its input).
On q1 Ev asked whether a zero-redundancy form exists, e.g. three points.
The PR body's "no zero-redundancy form works" was too strong and is
corrected: counting dof, a full turn is the blow-up of a = b in the
partial-arc family, so every condition-free form (Z: bulge or via point
plus a separate full-turn arm) reads a full turn differently, and A2
trades that split for a verified carrier. A2 is still recommended, and
Z is offered as coherent. On q3 Ev asked whether `circle` should be
sugar for `circle_split(n = 1)`. The proposed answer is one kernel,
with `circle` kept as its own verb in the program so it reads back as
written. Both q1 and q3 await Ev.

## 2026-09-25 — Ev rules q1 (A2) and q3 on #3218; q4 reopens

Ev chose A2 ("not super elegant but it seems principled and easy to work
with") and agreed q3 (`circle` lowers through `circle_split`'s kernel
with n = 1 and stays its own verb). The doc texts are re-worded to match.
On q4 Ev had thought the vertex + bulge fixture door (`RawLoop`) was
already gone, and finds it odd to keep bulge alive for that door alone.
It is dev-only (absent from shipped builds since BOOL-9 / Q1 half ii).
Its users are about 290 test files and a handful of in-crate `#[cfg(test)]`
modules. The proposal is to keep the fixture door, since validate has
to be tested on tables the algebra refuses, but have it take the
canonical segments. `ProfileVertex` as a bulge record retires, and the
fixtures migrate through a helper that calls the algebra's own `Bulge`
lowering, so the only bulge→carrier converter left is the algebra's.
Ev asked what bulge residue would be left in the kernel under the q4
plan. Answer on #3218: one fixture constructor stays in `profile`
behind the existing `raw_door!` test gate, taking canonical segments
and containing no bulge. The bulge helper moves to test-support and
forwards to the algebra. `bulge_from_center`/`_via` retire from
`pncad`, with an announcement to LIB first. The bulge accessor is
deleted in the unit that removes its last reader. `arc_to(Bulge)` is
the only survivor. The doc texts no longer promise a "derived view".

## 2026-09-25 — #3218 ruled and merged; six build units filed

Ev gave a 👍 on the residue answer, which closes q4. The row carries the
ruling and has moved to `spec`, with `needs_ev` cleared. Units 1–6 are
filed as its children. Unit 1 (`canonical-segment-type-in-profile`) is
dispatchable, and 2–6 are parked on their predecessor. Review tier for
unit 1: **dual**. It is an architectural change with broad reach, and
its byte-identity claim is what every later unit stands on. An
announced-seam note goes on EMIT's log for unit 4's `Piece(0/1)` →
`Carrier` re-spelling.

## 2026-09-25 — lowering unit 1 merges (#3224, DR-5)

`canonical-segment-type-in-profile` is closed. Both dual reviewers found
the same MAJOR (a b = 0 arc verb stored a poisoned carrier, which made
the anchor's area NaN and `lift` emit `ArcTo(b:0)`). It is fixed at the
root: the stored kind is the exact-zero read at every lowering. The
tally is unchanged at 0 because the MAJOR was bilateral.

Class-level findings recorded:
- "Is this a line?" has two layers: the stored exact-zero kind and the
  validated ε-kind. The two are documented on `Segment`.
- 4·atan b is spelled in about six places. Units 2 and 5 own them.
- Bulge stays in storage until unit 5 retires it.

Orchestrator's own miss: the implementer never compiled the
probe-feature tests, and hosted `clippy --all-features` and
`k-lint dev-probe` went red. Unit briefs now say `--all-features`.

Next dispatchable:
- `fixture-door-takes-canonical-segments`: mechanical, D, single
  review;
- `geom-brep-sketch-segment-full-turn`: H, dual review.

Both are unblocked by this merge. They touch different crates and can
run in parallel.

## 2026-09-25 — the fixture-door migration (#3231): review adjudicated

The review was a single FULL review: APPROVE-WITH-FIXES, 0 MAJOR. The
reviewer executed the PR's rewrite script over all 250 migrated files at
the merge base and diffed the result against head, and C1 held
(bit-identical migration). All taken:
- **Canonical-door loops drift, and badly.** They drift under
  `map_scalar`/`reversed`, and for a one-segment circle the drift is a
  NaN centre, not last bits. The `RawLoop::new` doc now says so.
  `one-segment-loop-through-builders` gains a prerequisite: re-lowering
  must carry the stored carrier before n = 1 is admitted, so
  `store-constructed-carriers`' carrier half moves ahead of unit 3.
- **§6 reverted.** My §6 edit is reverted. The sentence is Ev's
  ("per Evan", 0dca9f945), and `bulge_loop` still authors a vertex+bulge
  chain, so the old wording is the accurate one.
- **Stale comments.** Three are fixed.
- **New carrier→bulge copy.** The copy in `RawLoop::new` is added to
  unit 5's inventory.

Declined, with reasons:
- **Deleting the dead shut arm of `raw_door!`.** It would change
  BOOL-9's reviewed seal shape and census row for no behavioural gain.
  The `expect(dead_code)` works: the reviewer showed that a shipped
  call un-fulfils it and a re-export gives E0365.
- **The seal row's self-consistent tan check.** It is left as is; unit
  5 deletes the derivation it checks.

Also on this PR: main's `topo` stopped compiling (the `RingMeetsOuter`
`Display` was missing three `RingContact` arms after #3185 met
`a3d5c47e1`). The fix is ported here and announced on ATREST's log.

## 2026-09-25 — #3231 merged; resequenced; unit 2 dispatched

`fixture-door-takes-canonical-segments` merged as #3231. It also
carried the fix for main's `topo` compile break (announced on ATREST's
log).

**Order now.** The review of #3231 showed that re-lowering from
(chord, kept bulge) turns a one-segment circle's carrier into NaN. So
the order is:
1. `geom-brep-sketch-segment-full-turn` (unit 2, dispatched now);
2. `store-constructed-carriers` (unit 5; moved ahead, because it makes
   re-lowering carry the stored carrier);
3. `one-segment-loop-through-builders` (unit 3);
4. `circle-lowers-to-one-segment` (unit 4);
5. `pncad-surface-for-canonical-segments` (unit 6).

The rows' `blocked_on` fields are updated to match.

**Unit 2's tier: dual.** It is the shared numeric representation of
every profile-built edge, and it is expected to move bits in ulps. Its
spec forbids any decision flip.

## 2026-09-25 — unit 2 (#3254) stops on three moved decisions

The implementer stopped, as the spec requires, on three results:
- **Recut refusal becomes Ok.** At Interval / 1e-12 the recut now
  decides. The likely cause is that `restrict` keeps the exact parent
  carrier.
- **r1_annulus certifies less.** Its certified fraction fell at
  eps = 1e-6.
- **`sym_thin_strip` loses Theorems.** Four decisions went from Theorem
  to NumericZero, and the cause has not been attributed.

Rulings, sent to the lane:
1. **The recut flip** is accepted only on a demonstrated soundness
   check: the restricted enclosure must contain the true sub-arc. The
   escalation constant is re-pinned against a fixture that still
   reaches it. If no fixture does, a row goes to the constant's owner.
2. **The ceiling fall** is attributed by toggling each change. The
   `abs` span against the sign-by-turn span is my call if that turns
   out to be the trade. The pinned ceiling is never lowered.
3. **The lost Theorems** are attributed against main alone first. They
   are not re-baselined until attributed.
4. **The `turned_span` conflict** with DECIDE's `props/sign-hull` is
   announced on DECIDE's log.

Review stays dual, dispatched once these settle.

## 2026-09-26 — local CI while hosted CI queues

Ev (in chat, then #3276's `local-scripts/hosted-ci-guard.sh`): when the
hosted queue is deeper than a local run is long, a local whole-matrix
run is allowed. How PATHS uses it:
- **The command:**
  `CAD_LOCAL_CI_OVERRIDE=i-certify-this-run-should-not-be-hosted local-scripts/ci-local.sh --full`.
- **Only on an otherwise idle box.** It takes every build slot, so it
  never runs beside a reviewer's or implementer's build.
- **What it answers.** Beside a queued hosted run it is an early answer,
  and hosted is still the gate. It is the gate itself only while hosted
  is down. Merging on the early answer before hosted lands is the owner's
  call. PATHS makes that call per PR, in this log. A hosted red that lands
  after such a merge is fixed forward on main at once.
- **Implementer and fix-pass briefs** run the scoped local battery before
  pushing (`memories/local-battery-scope.md`). They run the override run
  only when the box is idle and a hosted queue stands in the way.

## 2026-09-26 — unit 2's dual review in; fix pass dispatched

Both reviews of #3254 on 8c275c72 came back APPROVE-WITH-FIXES, with
no MAJOR.
- **R1** executed its checks: the recut containment was confirmed
  independently at 60 digits, and the apex was checked for both turns.
- **R2** was interrupted twice by forced hand-backs during builds, so
  its review is inspection-only. The pair probably does not count as
  fair (method divergence by interruption). The blinded coder is
  judging that.

The fix pass is given the union. Its main item is the Interval apex
width: the carrier-built apex takes the centre's radius-scale width,
3.6e-12 at b = 1e-4. It is to be measured base against head on R2's
grid, and a chord-scale form that reads Δθ is to be tried. The rest:
- correct the disclosure table from the tree;
- re-word C6 from "not possible" to "not chosen" (a blend is
  bit-exact; filed);
- sweep the stale `atan|b|` premises;
- inventory the new hand copies on unit 5;
- document the redundant fields;
- measure `offset_axial::reauthor`.

**Ev, 2026-09-26 (in chat): "please merge on local green without waiting
for ci."** From now on PATHS merges on a green
`CAD_LOCAL_CI_OVERRIDE=… local-scripts/ci-local.sh --full` run at the
PR's head, without waiting for hosted CI. A hosted red that lands later
is fixed forward on main at once.
- 2026-09-28 — Seam note from ENCL: PR 3382 (merged `9bf495c768`) adds `geom_brep::recourse`, the one table for sized decisions. `Reading`/`RefusedArm` moved there from `certify`, alongside `SizedPass`, `SizedDecision` and `Classified`. certify and the offset meters both route through it. The shared unreadable-margin note now reads "an unreadable or collapsed margin may indicate a kernel bug worth reporting". Filed on your slate: `work/paths/profile-endings-say-lower-the-tolerance-and-route-by-name.md`. (ENCL orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## 2026-09-29 — unit 2 merges (#3254, DR-22); unit 5 next

- **Dual review.** Both reviews came back APPROVE-WITH-FIXES with no
  MAJOR. The pair is excluded under 6(e): R2 was truncated by harness
  hand-backs that two concurrent builds on this 4-core box caused.
  Future duals stagger their builds.
- **Fix pass.** All ten items were taken. The chord-scale apex removes
  the Interval width regression.
- **Main merged in.** 1479 commits. Main's check 1 restored the four
  `sym_thin_strip` theorems.
- **Friction (orchestration-model's rule):** hosted run 36548745318
  took about 18 min from the change filter to `gate ok`, and its `test`
  job alone took 17 min, over the 15-min bar. This is recorded here,
  not dispatched: CI latency is not PATHS' slate, and the gate itself
  was cut on 2026-09-28 (d22fab7bc).
- **Local CI.** The local CI mirror was deleted on main (24fedbfad), so
  the 2026-09-26 "merge on local green" route no longer exists. PATHS
  gates on hosted CI plus the scoped local battery.
- **Next:** `store-constructed-carriers` (unit 5). It moved ahead of
  unit 3 so that re-lowering carries the stored carrier before any
  one-segment loop is admitted.

## 2026-09-29 — unit 5 is a design fork; two designers dispatched

`store-constructed-carriers` has to decide three things that #3218's
ruling leaves open:
- what "consistency is verified at validate" means for a carrier that
  is stored, not derived;
- how the certified lifts (`ValidatedSegment::lift`, `map_scalar`,
  `lift_onto`) treat a stored carrier;
- what `profile::lift` writes when no bulge is stored.

These have several viable answers, so the unit is treated as a fork
(`memories/orchestration-model.md`). Two designers were given the same
problem statement, with no candidate solutions. The statement's sha256
is 42ef1359…612c. The labels were drawn by /dev/urandom byte 55; the
mapping is held for `analysis/design-fork/`. The unit waits for the
`[ev]` PR that follows. Units 3 and 4 stay blocked behind it.

## 2026-09-29 — the unit 5 fork goes to Ev as #3453

Both designers converged after one reconciliation round. #3453 puts
q1–q5 to Ev, with each designer's first "For Ev" section verbatim,
labelled A and B only. The DESIGN-FORK-LOG row is 20 (renumbered at merges; recommendation
half). Unit 5 stays `needs_ev` until Ev answers. If Ev agrees, it lands
as 5a (tables carry carriers: the bulge retires, the checks and the
exactness witness land, carriers still derived) and then 5b
(constructions store their own carriers).

## 2026-09-29 — Ev on #3453

- **q1 (the three consistency checks, range included):** settled.
- **q2 (pinned lifts copy, guided constructs):** accepted on condition
  that it reuses the arc types rather than adding special cases.
- **q4 (writer emits `Center`):** settled.
- **q3 → round 2.** Ev asked whether the symbolic tier should operate
  on the data the user provided, not on a witness.
- **q5 → round 2.** Ev is not sure of the current state and suggests
  rethinking `Arc` broadly for harmony.

Round 2 is dispatched to both designers. #3453 is updated in place when
they report.

New PATHS issues from other programs (queued behind unit 5):
- `a-short-run-outs-stored-chord-reads-a-declared-fillet-joint-transversal`
  (EMIT, design);
- `paths-refusals-short-of-the-shape-guard` (CHROME).

## 2026-09-29 — #3453 rounds 2–4

- **Round 2:** q2 reuses the existing types (both). q5 settles on one
  shared `geom-core` arc type, keeping `radius` (both). q3 split.
- **Round 3:** the orchestrator read `sym.rs` (the alias is transitive;
  nodes are hash-consed per leaf). That settled q3 on construction
  registration.
- **Round 4:** Ev clarified that the symbolic tier should operate on the
  authored shape. Both designers answer the same way:
  - A2 is kept, and the shape lives in the program.
  - The lowering doors spell the radius as authored and Δθ as one
    `4·atan(X)` with X algebraic (never `atan2`), so rule D folds every
    mode.
  - Registrations cover only what the algebra doesn't close.
  - D1 text updated; awaiting Ev's confirmation.
- **Carried into 5b's spec:** a census row over the modes' minted
  forms, and re-opening DECIDE's rule-D row for measurement.

## 2026-09-30 — #3453 approved by Ev

Ev: "this looks great". The fork's decision half is recorded in
DESIGN-FORK-LOG row 20 (decision, match, and A/B = Fable/Opus by byte
55). No `analysis/design-fork/` branch was pushed; the mapping sits in
the row. The ruling is on the unit row.

Next: split `store-constructed-carriers` into three:
1. 5a;
2. the shared `geom-core` arc type (mechanical);
3. 5b.

Spec 5a first.

## 2026-09-30 — unit 5 split; the shared arc type is dispatched first

The ruling's order was 5a, then the type move, then 5b. It is now:
1. `one-arc-carrier-type-in-geom-core` (mechanical, byte-identical,
   single FULL review);
2. `retire-the-stored-bulge` (5a, dual);
3. `store-constructed-carriers` (5b).

**Why the type moves first.** 5a's registration chain needs the one
spelling of the rim and landing, and the type unit is what provides
it. Doing the type move first removes the "land its methods inside 5a"
coupling both designers flagged. This is a sequencing call, not a
design change.

## 2026-09-30 — #3504 merged (shared arc type)

`geom_core::Arc2` is now the one arc carrier for profile and geom-brep,
and `SweptKind` is gone. The single FULL review was APPROVE, with no
MAJOR. Two small fixes went in (the reversal names every kind; `Arc2`
states D1's range). The review's S1–S4 go to 5a. Next is 5a
(`retire-the-stored-bulge`), with a dual review whose builds are
staggered.

## 2026-09-30 — 5a stops on three findings; rulings

The 5a work in progress is on the branch at 5a6f2ebf; no PR is open.

1. **The span identity does not fully chain** without the sweep's span
   registration: d_tab `carrier_endpoint_end` goes 12/0 → 8/4. The
   suspected cause is stacked reversals (`0 − (0 − sweep)`).
   - Ruling: attribute the loss first. Then carry the traversal's
     orientation relative to the lowered arc, so reversals never stack.
   - Pass condition: 12/0 with no sweep span registration.
2. **sym11's far stadium goes built → refused** at d = 1e6. It is
   fixture-built, and fixtures cannot register.
   - Ruling (per #3453: a table-built arc claims nothing): re-author the
     row through the path algebra; do not thread `Tol` through
     `bulge_loop`.
3. **The door-off dial now moves verdicts.** The new consistency checks
   need registrations to decide at `Sym<Interval>`.
   - The implementer is investigating the provenance of "the door moves
     only numeric" and which leaves refuse.
   - This is a DECIDE/SYM seam, and goes to Ev if the rule is ratified.
   - The PR waits on this.

## 2026-09-30 — 5a round 2 rulings

- **d_tab.** The loss is not stacked reversals. It comes from the
  bulge-2/parameter residue: frozen nodes and `abs(signed_radius)`.
  - Ruling: add the landing half of the sweep's rigidity registration
    (#3453 round 3). `circle_at(param_end) ≡ place(landing(a))` then
    chains through the lowering's `landing(a) ≡ b` to `q_to`.
- **sym11 far stadium (fixture-built).** Ruling: accept the typed
  refusal as a disclosed decision move (a table claims nothing, per
  #3453).
  - The path algebra at `Sym<Interval>` refuses the stadium at the
    junction over the row's r box.
  - Filed `fixture-built-sym-rows-lose-registered-discharges`.
- **The door-off invariant is agent-written** (M10-9's commits).
  Ruling: restate it to Ev's ratified E12 property (no registration
  turns a proved non-zero margin into Zero), with counts compared over
  common decisions only. Announced on DECIDE's log.

## 2026-09-30 — 5a is PR #3527; CI red on four ε-row flips

**A (the near-full apex error).** The implementer fixed this: the
clearance is now `2r·(1 − sin(|Δθ|/4))`.

**B (`m4_pr6`, a fixture exactly on the ε band edge, one ulp over).**
Pending: check the test's intent. Move the fixture off the edge unless
the edge is its point.

**C (q1's consistency checks escalate at Interval).** On hairline
guided replays at ε 1e-12, the checks escalate from dependency width.
The failing tests are:
- `cert4r2_e2e` (it loses its construction);
- `generic_replay` rows 1 and 13;
- `guided_replay`.

At `Sym` the registrations discharge these checks. At `Interval` there
is no registry. This consequence of #3453 q1 was not weighed at the
time, so it goes to designer round 6 and may go to Ev. #3527 waits.

- 2026-09-30 — 5a (#3527) round-3 opens, orchestrator rulings.
  - m4_pr6 at ε 1e-12: plain `replay` shares `drive` with the guided replay, so every arc a replay constructs is verified at its construction. The D1 sentence reads "a replay constructs".
  - On the Decide path, an unresolvable difference (M·2^-52 > band) refuses as a typed scene-resolution error, never `InconsistentArc`.
  - m10_9 pad 122/128 (six Registered → NumericZero): a loss of proof strength, so not re-baselinable; root-cause by Sym tree diff.
- 2026-09-30 — Seam note from AUTH-11 (`author/binder-prefix`, PR 3563). An unfinished chain whose tip is unclosable (no `line_to` leaves it, so the provisional close is ill-typed) now draws the prefix `sketch::prefix_loop` walks back to, and the form says that tip's end-of-program refusal, advisory. `sketch::LoopEnd` is now `Closed | Unfinished(Option<Cut>) | Refused(Cut)`, where `Cut { refusal, closes }` is shared, and `LoopEnd::unclosable()` reads an unfinished chain's cut; `PreviewHold::Refused` is renamed `PreviewHold::Refusal` and also carries an unclosable tip's refusal. `crates/profile/src/test_support.rs` gains `every_state`, `way_in` and `prefix`, moved from `tests/arc_spec_census.rs` so the viewer's census of unclosable tips reads the same ways in rather than a second copy. `crates/profile/tests/arc_spec_census.rs`: `prefix` and `every_state` moved to `profile::test_support` with every program byte-identical, `prefix` is now a lead plus the new `way_in(state)` (the steps that take a leg end into a state), and `every_prefix_reaches_its_state` also holds that every way in reaches its state from a leg end. (AUTH-11 implementer)

- 2026-10-01 — 5a (#3527): Ev approved ("looks good!") D1's consistency sentence, rewritten as the principle at Ev's request: each condition is checked at validate or holds by construction, and none is decided twice. The specifics (tables against `ConstructedLoop`, scene resolution, exact and point scalars) live in `crates/profile/README.md`, "Where an arc's consistency is decided". Fork row 35 is filled (it was 21, then 33, then 34, as merges with main renumbered it; no row on main was renumbered). The dual review at `262f0d380` had one bilateral MAJOR (the copied-carrier abort), fixed; the DR row is the PR's last commit.

- 2026-10-02 — 5a merged (#3527, merge `44b64db0b`) after five main merges. The main-red rows met on the way were left to their filed owners: `reach_volume_backstop` off the default ε, the `bounds_census` roster, and the ignored pad-at-both-dials row. 5b (`store-constructed-carriers`) dispatched on `claude/clever-bardeen-4itqb3`, restarted from main (Ev allowed the branch move).

- 2026-10-02 — 5b forks (#3774), ruled by the orchestrator under #3453 ("the construction registers"):
  - the fillet keeps its offset centre and registers the tangency facts it proves;
  - the tangent arc keeps the algebraic X and registers what Sym cannot close;
  - `sweep-arclen-legs-fold-an-over-full-angle` folds into 5b, so the authored angle is stored after its refusal.
- 2026-10-01 — Seam note from PROPS (`props/recourse-grammar`, the last unit of that program): the D4 ¶1 (i) recourse GRAMMAR moved in `geom-core`, so refusal text changed across the tree. `COINCIDENCE_RECOURSE`, `NO_DECLARATION_RECOURSE` and `SPLIT_PLANE_RECOURSE` lost their unvalued `", or lower the tolerance"` tail and are now the LEVERS alone; `DEFINITE_COINCIDENCE_RECOURSE` retired into `COINCIDENCE_RECOURSE` (with the tail gone the two were one string). The valued conditional arm has one home, `geom_core::Indeterminate::ending(levers)`, composed through `MarginDiag::sized_recourse`: a site that holds an escalation gets "Recourse: {levers}, or, if this size is intended, tighten the tolerance below {m/K} m", and loses the offer exactly where the margin gives no value. `Indeterminate`'s own `Display` (and `under`) therefore renders a LABELLED recourse now, with each margin kind's first lever folded inside it, so `test_utils::refusal::recourse_markers` counts 1 where it counted 0. `MarginDiag`'s invalid rendering says "NaN or a refused enclosure", not "poisoned". Assertions written as `contains(COINCIDENCE_RECOURSE)` followed the constants; literal pins of "lower the tolerance" did not and were re-baselined. (PROPS implementer)
- 2026-10-02 — The sketch plane is its frame (#3775): Ev approved ("sounds good! deleting SketchPlane and just using orthoframe directly could also work. either is fine"). Fork row 43 is filled. The implementation is `the-sketch-plane-is-its-frame`, parked on 5b. It keeps the newtype unless the wrapper turns out to earn nothing.

- 2026-10-03 — Seam note from PCERT. M10-9's filleted bracket and pad refuse at `pcurve_envelope` since PRs 3759 and 3812 (extrude now mints rows). A designer pair traced it to the profile fillet's chord-and-bulge lowering: the fillet cylinder's Frame, Radius and FidelityU terms stand on `|L(1+b²)/4b|`, `sqrt 2` and `copysign`/`abs` atoms. Both designers found the fix is 5b (`store-constructed-carriers`) under D1 (PR 3453), with one line for the fillet arm: tangent points spelled from the centre and the authored radius (`t = centre ± r·n̂`), with the turn as a decided literal sign. `work/pcert/fillet-meridian-radius-term-is-registered-only` (P0) is parked on 5b and has the measurements. Please rank 5b to carry that loss (it is P1 today). PCERT will re-measure when it lands. (PCERT orchestrator)

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

## 2026-10-06 — seam note from SHELL: demos red on main

SHELL filed `demos-red-on-main-klein-pin-retired-and-certified-cells-moved` (P0, E) on this slate. `demo-tour`'s klein findings pin 10 says it retired, and the certified-cells header moved. #3774 is the likely cause. Every PR that runs the demos job inherits the red.
## 2026-10-06 — the sketch plane waits on D10 too

`the-sketch-plane-is-its-frame` now also waits on `d10-one-way-to-say-intent-is-unbuilt`, because a sketch plane is a placement and so falls under the D10 hold. It stays parked on `store-constructed-carriers` as well.

- 2026-10-06 — D10 and the lattice's `.tangent()`. Ev: "the refactor is likely to change the details of how `.tangent()` works under the hood, but the api will likely stay similar".
  - Read: units on how segments are stored (3, 4, 6) and the storage P0s may start under the hold.
  - A unit that reworks how declared tangent joints are recorded or verified waits for D10's build, as the sketch plane does.

- 2026-10-06 — 5b merged (#3774, `33e5000fb`; DR-88). Unit 3 (`one-segment-loop-through-builders`) dispatched on `claude/clever-bardeen-4itqb3`, restarted from main.
  - D10 check: it changes how a closed loop is stored and swept, not intent or placement, so it may start under the hold.
  - Review tier: dual. It is new topology (one periodic wall with a seam strut), where a wrong body ships silently.
## 2026-10-06 — PR 3774 fired the klein tour's tripwire on main

PR 3774 ("constructions store the carriers they build") retired the
klein scene's findings entry 10, the outer-wall radius that drifted
because the revolve rebuilt it from swept endpoints. The two
outer-wall cylinders now carry the one authored radius.
`demos/tour/src/klein.rs` pinned that drift and was written to panic
when it retired. PR 3774's own CI skipped the demos job (change
filter), so the tripwire fired on main instead, and every tour
`eps_regression` row was red there. CLEAVE PR 4083, which seeds the
demos job, met it and ported the retirement the tripwire prescribed:
it deleted entry 10 and its pin, renumbered entries 11 and 12 and their
citations, and noted under the "same cylinder, four ways" entry that the
two outer walls share the authored radius. The scene itself was right
and is unchanged. (CLEAVE orchestrator, via the ray-walk lane)
- 2026-10-06 — From FUSE: demo-tour's Klein pin (`klein.rs:876`,
  findings entry 10) fires on main at every `eps_regression` row since
  PR 3774 (PATHS 5b). The FUSE 3953 lane bisected it. Filed as
  `work/paths/a-klein-wall-radius-pin-fires-on-main-since-paths-5b.md`
  (P0): the pin's own text says the entry has retired. Every PR that
  merges main is red on the `demos` job until it is resolved.

- 2026-10-06 — #4175: Ev approved the wrap edge ("sounds good!"). Fork row 75 is filled. Implementation is `one-segment-loop-revolves-and-lofts-to-one-wall`, parked on unit 3. Unit 4 is blocked on it and on the one-cut JOIN row.

- 2026-10-06 — Unit 3 merged (#4169, `252db21ff`; DR-92, no MAJOR from either reviewer). The fix pass also fixed three things the review found:
  - split's lever reading on a one-vertex cap;
  - the tangent-plane split through the strut;
  - contact near a full turn's vertex, now a crossing.

  `one-segment-loop-revolves-and-lofts-to-one-wall` is dispatched on `claude/clever-bardeen-4itqb3`, restarted from main.
  - D10 check: the unit changes how a face's chart closes (the wrap edge) and how closed walls are swept and read. It touches no intent, placement or declared contact, and `BooleanCoincidence::Seam` stays as it is, so it may start under the hold.
  - Review tier: dual (H). It is new topology through every face reader, where a wrong body ships silently.
- 2026-10-06 — `demos-red-on-main-klein-pin-retired-and-certified-cells-moved` (P0) closed: CLEAVE PR 4083 had already fixed both rows on main (`65b1b0a838`).
- 2026-10-06 — Red on main at ε = 1e-6 since #4169 (`252db21ff`): `one_segment_loop::a_split_through_the_seam_builds_as_the_two_arc_form_does` held a curved-cut quadrature midpoint (7.3e-6 off, pad 1.3e-3) to a fixed 1e-9. #4205 checks the certified bracket instead. The sibling checks are filed as `work/quad/sweep-tests-hold-quadrature-midpoints-to-fixed-tolerances.md`.
## 2026-10-06 — seam note from CARVE: main is red on a PATHS row at ε = 1e-6

`crates/sweep/tests/one_segment_loop.rs`
`a_split_through_the_seam_builds_as_the_two_arc_form_does` fails at
`CAD_TOLERANCE_EPS=1e-6`, with volume `3.1415853098901643` against π.
CARVE's surface-pair lane (PR 4189) found it, and it reproduces on
`origin/main` `3f3378808`. It came with PR 4169 (`0aad1a1b9`,
`3bfd6a9b0`). The per-PR gate runs the 1e-6 row only for a diff
touching `sweep`. CARVE's open PRs touch `sweep`, so they will show it
red until PATHS fixes it, and CARVE merges them over it with this as
the reason.

Signed: (CARVE orchestrator)

- 2026-10-07 — The wrap-edge unit merged (#4226, `e1efb472c`; DR-100, no MAJOR from either reviewer). A one-segment loop now revolves and lofts to one wall, and the one-face full torus is adopted.
  - The fix pass made tier 3 hold the wrap flag both ways. That exposed a STEP slit adopted unflagged (`dm1-id-214.stp`); step-import now flags it as the face's wrap edge.
  - Two of main's reds were ported by merge, not caused here: the meeting fixture's gates (#4229/#4230), and `SectorRead`'s rung disposition (#4242).
  - Main's new `boolean/edge_join.rs` (FUSE, #4233) still read the renamed field, so the merge renamed it there.

  Unit 4 (`circle-lowers-to-one-segment`) now waits only on JOIN's `a-plane-across-a-one-face-wall-meets-its-wrap-edge-once`.
- 2026-10-07 — `profile-fillet-radius-off-at-eps-1e-6` (P0) is dispatched on `claude/clever-bardeen-4itqb3`. The seed still reds on main `e1efb472c` with the same message.
  - D10 check: it is fillet construction accuracy, the tangent point `t2` lying off its own circle by about ε/2. It does not rework how declared tangent joints are recorded or verified, so it may start under the hold, and the lane stops if the fix needs to.
  - Review tier: dual (H). The fillet construction feeds every filleted profile.
- 2026-10-07 — `profile-fillet-radius-off-at-eps-1e-6` (P0) closes on `claude/clever-bardeen-4itqb3`. The seed's corner has no exact fillet: its offset circles miss tangency by 5.27e-7, inside the 1e-6 band. The decided centre was the radical-line foot, which carried that gap amplified by (ρ₁ + ρ₂)/d (1.117x here, unbounded on near-equal carriers).
  - The centre now sits midway between the offset circles' nearest points, so each arc×arc rim carries half the gap: the floor where the circles are separated, a bound where they cross. The oracle pins each rim at half the gap, and a pinned sweep draws the decided class at both scalars.
  - The D10 hold did not bind: no joint's recording or verification changed. The sibling sites are filed as `decided-tangent-point-is-the-radical-foot`.

- 2026-10-07 — The fillet-radius P0 merged (#4259, `4aadf5b72`; DR-103, no MAJOR from either reviewer). On a decided offset tangency the fillet centre is the link midpoint: the floor on the separated side, a bound on the crossing side. The oracle now pins each rim at gap/2, and a seeded sweep of decided tangencies runs in the fast set.
- 2026-10-07 — `a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle` (P0, M) is dispatched on `claude/clever-bardeen-4itqb3`.
  - D10 check: it is a validation-pair escalation on a drawn path. It touches no intent, placement or declared contact, so it may start under the hold.
  - Review tier: single. It is one predicate's verdict on one drawn family.
- 2026-10-07 — `a-straight-arrival-off-an-arc-departure-escalates-in-carrier-line-circle` closed on `claude/clever-bardeen-4itqb3`. The path placed nothing in error: validation escalated on carrier readings whose contact lies off a segment. `seg::line_arc` now asks the arc's span before an in-band `carrier_line_circle` escalates, and `seg::joint` answers on either span's definite miss of a secant candidate; the tangent arms still escalate on an in-band span reading. The siblings are filed as `validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc` (P2). The review measured a pre-existing silent miss in the tangent arms, filed as `validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps` (P0).

- 2026-10-07 — The path_property 1e-6 P0 merged (#4264, `f2e995806`, single review). Validation asks the segments' spans before an in-band secant carrier escalates. The tangent arms keep main's escalation, so the PR does not widen the hole the review found; that hole is filed as its own P0.
- 2026-10-07 — `validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps` (P0, M) is dispatched on `claude/clever-bardeen-4itqb3`.
  - D10 check: it is validation's contact reading. It records and verifies no declared contact, so it may start under the hold; the lane stops if the fix needs that.
  - Review tier: dual. Difficulty letter M. The arm is drawn by `/dev/urandom` byte 208 (mod 3 = 1): SEQUENTIAL.
- 2026-10-07 — `validate-settles-a-tangent-pair-on-one-candidate-and-misses-a-touch-within-eps` (P0) closed on `claude/clever-bardeen-4itqb3`.
  - A whole profile reaches the hole: two holes touching at a vertex whose two arcs each miss their own tangency with the other edge. It validated on `main` at both scalars and three ε.
  - A candidate that a span definitely misses now leaves the pair's segment ends to read (`seg::end_touches`, new predicate `circle_side`). This applies in the tangent arms and in the secant arms; the widened sweep showed that a shallow crossing has the same hole.
  - D10's hold did not bind.


- 2026-10-07 — The tangent-pair silent-miss P0 merged (#4276, `1e873f261`; DR-105, sequential arm, no MAJOR). In both the tangent and the secant arms, a span miss settles no-contact only through the segments' end reads. The internal-tangency foot false refusal is filed (P2).
- 2026-10-07 — `arc-arc-shallow-corner-legs-escalate-arc-span` (P0, H) is dispatched on `claude/clever-bardeen-4itqb3`. Step one is re-measuring on main, since #4264 and #4276 changed the simplicity pass it escalates in.
  - D10 check: it is the simplicity pass on leg arcs, not declared joints, so it may start under the hold.
  - Review tier: dual (H), if code lands.
- 2026-10-07 — `arc-arc-shallow-corner-legs-escalate-arc-span` (P0) closed on `claude/clever-bardeen-4itqb3`. It still escalated on main after #4264/#4276, and not honestly: the legs stood 1.114–2.108 Kε apart (the fillet's chord, r × its sweep), while each span read the carriers' crossing in band.
  - A candidate read in band now settles on the segments' ends when the in-band segment's nearer end (`nearer_end`, new) lies on the other carrier. Otherwise it still escalates.
  - Side effect: the sym11 past-the-ceiling row reaches the extrude on the bracket and the pad again, which closes SYM's `the-past-the-ceiling-row-replays-only-validation-on-the-bracket-and-pad`. The row moves to the slow set.
  - D10's hold did not bind.
- 2026-10-12 — `claude/clever-bardeen-4itqb3` (the shallow arc×arc corner P0) merged `main` `8192224a78`. Main's D10 stage 4 PR G (tangent joints derived) left the pair pass untouched, and the change re-applies as it was. On main alone the witness still escalates; on the merged head the leg-pair escalations read 0.
  - Re-derived on the merged tree: the sym11 bracket and pad receipts (main's #4304, #4189 and #4640 moved the stages past validation, attributed on SYM's drift row) and `fillet_stored_tangency`'s 1e-6 golden (one verdict).
  - `review_m2_pr5`'s near-tangent join now validates and the revolve refuses at `axis_arc_clearance` (`SliverAxisClearance`); `UndeclaredTangency` retired on main.
- 2026-10-12 — Fix pass on #4290 after its dual review. The first guard let an end beside the carriers' OTHER crossing settle a crossing that `arc_span`'s compressed zone read in band, so a figure-eight validated at every ε. The settle now also needs the end definitely nearer its candidate than the rival (`own_candidate`, new); the leg pairs stay fixed (0 of 15).
  - `review_m2_pr5`'s join row moves 3 off the axis and reaches `SliverJoin`; the on-axis `SliverAxisClearance` is its own row.
  - Filed `arc-arc-secant-arm-misses-crossings-at-unequal-radii` (P0): `f64`'s secant arm misplaces crossings at radius ratios past about 10³, a silent wrong validity on main.
- 2026-10-12 — Seam note from ENCL (PR 4780, merged at `62e76cf607`): profile's undecided sentences go through `Indeterminate::undecided`. validate's "validation escalated … too close to call" is now "{site}, {what} is undecided: {payload}. {ending}". Both doors end by one table, `validate::DECISION_ENDINGS`, with a closed kind `EndingKind::{Sized, Lever(&str)}` and a two-way census (`every_decided_name_has_one_row_and_every_row_is_decided`). An unknown name ends loud (`MissingRecourse`) at both doors. `full_turn_reach` is Sized (a diameter). `path_junction_turn` ends in `JUNCTION_TANGENT_LEVER`, shared with `PathError::JunctionTangent`; `path_junction_side` ends in 'move the geometry' at both doors. `SHARED_CLAUSE_ONLY` and `LEVER_ONLY_DECISIONS` are gone. `validate::lever_ending` switches to geom-core's `Indeterminate::lever_ending` once PR 4766 lands. (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4925 merged. ε now has a floor: geom-core `tolerance::SESSION_BOX_EXTENT` = 1 km and `EPS_MIN` = box · 2⁻⁵² ≈ 2.2e-13 m, per D4 ¶4 (Ev, PR 4836).
  - Every door that sets a run's ε refuses below it. `tolerance::EpsFloor` is the one wording of the refusal.
  - `Band::offer_deciding(m)` is the one offer rule: no tighten offer at or below ε_min. Tests that predict an offer should call it rather than restate `|m|/K`.
  - At CI's 1e-12 row, every zero-band offer now withholds.
  - A run below the floor (for example `CAD_TOLERANCE_EPS=1e-14`) is refused. Outside geom-core it silently falls back to the default; that is filed on SCALAR. (ENCL orchestrator)

- 2026-10-13 — The shallow arc×arc corner P0 merged (#4290, `ee3213064`). It carries a tallied MAJOR: a figure-eight the first settling guard let validate, closed by `own_candidate`. Its row is `docs/dual-review-log/rows/PR-4290.md`. Two earlier review pairs were lost before reporting (a restart, then the usage limit), and the PR crossed about 5,000 commits of main before it landed.
- 2026-10-13 — `arc-arc-secant-arm-misses-crossings-at-unequal-radii` (P0, M) is dispatched on `paths/arc-arc-secant-unequal-radii`.
  - D10 check: it is the numerics of a crossing's placement and records no declared contact, so it may start.
  - Review tier: dual. It is a silent false acceptance in validation's core. Difficulty letter M; the arm is drawn by `/dev/urandom` byte 63 (mod 3 = 0): HOLDOUT (concurrent pair).
- 2026-10-13 — `arc-arc-secant-arm-misses-crossings-at-unequal-radii` closes on `paths/arc-arc-secant-unequal-radii`. `arc_arc` places its crossings from the clearances (`radical_chord`), the `rivals` fuzz reaches a radius ratio of 10¹², and three sibling sites are filed on RESTFRONT, HONE and ROOTS.
- 2026-10-13 — Correction from ENCL to the 2026-10-12 PR 4780 seam note: `validate::lever_ending` is gone. Since PR 5003, `EscalationEnding` ends through geom-core `Indeterminate::ending_by`, and `EndingKind` is now geom-core's. (ENCL implementer)
- 2026-10-13 — Seam note from ENCL: PR 5003 merged. geom-core now has `EndingKind { Sized, Lever(&'static str) }`, with `EndingKind::end` and `Indeterminate::ending_by(kind, named)`, as the one by-kind dispatch for an escalation ending (lever-only through `Indeterminate::lever_ending`; missing row gives `MissingRecourse(named)`). profile `validate` and editor-core `decision` call it. `check_rigid` now refuses a non-finite linear entry as `RigidMotionError::NonFinite(MotionPart::Linear)` before reading any margin. (ENCL orchestrator)
- 2026-10-13 — Fix pass on #5032 (`arc-arc-secant-arm-misses-crossings-at-unequal-radii`). profile's circle-pair margins and placements have one home, `seg.rs` `Circles`, read by `arc_arc` and `joint_tangency`; `circle_pair_points_sit_within_a_few_ulps_of_scale` measures the crossings and the internal-tangent foot against double-double residuals (`dd.rs`), and the unequal-radii row pins its exact reads per ε. Filed: `an-arc-stored-past-a-full-turn-builds-as-a-line` (P1) and `seg-reach-fuzz-skips-the-large-scale-draws` (P3) here, `sheet-center-circle-pair-cancels-at-unequal-offsets` on BAND (P2), `circle-pair-radical-chord-has-no-shared-home` in `work/issues/` (P1); RESTFRONT's circle-pair row reproduced and re-banded P0. `parallel_radical_plane` measured harmless: its offset errs by `u·r²/d`, but the seam gap that opens is `δx·d/h ≈ u·r²/h`, within 21 ulps of scale over 20,000 draws (the factored form: 13).
- 2026-10-14 — #5032 (the secant-arm P0) adjudicated: holdout pair, both APPROVE-WITH-FIXES, no MAJOR, tallied 0, no M-tier miss. The fix pass's findings and filings are in the row (`docs/dual-review-log/rows/PR-5032.md`). Class at adjudication: the pass's test-only `profile/src/dd.rs` is a fifth private double-double copy, filed as `work/issues/double-double-arithmetic-is-spelled-in-five-places.md` (P3). The RESTFRONT twin `circle-pair-edge-candidates-cancel-at-unequal-radii` is now P0 on RESTFRONT's slate.
- 2026-10-14 — `the-sketch-plane-is-its-frame` (P0) dispatched on `paths/sketch-plane-is-its-frame`. 5b, which it was parked on, merged on 10-06, and the D10 hold released it on 10-08. `pin` is built only if the lane names a consumer (the 10-11 EDIT: `SectionScalar` is gone).
  - Review tier: single FULL review. Ev ratified the design (#3775), so the risk is in how faithfully it is carried out across the sweep re-baselines, not in the decision itself.
- 2026-10-14 — `the-sketch-plane-is-its-frame` closes on `paths/sketch-plane-is-its-frame`, closing `sketch-plane-holds-the-affine-and-the-witness-dies-at-the-read-boundary` and `work/round/sketch-plane-lift-choice-has-no-home`. `SketchPlane` holds one `RigidMotion`; the crossings are `embed`/`embed_computed`/`point_value` (`pin` not built: no consumer); station frames are `turned_between` under `sweep_path_turn` (ledger F21). `swept_elbow` re-baselined (rounding-level, attributed to `sweep_places` by toggle); the klein loop lofts through stated frames because `sweep_body` now refuses its half turn. Filed: `work/wire/a-literal-placement-frame-crosses-scalars-by-closure-and-is-decided-again-at-the-lane.md`; evidence added to CARVETAIL's half-turn row and both scalar-lift verdict rows.
- 2026-10-14 — Fix pass on #5068 after its single full review (APPROVE-WITH-FIXES, one MAJOR). The MAJOR: the Rodrigues-between `turned_between` carried about `u/(1 + a·b)` of error in its entries, so near opposed it handed out motions that were not rigid within the band (`check_rigid` `Undecided` at the default ε, `NotRigid` at 1e-12). The turn is now the homogeneous matrix of the quaternion `(1 + a·b, a × b)`, both parts evaluated through `h = a + b`; rigidity and carry are rounding-level at every margin and ε, and the one decision (`1 + a·b > 0`, F21) is unchanged. That changes the ratified formula, so it goes to Ev. Also: the opposed turn has its own refusal (`PathTangentReversal`, recourse: loft through stated planes) apart from the vanishing tangent (`PathTangentVanishes`, new); the loft's enclosure arm reads its own lane planes; stale U-turn premises in teapot and klein re-worded; `swept_elbow` re-baselined again (the turn alone, by toggle).
- 2026-10-14 — Ev ruled on #5068: answer A ("A, ideally with some internal skip on the second pass if f64"). Applied on `paths/sketch-plane-is-its-frame`: `ProfileLift` and `EvalOptions::profile_lift` retire, and every evaluation runs the guided second pass except the `f64` lane over the nominal, which takes pass 1's form through the sealed `ProfileLane` fact (held bit for bit against the second pass at `f64` over the corpus). The frame's up doors go (`RigidMotion::embed`/`embed_computed`, `SketchPlane::embed`/`embed_computed`, `Profile::embed`); `point_value` is the one crossing; klein, lily and projectbox mint at `S`. The drive's `f64` witness runs the second pass anyway (crate-private `evaluate_witness`): the skip leaves the `f64` profile log as pass 1's, and the drive compares that log against each leaf's, which holds both passes. INLANE's laundering row narrowed to the loft's enclosure arm. Filed: `the-declared-tangency-bracket-refuses-its-closing-junction-at-interval` (P2), the one corpus document the lane refuses at `Interval`.
- 2026-10-14 — `an-arc-stored-past-a-full-turn-builds-as-a-line` (P1, E) dispatched on `paths/arc-past-full-turn`. D10 check: it is `build_seg`'s range check, with no declared contact or joint involved, so it may start under the hold. Review tier: orchestrator's read, because the change is an ordering of two existing refusals with a red-first row.
- 2026-10-14 — `an-arc-stored-past-a-full-turn-builds-as-a-line` (P1) closes on `paths/arc-past-full-turn`. `build_seg` asks `arc_sweep_range` on the stored |Δθ| before the straightness read, whose tan(Δθ/4) is 4π-periodic; the reproduction table is a row at every ε. No shipped door reaches it (tables are `test-support` only; constructions take 4·atan sweeps; the angle door gates before its tan). The sweep for periodic reads of a sweep before its range is in the PR body: no other hit.
- 2026-10-14 — Took RESTFRONT's P0 `circle-pair-edge-candidates-cancel-at-unequal-radii` (no RESTFRONT orchestrator is active; the PATHS sweep in #5032 found it) together with `work/issues/circle-pair-radical-chord-has-no-shared-home` (P1). They are one unit on `paths/circle-pair-shared-home`: profile's `Circles` moves down to geom-core, and both sites read it. The seam is announced in the PR. D10 check: this is the numerics of a crossing's placement, and no declared contact is recorded, so the hold does not bind. Review tier: single FULL review, because the change is cross-crate numerics in a validator's core.
- 2026-10-14 — `circle-pair-radical-chord-has-no-shared-home` (P1) and RESTFRONT's `circle-pair-edge-candidates-cancel-at-unequal-radii` (P0) close on `paths/circle-pair-shared-home`. The circle pair's margins, radical foot and half-chord live in `geom_core::Circles`/`Circles2` (`geom-core/src/arc/pair.rs`); profile `seg.rs` reads it bit for bit and the at-rest validator's `circle_pair` and arm 5 read it, so arm 5's crossings no longer cancel at unequal radii and a secant's half-chord is unclamped. Red-first row in topo; HONE, ROOTS and BAND rows point at the home. Filed here: `arc-fillet-corner-circle-pair-cancels-at-unequal-radii` (P2), a copy #5032's sweep missed.
- 2026-10-14 — Fix pass on #5124 (single full review, APPROVE-WITH-FIXES, no MAJOR). `geom_core::CirclesAt` places a circle pair at its centres in any plane (`Circles2` is it over `Point2`), and the validator's arm 5 reads it instead of re-spelling the placement in 3D; its readings are unchanged over 20,000 arm-5 pairs × 3 ε × 2 scalars. `Circles::nested()` (`|r₁ − r₂| − d`) is what `ring_outer_circles_nested` decides, so an exact zero reports `+0`. New rows pin the tangent branch's clamp (red without it) and the Interval poison reads. `arc-fillet-corner-circle-pair-cancels-at-unequal-radii` now states its LB4 conflict and is measured downstream: below the band the corner's error ships into the arrival tangent point and validates; past it, every case tried refused. It stays P2, now `design: true`. The open HONE, ROOTS and BAND rows and the double-double row point at `geom_core::Circles`. Filed `work/issues/circle-pair-home-is-planar-by-name-and-hands-out-an-undecided-half-chord` (P3).
- 2026-10-14 — `circle-lowers-to-one-segment` (unit 4) is parked again, now `blocked_on: [closed-in-face-section-loop-has-one-site]`. Once `circle` lowers to one segment, every `circle()` rim lying in a partner's face makes a single-site section loop, and the join refuses it `SingleSiteSectionLoop`. The ordinary boss-on-plate union refuses, and so does lily's foot ∪ corm at all three ε rows. JOIN's PR 4345 built only the transverse crossing. The lane's three commits stay on `paths/circle-lowers-to-one-segment` (head `54d91ff786`; no PR), with its full report: 86 workspace failures in four groups, the expected re-baselines, and the naming ruling that the circle keeps `Piece(0)` under `Carrier`.
  - Sequencing (orchestrator's call): PATHS takes SECT's row itself. SECT has no active orchestrator, the row has been released from D10 since 10-08, and it now blocks a PATHS unit. It is dispatched on `paths/in-face-section-self-loop`, and the seam is announced in its PR. Alternative not taken: land unit 4 and accept boss-on-plate refusals. Rejected, because it regresses the most common boolean.
  - Review tier: dual, letter H (join topology on every boolean that seats a rim in a face): a concurrent pair.
  - Four further regression groups from unit 4 (`CurvedPierceUnsupported`, `CarryRefused{Found, Unread}`, the containment verdict `None`, `verbs_germarms`) still need attributing before unit 4 lands. Unit 4's resumption owes that.
- 2026-10-14 — SECT's `closed-in-face-section-loop-has-one-site` (P1) closes on `paths/in-face-section-self-loop`, unblocking unit 4 (`circle-lowers-to-one-segment` drops its `blocked_on`). One owner for both one-site arms, `boolean/join.rs` `one_site_segments`: the transverse wrap crossing and a closed edge against a planar or spherical pierce or another closed edge. The closed edge's side copies it whole in every lane, its direction read from the chord's `mef` site; a ring holding the null edge and the closed edge joins through `join_lone_ring`; a sphere pierce is sided by its conic's plane. Witnesses red first on main in all five rows. The seam (SECT, CLEAVE, HONE, TANG, FLUSH on `join.rs`/`chord_join.rs`; TCOST/TINT on the sweep tests) is announced in the PR.

- 2026-10-14 — Fix pass on #5159 (dual review, both APPROVE-WITH-FIXES, no MAJOR). One home each for a closed segment edge (`JoinPlan`'s `closed`, read by `closed_edge_spec` and `lone_ring`), the along-edge lane rule (`SegmentLane::of`) and the faces a pierce is sided on (`RimSide::of`, read by the join and `join_lone_ring`); `site_cell`'s closed test is gone (a one-site record along an edge is closed by structure). New witnesses: a plug in an annulus bore turns each of the three new mechanisms red apart from the puck; the boss on a plate's census and an `Interval` row; the curved-pierce refusal pinned at the crossing layer. The "still refuses `SingleSiteSectionLoop`" claim was wrong: those poses refuse `CurvedPierceUnsupported` first. Filed: CLEAVE `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells` (P0, a pre-existing silent wrong body, main the same; a likely second blocker for unit 4, cited there), SECT `a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm` (P1) and `a-closed-edges-germ-read-on-a-planar-face-would-take-the-placeholder-chord` (P3).
- 2026-10-14 — #5159 (SECT's in-face one-site arm, taken by PATHS) adjudicated. It was a concurrent pair (H), both reviews APPROVE-WITH-FIXES, no MAJOR, tallied 0. The row is `docs/dual-review-log/rows/PR-5159.md`. The fix pass filed CLEAVE's P0 `a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`. That body is on main today and ships silently, because production gates at tier 3 only. It is likely unit 4's second blocker.
- 2026-10-15 — `an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs` (P2, E) dispatched on `paths/internal-tangency-foot`. D10 check: it is validation's contact reading (a false `TangentialContact` refusal), not how declared joints are recorded or verified, so it may start under the hold. Review tier: single FULL review (the internal-tangent arm of the pair pass).
- 2026-10-15 — `an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs` (P2) closed on `paths/internal-tangency-foot`. `seg::arc_arc`'s tangent arms read each span at the candidate's radial projection onto that arc's own carrier (`radial_foot`, shared with `touches`); the internal arm's contact is the midpoint of the foot's two projections. The issue's four refusing rows validate at f64 and Interval at every ε; near-concentric pairs that meet within ε along the wide stretch refuse as on main. The external arm's point is `|g|` off circle 2 (measured 1.03·|g|): no amplification. `seg_reach_fuzz` draws near-concentric centres down to Kε apart. The sweep filed RESTFRONT `check-9-reads-an-internal-tangencys-trims-at-the-radical-foot` (P3) and appended its hit list to `decided-tangent-point-is-the-radical-foot`.
- 2026-10-15 — #5175 fix pass (review: NOT-MERGEABLE-AS-IS, MAJOR-1). The internal-tangency contact at the midpoint of the foot's projections let a cusp on near-concentric carriers that cross 0.24 rad from its joint validate (the shared vertex discounted it); `main` refused it only through the foot's offset, and refused smooth joints on the same carriers with it. `arc_arc` now asks `carrier_circles_chord`, the radical foot's depth inside both circles, where the internal clearance reads Zero: past the band the pair is two crossings, read as a secant's; a tangency's candidate in either arm is the midpoint of the circles' nearest points, read by both spans (the per-span projections and `radial_foot` are gone, so there is nothing to swap). `end_in_stretch`'s premise is restated on the chord. Pinned by `a_cusp_on_near_concentric_carriers_refuses_its_crossing`, `a_tangency_reads_at_the_midpoint_of_the_nearest_points` and `seg_reach_fuzz`'s new joint search through `validate::judge_pair`. Filed `a-cusp-on-nested-near-concentric-carriers-validates-along-a-long-eps-stretch` (P0: the `g ≤ 0` half, no crossing, a question about how long a tangent cusp's ε-stretch may run) and `a-vertex-in-an-arcs-gap-within-eps-of-its-carrier-exhausts-the-rays` (P3, NOTE-3). D10 untouched: the joint pass still records the joint tangent.
- 2026-10-19 — #5175 (internal tangency on the carriers) merges after three review rounds. Round 1 found a MAJOR (a silent cusp at the midpoint read), which the first fix pass closed with the chord decision. Round 2 found needless compound-joint escalations, which the second fix pass removed; its tightened joint fuzz then found and fixed a further silent cusp (`Settle::ApartFromEnds`). Round 3 found nothing silent. Filed from round 3: `a-smooth-joint-at-a-near-concentric-crossing-escalates-on-the-apart-from-ends-settle` (P2, a fail-loud regression on narrow geometry), and a wider witness on the P0 `a-cusp-on-nested-near-concentric-carriers-validates-along-a-long-eps-stretch` (not only nested carriers). Review tier: single FULL review, run three times.
- 2026-10-19 — Final fix pass on #5068 after the delta review of ruling A (APPROVE-WITH-FIXES, no MAJOR). M1: the `f64` skip relogs, in pass 2's place, the pre-pass's recording less `profile::PINNED_UNDER_GUIDANCE` (`k_stats::relog`), so `f64` and lane profile logs diff alike; `evaluate_witness` retires, and a test-support door runs the evaluator's own pass 2 for the bit-identity row, which now holds logs too; a new row diffs `f64` against `Interval` profile logs over the corpus (63/63 parted without the relog). M2: `plate_param` and `boss_union` keep their in-step rows and gain apart rows to their pinned partings (INLANE's two items). Merged `origin/main` across INLANE's derived lift (#5248), retired under ruling A; ledger row F22 (main took F21), fork row 172. Filed `work/inlane/the-loft-enclosure-arm-re-enters-its-sections-canonical-form-as-constants`; diagnosed `the-declared-tangency-bracket-refuses-its-closing-junction-at-interval` (an `atan2` branch cut at Interval).
- 2026-10-19 — #5354 merged (Ev: "that plan sounds good!"). The cusp's ε-stretch is the joint's own, and the joint pass reads its turn. The implementation is dispatched on `paths/joint-reads-its-turn`: `seg::joint_tangency` decides from `path_junction_turn` levered by the arm; docs at `validate.rs` step 3 and `end_in_stretch`; pinned rows; the P0 closes as a contract note; the designers' off-question findings are filed. D10: Ev ruled "now". Review tier: single FULL review (every profile's joint verdicts).
- 2026-10-19 — PR 5377 (`paths/joint-reads-its-turn`) implements #5354's ruling. `seg::junction_turn` (`path_junction_turn`, unsigned) is the one home of the junction's turn. The path door's `junction_check` calls it at the arriving arm; `seg::joint_tangency` calls it after carrier identity, at the longer leg's arm (canonicalization may reverse a loop, and the longer arm never records a tangency the door built as a corner). Verdicts that moved: crossing joints turning 3e-7–2e-5 rad are corners, not decided tangent cusps; the 95ε witness escalates at its joint (1.006ε, the number the extrude read), and the brief's "validates" did not hold for it; flattened fillets on arc legs and short arc remnants beside a fillet refuse at the door, the latter still blamed on scene resolution (`a-short-run-outs-…`, widened); the editor-core √ε junction is a corner, so the coincidence door's `ProfileJunction` arm has no reaching document (filed on COINCIDE). Docs say no pass reads a joint's ε-stretch (`[topology.census]`). The P0 closed as a contract note. Filed: `seg-candidates-skips-…`, `a-smooth-joint-at-a-near-concentric-crossing-…` (here), RESTFRONT's cusp curvature-cap arm, CARVE's smooth in-band SliverJoin, STRUT's step-8 doc, COINCIDE's unreachable arm; appended TWICE's wedge-reach row (now reachable through extrude).
- 2026-10-19 — PR 5377 fix pass (review: NOT-MERGEABLE-AS-IS; F1 ordinary fillets on long legs refused, F3 door and validation disagreed). Under the orchestrator's ruling the junction turn is levered by the SHORTER of the two legs' arms, in the door and in validation, through the one `seg::junction_turn`: two legs lie side by side only over the shorter one, the minimum is symmetric under reversal, and the 95ε witness then validates as #5354 ruled (0.0106ε). The door reads early at the arriving arm and re-reads at the shorter arm once the departing leg is stored; `seam_arrival_check` and the joint pass's `junction_reverses` read the same arm; the stored-form read names its turn `path_fillet_joint_turn`, whose in-band refusal renders the stored-form recourse; the scene-resolution floor is `2⁻⁵²·scale·arm·(1/b_prev + 1/b_next)` and its recourse says nearer the origin or a smaller radius than that distance. Verdicts that moved: window fillets build on their stored chord (main refused them `chord_side`); some shallow arc × arc fillets build and validation's pair pass escalates `carrier_line_circle` (filed `an-adjacent-line-arc-pair-…`, P2); short run outs no longer reach `FilletCarrierBelowSceneResolution` (`a-short-run-outs-…` re-measured, narrower). Filed `arc-lever-caps-a-large-sweep-arcs-arm-at-its-chord` (P3, design: its fix would make the 95ε witness escalate). TWICE's wedge-reach row re-banded P2: at depth 4 and through `revolve`, 15ε and 19ε build a sharp edge on a joint the profile declared tangent; `a_revolved_wall_arc_join_reads_its_reach` pins the revolve's `SliverJoin` at 5ε and 10ε.
- 2026-10-19 — #5068 (the sketch plane is its frame, with Ev's ruling A) merged as `8b15425279`. The CI reds were three 1e-12 sweep rows, inherited from main and filed on BAND, GERM and FLUX. #5175 merged earlier the same day. The cusp ε-stretch fork is `[ev]` #5354 (row 173).
- 2026-10-19 — `the-declared-tangency-bracket-refuses-its-closing-junction-at-interval` (P2, E) dispatched on `paths/chord-director-components`. D10 check: it is how a chord's direction is computed at `Interval`, not declared contact, so the hold does not bind. Review tier: orchestrator's read, because the fix is the row's own diagnosis plus a sweep of its siblings.
- 2026-10-19 — `the-declared-tangency-bracket-refuses-its-closing-junction-at-interval` (P2) closes on `paths/chord-director-components`. A chord's director is its components over its length (`path::chord_director`, no band decision: the lever gates and shape validation own the length); `line_to_point`, `line_to_start`, `arc_angles` and the fused `Bulge`/`Center` incomings turn that ray rather than an `atan2` angle. `declared_tangency` is green at `Interval` and un-pinned in both corpus rows. At `f64` the bracket lifts bit-identically, one stored arc centre and `declared_tangency`'s four points moved by ulps; under the symbolic tier the pad proves 162 more decisions and its ceiling rises to 3.152e3·ε (pins re-baselined, the ladder row re-witnessed). Filed `a-turn-reads-its-incoming-angle-off-atan2s-branch-cut-at-interval` and `a-zero-length-line-leg-refuses-as-a-tangent-junction`. D10 untouched.
- 2026-10-19 — Design fork weighed: how an author states an arc's ARRIVAL direction (`arc-closer-constructed-from-arrival-tangent`, `via-and-center-arc-closers-declared-arrival`). Two designers, three rounds. Round 1 split on where the end direction is spelled (inside the spec, or as a director step after it). In round 2 each took the other's position. Round 3 converged: one new endpoint-full `arc_to` mode, `Arrive`, whose end (a target with a director, or `Start.arrives_tangent()` alone) is in the spec over one shared `Director` value; it is admitted on the fused rows; the token checks or constructs by the leg's DOF; `Via`/`Center` take the token. The one point left to Ev is whether §2c round 9 binds a new mode's fused sites. The `[ev]` PR is #5374 (`paths/arc-arrival-mode`); both rows carry `needs_ev`. Filed: `no-fused-verb-takes-a-departure-tangent-arc-toward-a-point` (P2, design) and `via-arrival-start-rustdoc-hides-which-direction-its-director-binds` (P3).
- 2026-10-19 — Arrival fork, rounds 4–6, on Ev's reframing in the `[ev]` PR. Ev saw that a fused `Arrive` arc meets its direction mid-arc rather than at its end, and proposed "make it a way to write a fillet, just not necessarily arriving tangent". In round 4 both designers endorsed that framing. In rounds 4–5 they crossed between a line-side builder and an arc-arrival builder. Round 6 converged on the arc-arrival builder: with no `r`, the anchor is forced to be the tangent point, so the line grammar would spell one loop twice. The result: no new `ArcData` mode, and a no-argument verb on a point tip that opens an arc-arrival builder whose start is pinned at the tip. Ev confirmed the reading (tangent where it arrives, free where it leaves) and named the verb `arc_onto()`. `docs/PATHS-DESIGN.md` is rewritten to it. The token half (check or supply; `Via`/`Center` admitted) is unchanged. Filed `radius-arrival-has-no-start-anchored-close` (P3, design).
- 2026-10-19 — Arrival fork, round 7: harmony pass and Ev's choice. A harmony pass over the `[ev]` PR's `arc_onto()` builder found four things.
  - The builder's `.to(Start)` tangent closer was a second spelling of `Start.arrives_tangent()`. That breaks Ev's 2026-09-02 ruling that the token is the whole arrival vocabulary.
  - The worked `B` refused: its waist lay on the spine, which made a cusp and a self-touch.
  - The `.fillet(r).at(p)` start cell was missing.
  - The "constructing closers take no token" sentences contradicted the `Via` arrival.
  It recommended `arc_onto(target)` followed by one director, the `Via` arrival's shape, and Ev took it. The doc is rewritten to that spelling. The `B`'s waist is now `(0.25, 1)`, checked numerically: a sharp turn of about 124°, no crossings. `ArcLegOnOpenFillet` is added for a fillet arrival's bare anchor. `radius-arrival-has-no-start-anchored-close` is restated, and `via-arrival-start-rustdoc-hides-which-direction-its-director-binds` has its premise corrected.

- 2026-10-19 — #5374 merged (Ev ratified `arc_onto(target)` + director and the token check-or-supply half). Implementation dispatched on `paths/arc-onto`, covering both rows (`arc-closer-constructed-from-arrival-tangent`, `via-and-center-arc-closers-declared-arrival`). Review tier: single FULL review (a new verb with its own kernel geometry and a seam rule across every closer).

- 2026-10-19 — PR 5384 (`paths/arc-onto`): `arc_onto(target)` + one director, built as one `transition_table!` verb with two pending states (`ArcOnto`, `ArcOntoStart`). Its geometry is `tangent_arc_to`'s kernel run from the target backwards, so the refusals are shared and the round trip is exact. `Start.arrives_tangent()` is admitted on the `Via`/`Center` legs and the `Center` arrival (checked), and on the `Via` arrival (supplied, closing at the verb). It is projected through editor-core (an additive wire variant), the viewer, pncad-py and the facade. Closes `arc-closer-constructed-from-arrival-tangent`, `via-and-center-arc-closers-declared-arrival` and `via-arrival-start-rustdoc-hides-which-direction-its-director-binds`. Files `tangent-arc-ahead-in-band-builds-a-straddling-arc-at-interval` (a pre-existing Interval NaN in the shared kernel). (implementer)

- 2026-10-19 — PR 5384 fix pass (single FULL review, APPROVE-WITH-FIXES, one MAJOR).
  - MAJOR fixed at the root: the shared tangent-arc kernel now emits the line its collinear DECISION names and escalates an undecided reading. Before this, an arc built off a sub-band `across` closed silently at Interval and stored a ~1e16 carrier at f64.
  - Every closer now classifies its seam through `seam_joint`.
  - Filed `tangent-arc-near-a-full-turn-loses-its-end-vertex` (P1, silent at f64 for R ≈ 1e4), because its fix moves every tangent arc's stored bits and wants its own unit.
  - Closed `tangent-arc-ahead-in-band-builds-a-straddling-arc-at-interval` as fixed. (implementer)
- 2026-10-19 — chord director (#5370): dispatched at "orchestrator's read"; the unit grew to five chord sites, symbolic-tier moves and re-baselines, so its tier is raised to a single FULL review.
- 2026-10-19 — #5370 fix pass after its single FULL review (NOT-MERGEABLE-AS-IS, MAJOR-1: `arc_angles` stored `atan2(ray)`, so `.turn` and the `Sweep` extension after an arc ending along −x read the whole circle at `Interval`). Every direction a later step turns is now its ray rotated (`path::turned`); `Dir` carries no angle; `a-turn-reads-its-incoming-angle-off-atan2s-branch-cut-at-interval` closes. The chord door always prescales by its largest component (MINOR-2, -3); the fused incomings' start tangents are pinned against an independent derivation (MINOR-4); NITs and style addressed. Filed `chord-and-component-directors-could-share-one-normalizing-core` (P3).
- 2026-10-19 — Main red after #5370: two viewer slow rows (`review_pick_r2` tally, `display_budget` table) moved through `declared_tangency` alone, attributed by running pre- and post-#5370 trees (#5384 moved nothing). Both moves are right (the bracket's vertices on the nominal bits; the arc's sweep an ulp above `FRAC_PI_2`); re-baselined on `paths/viewer-slow-after-chord-director`. The gate ran viewer's fast set, not its slow set, by design: the slow set runs for the crates a diff seeds.
