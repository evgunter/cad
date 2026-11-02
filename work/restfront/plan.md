# RESTFRONT — the plan

a planar face's region at rest: ring nesting, loop simplicity, check 9's planar arms

Opened 2026-09-27 by ATREST's close
(`docs/doc-ledger/atrest-leaves-the-tracker.md`). Split on 2026-10-29
at 133 budget points against 30 (`log.md`): the rest of the slate went
to RESTCURVE (the region off the plane), RESTSHELL (solids and shells),
RESTGATE (what the at-rest gate admits on), RESTDATUM (a carrier
against its datum and its faces) and RESTWORD (how validate decides
and words its refusals).

## The slate

**19 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P1 | `check-9-does-not-check-a-ring-nested-inside-another-ring` | M |
| P1 | `tier3-never-checks-a-planar-loop-is-simple` | M |
| P1 | `check-9-lines-meet-falls-through-to-a-definite-meet-off-both-spans` | E |
| P2 | `validators-accept-a-lamina-slit-or-zero-area-membrane-face` | M |
| P2 | `check-6-planar-arm-skips-ellipse-and-nurbs-loops` (spiric and NURBS) | M |
| P3 | `a-corner-is-a-slice-has-three-homes` | M |
| P3 | `check-9-lines-meet-escalates-an-in-band-side-before-its-collinear-arm` | E |
| P3 | `check-9-reads-an-internal-tangencys-trims-at-the-radical-foot` | E |
| P4 | `validate-rs-reads-the-doubled-cusp-as-the-kissing-union-two-slit-edges` | E |

## Order

1. **The wrong answers in check 9's line arm** (`lines_meet`'s
   fall-through Meet, then its order), with the radical-foot trims: all
   three are E rows on one function and ride one unit.
2. **Ring-in-ring nesting, then loop simplicity, then the slit and the
   membrane.** All three are "a planar region that is not what its
   loops say"; ring-in-ring uses the carrier walk ATREST-9 and ATREST-12
   built unchanged, and simplicity reads edge interiors against each
   other, which the slit row needs too. Read the three together before
   speccing, and check JOIN's corner arm (`pinch_corner_errors`), which
   closed the repeated-vertex half of simplicity.
3. **Check 6's remaining carriers** (spiric, NURBS), following the shared
   winding in `loop_winding.rs`, not a second winding here.
4. **The corner rule's three homes**, and the doubled-cusp prose.

## Review posture

The orchestrator's read for P4 and prose rows, a single review for a
new check arm, and a dual pair for a change to what tier 3 refuses on a
body a kernel door produces (`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.

## RESTCURVE's slate, merged in 2026-11-02

RESTCURVE merged into RESTFRONT on 2026-11-02 (`docs/doc-ledger/restcurve-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/restfront/`.

RESTCURVE — the face region off the plane: chart singularities, curved containment, curved meeting arms

Opened 2026-10-29 by RESTFRONT's split (133 budget points against 30;
`work/restfront/log.md`, 2026-10-29). Every row came from
`work/restfront/` by `git mv`, ids and bodies kept. The sibling
tracks, all on `crates/topo/src/validate.rs`: RESTFRONT (a planar
face's region), RESTCURVE (the region off the plane), RESTSHELL (solids
and shells), RESTGATE (what the at-rest gate admits on), RESTDATUM (a
carrier against its datum and its faces), RESTWORD (how validate
decides and words its refusals).

### The slate

**30 budget points** against 30 (2026-10-30).

| pri | item | cost |
|---|---|---|
| P1 | `validate-tier3-curved-boundary-containment` | H |
| P3 | `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges` | H |
| P1 | `loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` | H, design |
| P1 (parked) | `tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity` | M |
| P3 | `a-sampled-edge-the-chart-singularity-reading-cannot-settle-is-exempt` | M |
| P3 | `a-loop-the-chart-singularity-walk-cannot-certify-is-exempt` | M |

### Order

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

### Review posture

RESTFRONT's, unchanged: the orchestrator's read for P4 and prose rows,
a single review for a new check arm, and a dual pair for a change to
what tier 3 refuses on a body a kernel door produces
(`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.

### Handoff (2026-10-30, end of the first sitting)

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

### Handoff (2026-11-02, end of the second sitting)

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

## RESTSHELL's slate, merged in 2026-11-02

RESTSHELL merged into RESTFRONT on 2026-11-02 (`docs/doc-ledger/restshell-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/restfront/`.

RESTSHELL — solids and shells at rest: checks 7 and 10, overlapping material, orientation

Opened 2026-10-29 by RESTFRONT's split (133 budget points against 30;
`work/restfront/log.md`, 2026-10-29). Every row came from
`work/restfront/` by `git mv`, ids and bodies kept. The sibling
tracks, all on `crates/topo/src/validate.rs`: RESTFRONT (a planar
face's region), RESTCURVE (the region off the plane), RESTSHELL (solids
and shells), RESTGATE (what the at-rest gate admits on), RESTDATUM (a
carrier against its datum and its faces), RESTWORD (how validate
decides and words its refusals).

### The slate

**25 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P1 | `an-inside-out-slab-thin-enough-for-its-volume-to-be-in-band-finishes-at-rest` | M, design |
| P1 | `tier-3-admits-two-solids-of-one-body-whose-material-overlaps` | M |
| P2 | `check-ten-falls-silent-on-a-shell-whose-every-vertex-touches-another` | M |
| P3 | `check-10-is-silent-where-point-in-solid-refuses` | M |
| P3 | `check-10-is-quadratic-in-shells-per-solid` | M |
| P3 | `check-7-stops-body-wide-so-one-solids-defect-hides-anothers-orientation` | M |
| P3 | `tier-three-accepts-two-coincident-duplicate-shells-in-one-solid` | M |
| P3 | `shell-nesting-witness-refuses-on-a-point-scoped-reading` | E |
| P3 | `same-surface-faces-sharing-an-edge-need-not-carry-one-sense-bit` | M |

### Order

1. **The two P1s**: the thin inside-out slab (a design question first:
   which reading decides the side) and two solids' overlapping
   material (check against ATREST's census, which already refuses it
   at tier 3', before building a second reader).
2. **Check 10's silences**, then check 7's body-wide stop, the
   duplicate shells and the sense bit.
3. **Cost**: check 10's n(n−1) probes.

### Review posture

RESTFRONT's, unchanged: the orchestrator's read for P4 and prose rows,
a single review for a new check arm, and a dual pair for a change to
what tier 3 refuses on a body a kernel door produces
(`memories/orchestration-model.md`).

No exit criteria are set: the program closes when its slate is empty.
