# BAND — the plan

the blend and rim bands: what a roll builds over a rim, and what it may call the result

Opened 2026-09-20 by CARVE's priority-seam cut
(`work/README.md`, Track size). Split 2026-10-26 along its own priority
seam: its P3 tail of conservative refusals and wide meters went to
BANDRULE, BANDREACH and BANDMETER, and three chain-walk and corner rows
to CARVEREST (`work/band/log.md`).

## The slate

**20 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-concave-fill-and-a-whole-shell-inside-a-bands-region` | H | step 4 of the band-feature rule: whole shells inside the region, and the seal |
| P2 | `a-rim-split-at-its-carriers-anti-seam-refuses-at-interval` | M | a foot at a curved rim's anti-seam reads the whole turn at Interval |
| P2 | `a-knife-edge-plane-blend-divides-by-one-plus-the-normals-dot` | M | `plane_plane_blend` divides by `1 + n_a·n_b`, which cancels at a knife-edge wedge |
| P2 | `blend-arms-perp-unit-is-an-undecided-gram-schmidt` | E | `perp_unit` is a hand Gram-Schmidt on an undecided normalize |
| P2 | `plane-sphere-blend-spine-radius-by-differences-of-squares` | E | `plane_sphere_blend` takes two radii as differences of squares |
| P3 | `blend-rim-and-seam-walks-read-a-ring-by-an-unproven-cycle-walk` | E | the rim split and wall seam take a ring's members from an unproven walk |
| P3 | `unclaimed-half-edge-read-as-a-minus-half-in-band` | E | a half-edge its edge does not claim is read as the minus half |
| P3 | `a-blend-of-no-edges-returns-its-input` | E | the fillet and chamfer doors return their input for an empty edge list (`design`) |
| P3 | `swept-operand-merge-repairs-left-in-tests` | E | test repairs of swept operands a run-wall sweep made no-ops |
| P4 | `blend-split-rows-and-must-carry-prose-residue` | E | split-row spellings and must-carry prose copies |
| P4 | `blend-surgery-invariant-ends-in-a-tag` | E | `SurgeryInvariant` calls itself a kernel bug in a typed refusal |
| P4 | `surgery-cites-an-unratified-junction-end-pick-precedent` | E | the bounds-lane reads cite an unratified precedent |
| P4 | `swept-run-walls-style-residue` | E | PR 3736's delta-review style residue |

## Order

Step 4 of `a-concave-fill-and-a-whole-shell-inside-a-bands-region`
first if the track is picked up for its rule: whole shells (a void in
`R`, an island in `F`) and the seal (a through-hole turned blind, a
blind hole turned void) share one carve, `topo::insert_void`'s
bookkeeping, and `docs/DESIGN.md`'s void-insertion convention already
names the blend's seal as a producer. It costs H and gets a concurrent
dual review. The two surgery walk rows
(`blend-rim-and-seam-walks-…`, `unclaimed-half-edge-…`) read the same
rings step 4 carves and ride well beside it. The two P2 E rows are
independent and cheap; the anti-seam Interval refusal should share one
helper with `surgery::arc_window` (its item says so). The knife-edge
row is probed for reachability before anything is built. The P4 rows
have no order; take one when a unit touches its file.

## Review posture

Per unit, by the review tiers in `memories/orchestration-model.md`;
the model A/B the opening text inherited is suspended, so there is no
triage question left to answer. Each dispatch names its tier in the
log.

## BANDRULE's slate, merged in 2026-11-02

BANDRULE merged into BAND on 2026-11-02 (`docs/doc-ledger/bandrule-leaves-the-tracker.md`). Its plan follows as it stood, headings one level down; its rows are now in `work/band/`.

what a band refuses that its rule would build, or refuses under the
wrong name

Opened 2026-10-26 by BAND's priority-seam split (`work/README.md`,
Track size). Nothing dispatched.

### The slate

**26.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-feature-inside-a-region-that-is-no-disk-or-annulus-is-not-carved` | M | a feature wholly inside a straight band's region that is neither a disk nor an annulus refuses `WhollyInside` |
| P3 | `a-feature-inside-a-turns-overrun-is-not-carved` | M | a bore or bar wholly inside a turn's overrun refuses |
| P3 | `a-cup-on-a-strip-beside-a-band-not-cut-off-at-both-ends-is-not-carved` | M | a cup on a strip beside a band that is not one link cut off at both ends refuses at the screen |
| P3 | `a-cup-leaning-off-the-spine-under-a-fillet-is-charged-its-boxs-diagonal` | M | a leaning cup under a fillet is charged its box's diagonal |
| P3 | `a-ruled-bands-cup-has-no-core-to-read-its-far-faces-in` | M | a cup inside a ruled cut-off's region refuses as a bound |
| P3 | `a-blend-refuses-a-touch-in-its-strip-so-no-real-touch-reaches-its-consumption` | M | a touch inside the strip refuses before any carry |
| P3 | `a-bands-core-depth-reads-no-speed-for-a-spiric-or-spline-crease` | E | the core depth reads no speed for a spiric or spline crease |
| P3 | `chamfer-ends-in-a-curved-end-face` | M | a chamfer's straight band refuses a curved end face whose section is exact |
| P3 | `ruled-band-refuses-a-joint-on-one-support-pair` | M | a ruled crease split by a valence-2 vertex refuses as junction carry-through |
| P3 | `unmerged-subdivided-wall-rim-fillet-names-no-merge-recourse` | E | a rim split across two same-key wall faces names no merge |
| P3 | `dependent-normals-names-a-run-out-policy-the-tag-says-cannot-help` | E | `DependentNormals` names a run-out policy its own doc rules out |
| P3 | `dependent-normals-refusal-carries-no-margin-for-its-ending` | E | `DependentNormals` carries no margin for its valued tighten |
| P3 | `blend-two-geometry-refusals-meet-no-shape` | M | two `UnsupportedGeometry` sites render a shape recourse for a fact it does not address |

### Order

The six feature rows are the band-feature rule's remaining shapes
beside BAND's step 4 (whole shells and the seal). Read step 4's
state in `work/band/` before dispatching one, since both extend the
band's carve of what lies inside its region. `a-ruled-bands-cup-has-no-core-to-read-its-far-faces-in` and
`a-bands-core-depth-reads-no-speed-for-a-spiric-or-spline-crease` both
read the band's core and may share a unit. The two `DependentNormals`
rows are one unit, and the policy half is corner-taxonomy ground
(OQ6): weigh it before building. `chamfer-ends-in-a-curved-end-face`
lands after the run-out row's local planar carve, as its body says.
The rest have no order.

### Review posture

Per unit, by the review tiers in `memories/orchestration-model.md`.
Each dispatch names its tier in the log.
