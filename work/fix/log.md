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
