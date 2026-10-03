# ZIP — the plan

The declared-REST zip and the seam zip: closing the boolean's seams
once the join has matched them.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size), and cut again on 2026-10-02 along its layer seam: JOIN took
the join and FUSE took the merge door and the rebuild. Nothing dispatched.

## The slate

**27 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `rest-zip-seam-chord-on-cylinder-wall` | H | the declared-REST zip leaves a straight seam chord on a bore wall |
| P0 | `a-round-tube-standing-on-a-plate-refuses-seam-orientation` | H | a round tube on a plate refuses `SeamOrientation` in every order |
| P1 | `a-boss-flush-with-a-block-edge-refuses-its-declared-union` | H | a flush boss refuses its declared union: seam chord between two isolated pierce points |
| P2 | `a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence` | E | every REST zip refusal ends on "declare the coincidence" |
| P3 | `a-boolean-reports-one-undeclared-contact-per-refusal` | M | n undeclared contacts cost n round trips |
| P3 | `rest-zip-drops-the-euler-operators-refusal` | M | the REST zip discards the Euler operators' own refusals |
| P3 | `slit-zip-band-run-across-two-loops-is-reached-by-no-row` | M | `slit_zip`'s two-loop band run is reached by no row |
| P3 | `the-seam-zips-kef-leaves-the-wall-it-closes-half-minted` | M | the seam zip's `kef` leaves its wall half-minted |
| P3 | `unclaimed-half-edge-read-as-a-minus-half-in-zip` | E | an unclaimed half-edge read as a minus half |

## Order

`rest-zip-seam-chord-on-cylinder-wall` first, because it is the one
geometric wrong answer here: a straight chord stored on a cylinder
wall. `a-boss-flush-with-a-block-edge-refuses-its-declared-union` meets
the same seam-chord frontier on a planar boss and probably belongs in the
same lane. `a-round-tube-standing-on-a-plate-refuses-seam-orientation`
(`zip.rs`'s seam pairing) is independent and can run in parallel.

`rest-zip-drops-the-euler-operators-refusal` and
`a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence`
are both about what the REST zip's refusals say, and are cheapest as
one lane. `unclaimed-half-edge-read-as-a-minus-half-in-zip`'s `join.rs` site,
`ring_run_ccw`, closes with JOIN's
`ring-run-winding-is-a-second-spelling-of-the-loop-winding-sum`;
its `rest.rs` site is this track's.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md`; the log names each tier and its
reason.
