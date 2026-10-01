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
