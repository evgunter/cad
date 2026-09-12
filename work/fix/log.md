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

### `two-d-director-doors-skip-the-finiteness-question` closed (PR 2356, 2026-09-11)

The unit this program's slate existed for, and the only one of the wave
to get a correctness reviewer. Five doors that decided a length's SIGN
without asking whether it was a finite NUMBER now ask first and refuse
typed. CI green on the fix-pass head (34 success, 4 skipped).

**Claim 1 — "only the order of questions changes" — survived a mutation
test.** The reviewer removed both the `definitely_positive` and the
`sector_shape` gates and saw exactly the new rows plus the three
re-pinned NaN rows go red, and nothing else. So the pins are
load-bearing and discriminate *refused for finiteness* from *refused*.
That evidence still stands over the fix-pass head: every
`is_finite_length` line the fix pass touched is a **comment**, and no
gate call site changed.

**The mechanism behind row 3, which nobody had.** The lane's original
account was wrong and the reviewer's correction was right, and the fix
pass found the reason both were half-blind: **`Real::min` propagates
`NaN` but not infinity.** `min(3, ∞) = 3` hides the overflow end and
lets rung 1 pass; `min(3, NaN) = NaN` stops the poison end there. That
single asymmetry generates the whole eight-shape table — two silent
`Ok`s with `full_circle` set (one of them returning a
*plausible-looking* non-zero bisector out of a collapsed chord, which
is the worse of the two because nothing downstream can smell it), rung
3 spikes with it clear, and the NaN shape refusing at rung 1 in both.
"`min` hides one chord" was true and silent about **which end**, and
that silence is exactly the gap the false rung attribution fell
through.

**m2 was acted on rather than commented.** The dead FFI word came from
a variant combination the type permitted and the kernel could not
build. The fix is a new `FrameVector` (four arms — the vectors that can
actually overflow) as the `NonFiniteLength` payload, with
`From<FrameVector> for FrameInput` for the shared `Degenerate` arm, so
the dead combination is **unrepresentable** rather than documented.
`FrameInput::ReferenceLadder` stays, and its docs now carry the
straddling-enclosure construction that makes it reachable at the
generic signature — replacing an orthogonality argument that only holds
at a point scalar.

**m5 bit harder than it read.** The two rewritten assertions were not
merely tautological: the NaN shape's rung-1 refusal is a `decide`
**escalation** and the spike is the body's own `invalid()`, and the two
**compare equal** — so the rewrite could not distinguish an escalation
from a spike refusal at all. Literal pins on `predicate`/`margin`/`band`
restored across the boundary, with that reason at the site.

**Declined, with reasons, and I endorse both**: no end-to-end row
through `boolean`/`split_reduce` (it needs a >1e154 orbit chord
surviving body construction) and no Python row for `carrier_tangent`
(it needs an arc anchor 1e200 from its centre, likely tripping an
earlier gate). The lane's sentence is the right standard: *I would not
add a Python row I cannot run locally and cannot confirm exercises the
door it names.* A row that passes for an unknown reason is worse than
no row.

**n5 declined and documented rather than silently kept**: binding the
norm once at `revolve/axis.rs` would swap `Margin::norm2` for
`Margin::of`, and which dimensional door a decided quantity comes
through is not cosmetic. Written at the site so nobody "fixes" it.

### `census-flattens-the-typed-chart-region-declines` closed (PR 2354, 2026-09-11)

Twelve typed chart-region refusals stop flattening onto one
`CensusUnsupported`; the variant carries the refusing lane's own
refusal and renders its sentence. The style review broke one of three
claims and the fix pass turned that break into the unit's best work.

**The recourse claim failed, and mechanising the repair found a ninth
arm.** The unit dropped a blanket recourse tail on the grounds that the
cause supplies one; **eight of twelve did not**, so a `TouchingBoundary`
decline rendered strictly less usefully than before. Two repairs were
available — restore a tail, or give the arms their recourse. The lane
took the second, because it makes `chart_region.rs:202`'s existing
claim (*"every arm names its recourse"*) TRUE rather than leaving an
unenforced invariant behind a compensating wrapper. It then wrote the
claim as a row, and **the row immediately found a ninth arm**
(`NonPlanarTrim`) that a careful reading by eye had missed.

The row is documented as a **floor, not a proof**: it is a
verb-vocabulary check over rendered literals and cannot tell a recourse
from a sentence with a verb in it. That is the right disclosure — and
the ninth arm is the argument for writing such a row anyway.

**The contact lane is now carried whole**, which resolved the style
review's taste finding along with its correctness one. The first draft
reduced `ContactRefusal` to a `&'static str` and re-prosed it, and the
new prose appended exactly the two-arm menu `contact.rs:195` says in
terms is a false lead. Carrying the refusal whole fixes both: the
ratified composition holds, and `CensusUnsupportedCause` stops being an
enum that exists to prevent flattening while flattening one of its own
lanes.

**A CI gate caught the withdrawal's consequence, and the gate's message
was the useful part.** Pulling the façade carry left
`CensusUnsupportedCause` an undecided payload rung under a curated
carrier, and `payload-rung sweep` refused it with two named outs:
carry it where its carrier is carried, or argue the non-carriage beside
that carrier. The lane took the second and **wrote the falsifier into
the argument** — a Python caller who must tell a stopped search from a
thin overlap gets the sentence and no word. An argument that names what
would refute it is the shape this repo keeps asking for and rarely
gets.

**Two judgement calls the lane flagged rather than buried**: it cited
the argument's home as the carrier's crate rather than the façade's
(the only such row in that table, and right — the argument is about
what a façade carry would OWE, so it belongs where whoever carries it
will be reading), and it registered `argued` rather than `filed`
because a pointer to an item that did not yet exist would be worse than
none. The item now exists —
`work/lib/census-decline-class-not-published-at-the-python-door`, PR
2357 — so that row can gain its pointer once both are on main.

**Class B, swept and filed**:
`validation-arms-delegate-a-recourse-their-carriers-do-not-give`. Four
more `ValidationError` arms make the assumption this unit broke, two of
them wrappers contributing four words (`"tier 3: {error}"`) over a
carrier that names no repair in 0 of 2 and 1 of 9 of its literals. Six
arms carrying `Indeterminate` are sound and are not on the list.

### `direction-underflow-reports-zero-length` closed (PR 2359, 2026-09-11), and two residues filed that the PR only disclosed

PR 2359 merged at 09:45 and **the item never left `review`** — the
board carried a merged unit as in-flight until this orchestrator pass.
Closed now. Worth naming rather than quietly fixing: the merge
commit's own predecessor is `work: record PR 2359 on the underflow
item`, so the lane wrote the header once, before merge, and nothing
wrote it after. A unit that records its PR at open and closes at merge
needs the second write to be somebody's, and on this program it is the
orchestrator's.

**Two residues were disclosed in the PR body and nowhere else**, which
is exactly the shape `work/README.md` legislates against — a residue
that lives in prose is invisible to the re-homing sweep and dies with
the directory. Both now have files:

- `underflow-gate-owed-at-five-more-doors` — the sweep's rows 2–6.
  Re-verified against this head (all five `is_finite_length` call
  sites still stand where the sweep put them), and narrowed to **four**
  arms, because row 5's content turned out to be the second residue
  rather than a missing arm.
- `path-error-numbers-below-1e-9-render-as-zero` — row 5, and the PR
  under-reported it. It filed the finding as *"the `num()` formatter
  prints `1e-180` as `0`"*, which reads as an underflow symptom. It is
  not. Executing `num`'s exact body over a range shows the threshold
  is a hard-coded absolute `1e-9` (`tol = 1e-9 * x.abs().max(1.0)`,
  where the `max` pins it for every `|x| <= 1`), so **`1e-12` and
  `1e-30` render as `0` just as `1e-180` does, and `-1e-30` renders as
  `-0`**. The kernel's unit is the metre, `num` is applied to `margin`
  and `arm`, and a picometre margin is precisely the quantity a
  tangency refusal exists to report. Three `Display` impls, 38 call
  sites.

The general lesson, since this is the second time this program has
found it: **a lane's disposition column is a hypothesis about a site
it did not take.** "Not this unit" was right in all five rows; the
one-line characterisation attached to the row that got the most
scrutiny was still wrong, in the direction of making the defect look
narrower and more exotic than it is. The residue rows are worth
re-executing at filing time, not transcribing.

### `collapsed-string-continuations-ship-space-runs-to-users` closed (PR 2364, 2026-09-11)

Twenty-seven rendered messages stop carrying a mid-sentence run of
spaces. Every repair is one backslash; CI green on the full matrix
(38 checks, twelve `test (…)`, five `k-lint (gate, …)`, nothing
narrowed — verified against the check-runs API, not from the lane's
report).

**The item's own list was a third wrong in both directions**, which is
the second time this program has caught that in one day. It named ten
`src` sites; its own pattern returns 27 at the merge base, the line
numbers had drifted, nine of its hits are **not the defect** (column
alignment in printed tables, indentation after an explicit `\n`,
Rust-source fixtures where the indent is the thing being matched), and
nine genuine sites its pattern **cannot see at all** — three because
the run follows an em dash or a colon rather than the
`[a-z,.;)]` the pattern requires, three because they are in
`demos/tour/`, outside the `crates/` it searched.

**The worst site was not on the list and is not an assertion.**
`crates/viewer/src/pane/create.rs:1133,1140` are
`on_disabled_hover_text` and `on_hover_text` — text a GUI user reads
with nothing wrong, carrying an 18-to-22-space gap.
`editor-core/src/refactor.rs:304` is a `Display for SplitError`,
shipped on an ordinary refusal.

**The guard question is now answerable, and both of the closed row's
guesses about it were wrong.** It routed the guard to CIW and proposed
a threshold of four spaces.

- **Not CIW's.** `work/ciw/program.md`'s `keep_out` says
  `scripts/gates/*` is code-quality Track K's. And the grep option is
  wrong regardless: the needle is *inside* string literals and
  `gate_rust_code` builds the code-only view — the obstacle PR 1809
  already routed around by writing its census in Rust. That census is
  `crates/pncad-py/src/prose_census.rs` and it is the home.
- **Four is inside the noise.** The lane classified every hit by hand
  and measured both directions: at run ≥3 the pattern is 67% false, at
  ≥4 it is 37% false, at ≥9 it is 7%. Every one of the 27 real sites
  carries a run of **≥10** — a collapsed continuation swallows a whole
  source indent and the shallowest here is ten columns — while
  deliberate alignment clusters at 3 to 8. The closed row's threshold
  sat inside the alignment cluster, which is exactly why its own hit
  list was a third false.

Filed as `collapsed-continuation-guard-belongs-in-the-prose-census`
with the spec, the table, and the two things the lane refused to
overclaim: that the final `= : -> |` exclusion is fitted to 43 hits
and may not generalise, and that the source-text fixtures fall out
under the run floor **by luck rather than by design** and will not
keep doing so.

**A measured false-positive rate is what a guard proposal owes and
almost never carries.** The closed row asked for a gate and supplied a
regex; what made the question decidable was a pass that ran the regex,
looked at every hit, and reported where the classes actually separate.
Worth asking for by default the next time a row proposes a mechanical
guard.

### `validate-drops-the-material-sign-refusal-silently` closed (PR 2365, 2026-09-11) — the item was wrong, and the lane was right to refuse the fix it asked for

The row said `validate.rs`'s `Ok(Unencoded) | Err(_) => {}` was *"a
refusal examined and discarded"*, *"arguably worse"* than the flatten
class, and *"the shape A5's letter exists to forbid"*. I dispatched it
question-first — establish reachability, and read A5 rather than the
row's summary of it. Both instructions earned their place.

**`Err(_)` IS reachable** — executed, not argued. The lane swapped the
arm for a push and ran `sweep`'s
`f4_an_oblique_trihedron_builds_and_reports_volume_uncomputable`: five
faces refuse (`FaceKey(1v5)`, `(11v9)`, `(14v7)`, `(17v3)`,
`(18v15)`). So the arm is not deletable.

**A5 is not about this.** A5 is `crates/editor-core/ASSEMBLY.md:164`,
*"the at-rest gate"* — `assembly::assemble`, product gathering,
`MintedDeclaration`/`RefusedRef`. It governs `editor-core`, not tier
3's check 6, and forbids nothing here. It arguably cuts the other way:
A5 has a *third* verdict (`Uncertified`) precisely so "not decided"
need not be forced into the refusal channel.

**And the raise the row asked for would have been a real regression —
which the tree already documents at the site.**
`crates/geom-brep/src/props/curved.rs:1174` says in terms:
*"[`boundary_material_sign`]'s callers must treat an error as exempt
(the check-7 posture)"*, and then names the exact failure: *"tier 3's
curved check 6 raises a `CurvedSenseInverted` from the wrong ±1 and,
check 7 being gated on `errors.is_empty()`, SUPPRESSES the honest
`NotIsoRectangle` the flux lane raises on the same face."* The
mutation run reproduced that sentence exactly — with the raise in,
five `CurvedSenseInverted`s were the whole error vector and the
`VolumeUncomputable` was gone. **The row proposed introducing a bug
the code had already written down.**

**So what landed is legibility plus a real pin.** The arm splits in
two, each stating its own invariant — an ANSWER of "no side encoded"
and an EXEMPT refusal are different events. No behaviour change.

The durable half is the test. `m5_pr12_fix_pass.rs`'s one witness
asserted through `format!("{errs:?}").contains("VolumeUncomputable")
&& contains("NotIsoRectangle")` — a substring match over a Debug dump,
which passes on any output carrying those words anywhere. It is now
structural and pins **both halves including the negative**: no
`CurvedSenseInverted`, AND a `VolumeUncomputable` carrying
`NotIsoRectangle`. The exemption was load-bearing and unpinned; now
turning it into a raise goes red.

### The pattern under three items in one day

`direction-underflow`'s residue characterisation, the
collapsed-continuation site list, and now this row's A5 claim. Each
was filed by a competent lane from a real sweep; each got the *shape*
right and the *characterisation* wrong, in the direction of making the
defect sound sharper than it was. The row that claimed the strongest
authority — a named design clause — was the one that had not read it.

The cheap defence is already in the reviewer brief and is worth
promoting to how this program dispatches: **the dispatch is a
hypothesis.** A row citing a clause gets "read the clause, not the
row's summary" in its brief; a row citing a site list gets "re-derive
it". Both instructions paid for themselves today, and neither costs a
lane more than minutes.

**Flagged, not actioned:** open PR **#2339** (PERF-6) also edits
`validate.rs`, at or before ~line 2900 against this diff's ~4163. No
textual conflict; whoever lands second should look.

### Three more closed (PRs 2366, 2367, 2368, 2026-09-11)

**`path-error-numbers-below-1e-9-render-as-zero` (PR 2366).** `num`'s
absolute floor is gone: `tol = 1e-9 * x.abs()`, so the shortening is
the same proposition at every magnitude. The lane went one step past
the item and was right to. Fixing only the floor renders `1e-12` as
`0.000000000001` — eleven zeros mid-sentence — so notation now follows
the `Debug` form's own choice, fixed where `{:?}` is fixed and
exponential where it is exponential. **That is not taste: the repo had
already ruled on it in the other direction.**
`crates/sweep/src/blend/mod.rs:276` records E3 review item 3, where one
side printing `0.000000001` against a sibling's `1e-9` was the
divergence that got fixed, **toward** `{:e}`. I checked the citation
before accepting the widening. It also repairs a latent sibling:
`num(1e300)` was 301 digits of refusal sentence. No pin moved, and the
lane grepped for long-zero spellings to establish that rather than
inferring it from a green run.

**`debug-in-prose-at-blend-and-step-import` (PR 2367) — the live
panic is closed.** Red-on-base proved in both directions against the
real `reads_as_prose`, not a copy. The sentence was not invented:
`prose_census.rs:2018` had already written down the target string for
this exact site, and the lane took it. `EdgeKey`/`VertexKey` turn out
to be **tuple**-shaped (`slotmap::new_key_type!`), so every sibling arm
rendering `{edge:?}` directly is safe — this arm was the only live one
in the file because it is the only one whose payload is an enum with
struct variants.

**`verb-error-arity-renders-verbkind-through-debug` (PR 2368), and the
item was wrong about the defect.** It called `VerbKind` and `Arity`
"both fieldless". `VerbKind` is not: it carries `Boolean(BooleanOp)`.
So `{verb:?}` was writing **`Boolean(Subtract)`** into a user's
sentence — the enum's internal coordinate, naming a `Boolean` door
that does not exist in the kernel. Those three rows now say `Union` /
`Intersect` / `Subtract`, which are the production doors. This was a
wrong word reaching a user, not the style finding the row described.

### The class nobody filed: rosters and pins that sample the safe case

Two lanes found this independently today, in different crates, and it
is worth more than either instance.

- **PR 2367:** `recourse_tests::seeds()` sampled `Escalated` with
  `BlendSite::Chain` — the **sole brace-free arm of three**. What
  decides the rendering is the payload's variant, one level below the
  variant the roster enumerates, so a roster that looked exhaustive
  tested only the case that cannot fail. Both rosters in the fence had
  it; both now seed all three sites.
- **PR 2368:** `crates/verbs/tests/run_door.rs`'s byte pin covered six
  sentences, every one a fieldless verb through a fieldless door —
  **the one axis where `Display` and `Debug` agree exactly**. It could
  not tell which trait had run. It grew a row that can.

This is the reviewer brief's Q3 (*"a premise that excludes the failing
mode"*) showing up twice in one wave, and in both cases the roster was
*complete at its own level* and blind one level down. Neither would
have been found by asking "is this tested" — only by asking "what
input makes this row go red".

**Standing instruction for this program's briefs, from here:** when a
unit changes how a value is RENDERED, the lane owes an answer to *does
any existing pin discriminate the old rendering from the new one* —
and if the honest answer is no, the pin is part of the defect, not
part of the baseline.

### The characterisation drift, now at four of five

Every unit in this wave but one had an item whose *shape* was right
and whose *characterisation* was wrong, always making the defect
sound narrower or more exotic than it was: the underflow residue's
`num()` line, the collapsed-continuation site list, the validate row's
A5 citation, and the verbkind row's "both fieldless". These rows are
filed by competent lanes from real sweeps. The failure is not care;
it is that a disposition line about a site you did **not** take is
written from a reading, and nothing re-reads it before a lane builds
on it.

The two brief instructions that caught all four cost minutes each and
are now standing for this program: **a row citing a clause gets "read
the clause, not the row's summary"; a row citing a site list gets
"re-derive it at your merge base".**

## Second orchestrator handoff, 2026-09-11 — the review posture drops, and the slate is triaged whole

### Ev's instruction, and what it changes

**Light style reviews, or none at all; no A/B protocol** (Ev, in-chat,
2026-09-11). The A/B half was already the standing posture and is
unchanged. What moves is the style lane: `plan.md` promised every unit
one, and from here a unit gets one only if the orchestrator asks for it
off the back of the diff.

The three units `plan.md` named for a second correctness-focused
reviewer — `transform-rigid-refuses-described-nurbs`, the two census
declines, `split-crossings-skip-pattern-mate-ends` — are **all
closed**, so nothing on the live slate invokes that clause and dropping
it costs nothing that was going to be spent.

**The discipline does not vanish; it moves into the brief.** A lane
with no reviewer downstream gets the three instructions this log
earned in the 2026-09-11 wave, as standing text in every dispatch:

1. a row citing a clause gets *"read the clause, not the row's summary"*;
2. a row citing a site list gets *"re-derive it at your merge base"*;
3. a unit changing a RENDERING owes *"does any existing pin
   discriminate the old rendering from the new one?"* — where **no**
   means the pin is part of the defect.

And one addition that is this posture's own price: **a lane running
without a reviewer states every place it was unsure, explicitly, in
the PR body and its report.** Less review bought with more disclosed
uncertainty is a trade; less review bought with smoother prose is not.

### The board, read whole: nothing is waiting on Ev

Asked directly, and the answer is clean. **Zero `needs_ev: true` in
`work/fix/`** — the flag appears nowhere in this program's directory —
and no open `[ev]` PR belongs to FIX (2363 is the fuzzing policy, 2135
is BOOL-10, 1700 is M10's draft walk). `plan.md`'s one named ruling,
`nested-pattern-mate-heads-refuse`, closed.

Nineteen items are open and **every open decision on them is an
agent's, not Ev's.** Read one by one, they fall in four groups:

- **Dispatchable as written** (fix in the body, no decision):
  `cert-check-renders-through-debug`,
  `band-helper-duplicated-across-suites` (free half),
  `pair-subject-witness-strings-unswept`,
  `circle-constructors-are-literal-only`,
  `coherence-findings-have-no-consumer`,
  `error-census-keyed-on-bare-type-name`,
  `collapsed-continuation-guard-belongs-in-the-prose-census`,
  `underflow-gate-owed-at-five-more-doors`,
  `validation-arms-delegate-a-recourse-their-carriers-do-not-give`,
  `mate-clocking-has-no-gui-path` half (1).
- **Dispatchable, decision belongs to the lane** — the row names the
  fork and the lane resolves it at the site:
  `node-error-kind-has-no-fieldless-projection` (fieldless mirror, or
  carry `NodeErrorKind` itself),
  `levered-clash-margins-hide-their-arm` (a typed unit, a second arm
  in the sentence, or a small-angle argument — but see the re-home
  below before dispatching it),
  `census-containment-flatten-fabricates-its-diagnostic`,
  `normalize-without-the-length-question-two-more-sites`.
- **Question-first** — the output is an answer, and the diff is
  downstream of it: `unify-discipline-machinery-onto-registry`
  (dispatched today as a reading pass),
  `circle-constructors-are-literal-only` (where a parametric author's
  dimensions get checked, before the doc sentence),
  `transform-recertifies-through-the-narrow-lane`.
- **Two rows sharing one door question, and it is a lane's to settle:**
  `band-derivation-has-a-scalar-twin` and
  `band-helper-duplicated-across-suites` — does the
  `(zero, escalate)` pair get a named door, and on what? The free
  `crates/sweep` half was cut away from that question today
  precisely so a mechanical change is not held behind it.

**The three worth naming to Ev anyway, none of them blocking**, on the
ground that they add or change PUBLIC surface rather than rendering:
`transform-recertifies-through-the-narrow-lane` (a public generic
signature change on a kernel door — `transform_rigid` would take a
`T: Decide + CertifiedBounds` bound); `kind-mirrors-have-no-single-
declaration` (an `error_kinds!` macro generating four public error/kind
pairs tree-wide); and the band pair's door, if it lands as new
vocabulary on `Tolerance`. The kind-mirror row already pre-cleared the
nearest precedent itself — `scripts/gates/README.md:49` records Ev
rejecting a PROC-macro for the CI gates, an objection about an opt-in
rule that must hold everywhere, which does not reach a `macro_rules!`
that generates a declaration at the one site declaring the type.

### Wave dispatched, 2026-09-11

Five lanes, file-disjoint by construction so the merges do not race:

| unit | branch | ground |
|---|---|---|
| `cert-check-renders-through-debug` | `fix/certcheck-display` | `geom-brep/src/certify.rs` |
| `band-helper-duplicated-across-suites` (sweep half) | `fix/sweep-band-helper` | `crates/sweep/tests/*` |
| `pair-subject-witness-strings-unswept` | `fix/census-pair-order` | `topo/src/census.rs` |
| `circle-constructors-are-literal-only` | `fix/circle-parametric-door` | `editor-core/src/program.rs` |
| `unify-discipline-machinery-onto-registry` (reading pass) | `fix/unify-discipline-triage` | tracker + docs only |

Every lane is told to update **only its own item file** — not this log
and not `plan.md` — because five concurrent lanes editing one narrative
file is five merge conflicts, and one-file-one-item exists to make that
visible rather than to make it happen.

### Two mate rows are on ground FIX no longer has a clean claim to

Held back from this wave deliberately, and this is the finding the
triage turned up that is worth more than the triage.
`levered-clash-margins-hide-their-arm` says it *"needs S-MATE's assent
or a re-home"*. **S-MATE left the tracker on 2026-09-04**
(`docs/DOC-LEDGER.md` sweep 6), which reads at first like the
blocker evaporating. It is the opposite: its territory did not go
unowned, it was **inherited by two open programs**. Both
`work/docm/program.md` and `work/msolve/program.md` now carry
`crates/editor-core/src/mate/*` in `paths`. So the assent is still
owed — to DOCM and MSOLVE rather than to a program that no longer
exists — and a row that reads "the owner is gone, take it" would have
walked a FIX lane into two live fences.

**And MSOLVE is not merely the owner, it is working this exact
subject right now.** Open PR **#2116, "MSOLVE-6: the mate's lever is
the mated parts' own extent."** This row is about three levered
mate-fold clash margins reaching the refusal with their arm invisible.
Same file family, same quantity. Dispatching it here would have raced
a live PR on the lever it is about.

**Disposition: re-home `levered-clash-margins-hide-their-arm` to
MSOLVE** (a header edit and a `git mv`, per `work/README.md` —
the file MOVES, keeping its id, and never gets copied), once #2116
lands or MSOLVE says how it wants the two sequenced.
`mate-clocking-has-no-gui-path` sits the same way: its own `## Home`
section still names `work/mate/`, a directory that is gone, and
`plan.md` holds half (1) here on a fence that has since been claimed
twice. Both go to MSOLVE/DOCM before any lane sees them.

This is the "fired trigger is not a blocker" shape one level up from
what lint checks: the row named a blocker that has since closed, and
the closure did not free the row — it substituted two new owners for
one. Nothing mechanical could have caught it, because the row's
blocker was prose.

**The band row's counts were already stale at dispatch and the brief
says so.** The item claims 24 `crates/sweep` copies and 36 overall; a
shape-only grep on this head finds 30 in sweep and 60 across six
crates. That is instruction 2 firing before a lane even started, which
is the argument for keeping it standing.


### Wave results, 2026-09-11 — three landed, and all three closed by refuting their own row

PRs **2373**, **2372**, **2374**. The striking thing is not that they
landed; it is that **not one of them did what its item asked for**, and
in each case the row was wrong in the same direction.

**`unify-discipline-machinery-onto-registry` (PR 2373) — step 1 had
already shipped.** The brief offered two arms, spec or `parked`, and
the answer was neither: the sink is `crates/editor-core/src/finding.rs`,
whose own header reads *"The document layer's finding sink
(DISCIPLINES-DESIGN DS8; **#981 part 1**)"* — `#981` being this row's
own `github:` number. Three consumers wired, not the two DS8 needs:
`CheckFinding` (`checks.rs:381`), `UndeclaredContactFinding`
(`eval/mod.rs:1376`), `AtRestFinding` (`assembly.rs:450`, which the
row never named). It landed as PR **#984**, on 2026-08-25, *before
this row was ever homed here*. The row has sat on the slate since the
program opened, held as the one item whose fix was not written in its
body, while the thing it was tracking was already in the tree.

**Here instruction 1 fired in the direction nobody expected.** The
four previous instances were rows that misread a clause. DS8 said
exactly what this row claimed. What was four months stale was the
row's **premise about the tree** — and no amount of reading the clause
would have caught that. Only reading the code did. The instruction
wants a second half: *read the clause, AND check the row's claim about
the tree is still true.*

**`pair-subject-witness-strings-unswept` (PR 2372) — the cited
settlement says the opposite.** The row reads PR 1750's unordered-pair
settlement as reaching the rendering. `validate.rs:317-344` says, in
the same paragraph: `FacePair` is *"the candidate face pair, **in the
arm's own order**"*, and the hand-written `PartialEq`'s comment is
*"the order is **kept in the value** (the arm's own, and what `Debug`
prints) and **dropped from the comparison**."* Unordered for equality,
ordered in what it prints. `CensusSubject`'s own ratified `Display`
prints the arm's order for that reason — so the row's complaint, if
sound, would condemn the impl the same PR wrote. The row's stated harm
(*"two runs that differ only in arena order"*) is not a state D9
admits at all (`docs/DESIGN.md:797`).

So both pair sites already showed the right order and now say why.
**And the lane found the real defect underneath**: `UndeclaredContact`'s
`witness` is documented *"a debug rendering of the witnessing
**position**"* and both `Display` arms put it after *at* — while at
`:1677` and `:2830` that locative slot holds a **repeat of the subject
the same sentence already names**, at `:2830` verbatim, the same two
keys through the same `{:?}` two clauses apart. Filed as
`census-witness-string-repeats-the-subject`, not fixed, because
`:1677` has no position to give without changing CURVED's predicate
and `witness` is `String` rather than `Option<String>`, so "no
position" has no spelling.

**`circle-constructors-are-literal-only` (PR 2374) — and the wrong
characterisation this time was the ORCHESTRATOR's.** The disposition
written on this row on 2026-09-11 asserted the constructors *"are the
**dimension-checking** door, and they return `DimensionError`"*, and
made the whole unit turn on it. **It is false.** `len_lit`/`ang_lit`
are `Expr::literal(v, dim)`, whose only refusals are
`LiteralCountIsInteger` — unreachable at a fixed Length or Angle —
and `NonFiniteLiteral` (`crates/editor-core/src/expr.rs:605-619`).
The only error `circle` can return is non-finiteness, which its
`# Errors` section already said. Dimension-wise the constructor does
not CHECK, it **PICKS**.

The disposition survived anyway, for a better reason than it gave: the
lane executed the question instead of arguing it, and found the check
at the **document** door — `check_node_slots`
(`crates/editor-core/src/edit.rs:1314`, comparison at `:1325`),
refusing `EditError::SlotDimensionMismatch` before a program enters
the document, on insert, on every slot write, and on a parameter
redeclaration. A probe run and reverted covered the two shapes the
existing pin does not.

### The characterisation drift is not a lane-quality problem

The log has now recorded this five times in two days, and **the fifth
was written by the orchestrator, in the act of dispatching a brief
warning lanes about the first four.** That settles what the pattern
is. It is not carelessness and it is not a lane defect: it is what
happens whenever anyone writes a confident sentence about a site they
have read rather than run. The dispatching seat has no immunity — it
has *more* exposure, because an orchestrator disposition arrives at a
lane carrying more authority than the row it corrects.

**Standing, from here:** an orchestrator disposition that asserts what
code DOES is subject to its own instruction. Either execute it before
writing it, or mark it as unverified so the lane knows to check it
rather than build on it. Three of this wave's five briefs told a lane
to execute before fixing; the one that made a claim of its own did not
hold itself to it.

### Two fence corrections landed

- **`eval/wire.rs` is WIRE's** as of WIRE opening 2026-09-11. This
  program's `keep_out` still described it as unowned, and the brief
  repeated that to a lane. Corrected.
- **The `census.rs` seam clause was an enumeration** — "the two
  typed-decline items edit it by recorded seam" — and FIX has now
  crossed that file with a third row and filed a fourth. Rewritten as
  a clause about the FILE, noting that CURVED's `keep_out` does not
  name FIX, so the record is one-sided and the crossing is invisible
  from CURVED's side. `census-witness-string-repeats-the-subject`'s
  fix reaches CURVED's chart-region predicate and wants CURVED's
  assent rather than announcement.

### A harness hazard, for whoever dispatches the next wave

Five concurrent lanes shared one scratchpad directory and collided on
filenames: `poll.py` and `pr.md` were overwritten mid-task, with four
concurrent `poll.py` processes running under three different argv.
Nothing was lost this time. A lane that writes a PR body to a shared
`pr.md` and posts it a minute later posts a sibling's text. **Give
each lane its own scratchpad subdirectory in the brief.** Not filed —
no program owns it and it is a harness convention, not a repo defect.

### `band-helper-duplicated-across-suites`, sweep half (PR 2377) — and the row's counts were wrong in both directions

**39 wrappers removed, 43 `use` sites now reaching
`crates/sweep/tests/common/approx.rs`, 47 files.** The item stays
`open`: the sweep half was cut away from the shared-home decision
precisely so a mechanical change would not wait on a design question,
and the decision is still undispatched.

**The re-derivation instruction earned its place again, and this time
against the orchestrator's own number too.** The item said 24 sweep
copies and 36 across three crates. The dispatch brief corrected that
to 30 sweep and 60 across six, from a shape-only grep. The lane, with
a grep that actually filters on the body, found **42 declarations in
sweep** and a remaining population of **22 `tests/` sites across six
crates** — `topo` 9, `geom-core` 7, `geom-brep` 3, `editor-core` 1,
`mesh` 1, `step-import` 1. The row missed three crates entirely and
undercounted `geom-core` sevenfold. Three successive counts, each
closer, none right until someone ran the right pattern.

**A second population the row has never scoped**: 20 more copies in
`crates/*/src` `#[cfg(test)] mod tests` blocks (`topo` 13,
`geom-brep` 5, `geom-core` 2). These **cannot reach a `tests/` helper
tree at all**, so the shared-home question as this row and its sibling
frame it does not cover them. That is a real re-shaping of what the
band decision has to answer, and it is now in the item.

**Four things the dispatch did not predict**, all of which the lane
handled and any of which could have reddened CI:

1. **`Band::new(tol.eps(), tol.k() * tol.eps())` IS `Band::linear(tol)`** —
   verified through `linear` → `from_zero_threshold(tol, tol.eps())` →
   `from_thresholds(tol.eps(), tol.k())` → `new(zero, k*zero)`
   (`crates/geom-core/src/predicate.rs:364-425`). Two copies spelled
   that way were collapsed. Outside the brief's literal filter, and the
   lane flagged it as its main judgement call; it is the same class, a
   spelling of one derivation, which is what the sibling row
   `band-linear-spelling-not-swept` was about.
2. **Three copies lived in helper modules, not suites** —
   `tests/common/cone_nappe.rs` held a second `band()` *inside the
   shared tree itself*, and `tests/shell8_common.rs` a third that six
   suites imported.
3. **Deleting a wrapper orphans its imports — 30 files**, and **the
   default feature lane cannot see them all**: three surface only under
   `--features interval`. A lane checking one lane would have pushed
   red. This is the twelve-job matrix doing exactly what it is for.
4. **The dead-wrapper re-check found one, not six**, and **the item's
   premise for it is false on this tree**: no sweep suite and not
   `all.rs` carries `#![allow(dead_code)]` — only the helper trees do.

**And the lane found the convention the whole class had been breaking.**
`common/mod.rs` requires a suite that keeps its own copy of something
the `common` tree holds to say so AT the copy, carrying the literal
``NOT `common::``. No band copy anywhere carried one. The single
deliberate survivor — `m9_2_chart_region_loft.rs`, a FIXED 1e-9/1e-8
band rather than the run's — now does.

### Filed from that sweep: `fixed-band-literals-are-an-unscoped-class`

55 sites decide against a hard-coded `Band::new(1e-9, 1e-8)` rather
than the run's band, and exactly one of them says why. At eps = 1e-6
and eps = 1e-12 a fixed band asks a different question from the run's,
so the class is worth a reading pass; a wrong collapse changes what a
row decides against, which is never free.

**One framing dropped, because it was checked and is false.** The lane
suggested these collide with the suites whose header declares *"ε
posture: no ε literal"*. Measured before filing: **the two populations
are disjoint** — no file carrying that header carries a fixed band. The
finding survives weaker than first stated, and the item says so, because
the stronger version is the more attractive one and would send a taker
hunting a contradiction that is not in the tree.

That check is the wave's fourth instance of the same thing: a real
finding whose stated mechanism does not hold. The lane disclosed its
own uncertainty on exactly this point ("did not verify intent rather
than inheritance"), which is what the no-review posture asks for and
what made the check cheap to run.

### `cert-check-renders-through-debug` (PR 2379) — the wave's one real rendering change, and the one that found its own pins

`CertCheck` now says its own words through a `Display` on its
declaring row, exhaustive and wildcard-free, and `CertifyError`'s
three arms forward. 21 variants, arms re-derived at **370, 412, 420**
(the item said 369 for the first).

**The decision went to the PHRASE, not the identifier** — `the
out-of-halfplane component`, not `SeamHalfplane` — and the argument
that settled it is one the item did not have. The item reasoned from
"these are not doors anyone CALLS", which is true and is why
PR 2368 went the other way for `VerbKind`. The lane found something
stronger in the file itself: **every other arm of that same `Display`
is English prose, and one of them already names a member of this very
taxonomy in prose** — `NotSecondOrderSeparated` writes *"the
tangency's second-order margin (relative transverse normal curvature,
tangent_second_order)"*, which is `CertCheck::TangentSecondOrder`
(`certify.rs:386`). The file had already decided; the three
`{check:?}` arms were the ones out of step. Checked before merging.

**And the hardcoded noun was measurably wrong.** `ResidualExceeded`
wrote *"{check} **residual** at sample …"* for all fifteen checks that
reach it, and **five of those meter no residual** — two sup bounds, a
parallelism defect, a component, an excess. So the word carries the
KIND of quantity and the sentence keeps only the grammar. That is a
wrong word reaching a user, the same shape PR 2368 found under a row
filed as a style finding, and it was invisible to the item.

**Instruction 3 did the most work here of anywhere in the wave.**
Four existing pins carried the rendered word, all `to_string()`
(Display), all re-baselined, all discriminating. **Two of the four
were on no list and the lane's own first grep missed them too** —
`SEAM_HALFPLANE_ESCALATED` matches neither "residual at sample" nor
"not a sampled check". What found them was sweeping all 21
identifiers inside string literals, and the blind spot is stated
rather than buried. Two further pins read `Debug`
(`poleguard.rs:143`, `recognize_pins.rs:344`, both
`format!("{e:?}")`) and correctly stay green: they pin the
coordinate, which this unit deliberately leaves alone. **Telling those
two classes apart is the whole of instruction 3** — had they been
Display pins they would have been part of the defect.

Three new rows in `certify.rs`'s own module so the unit does not lean
on a downstream corpus, each proven red-capable **by mutation rather
than by argument**. One of them asserts that no check's phrase IS its
identifier — which is what makes every row able to tell `Display` from
`Debug`, and is the direct answer to PR 2368's weak-pin lesson.

**A judgement call the lane flagged for overruling, and I checked it
rather than accepting it.** `check_residual` takes both a
`&'static str` predicate name and a `CertCheck`, hand-paired at 18
call sites — the shape of two vocabularies that drift. The lane
judged it not a defect because the pairing is deliberately
many-to-one, and filed nothing. Verified: `CertCheck::Surface1Residual`
is reached as `carrier_on_surface_1` (`certify.rs:1681`) and as
`tangent_on_surface_1` (`:1723`), one per certification lane. Two
vocabularies serving different purposes — a funnel name against a
taxonomy coordinate — not one restated twice. The judgement stands and
no file is owed.

### Wave closed: five dispatched, five merged, and what the slate looks like

PRs **2372, 2373, 2374, 2377, 2379**. Slate **19 → 17 open** (four
closed, two new rows filed: `census-witness-string-repeats-the-subject`
and `fixed-band-literals-are-an-unscoped-class`), **35 closed**.

**Four of five units closed by refuting their own row**, and the fifth
(`band-helper`) closed its half only after correcting counts that were
wrong in three successive tellings. Every one of these rows was filed
by a competent lane from a real sweep. The defects were real in all
five; the *characterisations* were wrong in four, always in the
direction of sounding sharper or narrower than the truth.

**What the no-review posture actually cost and bought.** Nothing was
caught late, and nothing merged that a reviewer would have stopped —
but that is not because the risk was absent. It is because every
brief carried "execute, do not read" and every lane reported its own
uncertainty, which is what let the orchestrator check the four
load-bearing claims cheaply before each merge (the `Band::linear`
equivalence chain, the `CensusSubject` settlement, `Expr::literal`'s
refusals, the many-to-one `check_residual` pairing). **The posture
works on the condition that the disclosed-uncertainty half is
actually honoured.** A lane that smooths over its doubts under this
posture ships unreviewed and unexamined at once.

## Wave 2, 2026-09-12

### The two mate rows are re-homed, and the stale line was a trap rather than a leftover

`levered-clash-margins-hide-their-arm` → **`work/msolve/`**;
`mate-clocking-has-no-gui-path` → **`work/docm/`**. Files MOVED with
their ids per `work/README.md`; FIX changed no code and took nothing.

The first row said it *"needs S-MATE's assent or a re-home"*, and
S-MATE left the tracker on 2026-09-04. Read quickly that is a blocker
evaporating. It is the opposite: **the territory was inherited, not
freed** — `work/msolve/program.md` and `work/docm/program.md` both
carry `crates/editor-core/src/mate/*` in `paths`. The assent was still
owed, to a live owner instead of a dead one, and a lane reading the
line as "the owner is gone, take it" would have walked into two live
fences. That is the `fired trigger is not a blocker` shape one level
above what lint checks: the blocker closed and the closure did not
free the row, it substituted two owners for one. Nothing mechanical
catches it, because the blocker was prose.

The split follows the charters as written. MSOLVE is *"assembly
SEMANTICS rather than document custody: … one refusal that reports a
false cause"* — a clash margin reaching the user with its arm
invisible is exactly that. DOCM is *"the persisted recipe vocabulary,
the `DocEdit` set …"* and already held half (2) of the clocking row at
`work/docm/plan.md:94-96`; an `AddMate` door refusing at authoring
time is the `DocEdit` set, so both halves go there rather than one
finding living on two slates. DOCM's charter is to rule and **hand the
build to FIX, CHROME or VIEW** — so if half (1) rules "refuse typed at
the door", it comes back here and this program takes it.

**And MSOLVE is not merely the owner of its row, it is working that
subject now**: open PR **#2116, "MSOLVE-6: the mate's lever is the
mated parts' own extent"**, against a row about three levered margins
whose arm the refusal drops. Sequenced against #2116, not beside it.

### A row arrived on this slate overnight, and it is FIX's own doing

`num-relative-tolerance-collides-above-a-decimetre`, filed by the DOOR
orchestrator — correctly onto this slate rather than carried on
theirs, since `path::num` is FIX's file and FIX closed two rows on
that helper on 2026-09-11.

**It is the opposite end of the range from the one FIX just closed.**
PR 2366 removed a `.max(1.0)` that pinned the tolerance ABSOLUTE at
1e-9 below a metre, rendering every sub-nanometre margin as `0` — the
exact margins those messages exist to report. The purely relative form
that replaced it is right at the small end. It is wrong at the large
end: verified at dispatch, `crates/profile/src/path.rs:1416` reads
`let tol = 1e-9 * x.abs();` and `DEFAULT_EPS` is `1e-9`
(`crates/geom-core/src/tolerance.rs:84`), so the tolerance crosses ε
at `|x| = 1` m and is a thousand ε at a kilometre. **Two lengths the
kernel can certify as different render as one number.**

The unit is dispatched with the trap named: the reviewer brief's
standing warning is that a lane closing a structural finding mints a
fresh instance of the defect it closes, and a fix here that
reintroduces a floor re-mints 2366's defect exactly. The cap form
leaves the sub-ε regime untouched.

### Wave 2 dispatched — five lanes

| unit | branch | ground |
|---|---|---|
| `num-relative-tolerance-collides-above-a-decimetre` | `fix/num-tolerance-cap` | `profile/src/path.rs` |
| `coherence-findings-have-no-consumer` (CheckId half) | `fix/chart-coherence-check` | `editor-core/src/checks.rs` |
| `underflow-gate-owed-at-five-more-doors` | `fix/underflow-gate-doors` | `geom-core`, `sweep`, `topo/sector_shape.rs`, `profile/path/arc_fillet.rs` |
| `validation-arms-delegate-a-recourse-their-carriers-do-not-give` | `fix/validation-recourse-arms` | four carriers, four owners |
| `error-census-keyed-on-bare-type-name` | `fix/census-declaring-path-key` | `pncad-py/src/prose_census.rs` |

**Two collisions designed out rather than discovered.** The num lane
and the underflow lane both have business in `crates/profile/src/path*`
— the underflow row's fifth site IS `path.rs:2907`, and the item
already says that site's real defect is the rendering, which is the
num lane's. So the underflow lane is told to take four arms, not five,
and to touch no file the num lane owns. Separately, three open rows
target `prose_census.rs`; only one is dispatched.

**An overlap found in triage and handed to a lane rather than left in
two rows.** `error-census-keyed-on-bare-type-name` and class 3 of
`prose-census-undecided-residue` (*"names with rival declarations …
resolving a bare name through its file's `use` items would decide most
of them"*) are one defect seen from two sides, and one repair closes
both. The lane takes class 3 in the same PR and shrinks the
`UNDECIDED` roster by what it decides; classes 1 and 2 stay open.
The lane is told to verify the overlap before building on it, because
it is my claim and not either row's.

### The brief gained two clauses this wave

**Instruction 4, new:** *an orchestrator disposition asserting what
code DOES is subject to instruction 1 too.* Written because the fifth
characterisation error of the 2026-09-11 wave was mine, in the act of
dispatching a brief about the first four. Two of this wave's briefs
carry my measured claims; both lanes are told to re-derive them and
report a correction as a finding in its own right.

**A per-lane scratchpad**, after last wave's five lanes shared one
directory and overwrote each other's `poll.py` and `pr.md` mid-task.
Nothing was lost, but a lane that writes a PR body to a shared path
and posts it a minute later posts a sibling's text.

### `error-census-keyed-on-bare-type-name` (PR 2402) — three of my framings were wrong, and the correction is worth more than the unit

The census is re-keyed on the **declaring path**: a written type resolves
through its module's `use` items and through re-exports (renames
included) to the module that declares it, and **resolution failure falls
back to the bare name**, so the change costs precision, never soundness.

**What the lane corrected, all of it mine or the row's:**

1. **The census the row names is not code.** #1111's error-type census
   was a HAND sweep and #1741 re-ran it by hand. There is nothing to
   "re-run keyed on `crate::path::Type`". `prose_census.rs`'s table is
   the tree's only live bare-name-keyed census, so that is where the
   re-key landed. The row read as a code change and is a method note.
2. **Only one of the row's two directions exists in that code.** The
   census indexes DECLARATIONS, and a re-export adds none, so the
   false-duplicate direction (`PathError`, `Refusal`) was never present
   — it inflates a hit LIST, which is what #1111 produced and this
   census never builds. *"Both directions close at once"* is true of
   the method and false of the code.
3. **The bare key was never unsound in this tree.** Instrumented at the
   merge base: 27 colliding names, 26 decided sites reach one, **zero
   wrong answers** — because rivals that disagree answer `Undecided`
   and rivals that agree give a verdict correct either way. So the
   justification is not "fixes a live defect"; it is that the key was
   sound only because the disagreement rule caught it, at the price of
   `Undecided`.

**And my overlap claim was wrong in the direction that matters.** I sent
the lane after class 3 of `prose-census-undecided-residue` on that row's
own words — *"resolving a bare name through its file's `use` items would
decide most of them"*. Measured: class 3 is **one row of 28**, and after
the re-key it is **still undecided** (`Option<profile::Verb>` now
resolves to a macro-declared type, so the verdict is unchanged and only
the roster line's REASON becomes true). **The roster shrinks by zero.**
The overlap was real; my estimate of its size came from the row and I
passed it on without measuring it. Instruction 2 applies to a dispatch
as squarely as to an item.

### The find under it: seven roster rows carried a false reason, and nothing could have caught them

`UNDECIDED`'s whole purpose is that *"a site this cannot decide either
gets its line here, WITH THE REASON it could not be decided, or gets
rewritten so it can be"*. Seven of its rows said *"declared at a type
this tree does not declare under that name — an alias, a re-export, or
one out of tree"*. All seven have an **empty candidate list**: the
census never typed the binding and never consulted the table. The
reason was false for as long as anyone read it.

**It could not have been caught**, and that is the durable part.
`every_site_this_census_cannot_decide_is_named_with_its_reason` compares
a tally against `roster(UNDECIDED)`, and `roster()` keys on
`(file, type, binding)` and a count — **the reason string is never
compared by anything.** A test whose NAME promises the reason asserts
only the site. That is the reviewer brief's Q5 — *what does this promise
that it doesn't do* — in a row this program built itself (PR 1809).

The real cause is a fourth class the residue row does not name: the
census cannot type a binding that arrives from a nested pattern
(`slot: SlotId::Profile { .. }`, `verb: Some(verb)`,
`endpoints: (u, v)`), an inner arm naming no variant path, a catch-all,
or a closure parameter. Reasons corrected in code; the defect filed as
`census-cannot-type-a-nested-pattern-binding`. Roster composition is now
7 positional, 13 `Real` scalar, 7 untypable bindings, 1 macro-declared.

**Instruction 3, and the lane built the pin rather than reporting its
absence.** Both rosters are byte-identical before and after, so nothing
in the tree discriminated the re-key. Three planted-tree rows now do;
the sharpest is one `Verb` name declared in two crates with opposite
shapes, where the SAME site text answers `Braced` or `Prose` according
only to which is imported. Run red first with `declaring_path` forced
to `None`.

**Flagged by the lane, not filed, because it did not establish it:**
`brace_shaped`'s cycle guard keys on the head name, so `Vec<Vec<Braced>>`
re-enters `Vec`, hits `seen`, and answers **`Prose`** — a guess toward
prose, which is the one silence this module exists to remove. Preserved
exactly and not a keying question. Worth a look by whoever takes class 1.

**A tooling trap worth carrying:** `territory --base origin/main` reads
the COMMITTED diff, so on a dirty tree it reports a vacuous
`0 path(s)` — which nearly had the lane file my fence claim as wrong.

### `underflow-gate-owed-at-five-more-doors` (PR 2401) — three of four doors, and the cut was MY wave design's cost

The gate fits at `frame.rs::definitely_positive`, `revolve::axis::AxisFrame::build`
and `sector_shape` (per chord), each with a new typed arm beside its
existing non-finite one and a red-first row **executed against a
gate-less tree**, not argued. `Vec2::norm_witness` / `Vec3::norm_witness`
become the one derivation of the witness.

**The decision the item asked for, made explicitly: the predicate stays
SCALAR and the WITNESS becomes vector-shaped.** The unenforceable half
of `is_underflowed_length`'s contract is the *derivation* of the
witness, not its pairing with the length, and one accessor per vector
type closes that in both dimensions without a predicate twin. A
vector-shaped predicate would re-derive `len`, which four of seven call
sites have already bound for their own `Margin` — trading an unenforced
witness for an unenforced length. The line that decided it is
`frame_from_unit_aim`'s stated contract: *"one evaluation, one
rounding"*. The lane argued the counter-case in the PR body and
declined to claim its answer is the only correct one, which is what
this posture asks for.

**Instruction 3, and the one existing pin PINNED THE DEFECT.**
`path_start_frame_refuses_true_degeneracy` listed
`Vec3::new(0.0, 0.0, 1e-200)` among "stationary points of the path",
asserting `Degenerate { Tangent, None }`. Verified: `1e-200` squares to
exactly `0.0` in `f64`, so that row was asserting that a direction the
format lost is a path with no tangent. The pin was not a baseline to
preserve; it was the defect written down. The other three doors had no
pin at all at underflow scale.

**The `min` hides this end harder than the overflow end it mirrors**, and
the lane measured it rather than reasoning it: at the overflow end
`min(3.0, inf) = 3.0` let a healthy-looking arm through, while here
`min(3.0, 0.0) = 0.0` means the underflowed chord always wins — never
silent, but never about the chord either, and the arm it names belongs
to the chord that was fine.

**The cut is mine, not the lane's.** `arc_fillet::carrier_tangent` needs
`PathError::UnderflowedDirection { dx, dy }`, and `PathError`, its
`Kind` and its `Display` all live in `crates/profile/src/path.rs` — the
file I told this lane to leave alone because the `num` lane held it for
this wave's whole life. Designing the collision out cost a forced cut.
**I judge the trade right** — a merge conflict between a 136-line
rewrite of `num` and a new error variant in the same file would have
cost more than the cut — but it is a cost and it belongs on the record
as a consequence of wave design rather than of the unit. The item stays
`open` with the remainder fully staged: the variant's home, that
`v.norm_witness()` is the witness, that `NonFiniteDirection { dx, dy }`
is the shape to copy, and that nothing pins the current
`DegenerateArcCenter { radius: 0.0 }` refusal so a red-first row is owed
with it. **`path.rs` is free as of PR 2399's merge, so this is a small
wave-3 unit, not a blocked one.**

**Two corrections to the item and to my brief.** Both said a `Vec2` twin
would be needed *"for the `profile` door"* — `RevolveAxis::dir` is a
`Vec2<T>` too (`sweep/src/revolve/mod.rs:146`), so the split across the
six sites is four `Vec3` and two `Vec2`, not five and one. And
`unit_from_components` has moved to `path.rs:2937`, not `:2907`.

**Fences, and two the `keep_out` does not know.** Fourteen paths across
six programs, all announced in the PR body. **BLEND's
`sweep/src/revolve/*` and the `topo/src/splitting/*` pair, plus CURVED's
claim on `boolean/*`, are not recorded in this program's `keep_out`** —
the lane flagged that the clause may want to learn them. It is the same
shape as the `census.rs` clause corrected yesterday: a `keep_out` that
enumerates crossings goes stale every time the program crosses somewhere
new. Worth considering whether that field should name FILES this program
may cross at all rather than the crossings it has made.

### `validation-arms-delegate-a-recourse-their-carriers-do-not-give` (PR 2403) — the complete chain, and every count in the item was wrong

Taken: the whole `VolumeUncomputable` carrier chain plus the shared
`BandError` — `BandError` (3 of 3), `MassPropsError` (3 own arms), and
**`PropsError`, a FIFTH carrier the item does not name**, reached
through `MassPropsError::Face` (4 of 8). Without that fifth the chain's
claim is false one hop down, which is the whole point of the row.

Settled the PR 2354 choice the same way 2354 did: **arms get their
recourse; the wrapper supplies no blanket tail.**

**Cut: `PcurveMintError` (TRIM) and `OffsetFitError` (PROPS)**, and the
lane's reason for cutting is the best argument for this posture I have
seen from a lane. Roughly thirty recourse clauses, written by a lane
reading two unfamiliar domains for the first time, **with no reviewer
downstream**, is how a confidently-worded wrong repair ships — which is
precisely the failure PR 2354 removed a blanket tail to avoid. A lane
declining scope *because* nobody will catch it is the disclosed-doubt
half of this posture working as intended.

**All four of the item's counts were wrong, and reading is what
corrected them:**

| row | item | reading |
|---|---|---|
| `BandError` | 0 of 2 | 0 of **3** — three variants; the zero is right |
| `MassPropsError` | 0 of 5 | 0 of 5 — **confirmed**, two of them delegations the match could not see through |
| `PcurveMintError` | 1 of 9 | 0 of **10** — and the single hit is a FALSE POSITIVE |
| `OffsetFitError` | 1 of 8 | **twelve** variants, several already naming their lever |

`OffsetFitError` is where reading most changes the verdict:
`BudgetExhausted` names `OFFSET_FIT_BUDGET`, `SampleCapReached` names
`OFFSET_FIT_SAMPLE_CAP`, `BoundNotFinite` names which lever is *not*
it. Those are rows that already name their repair — **a correct
finding, and the reason not to rewrite them blind.** The item said its
counts were a signal to read and not a verdict; that instruction paid
for itself four times.

**Instruction 3, answered per carrier rather than once.** `BandError`
had a pin that discriminated (`band_error_display`, all four
renderings, updated here — it would have failed otherwise);
`PropsError` and `MassPropsError` had **nothing** pinning any message
text, so for those two the missing pin was part of the defect. The
three enforcement rows are that pin, each proved red by mutation.

**The `MassPropsError` row is TRANSITIVE and that is the interesting
one**: its `Band`/`Face` arms carry no prose of their own, so it passes
only while its carriers name recourses — a defect two crates away
reddens it. That is a wrapper's assumption made to fail loudly instead
of documented, and it is the shape to copy wherever this class recurs.

**The routing question the item raised, answered by measurement:**
`crates/topo/src/props.rs` is named by **no open program**. `territory`
does not report it and the lane checked every open `program.md`'s
`paths` by hand — TOPO's list is explicit files and omits it, CURVED
claims `census.rs`/`boolean/*`/`splitting/*`, S-MESH claims
`coherence.rs`. Genuinely unowned, so the lane took it by announcement.
**Naming a permanent owner for that file is a program-charter question,
not a unit's and not this orchestrator's** — it is flagged here and
stays open.

**Filed out of fence, verified first:**
`work/exch/export-error-arms-delegate-no-recourse` — `ExportError`'s
`Corrupt { what }` (*"step export: corrupt body ({what})"*, the whole
message) and `NullScaffoldEdge`. `crates/step-export/*` is EXCH's
`paths`, so the lane reported and the orchestrator placed.

**And the lane named the real scope of the class, which the item did
not:** it is not four `ValidationError` arms, it is **every carrier
reachable from a delegating arm**, and a verb-match sweep cannot see
past the first hop. `PropsError` was the hop this PR had to absorb;
`PcurveMintError`'s `Certify`/`Escalated` and `OffsetFitError`'s four
delegating arms are the next ones for whoever takes the cut rows.

### `coherence-findings-have-no-consumer` (PR 2408) — and my "## Measured" disposition was wrong in a worse way than a stale count

`CheckId::ChartCoherence` is wired as a third resident, `CheckKind::Certified`,
reading `topo::examine_chart_coherence`, with three `CheckEvidence` arms and
a `ChartCoherenceLane` capability trait.

**The centre held.** `skipped` is CONFIGURATION and `unexamined` is DATA, and
nothing from one reaches the other: `Off` goes to `ChecksReport::skipped` with
**no finding**, while a loop out of the door's reach goes to `findings` as
`ChartCoherenceUnexamined` carrying `topo::Unexamined` whole. Pinned by a row
asserting the two reports **share not one word**, and cross-checked against the
door called directly so the resident's two finding classes are the door's two
lists, one for one. That is the not-examined-masquerading-as-examined shape
this program has now found three times, closed rather than documented.

### What I got wrong, precisely

My disposition read:

> **Four non-test match arms over it, in the whole tree**
> (`grep -rn 'CheckId::[A-Za-z]* *=>' --include=*.rs crates/`, minus
> tests) — the enum's own `kind()`, `reads_subject()`, `ALL`, and
> `crates/pncad-py/src/py/checks.rs`.

I re-ran that grep myself on this head. It returns four arms, in **two**
functions — `ChecksConfig::severity` (`checks.rs:236-237`) and
`py::checks::check_id` (`:121-122`). **Of the four sites I named, exactly one
is in its output.** `kind()` (`:79`), `reads_subject()` (`:108`) and
`Display` (`:117`) all spell their arms `Self::…` and are invisible to that
pattern; `ALL` (`:98`) is a `const` array and not a match at all. I omitted
`ChecksConfig::severity` — whose own doc calls itself *"the one match site the
closed enum walks a new check to"* — and `Display` entirely.

**So the number was right and the attribution was invented.** I ran a grep,
took its count, and then wrote down four site names from a different and
partial reading, presenting the two as if one produced the other. That is
worse than the stale counts this log keeps recording, because a stale count
announces itself the moment someone re-derives it, while a fabricated
correspondence between a measurement and a list survives re-derivation of
either half alone.

The real walk is **eight non-test sites**, and a new CHECK is not a new
`CheckId`: its findings need `CheckEvidence` arms with four more exhaustive
matches, plus the `ChecksConfig` literal, the `.pyi` and two Python suites.

**And the obstacle neither the row nor I saw:** `examine_chart_coherence`
takes `&Body<f64>` (`coherence.rs:672`) while `run_checks` is generic over
`T: Decide + AtRestPolicy + CertifiedBounds` (`checks.rs:731`). There is no
arm without a lane capability. **The ROW's "a consumer decision, not part of
relocating a condition" was closer to right than my disposition was** — I
called that sentence an overstatement and it was an underestimate of a
different thing. The conclusion (dispatchable) survives; the cost is about
three times what I wrote, and a public trait bounding two public doors is the
shape.

### Two process facts from this lane, both disclosed rather than found

**The lane reported green locally off a log that was still being written.**
It read a `test result: ok` tail before the run had finished, missed three
census guards, and CI caught them over two round trips. Nothing merged broken
and the lane recorded it on its own PR. This is the failure mode
`docs/prompts/implementer-discipline.md` §2 names — *a build is not a test*,
one level in: **a log is not a result until it is closed.**

**`work.py territory` gave a vacuous pass, for the second time in one wave.**
The lane reported *"0 paths in another program's territory"* while its own
prose correctly listed crossings into LIB, MESH and TCOST/TINT. I ran it
against the branch myself: **11 paths**. The census lane hit the same trap two
lanes earlier and caught it; this lane reported the vacuous result as a pass.
The substance was unharmed — the prose fence list was right and complete — but
the mechanical check did not run on this diff and was reported as though it
had. **A lane must run `territory` on a COMMITTED diff and treat a `0 path(s)`
answer on a cross-crate change as a result to disbelieve, not a pass.** Going
into every brief from here.

### The `keep_out` stops enumerating

Three stale clauses in two days — `census.rs`'s list of two items, `eval/wire.rs`
as unowned, and now BLEND's `revolve/*`, the `splitting/*` pair and LIB's
`pncad-py/*`. The failure is structural: a clause that lists the crossings this
program has MADE goes stale every time it crosses somewhere new, and a lane
reading a missing entry as a fence violation is the cost.

The clause now says so: the list records seams that needed a note, **not every
crossing ever made**, a crossing absent from it is not thereby unannounced, and
the instrument is `territory`, not the clause. The four standing crossings are
named so a lane need not rediscover them. FIX's `paths` are three globs and it
crosses by construction; pretending otherwise in a field lint reads at rest was
never going to hold.

### `coherence-findings-have-no-consumer` merged (PR 2408) — wave 2 closes five of five

PRs **2399, 2401, 2402, 2403, 2408**. Slate **18 → 17 open** (four
closed, four new rows filed: `fillet-leg-carrier-renders-raw-float-noise`,
`census-cannot-type-a-nested-pattern-binding`,
`chart-coherence-ships-off-and-nothing-schedules-turning-it-on`, plus
two placed on other programs' slates), **38 closed**, and two mate rows
re-homed out.

### The operational lesson, which cost the most time in this wave

**`update_pull_request_branch` does not fire CI in this repo, and it
leaves the PR's head record stale.** The API creates a real merge
commit on the branch — `git` sees it — but no `pull_request` run
follows, and the PR object keeps reporting the OLD head SHA for some
time afterwards. The merge endpoint then refuses with *"Head branch is
out of date"*, comparing that stale head against a moved base.

Three PRs this wave were sequenced through that call and were fine —
their re-runs genuinely fired and I read them. The fourth was not, and
**I read the 409's text as the explanation and stated it as fact**
("the required check has not run on that SHA") when what I actually had
was an error string and an inference. Ev caught it: the run WAS green.
The correct instrument was one command — compare `git rev-parse
origin/<branch>` against the PR's reported `head.sha` — and I reached
for the error message instead.

**That is the same failure this wave's log has been recording about
lanes, in the orchestrator's own operations rather than its
dispositions.** A grep's count attributed to sites it could not have
produced, and an error string's wording taken as the mechanism behind
it, are one habit: reading an artifact that is *adjacent* to the
question and reporting it as the answer.

**Standing, for this program's orchestration:** after any branch
update, verify the PR's head SHA against the remote ref before drawing
any conclusion from a merge refusal, and never paraphrase an API error
as a cause. Where a branch genuinely needs CI re-run, the way to get it
is a real commit the PR already owes — here, the item header this unit
had left at `status: review` while every other unit of the wave closed
its own row in its carrying PR. **Not an empty commit**, which the
discipline forbids, and which was never necessary.

## Wave 3 dispatched, 2026-09-12

| unit | branch | ground |
|---|---|---|
| `underflow-gate-owed-at-five-more-doors` (the cut fourth arm) | `fix/arc-fillet-underflow` | `profile/src/path/arc_fillet.rs` + `path.rs` |
| `census-containment-flatten-fabricates-its-diagnostic` | `fix/census-containment-cause` | `topo/src/census.rs` |
| `kind-mirrors-have-no-single-declaration` (**scale check + spec ONLY**) | `fix/error-kinds-scale-check` | docs + tracker |
| `validation-arms-delegate-a-recourse-...` (the two cut carriers) | `fix/recourse-arms-remainder` | `topo/src/pcurves.rs`, `geom-brep/src/offset_fit.rs` |
| `transform-recertifies-through-the-narrow-lane` | `fix/transform-nurbs-lane` | `topo/src/transform.rs`, `geom-brep/src/certify.rs` |

**Two units close a remainder this wave, and one of the two remainders
was this orchestrator's own doing.** `arc_fillet` was cut from PR 2401
because `PathError` lives in `path.rs`, which I had told that lane to
leave alone for the `num` lane's sake. `path.rs` is free now, the
previous lane staged the remainder completely — the witness accessor,
the variant to copy, and the fact that nothing pins the current
`DegenerateArcCenter { radius: 0.0 }` refusal — so this is a small unit
rather than a blocked one. The cost of designing that collision out was
one wave's delay, and I still judge the trade right.

**`kind-mirrors` is dispatched with a hard scope fence: it migrates
nothing.** The row wants a `macro_rules!` `error_kinds!` generating four
PUBLIC error/kind pairs tree-wide, and the orchestrator flagged it to Ev
as one of three rows changing public surface. So the unit is the scale
check the row itself names as owed first — whether `transition_table!`'s
grammar can express `BooleanError`'s 41 arms, their attributes and their
nested payloads — answered **by prototype and revert**, output being a
spec Ev can read or a written statement that the migration should not
happen. Nothing tree-wide lands unseen, and "not feasible" is an
outcome, not a failure.

**`transform-recertifies` is the second public-surface row and it lands
if it is right.** It is a real defect — a body tier 3 validates at rest
refuses at `transform_rigid`, so the kernel cannot move a body it calls
valid — and the item is explicit that injecting the lane *"adds no
certification capability the at-rest validator does not already have"*.
The lane is told to verify that sentence rather than repeat it, since it
is the one a reviewer would attack, and to make the public delta legible
in the PR body.

**Instruction 2 fired before dispatch again.** The transform row's table
says `crates/topo/src/validate.rs:2920` calls `recertify_nurbs_lane`; I
grepped that file on this head and found no such hit. The brief says so
and tells the lane to re-derive the whole table — and, per instruction
4, to report a correction if my grep is what is wrong.

**Two new standing clauses in every brief**, both earned last wave:
- **`work.py territory` reads the COMMITTED diff**, so a dirty tree
  answers a vacuous `0 path(s)`. Commit first, and **disbelieve a `0`
  on any cross-crate change.** Two lanes hit it; one caught it, one
  reported it as a pass.
- **A log is not a result until it is closed.** A lane read a
  `test result: ok` tail off a file still being written, reported green,
  and CI caught three census guards over two round trips. Wait for the
  exit code. This is `implementer-discipline` §2's *"a build is not a
  test"* one level in.

### The build box ran out of disk mid-wave, and it is an orchestration hazard not a lane one

The `error-kinds` lane reported the root filesystem wedged at **100%**,
down to **180K** free, and it blocked `git commit` with
*"index.lock write error. Out of diskspace"*. It got itself unstuck by
deleting `/root/.cargo/registry/cache` (~100M of re-downloadable
`.crate` tarballs, not the `src/` extractions builds read) and said
plainly that this was a reprieve and not a fix. It touched no other
lane's build state — correctly, since it could not know which were
live.

**That is the orchestrator's call and I made it.** Measured: nine
`CARGO_TARGET_DIR`s under `/home/user/*-target`, 23G in total. Eight
belonged to units of waves 1 and 2, **all merged**; exactly one
(`census-containment-target`, 8.7M) belonged to a live wave-3 lane.
Before deleting anything I confirmed **no `cargo`/`rustc` process was
running** and **no target directory had a file modified in the last ten
minutes** — the check that separates "stale" from "idle between
steps". Removing the eight freed **22 GiB**; the box went from 96M to
23G free.

**The hazard is structural and worth stating.** Every lane is told to
use its own `CARGO_TARGET_DIR` outside its worktree, which is right —
a shared one serves another lane's binary, and this program has the
scars. But nothing reclaims them, so each wave leaves ~2-3G per lane
behind forever and the fourth wave is the one that dies. Five lanes
were live when this fired and four of them could not have built.

**Two consequences for this program's orchestration:**

1. **Sweeping the previous wave's target directories is part of
   closing a wave**, alongside the log entry and the item headers. The
   safe test is the one above: no build process running, nothing
   modified recently, and the unit's PR merged.
2. **A lane that cannot build cannot tell you why in the usual way.**
   This one surfaced it only because it hit `git commit` and read the
   error; a lane that hit it inside `cargo` would have reported a
   confusing build failure. Worth a brief clause if it recurs.

`/home/user/cad/.claude/worktrees` holds a further 4.9G across sixteen
worktrees, most from merged units. Left in place — 23G is ample — but
it is the next thing to sweep.

### `kind-mirrors-have-no-single-declaration` — the scale check answered, spec merged (PR 2417)

**Feasible, for three of the four pairs.** `docs/FIX-ERRKINDS-SPEC.md`
is in the tree; the item is `status: spec`; **the migration has not
started and does not start until Ev has read §6.**

The answer came by prototype and revert, which is what the unit asked
for. Two grammar spellings are **forced, each a compile error first**:
generics must be bracketed (`$(< $($g:tt)* >)?` gives *"local ambiguity
when calling macro"* — which is why `transition_table!` brackets its
own), and bounds go in a bracketed `where` group because
`macro_rules!` cannot strip bounds off a self-type (E0229). The item's
specific worry dissolves: **no pair carries `#[non_exhaustive]` or
`cfg`**, and no variant in any of the eight enums carries a non-doc
attribute at all.

**Both claims the brief asked it to check came back sharp.**
*"Closable by a derive and by nothing else"* is **false**, demonstrated
rather than argued — the item's own later paragraph is the right one.
And on the proc-macro precedent: the distinction **holds** (the
`scripts/gates/README.md` rejection is of a mechanism for *enforcing*
an invariant everywhere by opt-in annotation, where omission is
invisible; generation is the opposite on that axis) — **but the item's
"no design question for Ev" does not follow from it**, and the spec's
§6 names five choices that arrive whichever way the clause reads. That
is a better answer than either yes or no.

**Two of the item's measurements are false at the merge base**, and
both concern `Attr`/`AttrKind`, which the lane took **off** the
migration list because it is not an error pair at all — it is the
appearance store's serde-persisted key/value pair with a wire format.
The item said its cited grep *"returns zero arms tree-wide, so nothing
anywhere matches on it exhaustively."* I ran that exact grep: **three
hits** (`crates/pncad-py/src/tags.rs:300-302`). And it could never have
seen `AttrKind::noun()` (`crates/editor-core/src/appearance.rs:112`),
an exhaustive match written in the `Self::` spelling the pattern cannot
match. **Its phantom direction is guarded twice over.** Both facts
predate the item.

**The counts are 43 / 31 / 10, not 41 / 28 / 10** — and my own check of
that correction is worth recording, because it failed the same way this
log keeps describing. I ran
`awk '/^pub enum BooleanError/,/^}/'` and counted 86, exactly double.
The range fires twice: `pub enum BooleanErrorKind` also matches
`/^pub enum BooleanError/`. A prefix match I did not think about
produced a clean, plausible, wrong number — and had I reported it
without asking why it was exactly 2x, it would have read as a refutation
of a lane that was right.

**Where the lane could not establish something it said so**: serde
attribute passthrough was **not** executed (the disk was full, so no
`cargo build`) — `#[derive(Debug)]` through a `meta` fragment is
proven, `#[serde(...)]` is not, and only the dropped pair needs it. The
prototype was type-checked (`--emit=metadata`), not run, and the report
says so rather than claiming more.

### `census-containment-flatten-fabricates-its-diagnostic` (PR 2420) — delivered whole, and my brief's safety claim was false

The three `ContainError` arms now reach the user as
`CensusUnsupported { subject: Entity(Face(k)), cause: Containment(e) }`
with `ContainError` carrying its own `Display` — it had none, which is
half of why the census had nothing to forward.

**The defect was worse than the item's wording, and the lane said how.**
`invalid(band, "pm_census_containment")` builds
`Indeterminate { margin: MarginDiag::Invalid, .. }`, and
`MarginDiag::Invalid` means *"the question was never validly posed"* —
poison. So the census was not reporting a wrong number; it was
reporting that **a named predicate had been posed and come back
poisoned, when no predicate of that name decides anything anywhere in
the tree.**

**The gate answered: the three arms are NOT one class, and that is the
argument for carrying the cause rather than against it.** Read from the
code: `ArcLoopUnsupported` is a modelling fact (an arc loop under three
vertices has zero polygon area), `RayExhausted` is a verdict that the
point sits within ε of the boundary, `Corrupt` is a kernel-invariant
violation. Three meanings, three repairs. The one property true of all
three and false of `Escalated` is the only one the site needed:
**nothing metred a margin.** No cut; the item closed whole.

### The brief's claim (2) was false, and the lane measured it

I passed on the item's and PR 2354's warning that re-routing these
three *"CAN MOVE THE ANSWER"* because
`editor_core::assembly::attribute` dispatches on the variant. The lane
read that function instead of taking it. I then verified the reading
myself:

- `CensusUnsupported { subject: FacePair(a, b), .. }` →
  `named(by_pair(a, b), Relation::Declined)` (`assembly.rs:1311`) — this
  **would** move the verdict;
- `CensusUnsupported { subject: Entity(EntityId::Face(_)), .. }` →
  `Attribution::Unattributed` (`:1320`);
- `CensusEscalated { .. }` → `Unattributed` (`:1370`).

So the routing chosen keeps the same attribution and **no
`AtRest`/`Uncertified` answer moves.** The warning was true of a
routing the lane did not take, and would have bitten had it chosen
`FacePair` — which is not available at any of `contain()`'s five call
sites anyway. **The subject choice is what makes this safe, and the
lane pinned it** by extending
`the_decline_relation_does_not_depend_on_which_lane_declined`.

That is the fourth correction to the dispatching seat in three waves,
and it is a different kind from the first three: the claim I relayed
was true in general and false of this repair. **A general truth
narrowed by a specific routing is still a claim about the tree**, and
instruction 4 covers it.

### Where the lane was weaker than it wanted, disclosed rather than implied

**The RED half of red-first was deduced, not run.** Its first push would
have been the red run and died at `cargo test --no-run` on a missing
import in its own fixture, so the test stage never executed. The AFTER
state is measured; the BEFORE state is a tight deduction (the fixture
demonstrably reaches the arm; the old mapping was an unconditional
four-line match) but not a measurement. **This is the first unit of
three waves whose red-first half is argued**, and the reason is the
disk, not the lane. Only one of three arms has an executed fixture at
all: `RayExhausted` needs all sixteen schedule directions to graze, and
`Corrupt` is argued unreachable through the public door.

**And `RayExhausted` is the arguable arm** — `PointInLoopError`'s own
`Display` calls it *"ill-conditioned at this tolerance"*, which is
escalation-shaped. It did not go to `CensusEscalated` because there is
no metred `Indeterminate` to give it and minting one is the defect;
consistency with PR 2354's routing of the structurally identical
`ChartRegionError::RayExhausted` is the tiebreak, and the lane called
it a tiebreak rather than a proof.

### Two findings placed in `work/issues/`, both verified first

`crates/topo/src/boolean/contain.rs` is claimed by **both** `bool` and
`curved`, so the owner is disputed rather than clear — which is the
case `work/README.md` reserves `work/issues/` for. Either may claim
them by moving the file.

- **`contain-error-drops-the-loop-its-carrier-named`** —
  `From<PointInLoopError>` discards the loop key that
  `RayExhausted { r#loop }` and `CorruptLoop { r#loop }` each carry,
  while the comment four lines below promises every arm *"names WHAT
  STOPPED and the repair that moves it"*. `ArcLoopUnsupported` beside
  them does name its loop. Survivable while nothing rendered it; PR
  2420 made it render.
- **`corrupt-operand-means-two-things-and-one-site-fabricates-a-vertex`**
  — `reduce.rs:1970` and `ops.rs:1642` map one arm to two different
  `BooleanError` variants. **And the sharper half is the orchestrator's
  on verifying it:** `reduce.rs` supplies
  `vertex: VertexKey::default()` for a refusal that has no vertex —
  **the third instance of the class this very unit closed**, a
  fabricated value in the field a reader would use to locate the
  problem.
