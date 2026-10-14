# CARVE — the plan

what a sweep verb builds and how it describes it

Re-scoped 2026-10-10, at the close of the sitting that landed six of
the seven P0 rows. The slate is what that sitting's reviews found
behind them, plus two rows other programs filed, cut at the P2/P3
seam (`work/README.md`, Track size). The P3 and P4 rows went to
CARVEREST (`work/carve/log.md`).

## The slate

**20 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door` | H | a loft section hinged on one of its own edges collapses that wall, and refuses at the attach gate rather than naming the pinned wall |
| P1 | `a-reflected-loft-placement-evades-both-normal-checks` | M | a loft section placed with a reflected frame passes both stacking decides and builds a zero-volume body that tier 3 accepts |
| P1 | `sweep-frame-is-a-minimal-rotation-from-the-start-tangent` | H +design | sweep_places carries each station by the minimal rotation from the START tangent, so the section spins near anti-parallel |
| P2 | `a-loft-caps-plane-is-summed-in-the-authored-vertex-order` | M | a loft cap's Newell plane is summed from the authored start vertex, so re-spelling a section moves the cap's bits |
| P2 | `sweep-dihedral-readers-drop-the-arm-rung` | M | extrude and revolve upgrade word an arm escalation as the wedge's sliver |
| P2 | `a-rational-wall-beside-an-integral-one-skins-its-shared-corner-off-unit-weight` | M | a rational loft wall skins its shared corner rows an ulp off weight 1, so its seam refuses against the integral neighbour |

Parked: `self-overlapping-spines-build-and-validate` (P0, H, design),
on SHELL-3's clearance certificate and CLEAR's
`window-of-refuses-an-untrimmed-iso-bounded-nurbs-patch` and
`self-intersection-drops-every-vertex-sharing-face-pair-globally`.
Its row carries the cost of the interim: the far-normal decide
(PR 4188) over-refuses the hood and the oblique arc sweep, and those
rows should build once the certificate lands.

## Order

`a-reflected-loft-placement-evades-both-normal-checks` first: it is
the one row here that builds a wrong body silently, and it lives on
the stacking fold PR 4188 just wrote. Then
`a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door`,
the same fold's other gap (the wall is refused, but at the wrong
door).

`sweep-frame-is-a-minimal-rotation-from-the-start-tangent` is a design
row: an Opus and a Fable designer weigh the frame law before anything
is built (`memories/orchestration-model.md`). Its answer decides
whether a sweep's v can become its path parameter, which the
designers of PR 4193 recommended and the orchestrator deferred to this
row; CARVETAIL's `a-half-turn-spine-sweeps-only-off-its-exact-tangents`
is the same frame's other symptom, and the weighing reads it.

Two of the P2 rows finish the loft's order-freedom. After PR 4193 the
walls and parameters are a function of the section set; these rows do
the same for the caps' bits and the rational corner rows. CARVETAIL's
`skinned-wall-weights-drift-an-ulp-along-the-stacking` is the same
skin interpolation rounding a weight that every section holds equal,
and the per-row lane the rational-wall row asks for should settle it
too.

`sweep-dihedral-readers-drop-the-arm-rung` (filed by ENCL) is refusal
truth. ENCL's PR 4450 made the reading reach `Display`, and the repair
is now `sweep::swept::sliver_text`'s arm wording plus a closed decision
per reading at each site.

## The D10 hold

None of these rows reads declared pairs, declared contact, placement
vocabulary or the node vocabulary. The sweep frame is the kernel's own
carrying law along a path, not a user's placement; a weighing that
finds it wants a user-declared frame stops and says so rather than
adding the vocabulary.

## Review posture

Protocol v7 (`docs/DUAL-REVIEW-PROTOCOL.md`), triaged per unit at its
dispatch. The last sitting's posture held: a single FULL review per
unit, because each was a confident wrong answer if wrong; the reflected
placement is the same shape and gets the same.
