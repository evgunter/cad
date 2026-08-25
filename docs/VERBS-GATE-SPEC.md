# VERBS-GATE — the boolean operand gate goes pair-scoped

Wave 2 unit 1 of `docs/VERBS-PLAN.md` (row 6). Branch `verbs/gate`,
PR to main. Difficulty logged pre-dispatch: **M**. Consumes block
VERBS-3 slot 3.

## The defect (the register's wall-3/4 finding)

The curved-boolean operand gate refuses per-face-KIND over the
WHOLE BODY: one cone or torus face anywhere makes every boolean
unavailable to the body, even when that face could never
participate in the cut. Klein wall 3: the bottle's three pieces
cannot be UNIONED (`CurvedBooleanUnsupported { kind: Torus }`)
though the faces that actually meet are supported kinds. Evan's
steering (recorded at plan row 6): lily wall 7's sphere×sphere
subtract refuses at this gate BEFORE the cut is attempted.

## The ruling (this spec's design call, per the plan's mandate)

**"Genuinely intersects" is decided at BOX-LEVEL conservatism**: an
unsupported-KIND face disqualifies the operation only if its
bounding box overlaps some face-box of the other operand. Boxes
over-approximate, so the gate may still refuse pairs exact geometry
would admit — conservative in the CORRECT direction (never admits a
pair the crossing pipeline can't handle) — and a face whose box
touches nothing of the other operand provably cannot enter any
crossing, so its kind is irrelevant. The refusal payload goes
pair-scoped: it names the offending KIND PAIR and both faces, not
the body ("face X (torus) may intersect face Y (plane); that germ
class is unimplemented"), with the box-conservatism stated (the
overlap is a may, not a does).

Same-body unsupported kinds that face each other (self-intersection
style operations) follow the same rule where the operation
consults them; where an operation never examines same-body pairs,
say so rather than gating vacuously.

## Riders (the box machinery this leans on — fix, not inherit)

- **#862**: the axial-slab arm widens by `radius` on EVERY
  coordinate including the axial one (37% over, measured), and its
  containing extent feeds false `CensusUndecidable`; second defect
  at the same lines — the axial projection reads single bracket
  endpoints, so under Interval the slab is built around one
  arbitrary endpoint. The gate's precision rests on these boxes:
  fix both here, re-verify the measured case, close #862.
- **#700**: census.rs re-derives boolean::boxes' min/max on a
  premise D1 killed — the sibling dedup; sweep it with #862's fix
  (one home for the box derivation). Close #700 if the dedup is
  complete; otherwise state what remains.

## Fences

- **No new germ classes** — cyl×cyl, cyl×sphere, sphere×sphere,
  cone/torus lanes are their own units (plan rows 7–10). The gate
  re-scope makes their absence honest, not smaller.
- The crossing pipeline itself untouched: everything downstream of
  the gate runs exactly as before on admitted operand pairs
  (bit-identical on previously-admissible operations — the cheap
  proof: existing boolean suites unchanged and green).
- The void-insertion door untouched.

## Acceptance

- **Klein wall 3 flips**: the bottle's pieces union (their meeting
  faces are supported kinds; the torus walls' boxes clear the other
  operand) — or, if their boxes DO overlap, the refusal names the
  actual pair and the wall re-pins to that honest form; determine
  which by building, not assuming, and say which in the PR.
- **Klein wall 4 stays refused** (the neck genuinely crosses the
  body wall through torus faces) — re-pinned pair-scoped.
- **Lily wall 7's refusal becomes pair-scoped** (the two spheres
  genuinely intersect; the germ class is row 9's): the wall's text
  updates to the true refusal, and its retirement text (recorded at
  plan row 6) still executes only when row 9 lands.
- A constructed admit case: a body carrying an irrelevant torus
  face unions with a plane/cylinder body whose faces clear it —
  red-able by reverting the gate.
- #862's measured case re-verified tight; planted-corruption rows
  for both box defects; existing boolean suites bit-identical.
- Note the drawn CI point in the PR body.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft. Merge origin/main before
opening the PR titled "VERBS-GATE: the curved-boolean operand gate
goes pair-scoped (closes #862, #700)"; confirm CI runs STARTED;
watch to completion. Do not merge.
