# RESTCURVE — the plan

RESTCURVE — the face region off the plane: chart singularities, curved containment, curved meeting arms

Opened 2026-10-29 by RESTFRONT's split (133 budget points against 30;
`work/restfront/log.md`, 2026-10-29). Every row came from
`work/restfront/` by `git mv`, ids and bodies kept. The sibling
tracks, all on `crates/topo/src/validate.rs`: RESTFRONT (a planar
face's region), RESTCURVE (the region off the plane), RESTSHELL (solids
and shells), RESTGATE (what the at-rest gate admits on), RESTDATUM (a
carrier against its datum and its faces), RESTWORD (how validate
decides and words its refusals).

## The slate

**30 budget points** against 30 (2026-10-30).

| pri | item | cost |
|---|---|---|
| P1 | `validate-tier3-curved-boundary-containment` | H |
| P3 | `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges` | H |
| P1 | `loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` | H, design |
| P1 (parked) | `tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity` | M |
| P3 | `a-sampled-edge-the-chart-singularity-reading-cannot-settle-is-exempt` | M |
| P3 | `a-loop-the-chart-singularity-walk-cannot-certify-is-exempt` | M |

## Order

1. **The chart singularity (P0) — closed by PR 6054.** The wrap-tip half (`tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity`) stays parked on the shared loop walk.
   Was: Its trigger (FOREIGN's import door)
   has fired; the row is re-aimed on the region reading and grown by
   `[ev]` PR 6014 (the wrap tip). Read the row's last two sections
   before the spec.
2. **One reading of a curved face's region: the shared loop walk and curved-boundary containment together**, weighed by one designer pair (they are one question), then built. The wrap tip follows.
   Was: **Curved-boundary containment**, the last unmarked deferral: it
   needs a walk on a curved chart, which the P0's region reading may
   share.
3. **Check 9's arms off the plane** (meeting, then corner), and the
   ring-on-a-curved-face wording, which the containment walk decides.

## Review posture

RESTFRONT's, unchanged: the orchestrator's read for P4 and prose rows,
a single review for a new check arm, and a dual pair for a change to
what tier 3 refuses on a body a kernel door produces
(`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.

## Handoff (2026-10-30, end of the first sitting)

The track is handed back as `ready`. Its P1s wait on Ev, and the dispatchable remainder is P3.

**What landed this sitting:**
- RESTFRONT split into six tracks (PR 6049).
- The P0 chart-singularity refusal (PR 6054, dual review).
- The shared loop structure: one both-sides predicate with two views, plus one empty-surface rule (PR 6088).

**Waiting on Ev, all on `[ev]` PR 6085.** It asks three decisions:
1. A DESIGN tier-3 "one region per face" clause.
2. The point-in-face doors replacing PR 4822's chart-polygon instrument.
3. The pole-form result (fork-log row 222): keep the wrap tip; a tip is any valence-1 end of a wrap edge; the wrap edge is the face's seam kept to its end.

The fork-log rows 221 and 222 carry the recommendation halves. Their second commits (Ev's decision, plus the A/B→model mapping from `analysis/design-fork/restcurve-curved-region` and `…/restcurve-pole-form`) go on that PR before it merges.

**On Ev's yes to 1 and 2:** unpark `validate-tier3-curved-boundary-containment`. Its unit 2 is the region arm: simple and disjoint cycles, then each cycle's side through the doors, open ends, torus windings. `walk_loop` retires and `region::boundary` reads through `surface_cycles`.

**On Ev's yes to 3:**
- Close `sphere-faces-take-a-per-face-chart-or-pole-vertices`.
- Unpark the wrap-tip rows: this slate's tier row, TESSWALK, SPLITDOOR and CARVE (FOREIGN follows).
- Dispatch PR 6097's fix pass (its review is recorded on the tier row): F1, a tip is any valence-1 end; F2, the pole read by identity in the mesh walk and in coherence; F3, a sizing that passes only zero; F5, re-owned to the row-75 crossing row.
- Land `a-wrap-tip-cap-loses-its-side-datum-when-the-chart-image-drops-the-tip` with or before the first door that mints a tip.
- PR 6097 stays held and unmerged until then.

**On a no, or a different form:** re-weigh with the designer pair. PR 6097 is then re-scoped or closed.

**Lessons:**
- Always pass FULL SHAs to cloud review sessions; a short one failed silently and cost an hour.
- Never force-push.
- Read a ratified sentence's code before explaining it to Ev: "a seam is a property of a face" was misread.

**One more lesson (2026-10-31):** never create a cloud session in the same tool batch as the lookup of its SHA. A parallel call sent a short SHA and a placeholder prompt; it failed at start and touched nothing.

## Handoff (2026-11-02, end of the second sitting)

The sitting ended because no P0 or P1 remains. It closed five rows:
- PR 6163: the region arm, `validate-tier3-curved-boundary-containment`;
- PR 6244: torus windows a hair apart at the certified scalar;
- PR 6252: the sphere door settles a pole at the certified scalar;
- PR 6307: the two-leg sphere path, and tier 3's pole side through the door, which retires tier 3's `walk_loop`;
- the hair-gap row it filed.

**State.** Tier 3 reads one region per face on every analytic kind, and reads a chart singularity's side through the face's door. The doors settle poles, hair gaps and antipodal crossings at Interval. Every rule above has rows that a mutant turns red.

**What is left (all P3 or unpriced):**
- `a-loop-the-chart-singularity-walk-cannot-certify-is-exempt`: tier 3 no longer reads `walk_loop`. Only step-import does, through FOREIGN's `import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave`. Close this row for tier 3 when that lands, or re-scope it to import.
- `a-sampled-edge-the-chart-singularity-reading-cannot-settle-is-exempt`: `place_on_boundary`'s sampled-carrier exemption.
- `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges` (H).
- `the-cone-door-reads-an-apex-generators-crossings-through-a-cut-angle`: it now cites the readings the other doors move onto (`span_at_nearer_end`, `first_harmonic_root_directions`). ROOTS' `an-angle-opposite-its-spans-middle-…` is the same class across the torus and ruled doors.
- `the-region-arm-is-silent-on-spline-faces-and-off-admits-edges`: it now also lists sphere and cone edges outside the doors' `admits`, which tier 3 exempts since `walk_loop` retired.
- Two unpriced rows.

**Follow-ups noted on PRs, not yet rows:**
- The sphere door keeps its own aimed-path schedule rather than `region::walk`'s (PR 6307's style finding).
- `CircleFamily::Rim`'s own-parameter route could take the point reading (PR 6244).

**Lessons from this sitting:**
- Every review round of a door change found a false refusal at the certified scalar. Brief reviewers to probe Interval hard, and verify each pass's door change with a base-vs-head differential.
- An implementer polling a long sweep in the foreground misses `next`-priority messages; send `now`.
- Two worktrees on one branch name move each other's HEAD. Work in a detached worktree and push by refspec.
