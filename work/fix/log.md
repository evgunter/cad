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
