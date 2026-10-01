# CURVED exit walk

**Program:** CURVED — the curved-operand boolean remainder (`work/curved/`).
Opened 2026-09-03, open for dispatch 2026-09-04. On 2026-09-20 (Ev,
in-chat) it was cut down to the spiric lane, with its boolean, tangency
and SSI lanes moved to REACH, TANG and CHART. **Charter and criteria:**
`work/curved/plan.md`. **Narrative:** `work/curved/log.md`.

The criteria are the plan's `## Exit shape`, quoted verbatim and taken
one clause at a time. They were written at the 2026-09-20 cut, after
PR-1a had merged and before PR-1b and the seam unit ran.

## The slate, as it stands

Thirteen rows. Twelve are closed; the thirteenth moves to OFFSET in
this PR.

| row | status | closed by |
| --- | --- | --- |
| `VERBS-C5ARMS` | closed | PR 1864 |
| `circle-residual-harmonics-needs-torus-arm` (CURVED-TORUS PR-2) | closed | PR 2535 |
| `torus-operand-boxes-span-whole-ring` (CURVED-TORUS PR-1) | closed | PR 1907 |
| `cylindrical-rest-pair-hits-planar-merge` (the merge door) | closed | PR 2105 |
| `spiric-carrier-ruling` | closed | Ev's rulings on #1858 |
| `spiric-rim-carrier` (CURVED-SPIRIC, two PRs) | closed | PRs 2566 and 2861 |
| `equator-seam-reauthor-refuses-the-hollowed-elbow` | closed | PR 3626 |
| `spiric-rim-window-reads-its-inner-equator-end-on-the-branch-cut` | closed | PR 3626 |
| `curved-escalations-offer-a-declaration-the-door-cannot-take` | closed (rider) | PR 3626 |
| `torus-meters-blocker-is-the-arithmetic-or-c9s-root-rule` | closed | Ev's ruling on PR 3517 |
| `c5a2-ledger-sample-143-collides-with-seatfw` | closed | PR 3514 |
| `teapot-walls-have-no-suite-row` | closed | recorded in its item |
| `reauthor-drops-the-sketch-plane-coordinate-of-segments-and-struts` | open → **OFFSET** (this PR) | filed by PR 3626's sweep |

## The criteria

> `Curve3::Spiric` and `Pcurve::Spiric` are landed, certified and
> exported;

**Met.**
- **PR-1a (#2566)** landed `Curve3::Spiric` with a deciding constructor
  (four named regime predicates), the dispatch census across the
  workspace, and `mint_carrier`'s kind-changing arm.
- **PR-1b (#2861)** landed `Pcurve::Spiric`. The torus wall certifies
  `SpiricIdentity` and the plane cap a closed-form statement, with four
  chart-vs-carrier gates priced into the envelope. It also added STEP
  export of a spiric edge as an export-only cubic B-spline with its
  bound stated in the file.
- Both ran the cross-model dual under the A/B protocol. Their rows are
  SP1A and SP1B in `docs/MODEL-AB-LOG.md`.

> the klein elbow hollows end to end and refuses at the props door
> with its payload named;

**Met for the sealed elbow; the opened arm stops one door earlier,
filed.** PR 3626 did two things:
- the equator seams re-author onto the moved corners' azimuths;
- the spiric rim's parameter window reads forward under a named decide,
  where before the sign of a zero at the `atan2` branch cut picked it.

The sealed klein elbow now hollows to check 7: `NotValid {
VolumeUncomputable { Face { …, Unimplemented } } }`, at a cap bounded by
a spiric. That payload is named in all five elbow rows (`torax_axial`,
`spiric_rim`, `shell7_seam_corner`, `torax_interval`, and `verbs_shell`'s
sealed arm).

`verbs_shell`'s opened arm (`shell_open`) instead stops at SHELL's lift
stage: `ShellError::Lift { … ReanchorOffCarrier … }`. The lift's door
does not classify the lifted solid as axial, so it falls to the
per-chart door, which misses at a curved junction. That is SHELL's
ground and is filed there
(`work/shell/shell-open-lift-takes-the-per-chart-door-on-the-klein-elbow.md`).
The criterion is met in the form CURVED can meet it. Ev may read the
opened arm as an unmet half.

> `docs/CURVED-SPIRIC-SPEC.md` is deleted per the ledger;

**Met.** PR 3622, with `docs/doc-ledger/curved-spiric-spec.md` naming
the SHA it is recoverable at and two spec errors found in delivery.

> the C9 ring `sqrt` question is opened as an `[ev]` conversation (on
> TANG if unasked at the walk);

**Met, and answered.** Restated for the tree after RING-3, the question
was weighed by an Opus and a Fable designer (design-fork row 30) and
put to Ev as PR 3517. Ev approved it: C9's operation list admits `√`.
The torus turned out to need no root, so its certificate is CHART's
ordinary work (`torus-certificate-runs-root-free-through-its-quartic`).
The door itself is LINALG's (`certification-gains-a-sqrt-door`).

> the walk convention applies.

**Met** by this file.

## Residue, re-homed by file before the sweep

- **2026-09-20 cut:** REACH took 49 rows (the boolean lanes and S-BOOL's
  residue), TANG took 8 (declared tangency, germ and pierce, the pinch
  design), and CHART took 3 SSI drive-bys along with TRIM's chart side.
- **Since the cut:** `topo-mints-indeterminates-outside-the-funnel` →
  REACH; `spiric-step-spline-bound-is-second-order` → EXPORT;
  `c5-plane-torus-cone-cylinder-arms` → GERM (the lily's plane×torus germ
  frame is live there; the Klein demo half waits on the two doors
  below).
- **Filed at the close:** `spiric-bounded-face-area-is-unimplemented`
  (PROPS) is the props door the sealed elbow and the vessel stop at. The
  SHELL lift row is the opened arm's door.
- **In this PR:** `reauthor-drops-the-sketch-plane-coordinate-of-segments-and-struts`
  → OFFSET, which owns `offset_axial.rs`.

`docs/CURVED-SPIRIC-DESIGN.md` stays. It is ratified text (Ev's rulings
on #1858). PROPS's new row cites its Q5 for the props lane, and its
Q3(ii) block note was corrected by PR 3517.

## The A/B record

CURVED ran blocks CURVED-B1 and CURVED-B2 under the model A/B protocol,
and both concluded with every slot executed. Both are folded in
`docs/MODEL-AB-LOG.md`. Band 2200–2299 stays claimed, with 2205 its
last ordinal. The seam unit ran after the protocol's 2026-09-23
suspension, on a single full Opus review.

## On ratification

`work/curved/` and this walk are deleted in the sweep, with a note
under `docs/doc-ledger/` naming the SHA they are recoverable at.
