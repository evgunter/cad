# ZIP — the plan

The declared-REST zip and the seam zip: closing the boolean's seams
once the join has matched them.

Opened 2026-09-20 by REACH's priority-seam cut (`work/README.md`,
Track size); cut along its layer seam on 2026-10-02 (JOIN took the
join, FUSE the merge door and the rebuild) and along its priority seam
on 2026-10-06 (the P3 rows went to ZIPTAIL).

## The slate

**29.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `rest-zip-seam-chord-on-cylinder-wall` | H | the REST zip leaves a straight seam chord on a bore wall |
| P0 | `a-round-tube-standing-on-a-plate-refuses-seam-orientation` | H | a round tube on a plate refuses `SeamOrientation` in every order |
| P0 | `blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex` | M | a shaft ending inside a full-turn bore refuses in the REST zip |
| P0 | `a-flush-declared-reflex-union-ships-the-wrong-volume` | M | a flush-declared reflex union ships the overlap twice |
| P1 | `a-boss-flush-with-a-block-edge-refuses-its-declared-union` | H | a flush boss refuses: seam chord between two isolated pierce points |
| P1 | `a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin` | H | a vertex inside a Rest contact has no twin |
| P1 | `a-declared-continuation-across-a-rabbet-step-leaves-six-loose-ends` | M | an L's fold-order union refuses six loose ends |
| P1 | `survivor-folds-a-corrupt-fusion-list-onto-a-dead-key-outside-the-contact-remap` | E | `zip::survivor` guards a corrupt fusion list in debug only |
| P2 | `a-rest-zip-refusal-tells-a-declared-contact-to-declare-the-coincidence` | E | every REST zip refusal ends on "declare the coincidence" |

## Order

**The seam chord (`rest.rs`'s `mint_chord`)** is the spine. Four rows
meet it: a cap rim floating inside a face leaves a chord the zip has no
site for (`blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex`,
`a-boss-flush-with-a-block-edge-refuses-its-declared-union`), mints a
straight one where an arc is owed
(`rest-zip-seam-chord-on-cylinder-wall`, whose only reproducers no
longer reach the zip), or needs a vertex the other solid lacks
(`a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin`). They are
measured together first, and one lane or a design weighing follows
from what that finds.

`a-round-tube-standing-on-a-plate-refuses-seam-orientation` (`zip.rs`'s
seam pairing) and `a-flush-declared-reflex-union-ships-the-wrong-volume`
(the zip admitting a union that is not a pure REST contact) are
independent and run in parallel. The two E rows are one cheap lane.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md`; the log names each tier and its
reason.
