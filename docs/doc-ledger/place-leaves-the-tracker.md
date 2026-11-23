# PLACE leaves the tracker — 2026-10-03

PLACE finished the placement slate after P2-core: split and inline at a gauge, and the mate frame they cross with. It opened 2026-10-02 at EDIT's exit (`edit-leaves-the-tracker.md`) and closed 2026-10-03 when its slate was empty. Its plan set an `## Exit shape` and no `## Exit criteria`, so under `work/README.md`'s closing rule no exit walk is owed.

Sweep SHA `24612a13bf94a529813c9559cb3c7a48d3e07d0f`.

    git show 24612a13bf94a529813c9559cb3c7a48d3e07d0f:work/place/<FILE>

**What landed** (the PR numbers are the record):
- Split and inline refusals state their recourse (#3872).
- Document order is read from the document, not off digest ids (#3882).
- P2-carry: split and inline keep a carried member's checked offset (#3885).
- P2-split, the answer-independent part: the anchor vote with gauges in the cut, `SeveredGauge`, the verbatim move of a cut holding gauges (#3908, DR-55).
- P2-retire: split moves the cut as selected, with no hoist and no inline sugar; `DocEdit::Promote` and `Fold` carry the convenience, per Ev's ruling (i) on `[ev]` #3888 (#3930, DR-58).
- P2-face: a face mate side names no face and crosses with its head; `Rebind` repairs it by construction (#3934, DR-61).
- Root order as A10 and A4 state it, per `[ev]` #3939 (#3946).
- A mate frame is a base and an offset, `MateFrame { base: Part | Face, offset: Placement }`, per `[ev]` #3920; the tour's crate follows the shelf (#3961, DR-64).

Ev's three forks this program raised are rows 51, 53 and 55 of `docs/DESIGN-FORK-LOG.md` (`[ev]` #3888, #3920, #3939). They are ratified into `crates/editor-core/ASSEMBLY.md` A3, A4, A10 and A11 (5).

**Where the build departed from the spec** (`edit-placement-spec.md`):
- The group hoist was specified for P2-split and never built. Ev's ruling (i) moves the cut as selected, and Promote and Fold do the rest.
- The face side's `SetMateFrame` was planned and dropped, because a face side names no face.
- The mate frame grew an offset (`[ev]` #3920), which the spec did not foresee.

**Residue:**
- `placement-step-slots-are-spelled-three-ways` (P3) went to RECIPE, whose ground the slot alphabet is. It falls under the intent refactor's hold (`[ev]` PR #3990).
- P3, the viewer's group-wide probe and place-where-shown, is OFFER's: `viewer-free-move-and-place-where-shown-over-a-whole-group`.
- Filed on MSOLVE during the program: `a-declaring-mates-alignment-is-never-read` and `a-mate-frame-axis-is-decided-against-a-length-band`.

No PLACE row was left open in the closed directory.
