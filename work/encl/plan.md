# ENCL — the plan

certified enclosures: what a certificate claims, and what it is worth

## Where the slate stands

Every row the program could act on without another program has been worked.
Those rows have landed through a series of merged PRs, which the log names one
by one. What is left falls into three groups.

**Waiting on PROPS' `f64-refinement-inside-an-enclosure-has-five-more-sites`.**
The fix is the convex form of `insert_once_ring` in
`crates/geom-core/src/spline/compose.rs`. ENCL does not take that site; it
belongs to PROPS, and PROPS has been told on its log twice.

| pri | item | why it waits |
|---|---|---|
| P0 | `offset-fit-at-tight-eps-refuses-every-curved-nurbs-chart` | the certified bound's floor is that site's insertion width |
| P1 | `patch-bound-offset-fit-recentring-origins` | measured once the insertion width is gone |
| P2 | `one-pass-refinement-would-cut-the-rational-bounds-widening-tail` | the same enclosure-width family |
| P3 | `a-rigid-map-still-refuses-the-bowed-approx-fixture-at-eps-1e-12` | the frame-dependent width is the same insertion width (measured: the convex form alone gives 0 of 93 refusals) |

When that fix lands, the P0 is taken first. It re-measures the offset fit's
tight-ε floor and its budget numbers.

**Waiting on SHELL's `shell-refuses-every-lofted-body-at-a-wall-seam-carrier`.**

| pri | item | why it waits |
|---|---|---|
| P3 | `a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters` | latent until a body with a curved `Approx` face and edges can be moved. It is a design fork, weighed by two designers once it is reachable. |

**Open and in flight.**

| pri | item | state |
|---|---|---|
| P3 | `certify-escalation-renders-the-coincidence-menu-unlabelled` | a lane is running (style review) |
| P4 | `domain-grid-homing-residue` | open. It is the residue left by the grid homing, and it is taken if the track has room. |

## Review posture

The review tiers are those of `memories/orchestration-model.md`, and every lane
runs on Opus. The log names the tier for each unit at dispatch.

## From STACK (merged 2026-11-20)

what a certified enclosure CLAIMS: the stackup's hull, the non-real contract and the forgeable certificate family

Opened 2026-09-20 by ENCL's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

### The slate

**17 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `certified-hull-padding-is-the-leaf-width-not-the-lane` | H | a stackup's certified worst-case hull pads by the LEAF width: a tier that certifies a study in fewer leaves reports a wider hull at the same leaf budget |
| P2 | `certified-lane-non-real-contract-audit` | D | Contract - what a certified enclosure lane owes when a value goes non-real (poison absorbs vs widens) |
| P2 | `contribution-bounds-via-dual-interval` | D | E5 contribution bounds from Dual<Interval> derivative enclosures (M10-4 deviation 3) |
| P3 | `a-certified-leaf-that-diverges-on-replay-refuses-typed-as-a-kernel-defect` | E | stackup's LeafDiverged refuses typed a replay-identity break D9 calls a kernel bug; the D9 test has not been asked of it |
| P3 | `stackup-pairing-reads-the-shells-dual-refusal-as-a-violation` | M | whether DL3's designed dual refusals (the shell's) read as `Unliftable` rather than a refused measure |
| P3 | `certificate-types-have-public-fields-and-are-forgeable` | D | MassProperties and the certificate family (PcurveCertificate, certify::Certificate, SsiCertificate, OffsetCertificate, CertifiedLeaf) have public fields — a downstream crate can forge a certified claim nothing computed |
| P4 | `in-crate-census-hand-writes-the-set-comparison` | E | stackup.rs's one-example-per-Unavailable-arm census hand-writes the roster comparison |

### Order

`certificate-types-have-public-fields-and-are-forgeable` first,
because it is the cheapest and it is the one row whose fix
(constructors, private fields) makes the other three easier to state:
once a certificate can only be minted by the lane that earns it, what
the lane OWES is a question with a place to live.

Then `certified-lane-non-real-contract-audit`, which is that question
written down, and the two frontier rows behind it.

### Review posture

OPEN, for this program's first dispatch. ENCL inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
