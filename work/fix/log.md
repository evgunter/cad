# FIX log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/fix/plan.md`. A/B band 1700–1799
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose FIX section is the
charter this plan restates. Opens now. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `transform-rigid-refuses-described-nurbs` from `work/issues/`
- `error-types-with-no-display-class` from `work/issues/`
- `no-parametric-loop-constructor` from `work/issues/`
- `coherence-findings-have-no-consumer` from `work/issues/`
- `unify-discipline-machinery-onto-registry` from `work/issues/`
- `census-decline-consults-one-face-of-pair` from `work/mate/`
- `interior-witness-budget-decline-untyped` from `work/mate/`
- `split-crossings-skip-pattern-mate-ends` from `work/mate/`
- `mate-clocking-has-no-gui-path` from `work/mate/`
- `nested-pattern-mate-heads-refuse` from `work/mate/`
- `tier-3-prime-findings-render-through-debug` from `work/lib/`
- `subject-body-drops-the-declared-contacts` from `work/lib/`
- `mate-contradiction-names-one-mate-twice` from `work/lib/`
- `pin-mismatch-recourse-emitted-twice` from `work/lib/`
- `unit-admits-non-finite-direction-norm` from `work/seat/`
- `band-linear-spelling-not-swept` from `work/seat/`
- `boolean-error-has-no-fieldless-kind` from `work/bool/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Orchestrator picked up (2026-09-04)

Session takes the program with no unit cut and no branch existing.
Two items arrived already closed (`pin-mismatch-recourse-emitted-twice`
finding 1, `subject-body-drops-the-declared-contacts`), both by the
lanes that filed them, before this program opened.

**Decisions taken unilaterally at pickup**, all recorded in `plan.md`:

1. **No A/B row on any unit** (Ev, in-chat, 2026-09-04). The 1700-1799
   band stays unclaimed; `docs/MODEL-AB-LOG.md` is not touched by this
   program. Review posture is one style lane per unit, plus a second
   correctness-focused reviewer on the three units that move a kernel
   answer rather than its rendering.

2. **No unit here carries a full adversarial review.** Ev's framing at
   handoff is the rule this program adopts: a one-PR item that would
   need an adversarial arm is an item cut wrong. Two items failed that
   test on reading and were re-cut rather than dispatched (below).

3. **`error-types-with-no-display-class` is cut into three PRs by
   fence** — viewer, editor-core, kernel-crate remainder. As one unit
   it is ~18 types across five crates and four `keep_out` fences, and
   the item's own text asks its taker to re-sweep rather than trust the
   list; one lane cannot own four announcements and one honest blind-
   spot sentence at that width.

4. **`unify-discipline-machinery-onto-registry` is held.** It is the
   one item on the slate whose body does not contain the fix: it names
   a seam and an order, not a diff. It gets its own spec pass or it
   leaves the program; it is not dispatched as a one-PR item.

5. **Branch convention.** Unit branches keep the program's ratified
   `fix/` prefix (#396). The orchestrator's own branch this session is
   `claude/program-fix-orchestration-caoqwh`, set by the session
   harness rather than by the #396 convention; `fix/orchestrator` is
   unused.

`nested-pattern-mate-heads-refuse` is the program's one ruling and
goes out as an `[ev]` PR, not a unit.

## First wave adjudicated (2026-09-04)

Six units dispatched, three delivered and reviewed, three implementing.
Corrections to the record, in the order they were established.

**The `error-types-with-no-display-class` re-cut was wrong.** The
three-way cut was a sound decision on the information in the item, and
the information was stale: every type the item names already carries a
`Display`, spelled `impl core::fmt::Display for`, which the sweep that
produced the list could not see because it grepped `Display for`.
`MigrationError` is not in the tree at all. Cuts 2 and 3 are empty,
verified here and not taken on the lane's word. The remainder is one
small unit; `plan.md` carries it.

**The `nested-pattern-mate-heads-refuse` ruling moved under Ev's
question** (PR 1731). Ev: (a) desirable, (b) uncertain — are there
natural cases? Reading `wire_transform` answered it and reversed this
orchestrator's recommendation. A `Transform` contributes NO `RolePath`
segment (spec D2, identity-preserving pass-through), so a mate
reference through one still names the minting `InstantiatePart` and
**mating to a transformed instance already works**. Add a pattern and
the name's node becomes the `Pattern`, whose input is the `Transform`,
and it refuses. The natural case for (b) is that asymmetry itself.
That also makes (b) one walk with (a), not a separate rule, so the
"ratify the fence" recommendation this program opened with no longer
stands. Proposed instead: rule both in, implement as one S-MATE unit
(a member-identity type change, not a FIX one-PR item), gated on one
measurement NOT yet established — `fold_pair` builds cosets from
authored alignment data and never reads the evaluated body, so if a
`Transform` between an instance and mated material is reachable, the
solve may be transform-blind for the case that already works.

**Territory: SHELL claimed `crates/topo/src/transform.rs`** when it
opened 2026-09-03, after this program's charter read the file as
unowned. `paths` here drops it; `keep_out` records the crossing. The
collision is live, not theoretical: SHELL also holds
`transform-rigid-refuses-approx-face` (#1020), and the NURBS arm this
program carries and the Approx arm SHELL owns are the two refusing
arms of the SAME two match statements. Flagged on PR 1730 with three
dispositions offered (land-and-announce, hand the item over, or SHELL
takes both arms as one unit); the territory's owner decides, and
nothing has been moved.

**Review posture, measured.** Style reviews are earning their cost.
Both delivered so far found a defect the unit's own claims covered
over: on PR 1738 the fix routed a non-finite direction into
`MateFault::DanglingHead` — a false cause the unit's sweep table
recorded as "fixed", and structurally the same defect the unit set out
to remove at the transform door; on PR 1732 the one row the unit
singled out as its deliberate exception hardcodes K in its second half
and goes red at `CAD_AMBIGUITY_K=30`. Both reviewers also corrected
this orchestrator on a brief claim. No unit has needed an adversarial
arm.

**Two lane errors worth the class line.** A lane filed a duplicate
issue into ANOTHER program's tracker directory from its own unit
branch (`work/m10/`); the item already existed and the filing was the
orchestrator's to place on the away channel, not a diff's to carry.
And a lane described an inherited CI red as unfiled debt when the item
existed. Both corrected. The shape: a lane that finds a defect outside
its fence reaches for `work/` before it reaches for its report.

**Standing note for every brief from here** (found by the band lane):
the `CI-Config:` trailer is read off the head commit ONLY and is
voided by any later commit, a merge included — so a requested lane
needs the trailer restated on every head.

## Wave 1 closed, wave 2 out (2026-09-04)

**Four units merged**: `unit-admits-non-finite-direction-norm` (1738),
`error-types-with-no-display-class` (1741), `band-linear-spelling-not-swept`
(1732), `split-crossings-skip-pattern-mate-ends` (1749). Two in fix pass:
`transform-rigid-refuses-described-nurbs` (1742), the census pair (1750).

**Every unit's review found something the unit's own claims covered
over, and no unit needed an adversarial arm.** The pattern across all
six is one thing: a lane's *executed* result was reliable, and a
*written* claim near it — its own, its item's, or the surrounding
docs' — was not.

- 1738: three separate written claims in one area were plausible and
  false or incomplete (`placement.rs`'s stale refusal name,
  `is_finite_length`'s "every direction door", `derived_offset`'s
  defence of its catch-all). The last is the durable one: the doc
  argues the catch-all is safe because the pattern node names the real
  cause, and the mate fault POISONS the document so that node never
  evaluates. Filed to S-MATE.
- 1742: the item asserted the refused walls carry "rational weights".
  Measured: every weight is exactly 1.0 — and under uniform weights
  Euclidean and homogeneous storage are indistinguishable, so the
  suite could not have caught the storage error its own argument is
  about.
- 1732: the row the unit singled out as its one deliberate exception
  hardcodes K in its second half; the reviewer's suggested repair
  would not have caught it (its sample points 10/3.5/30 all clear the
  arm that only overflows for K > 2).
- 1749: the item's premise was refuted by construction, and the
  *obvious* fix would have created the defect the item feared.
- 1741: the item's list came from a census keyed on a spelling the
  types do not use; cuts 2 and 3 were empty.
- 1750: the arm table is provable rather than sampled, and the lane's
  own ×2 correction rescued the CLAIM, not just the number.

**Two orchestrator errors, both caught by reviewers, both corrected in
place:** a "re-homed on main" claim that had not landed (fixed by
merging 1761), and telling a lane its branch was one tracker commit
behind when main was 14 commits ahead touching viewer sources.

**One duplicate filed by this orchestrator.** CHROME's style lane filed
the viewer free-move hole hours earlier with an executed row. FIX's was
deleted and folded in. Note what this says about the rule merged in
1757: it routes cross-fence findings through the orchestrator because
the orchestrator has the view the lane lacks — but **two orchestrators
do not have each other's view**, and only the away channel closes that.
Not amended on one instance; recorded so the rule is not read as
claiming more than it does.

**A second false attribution, self-caught:** two notes said PR 1749
made `member_of` public. Main landed it independently while 1749 was in
review; 1749 resolved onto main's spelling and deleted its own. The
consumer and the argument are 1749's; the home is not. Corrected on
both issues.

Wave 2 out: `no-parametric-loop-constructor`,
`mate-contradiction-names-one-mate-twice`. Held behind the census fix
pass: `tier-3-prime-findings-render-through-debug` (shares
`census.rs`). Still unscheduled: `coherence-findings-have-no-consumer`,
`boolean-error-has-no-fieldless-kind`, `mate-clocking-has-no-gui-path`
half 1.

## Announced from LIB (2026-09-09): a derive word on the four `checks.rs` enums, and `Hash` by symbol in `quantity`

LIB-MIRROR (PR #2271) adds `Hash` to `CheckId`, `CheckKind`, `Severity` and `Advisory` (`checks.rs:51,128,141,161`, `CheckId` keeping its `PartialOrd, Ord`), and gives `UnitDef`, `LengthUnit` and `AngleUnit` a hand-written `Hash` over the row's symbol plus `Eq` (`quantity/src/units.rs:563-585`, test at `quantity/src/tests.rs:528`) — the seal makes the symbol determine the row, so the hash agrees with the derived `PartialEq`; all under Ev's (A) ruling on `[ev]` #2265.

## Orchestrator resumed 2026-09-11: the board re-sorted against the repo

**Three rows were lying.** `boolean-error-has-no-fieldless-kind` (1806),
`nurbs-net-point-map-helper` (1742) and `prose-gate-has-no-mechanical-guard`
(1809) all sat at `review` with their PRs **merged on 2026-09-04** — the
board showed three units in flight and there were none. Each is closed
here with a `## Closed` section saying what landed. The shape is the
one this program keeps finding: an executed result was reliable and a
written record beside it was not, and the record here was the tracker's
own.

**Review posture for this stretch (Ev, in-chat, 2026-09-11):** light
style review or none, at the orchestrator's discretion, because the
units are very small; **no A/B row** (unchanged from the 2026-09-04
posture — band 1700-1799 stays unclaimed, `docs/MODEL-AB-LOG.md`
untouched). A unit that turns out to move a kernel ANSWER rather than
its rendering is re-cut or reviewed, not waved through.

**Claimed from `work/code-quality/` (Ev, in-chat):**
`tour-scenes-lift-componentwise-not-through-map` — a style sweep over
`demos/tour/src/*`, ground no open program's `paths` covers, fix
written with `lily.rs` as the worked example. Moved into this
directory per `work/README.md`'s claim rule.

### Wave dispatched 2026-09-11

Three lanes, disjoint files, each one item and one PR:

- `compile-fail-blocks-without-error-codes` — `fix/compile-fail-error-codes`.
  FIX's own glob (`crates/quantity/*`), no fence crossed. The brief
  makes the error code a MEASUREMENT rather than a guess: stable
  rustdoc does not verify the annotation, so a wrong code would make
  the unit a no-op wearing a fix's clothes.
- `mate-member-vocabulary-restated-in-refactor` —
  `fix/refactor-member-vocabulary`. FIX's own glob
  (`crates/editor-core/src/refactor.rs`); `crates/editor-core/src/mate/*`
  is MSOLVE's and DOCM's and the lane is fenced out of it. The sweep,
  not the one-line predicate, is the unit: this row exists because an
  invariant established by PR 1748's bugfix protected only the code
  that already knew.
- `tour-scenes-lift-componentwise-not-through-map` —
  `fix/tour-scenes-lift`. Unowned ground. The brief carries the one
  hazard a spelling sweep has: if a committed scene figure moves, the
  new lifting order is not the same arithmetic and that is a finding,
  not a re-baseline.

The item files are the LANES' to edit — the orchestrator does not mark
them `dispatched` from this branch, because one-file-one-item makes
that a merge conflict with the PR that closes them.

**A repo-history note, recorded because it cost a push.** `origin/main`
has been re-published: its history is 336 commits rooted at
`366591cf`, with **no merge base** against the pre-2026-09-06 refs. The
old `fix/orchestrator` branch on the remote is an orphan from before
that cut and cannot be merged into anything. This orchestrator works on
`fix/orchestrator-sep11` rather than force-pushing over it; the orphan
is left alone. Any lane that finds a branch with no merge base against
`main` is looking at the same thing.

### An inherited red, found by a lane that could not have caused it (2026-09-11)

The `compile-fail-blocks-without-error-codes` lane's run is the first
**code-tier** run over `main` since `c1ea73a2f` (#2320) landed a doc
link from the ungated `viewer::session` into the `app`-gated
`viewer::widgets`. Every `main` push in between classified below the
code tier, so the job that reads intra-doc links was `skipped` on all
of them and the break sat green for a week. Filed on the owning
program's slate as
`work/view/viewer-doc-link-crosses-the-app-feature-gate.md` and VIEW
summoned on its open PR; FIX does not absorb it.

Two things follow for this program's remaining waves:

- **`gate ok` is red for every code-tier PR in the repo** until VIEW
  lands the fix. Under Ev's 2026-08-31 ruling that does not block a
  merge once the red is annotated with its issue — but every FIX lane
  from here must say, explicitly, that the ONLY failing jobs are that
  one, and re-check rather than assume it.
- The silent-coverage class has a third face worth naming beside the
  two in `memories/agent-lane-operations.md` (a green job name over a
  skipped step; a run queued with zero jobs): **a run of green `main`
  pushes none of which executed the step at all.** A red reachable only
  from a code-tier change is invisible for as long as the repo is
  landing docs, and `main` being green is not evidence it ran.

### `compile-fail-blocks-without-error-codes` closed (PR 2335, 2026-09-11)

All eight bare fences in `crates/quantity/src/units.rs` name a code
**measured off `rustc`** at the pinned 1.97.0, not inferred — and the
measurement is what the unit was for. The prose around the blocks
called all three view refusals "a PRIVACY refusal" and the neighbouring
`UnitDef` rows are `E0451`, so the plausible guess was `E0451`
throughout; the two tuple-struct mint rows are in fact **`E0423`**,
because a tuple struct's private field refuses at the constructor path
rather than at the field list. Stable rustdoc never checks the
annotation, so that guess would have shipped a wrong code invisibly,
inside a unit whose whole subject is a block that pins nothing.

Every block already had a legal twin, so none was added and no block
needed the "cannot" escape; what the doc gained is the honest sentence
about what the twin catches and what the code annotation does not.

Not taken, deliberately: a per-block twin. Each group shares one today,
which catches a rename of the type or constant but not a shift in one
block's subject that leaves the shared symbol intact. That is a
restructure of the doc's existing shape rather than this unit, and it
does not earn a row — the limit is now stated at the site, which is
where a reader meets it.

CI run `34561282277`: 31 success, 5 skipped, 2 failure, the two being
the inherited viewer rustdoc red filed as VIEW's above. Merged with
that red annotated, per Ev's 2026-08-31 ruling.
