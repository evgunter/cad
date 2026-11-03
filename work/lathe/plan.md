# LATHE — the plan

what extrude and revolve build, and the readings they refuse by

Opened 2026-10-31 by CARVE's fourth priority-seam cut
(`work/README.md`, Track size; `work/carve/log.md`). Nothing
dispatched.

## The slate

**30 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote` | M | a full revolve's curved wall is two π-bands joined by two seam meridians nobody wrote (ruled: one band, a wrap tip at the axis) |
| P2 | `an-extrudes-cap-carrier-is-minted-by-newell-not-by-its-frame` | M | an extrude's cap carrier is minted by Newell, not as the frame's plane D10 describes |
| P2 | `an-extrude-cap-plane-normal-reads-the-profile-variables` | M | at Sym an extrude's cap normal is A/sqrt(A²) in the profile's variables |
| P2 | `a-loft-caps-plane-is-summed-in-the-authored-vertex-order` | M | a loft cap's Newell plane is summed from the authored start vertex, so re-spelling a section moves the cap's bits |
| P2 | `the-guided-derived-frame-extrude-escalates-as-a-kernel-defect` | M | under the guided lift the derived-frame extrude's endpoint residual escalates as a kernel defect |
| P2 | `a-near-collinear-kink-on-a-full-revolve-refuses-as-a-kernel-defect` | M | a validated near-collinear kink makes the full revolve refuse at the attach gate as a kernel defect |
| P2 | `sweep-dihedral-readers-drop-the-arm-rung` | M | extrude and revolve word an arm escalation as the wedge's sliver |
| P2 | `revolve-wall-kind-escalates-in-band` | M | classify_segment refuses in band where D3 takes the cone lane |
| P3 | `a-partial-revolve-by-a-fraction-of-a-turn-ends-off-its-axes` | M | a revolve by a written fraction of a turn ends off its axes by an ulp |
| P3 | `a-smooth-joint-whose-walls-part-in-band-refuses-sliver-join` | M +design | a smooth joint whose walls part in band refuses SliverJoin |
| P3 | `sweep-revolve-decision-endings-re-spell-ending-kind-end` | C | ClearanceDecision::ending and TubeDecision::ending re-spell EndingKind::end |

## Order

The full revolve's one band first: it is the P1 row, Ev has ruled it
(PR 6014, the tip kept on PR 6085), and FOREIGN's import flip waits on
it. Its row prices it M, but it re-baselines about 191 two-band
assertions; re-price it at spec time if that holds.

The three cap rows are one question: a cap's carrier is minted by
Newell over its points where it should be the plane its frame states
(`plane(frame, direction, depth)` for an extrude, the section's
placed sketch plane for a loft). Once it is, the extrude cap's normal
reads no profile variable and the loft cap's bits no vertex order. One
unit, or two on one rule. Measure the guided derived-frame escalation on that
head before dispatching it; it may close with them.

`sweep-dihedral-readers-drop-the-arm-rung` and
`a-smooth-joint-whose-walls-part-in-band-refuses-sliver-join` read the
same dihedral arms at the same sites. The second carries a design
question and is weighed by the designer pair first. The endings chore
rides with any unit on `revolve/mod.rs` or `revolve/tube.rs`.

## Review posture

Per unit at dispatch, under `docs/DUAL-REVIEW-PROTOCOL.md` as CARVE's
is.

## CARVETAIL's slate, merged in 2026-11-02

CARVETAIL merged into LATHE on 2026-11-02 (`docs/doc-ledger/carvetail-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/lathe/`.

CARVE's tail behind its P0 slate

Opened 2026-10-06 by CARVE's second priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

### The slate

**13 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `revolve-axis-clearance-is-the-torus-convention-under-another-name` | M | revolve's axis_arc_clearance decides the ring-torus convention R - r under its own name |
| P2 | `carve-refusals-short-of-the-shape-guard` | M | refusals the viewer draws that state no recourse, by the shape guard's census |
| P2 | `copysign-stands-in-for-a-nappe-and-a-max-fold-and-hulls` | E | the nappe selector and the revolve max-fold spell a two-way choice with copysign |
| P2 | `loft-arc-columns-end-on-the-carrier-not-the-vertex` | M | an arc's NURBS column starts and ends on the carrier, not on the stored vertices |
| P0 | `the-skin-cuts-an-arc-wall-at-each-sub-arc-joint` | H | the skin cuts a spline wall at each sub-arc joint, so a lofted arc wall carries a C⁰ knot inside it |
| P3 | `revolve-angle-is-a-signed-size-beside-a-directed-axis` | E | the revolve's signed angle stays (Ev, PR 3941); its zero and full-turn refusals state no recourse |
| P4 | `tube-escalated-runs-past-the-word-budget` | E | TubeError::Escalated renders 52 words |

Once parked on the D10 hold, which lifted when INTENT closed
(2026-10-30, #6103): `half-revolve-caps-are-never-an-operand` and
`loft-walls-keyed-per-segment-on-a-declared-carrier` are open, and
`loft-between-opposite-turning-joints-reverses-a-seam-between-stations`
is parked on `self-overlapping-spines-build-and-validate`.

### Order

`revolve-axis-clearance-is-the-torus-convention-under-another-name`
first: it is the one entrenching row, and its repair shape is written.
`copysign-stands-in-for-a-nappe-and-a-max-fold-and-hulls` is the same
file and a cheap rider. The three refusal-text rows (the shape guard, the revolve angle's
recourse, the tube's word budget) can run together.

### Review posture

OPEN, for this program's first dispatch, under
`docs/DUAL-REVIEW-PROTOCOL.md` as CARVE's is.
