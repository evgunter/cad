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

### An inherited red, and an orchestrator error on top of it (2026-09-11)

Two FIX lanes' runs were the first **code-tier** runs over
`crates/viewer/src/session/op.rs:773`'s link into the `app`-gated
`viewer::widgets`, and both came back with
`rustfmt + rustdoc (gate) + wasm32` and `gate ok` red. Every push to
`main` in the preceding week had classified below the code tier, so
that job was `skipped` on all of them — runs `34560796333`,
`34561272106`, `34561757424`.

**This orchestrator then filed it on VIEW's slate and summoned VIEW,
and both were wrong.** VIEW's PR #2332 was open at that moment with the
red *already measured, attributed and cleared in its own body*, and it
merged at 04:37Z. The item and its PR #2340 are withdrawn; the summons
is retracted on the thread.

Three things to keep from it, because the mistake is instructive and
the cheap reading of it is not:

- **The check that was skipped was reading the open PR list.** This
  orchestrator had that list in hand — it was read at session start to
  find FIX's own in-flight PRs — and #2332's title says *"stop the
  skip-mode viewer doc pass judging links"*. A red on another program's
  ground is not filed until that program's open PRs have been read for
  it, however well the reproduction is established. Reproducing a
  defect proves the defect, not that it is unowned.
- **The remedy was a design question already ruled on.** The item
  proposed three spellings at the link site; Ev had ruled the other way
  in chat the same day (link them, make `broken_intra_doc_links` inert
  on the DEFAULT-features viewer pass). An orchestrator proposing
  repairs on another program's ground is proposing inside a
  conversation it cannot see.
- **What was genuinely ours is the complement, and it is small.**
  #2320's own CI missed the link because a diff touching
  `crates/viewer` sets `RUN_VIEWER_TOOLKIT=true` and takes the non-skip
  path; the FIX lanes missed nothing and simply arrived from the other
  side, where the job is skipped for the whole class of pushes. The
  window had two independent reasons nothing read that link, one per
  side of the branch/main divide. A green `main` was evidence of
  neither — which is the third face of the silent-coverage class, next
  to a green job name over a skipped step and a run queued with zero
  jobs.

**Standing instruction for every FIX lane from here**: an inherited red
is reported with its evidence and NOT acted on — not filed, not
summoned, not fixed. The orchestrator places it, after reading the open
PR list.

### `mate-member-vocabulary-restated-in-refactor` closed (PR 2338, 2026-09-11)

**The item's premise was refuted by the tree.** `refactor.rs` already
asks `member_of`; PR 1749 landed the call and the comment together, and
what this row described as an unfixed predicate was fixed a week ago.
That is the fourth unit of this program whose *item* turned out to be
the unreliable half — the pattern the 2026-09-04 entry named, still
holding.

What was actually owed is the other half of "done", and it is the
finding worth keeping: **1749's unification landed unpinned.**
Reverting the predicate to the pre-1749 `matches!(…InstantiatePart…)`
spelling and running the suite gives 1197 passed, 0 failed — all four
rows 1749 shipped *with* its fix pass on both spellings. The fix was
real and nothing held it, so the next lane to touch that predicate
could have undone it in silence.

The new row reaches the direction the existing ones cannot. The
nested-pattern row's head is a `Pattern`, so a gate matching
`Node::InstantiatePart` skips it and is right **by accident** — it
cannot tell asking the vocabulary from matching a spelling.
`a_stranded_operand_over_an_instance_head_contributes_no_crossing` puts
a live `InstantiatePart` at the head and strands the operand, so only
the walk knows the reference resolves to nothing; a head-spelling gate
mints an `InterfaceCrossing` for a mate that never solved, which AQ8's
(b)-SKIP forbids. Red on the old spelling, green on the landed one,
with the compile verified to have tracked the edit rather than served a
stale binary.

Limit stated and accepted: the row proves the a-side of the
`is_mate_edge_end(a) && is_mate_edge_end(b)` conjunction; the b-side is
symmetric by construction, not by a row.

Fence named: `crates/editor-core/tests/fix_pattern_mate_crossing.rs` is
TCOST's and TINT's glob — one row appended to an EXISTING file, so no
new test target and no new per-binary codegen+link constant, which is
the part of that ground TCOST's charter is actually about.

CI run `34563223900`: **33 success, 5 skipped, 0 failure** — the first
fully green FIX run of the day, and the confirmation that VIEW's #2332
cleared the rustdoc gate on `main`.

### `tour-scenes-lift-componentwise-not-through-map` closed (PR 2341, 2026-09-11)

**The filing's measurement was wrong in three ways, and the lane
re-took it rather than inheriting it.** Population: **31 sites in 9
files**, not the 26 in 8 the title lists. The filing's grep was
single-line, so it missed every constructor `rustfmt` had broken across
lines — three more in `twopeg.rs`, one each in `bossplate.rs` and
`curvedcut.rs`, and the whole of `crosslap.rs`, a file the row never
names. `paths.rs`, which the title lists, **does not exist and never
did**. And `lily.rs` had been partly swept since the filing, so several
of the line numbers named nothing.

**Nothing moved, and that was measured rather than argued**: the
release render's entire output tree is byte-identical to the merge
base's — every STL, STEP, `scenes.json` and UV chart — and so is the
k-probe sweep's 1 590 255-sample CSV. That second one is the row that
could have moved, since `lily` is in the probe subset and its frame
lifting changed.

**The `from_frame` choice is inert in this corpus, which the row
presented as consequential.** Both sites take `map`, because a frame
this file composed at `f64` crosses once as a value — but `Scalar` is
implemented for `f64` and `Probe` only, `Probe` is a transparent `f64`
wrapper, and the tour never instantiates at `Interval`. At every scalar
the tour actually runs the two spellings are bit-identical.

Library gaps, filed rather than worked around in silence per
`memories/demo-purpose.md`: `diechamfer.rs:100` keeps its tuples
because `Point3` has no order and `Vec3` no sup-norm door —
`work/props/point3-has-no-order-and-vec3-no-sup-norm-door.md`, the next
instance of the class `vec3-point3-const-and-conversion-doors` closed
on 2026-09-05. PROPS has no open PR; checked before filing this time.

Kept beyond the letter of the sweep, deliberately: four sentences of
crate doc in `main.rs` stating the layer rule and pointing at `lily`'s
full statement. The rule now holds corpus-wide and a reader landing in
`az.rs` had no pointer to it — that is the invariant the sweep
establishes, and stating it is where a comment earns its place.

### An orchestrator error that cost a lane a CI window

The lane's PR sat from 04:31 to 04:40 with **no run at all**, and it
reported the cause as GitHub suppressing workflow triggers for PRs
opened through an app token. **That is not what happened**, and the
measurement refutes it: PR 2334 was created at 04:07:58Z and its run
was created at 04:08:02Z, four seconds later, on a head pushed before
the PR existed; PR 2335 the same, at 04:11:30Z and 04:11:34Z. Opening a
PR through the MCP tool triggers CI.

What actually happened is the CONFLICTING-no-run class already recorded
in `memories/agent-lane-operations.md` — and **this orchestrator caused
the conflict.** The claim that moved this row from
`work/code-quality/` into `work/fix/` landed on `main` from the
orchestrator branch *while the lane was moving the same file*, which is
precisely the merge conflict `work/README.md`'s one-file-one-item rule
exists to produce. A CONFLICTING PR gets no run, silently, and none
retroactively when the conflict clears.

The rule that follows, and it is narrower than "do not touch item
files": **the claim and the dispatch are one act.** A row claimed into
this program is claimed in the commit that dispatches its lane, or the
lane is told to do the move itself and the orchestrator keeps its hands
off the file — never both. The lane did the right thing (it made its PR
the claiming PR, per `work/README.md`) and paid for the orchestrator
doing it too.

### `checks-product-refusal-degrades-to-string` closed (PR 2344, 2026-09-11)

`ProductErrorKind` (10 fieldless variants) with an exhaustive `kind()`,
and both `Subject::Unavailable` and `ChecksError::Product` carrying it
beside the prose. Raise sites write neither field: `Subject::refused`
and `ChecksError::product_unavailable` pair the two off one error.

**A deviation from the item's letter, and it is an improvement rather
than a shortcut**, so nothing further is owed. The item asked for
`kind: ProductErrorKind`; what landed is `Option<ProductErrorKind>`,
because `Subject::Unavailable` has a second inhabitant that is not a
refusal at all — `Subject::not_needed()`, the run where no enabled
resident asked for a subject. `None` is the honest value there, and
minting a kind to fill the field would publish a class no
`ProductError` can carry. Pinned by its own row. The alternative —
splitting `NotNeeded` out as its own variant — would move which refusal
`run_checks_on` raises, which is an ANSWER, not a rendering, and was
correctly refused for this unit.

**The first run was red and it was the lane's own**, which is worth
recording because nothing local could see it: a new name in
`pncad::document` reds `test_binding_census.py`, whose census refuses a
façade name that is neither bound nor dispositioned. `cargo build`,
`cargo test -p editor-core`, three `clippy` invocations and
`cargo fmt --all --check` were all clean over it. Any FIX unit adding a
re-export to `pncad::document` owes that census a row.

**Guard coverage is complete for the first time**: all 10 arms
constructible, against 1806's 34 of 41. Each guard direction verified
red on plant and reverted — a phantom kind naming itself at `E0004`, a
mis-projected arm, a hardcoded kind at the door, and a constructor
dropping the class.

**Two findings placed, each checked against the owner's open PRs
first** (the check this orchestrator skipped earlier today):

- `work/docm/part-fault-partproduct-degrades-the-product-refusal.md` —
  `eval/parts.rs:420`, DOCM's fence, now a one-field change because
  this unit minted the class it needs.
- `work/fix/node-error-kind-has-no-fieldless-projection.md` — three
  doors (`drive.rs`, `mc.rs`, `stackup.rs`) with a `NodeErrorKind`
  value in hand, rendering it away. Invisible to 1806's sweep because
  its field-name pattern lacked `cause`. Homed here because `mc.rs` is
  unowned; the row carries the correction that `NodeErrorKind` is not
  missing, only unprojected.

**A tripwire instance appended to TINT's row**, not fixed:
`docm5_subject.rs`'s `gathers_in` counts source-text lines containing
`" product("` and accused an ordinary method declaration of being a
second gather call. The lane renamed around it. It is still armed, and
it is a *second grammar* with that row's failure mode — matching a CALL
by text, where the row's existing instances scan declarations.

And `kind-mirrors-have-no-single-declaration` is updated with the thing
four instances have now established: **the pairing direction is
closable by a derive and by nothing else.** Three hand-copied guards is
the evidence, not an argument.

### A sequencing decision taken, not asked (2026-09-11)

`two-d-director-doors-skip-the-finiteness-question` is the most
valuable row left on this slate — five doors that decide a length's
SIGN without asking whether the length is a finite NUMBER, so a `1e200`
component takes a definite-positive decision and then normalizes to the
zero vector. Measured outcomes include
`mirror_across_plane(origin, (1e200,0,0))` returning `Ok(IDENTITY)` — a
mirror that mirrors nothing, silently — and two of the five are
reachable from a public door with no pre-validation.

**It is not dispatchable yet, and its own body says why**:
`is-finite-length-homed-in-the-query-seat` *"is the ruling that unlocks
four of these five rows at once"*. `profile` depends on `geom-core`
alone and cannot reach `topo`, so while the predicate lives in
`topo::query` two of the five doors cannot ask the question in its one
spelling — and a second spelling is what this family's ruling is
against.

So the homing unit is dispatched first, with the decision taken rather
than asked: **`is_finite_length` moves to `geom-core`.** The alternative
— leave it in the query seat and carry the header sentence — is
recorded here as rejected, on reachability: it does not merely cost
tidiness, it leaves live silently-wrong behaviour closed to the one
spelling. (Ev, 2026-09-06: sequencing decisions with a recommendation
do not wait.)

**The item was parked on a program that no longer exists.** It asks the
question of SEAT — *"where it lives is SEAT's call, not a passing
program's"* — and `work/topo/program.md`'s `keep_out` still names
`topo/src/query.rs` as SEAT's ground. There is no `work/seat/`. A row
whose owner closed is not blocked, it is unowned, and this one has been
sitting on a dead trigger since the sweep that re-homed it. Worth a
check across the board: `keep_out` prose naming a closed program is
invisible to lint, which resolves ids and not sentences.

### `literal-k-where-the-runs-k-belongs` closed (PR 2346, 2026-09-11)

Per-site, as the item demanded — and the per-site reading is what made
it worth a unit, because the six sites split three ways:

- **Two mean the run's K** and now consult `Tol`. Verified live rather
  than assumed: in both, the band reaches a door where a residual in
  (ε, K·ε) escalates, so the escalate edge is read.
- **Two are genuinely arbitrary** — the item's reading, which the lane
  checked rather than inherited: `tube_ladder` and `MarchTol::from_band`
  read `band.zero()` and nothing else. They now carry a sentence saying
  the escalate edge is inert, and a multiplier that does not read as K.
- **One is deliberately pinned on BOTH edges, and "consult `Tol`" would
  have been WRONG there.** `pcurve_p1a_meter.rs` asserts
  `d·sec α = 1.1316 ε` is in band; patched to the run's K and run at
  `CAD_AMBIGUITY_K=1.05`, **two of four rows go red**. Reverted, and the
  site now says why it keeps ×10. That is the row that justifies the
  item's insistence that this is judgement and not a sweep.

**Running at another K found two defects red on `main`'s own tree**,
neither reachable by reading: `dsc_checks.rs` (a slab `10ε` thick, so
the escalation it asserts happens only for K > 5 — red at K = 3 and
1.05) and `bool3_torus_doors.rs` (shell measured at the run's band, law
computed from a literal `10·ε` — at K = 100 it fires its own law
assertion, *a false accusation against the kernel*). Both repaired in
the unit under the sibling-sweep rule. They are outside FIX's `paths`
and inside the item's class; repairing a row that is actually red under
a legal configuration beats reporting it, and the fence is named.

**Three findings placed, each against the owner's open PRs first:**

- `work/ciw/no-ci-row-runs-the-suite-at-a-non-default-k.md` — the one
  that matters most. **No workflow mentions `AMBIGUITY_K` at all**, so
  all twelve `test (…)` jobs run at `DEFAULT_K` while the eps axis gets
  three values × two lanes. Three K-dependent defects in one day, none
  findable by the gate. `k_probe_sweep.sh` runs on every code-tier run
  and is not this coverage — it measures margins to inform K, it does
  not execute at another one.
- `work/props/band-has-no-door-for-an-explicit-eps-with-the-runs-k.md`
  — **four** suites want the door, not the one the item named.
- `work/tint/torus-tangency-shell-floor-does-not-scale-with-k.md` — a
  fixed floor against a shell growing as K^⅓, red at K = 30 on `main`,
  latent only because nothing draws that axis.

Note the shape the CIW row shares with this morning's viewer doc-link
red: **a defect class reachable only from a configuration the gate
never draws is invisible for exactly as long as nobody varies it by
hand.** Two instances in one day, on two different axes.

Sweep: 125 `Band::new` call sites at the merge base (the parent unit's
close recorded 117 at `7514cc6`), 11 fixed, 6 classes dispositioned.
Blind spot worth carrying: a ratio spelled as **two literals** — the
~50-site `Band::new(1e-9, 1e-8)` family — is separable only by reading,
not by any pattern.

### `verb-and-dimension-render-through-debug` closed (PR 2347, 2026-09-11)

**Half 2 was already discharged, with a zero diff.** All four
`Dimension` labels render through `Display` on `main` today, carried by
PR 2053 (VIEW, 2026-09-06); the item's citations were stale because
`app.rs` moved to `pane/properties.rs` and `session.rs` to
`session/refuse.rs`. That is the **fifth** unit of this wave whose item
turned out to be the unreliable half. The pattern is no longer worth
re-noting per unit; what is worth noting is that in every case the
*executed* re-measurement was cheap and the *written* claim was free —
which is the whole argument for briefing every lane to re-measure.

**Half 1, and the one real decision in it.** `Verb`'s `Display` is
declared **on the macro row** (`verb LineTo(Target<T>) = "line_to" …`)
rather than as a hand-written `match` beside the table. A match would be
compile-caught in both directions, so this is not about safety — it is
that the word is then a second place the vocabulary is written. The `=`
spelling is the repo's own; `viewer/src/vocab.rs`'s `vocabulary!` uses
it for exactly this and cites `transition_table!` as its precedent.

**The word is the algebra's spelling, not English prose** — `line_to`,
not "line to". Three reasons, and the third is the one I would not have
predicted: the viewer already renders the near-identical sentence from
`PathVerb::label()` with these words, so a verb *picked* as `line_to`
and *refused* as `LineTo` was two names for one thing; and
`profile`'s own ratified rule permits `Debug` where "a prose paraphrase
would not find it", which `line_to` satisfies because it is the method
identifier the row declares. A prettier word would have broken the rule
the crate already carries.

Accepted as reported: `FarEndTo` and `CloseTo` render `to (far end)`
and `to Start (close)`, which read slightly oddly mid-sentence. They
are the chrome's own labels, and diverging would reintroduce the
two-names problem the change exists to remove. Consistency wins.

Accepted, with the comment correction that came with it: the
`sketch.rs` site now renders the verb as prose while the state half
stays `Debug` for the table-coordinate argument. `program.rs`'s comment
previously read as universal and now says which sentence is which — a
comment narrowed to what it actually defends, which is the repair, not
a concession.

**Findings placed:**

- `work/fix/verb-error-arity-renders-verbkind-through-debug.md` —
  `crates/verbs/src/run.rs:226`, a **seventh** crate. Homed here because
  `crates/verbs/*` is unowned. It is justified in place and pinned, and
  that justification ("the doors' own names") is precisely the argument
  a `Display` would make explicit.
- Two facts appended to
  `work/issues/the-prose-word-for-a-kind-has-four-spellings-and-only-display-is-censused.md`
  — the macro-row form of spelling 1, and `tip_state_words` as a fifth
  instance of spelling 2 with no table to hang a word on. The
  PathVerb-mirror question already has rows on VIEW's and
  code-quality's slates; nothing new filed there.

**The instrument note worth keeping.** `prose_census.rs` could not have
found this defect and was correctly not leaned on. It judges
brace-shapedness — the struct-dump class — so a fieldless enum with no
`Display` reads as `Prose` by its verdict; it does not expand
`macro_rules!` bodies; and none of the `Dimension` sites sat inside a
`Display` impl. A guard that exists for a neighbouring class is not
coverage for this one, and the lane saying so beats a lane quietly
assuming it.

### `is-finite-length-homed-in-the-query-seat` closed (PR 2349, 2026-09-11)

`is_finite_length` now lives in `crates/geom-core/src/real.rs`, beside
the `Real` it is generic over and the `is_poison` it asks through, and
`topo::query` kept **no** re-export — one name, one place a reader can
find it. The header paragraph calling it the seat's second *public*
exception is deleted, and `decide_unit_direction`, which that header
called "a THIRD", is renumbered.

**The `pub` that PR #1738 added had zero remaining external users.**
SEAT-DN (#1987) had already collapsed `UnitVec3::new` and
`eval::wire::unit()` onto `decide_unit_direction`, so the only code
caller in the tree was that one body. The move cost nothing because the
thing it was moving had already stopped being reached — which is worth
recording as a small case of the standing pattern: the *written* state
(a public predicate with a documented exception) outlived the
*executed* state by two units.

**Nothing moved, and the establishing facts are the right ones**: the
four-line body transcribed byte-identical, `UnitVec3Error`'s three
`Display` arms and `NodeErrorKind::NonFiniteDirection` outside the diff
entirely, no funnel site renamed so no K row can move, and no golden,
k-lint baseline, render or census file in the diff.

One addition, disclosed and kept: a direct row at the new home, because
after the move `geom-core` held a public predicate **its own crate
never exercised**. That is a real gap the move created and closing it
in the same PR is right.

Also kept: a doc paragraph on `Vec2::normalize` and `Vec3::normalize`
saying what the overflow end costs a door that decides the sign first.
The 2-D half is not decoration — rows 4 and 5 of the door class are
`profile`'s 2-D directors.

**The unlock fired.** `two-d-director-doors-skip-the-finiteness-question`
is updated: its two `topo::query::is_finite_length` spellings are now
`geom_core::is_finite_length`, and its "What separates them" section is
answered — all five doors can reach the predicate, so the per-crate
reachability split collapses. What still differs per door is only the
refusal: a typed arm, its sentence, and its K/census consequence, plus
a new public `PathError` arm for the two `profile` rows.

### Our own fence named a dead owner (2026-09-11)

The lane found `work/fix/program.md`'s `keep_out` saying
*"crates/geom/src/* and geom-core are S-CERT's until its exit"*.
S-CERT is closed and that ground is PROPS's glob. Repaired here, along
with the `eval/wire.rs` clause, which cited LIB **and SEAT** for a path
no open program's `paths` covers.

That is the second instance in one day — `work/topo/program.md` says
`topo/src/query.rs` is SEAT's, which is what parked the homing question
on a program that cannot answer. Filed as META's
`work/meta/keep-out-prose-can-name-a-program-that-no-longer-exists.md`,
and deliberately shaped against META's open #2337: that PR's `_names`
check warns rather than errors because many ids are ordinary English
(`view`, `shell`, `fix`), and **this detector has the opposite shape** —
it matches names that are NOT live ids, and the interesting ones
(`SEAT`, `S-CERT`, `S-MATE`) are closed programs' own spellings, which
`docs/DOC-LEDGER.md` already records for another reason. `work/topo/`'s
clause is TOPO's to repair and was not touched.

**A lane edited a sibling FIX item and asked whether that was allowed.**
It repaired a `vec.rs` line range in
`direction-underflow-reports-zero-length` that its own diff had shifted.
Ruling for this program: **yes, when the sibling is not dispatched** —
a citation the lane's own diff invalidated is the lane's to fix, and
`work/README.md`'s conflict hazard is about two parties editing one
item at once, not about touching a quiet row. A lane must not touch a
row that is `dispatched` or `review`, because that one has a lane.

### `boolean-kind-not-published-at-the-python-door` closed (PR 2350, 2026-09-11)

**The premise moved again — the sixth item of the day whose written
state was stale, and the most instructive of the six.** The item says
the fix is *"`boolean_error_tag` beside `path_error_tag`, over
`BooleanErrorKind`'s 41 variants, plus an accessor"*, and names two
reasons PR 1806 did not carry it, one of which is a vocabulary
decision this orchestrator then spent a paragraph deciding.

`boolean_error_tag` **already existed** — exhaustive over all 41 arms,
zero `_` arms, feeding `EvaluationError.inner_kind`, with all 41 words
already in the committed `TAG_INVENTORY`. The "41 FFI names is a
vocabulary decision" that both the item and this orchestrator treated
as open had been settled in the tree the whole time, and the decision
landed on was already the tree's.

What was actually missing is one word narrower and nothing in the item
names it: **the keying.** The map took the whole `BooleanError`, and
the evidence carries only the class — a finding is `Clone + PartialEq`
and `BooleanError` is neither. So the fix is a re-key onto
`BooleanErrorKind` (the `path_error_tag` shape), one map serving both
doors, plus the accessor. **No new FFI word is minted and the tag
inventory is unchanged byte for byte.**

Worth drawing the lesson sharply, because this orchestrator is the one
who got it wrong: **an item's "why this was not done" section is the
staleness-prone part, not its defect statement.** The defect was real
and still true; the *reasons* were a week old and one had expired. A
decision taken to unblock a reason that has expired is wasted work at
best, and at worst it lands a second answer beside an existing one.
The brief should have said "establish what exists before deciding what
to add", which is what it says for defect statements already.

**A design call the lane made and I endorse**: a new attribute
`boolean_variant` rather than overloading `CheckEvidence.inner_variant`.
The shell and boolean tag alphabets are not disjoint — `band` and
`escalated` are words in both — so one attribute carrying either would
be readable only after branching on `variant`, which is exactly the
property `check_payload` exists to preserve.

Measured rather than assumed, both directions: all 41 tags are the
mechanical snake_case of their variant names (zero deviations), and
`BooleanErrorKind` has no phantom variant (41 kinds, 41 distinct
projections).

### A tooling gotcha that has now cost two lanes a wrong answer

**`python3 scripts/work.py territory` compares `origin/main...HEAD`, so
it reports `0 paths` until you COMMIT.** A lane running it on a dirty
worktree gets a clean bill for a diff that crosses four fences. This
explains the `mate-member-vocabulary` lane's "0 paths in another
program's territory" earlier today, which became "1 path, tcost's" the
moment the work was committed. Every brief from here says: run it
after committing, never before.

### `census-flattens-the-typed-chart-region-declines` — in review (PR 2354, 2026-09-11)

**The item's own warning was half wrong, and finding that is the
unit.** It says *"a new variant OR a carried cause moves the
`AtRest`/`Uncertified` decision with it"*, and treats that as the
reason the fix is door-shape work. Measured:
`editor_core::assembly::attribute` keys on the `ValidationError`
**variant** and on the `CensusSubject` **shape** — `FacePair` →
`Declined`, every other entity kind → `Unattributed`. A **payload
participates in neither.** So the warning holds for a new variant,
which forces a hand classification through an exhaustive match, and is
**false for a carried cause**. That asymmetry is what chose shape 1,
and it is why this lands as a rendering change rather than an answer
change.

The invariance is right on the merits and not merely convenient: the
decline relation is *the census neither certified nor contradicted this
declaration*, which holds whichever lane declined. A budget exhaustion
leaves a mate exactly as unrefuted as a non-planar trim does, and that
is what `Uncertified` means. Pinned as
`the_decline_relation_does_not_depend_on_which_lane_declined`.

**The alternative design was rejected on a measurement, not a
preference.** Shape 2 splits "cannot decide this geometry" from "the
schedule stopped". The split is **not clean**: of twelve arms, exactly
**one** (`WitnessBudgetExhausted`) is "stopped looking".
`RayExhausted` — which this orchestrator's brief flagged as the likely
second, and told the lane to check rather than assume — is **not** one:
its schedule is fixed, no extra budget decides it, and the recourse is
ε or the geometry. `MissingCache` (a body-state fact) and `Corrupt` (a
kernel-invariant violation) fit **neither** bucket. A two-way split
would have filed four arms under a label that misnames them, which is
the `refusal-text-is-not-cause` defect this item exists to remove.
The item says eleven arms; there are twelve.

**A style review is running on it** — the only review of this wave.
Not because it moves an answer (the lane argues, with a row, that it
does not) but because it is the session's largest diff, it crosses four
fences, and its reach into LIB is a judgement call the lane itself
flagged as the half a reviewer would most likely want trimmed.

**Two siblings filed from its sweep:**

- `census-containment-flatten-fabricates-its-diagnostic` —
  `census.rs:819` flattens three `ContainError` arms onto
  `CensusEscalated` and **synthesizes** the `Indeterminate` it carries,
  so the message names a margin nothing measured. Worse than a flatten:
  a fabricated number in the field a reader uses to judge how close the
  call was. Re-routing those arms **can** move the answer, by the very
  variant/payload asymmetry this unit established — so it is a unit of
  its own, not a thread.
- `validate-drops-the-material-sign-refusal-silently` —
  `validate.rs:4099` folds `boundary_material_sign`'s `Err` into its
  `Unencoded` arm and raises nothing. A different class and arguably
  worse: a flattened refusal is a bad sentence about a real event, a
  dropped one leaves no record the event occurred, and the validator
  reports a clean pass over a question it could not answer.

**Disk, and a rule for this orchestrator.** The box hit **1.8 GB free**
while three lanes were live: this lane's `CARGO_TARGET_DIR` had reached
**19 GB** and was still there after its PR was open. Reclaiming a
finished lane is the orchestrator's job and "finished" means *the
report is in hand*, not *the PR is merged* — a review lane's target is
pure waste the moment it reports, and a lane's target is waste the
moment its PR is green.

### `two-d-director-doors-skip-the-finiteness-question` — in review (PR 2356, 2026-09-11)

The unit this program's slate was actually for. Five doors that decided
a length's SIGN without asking whether it was a finite NUMBER now ask
first and refuse typed, in the one spelling the homing unit made
reachable. CI green: 34 success, 4 skipped, all 12 `test (…)` points and
all 5 `k-lint (gate, …)` unifications.

**The item was wrong in both directions, and only executing it found
that.** It named five rows and claimed *"every remaining instance is
below"*:

- **A SIXTH site it never named.** `point_at(origin,(0,0,1),(1e200,0,0))`
  returned `Ok` with `c0=(0,-0,0)`, `c1=(0,0,-0)` — a second *silent*
  `Ok`, not a mis-named refusal, in a door the item listed only for its
  other argument.
- **Row 3 does not reproduce as filed.** The item argued both
  `sector_shape` arms collapse after a definite-positive decision.
  Measured: three of four shapes already *refused* — downstream, at
  rung 3, with the wrong cause and `COINCIDENCE_RECOURSE` attached —
  and the one genuinely silent shape needs one chord finite, one not,
  and `full_circle` set, because `min` hides ONE non-finite chord and
  rung 0 asks each chord separately. The defect is real; the mechanism
  the item describes is not the one that produces it.
- **Row 5 reproduced exactly as argued** and was executed here for the
  first time by anyone.

Three of the five "measured" rows in that table were argued rather than
run when filed, and two of those three were wrong. That is the
strongest instance yet of this program's standing pattern, and it
lands on the row that mattered most.

**One scope expansion, flagged by the lane rather than absorbed.**
`sector_shape` needed a refusal **channel**, not just an arm: it
returned `Result<_, Indeterminate>`, which can say nothing but a band.
It now returns `Result<_, SectorFault>`, with one new arm each on
`BooleanError`, `BooleanErrorKind` and `SplitReduceError`. That is the
same cost rows 4 and 5 were budgeted for (a new public arm, its
sentence, its kind row, its Python tag), arriving at a door the brief
did not expect to pay it. A refusal channel that can only say "band" is
itself a `refusal-text-is-not-cause` defect, so the widening is the fix
rather than a detour.

**A structural CI blind spot, found by going red.**
`crates/pncad-py/src/py/place.rs` matches `FrameError` exhaustively and
sits behind the non-default `python` feature, so
`cargo build/clippy --workspace` is **structurally blind to it**. A
lane that verified locally with the natural command would have pushed a
break. The reviewer is asked whether other exhaustive matches on the
changed enums hide behind non-default features.

**A correctness reviewer is on it** — the plan's own rule (a unit that
moves a kernel ANSWER rather than its rendering gets one), and the only
unit of this wave to need it. The one claim that decides whether the PR
is what it says it is: *only the order of questions changes*.

**Two more sites filed rather than taken**, on this program's slate as
`normalize-without-the-length-question-two-more-sites`: a sibling class
at `topo/src/chart_region.rs` where the norm is in the **denominator**
of the decided margin, so an infinite lever decides a spurious `Zero`
and that arm then normalizes — argued, not executed, and the reviewer
is asked to execute it.

**Disk: this lane could not run `doc-gate.sh` because the box hit 100%.**
That was this orchestrator's fault, not the lane's — a finished lane's
19 GB target was still sitting there. The lane pruned its own target and
never touched another's, which is the right behaviour, and said so.
