# FORK-S5Q — what quiets an at-rest finding (designer A)

## For Ev

**Recommendation.** A finding quiets only under an assertion that commits to what the finding observed. A finding observes the sign of C5's signed gap: a contact observes g = 0 between two faces, and an interference observes g < 0 over a region. So:

1. **Same measure: only `Gap`, read directly.** The assertion's value must be the output of a `Gap` measure, with no definition in between. The measure's two selections must be faces of the finding's two world copies, and the faces must be **opposed**: their material sides face each other. `Distance`, `MinClearance` and `Angle` quiet nothing. *(sure)*
2. **No straddling: what the assertion admits lies inside the finding's stratum of g.**
   - A contact (g = 0) is quiet only under `= b` with b decided Zero.
   - An interference (g < 0) is quiet only under `≤ b` or `= b` with b decided Negative.
   - `≥` never quiets, and neither does `≤ 0`.
   - The verdict must also hold in the same evaluation. *(likely; sure on `≥ 0` and `≤ 0`)*
3. **Same site: the assertion quiets the part of the finding its gap describes.**
   - **Contact:** each of the finding's two faces is either the asserted face of its copy or a face structurally one with it: same carrier canonical form and same material side, as decided by stage 4's door.
   - **Interference** (one finding per connected overlap): every face piece bounding the overlap must lie, closed, between the carriers of one holding asserted pair. *(likely)*

**Premise check.** The three phrases are one question: what does a finding observe, and when does an assertion say the same thing? C5 already answers part of it: "the census classes are the strata of g's zero set", and `gap` is "the ONE sign convention" (`eval/measure.rs`). A finding *is* a sign observation of `Gap`. Once that is stated, two of the three phrases follow:
- "same measure" means `Gap`;
- "does not straddle zero" means "inside the finding's stratum".

Only "same site" is a real design choice. A rule that does not start from the gap's meaning goes wrong in three ways:
- **`Distance = 0` would quiet a clearance fit.** On two cylinders, `Distance` is the axis offset (`eval/measure.rs` `distance`), so `Distance(bore, pin) = 0` says "coaxial" and holds for a clearance fit.
- **`MinClearance = 0` would quiet interference.** It is 0 for touching bodies and for overlapping ones alike, so it straddles in sign.
- **A `Gap` over aligned faces would quiet almost anything.** For two tops facing the same way, or two pins, the region "between the carriers" is a half-space and not a thin shell. `Gap` itself documents that an aligned pair is "a flush/containment configuration, not a mate".

### Ratified text I would change

**D10 Assertions, last sentence** (ratified through #4220; I could not trace whether these words are yours or agent text approved in passing). The current sentence:

> A finding is quiet exactly when an assertion on the same measure at the same site has a bound the observation meets and that does not straddle zero: …

Proposed wording:

> A finding observes the sign of the gap (C5) between two copies, and is quiet exactly when holding assertions say the same: each reads a `Gap` over an opposed pair of the two copies' faces, admits only values of the finding's sign (`=` zero for a contact; `≤` or `=` a negative bound for an interference), and covers the finding's site — a contact's two faces, or faces structurally one with them; every face bounding an overlap lying between the carriers of one such pair.

This changes no decision in D10. Its own examples ("that the gap is zero", "a bound on one side of zero") already say this. What it replaces is the general phrase "does not straddle zero":
- `≥ 0` does not straddle zero, yet it must not quiet a contact;
- `≤ 0` arguably does not straddle zero either.

The principle underneath is the stratum, so the text should say stratum.

**Secondary: spelling zero** *(likely)*.
- Today "the gap is zero" can only be written with a free `Length` variable valued `0 mm`, because a dimensioned literal is banned and the constant `0` is a `Scalar`. That variable is editable, can be toleranced, and the GUI offers to share it with every other `0 mm`.
- By VR6's own rule ("the spelling decides the meaning"), contact is the shape of the statement, not a value from a family.
- I would let the exact constant 0 stand at any dimension, since it is the one length that is the same in every unit. That means a line each in VR5 and D10 Variables ("the only constants are …").
- Without that change the rule still works: the stratum test reads b's sign at rest.

### The three choices, as final states

**Same measure.**
- *(a, recommended) `Gap` only, read directly.*
  - Unrepresentable: quieting by an unsigned measure, and quieting through a definition. `−gap ≥ 5 µm` must be written as `gap ≤ −5 µm`, which is one way to say it.
  - Cost: a user who wrote the definition rewrites it.
  - The coverage limit is `Gap`'s arms:
    - pairs it covers: parallel planes, concentric spheres, coaxial cylinders;
    - pairs it does not cover: cylinder on plane, a vertex or edge on a face, a box corner in a cylinder, cones, tori, NURBS.
  - A contact or interference on an uncovered pair stays **loud forever**, and its recourse is to construct it (a mate). That is the fail-loud direction. It grows by adding `Gap` arms (edge×plane is natural, since `Edge` selections exist), never by loosening the rule.
- *(b) Allow affine definitions over one `Gap`.* This needs an inverse-image calculus on `Expr` for a convenience. I reject it, but it can be added later because it only quiets more.

**Same site, contact.**
- *(a) Exact face identity.* A block resting across a plate whose top is split by a groove into two coplanar faces needs one assertion per face.
- *(b, recommended) Structural carrier identity per copy.* One assertion covers both faces when the plate's two tops are one construction. If they are coplanar only by value, they stay separate. This reads stage 4's canonical forms and adds no new mechanism.
- Both refuse an assertion over the **unplaced bodies**. Construction-frame carriers are a different object from the copies' carriers, and agreeing today agrees by value. *(likely)*
- A later rung could accept bodies whose two placements are structurally one pose (an instanced subassembly), and that would only quiet more.
- Cost at rest: for each finding, look up assertions by copy pair through `Doc::reads`, then read the verdict, the bound's sign and the canonical-form equality. All of it is symbolic, with no geometry.

**Same site, interference.** Consider a flanged bushing pressed into a housing (r 5.01 into r 5), whose flange is mis-dimensioned so that it sinks 0.1 mm into the housing's top. The whole interference is one connected overlap.

| Rule | Pressed bushing, flange right | Flange sunk 0.1 mm | Pin also cuts a second feature |
|---|---|---|---|
| (a) copy pair | quiet | **hidden** | **hidden** |
| (b) both asserted faces bound the overlap | quiet | **hidden** (both faces still bound it) | loud |
| (c, recommended) every bounding piece lies between asserted carriers | quiet | loud: the housing-top piece at r > 5.01 is outside the shell | loud |
| (c), with a second assertion `Gap(top, flange_bottom) ≤ b` | quiet | quiet (intended, and stated) | loud |

- (c) is "same measure at the same site" made literal. `Gap(f, g)` describes exactly the shell between f's and g's carriers, so the assertion explains the part of the overlap inside that shell and nothing else. With opposed faces, the shell's thickness is |g|.
- It represents several intended overlaps in one component, with one assertion each.
- What it cannot tell apart: where, inside the asserted shell, the overlap sits. A lid's skirt rubbing laterally inside the asserted z-band reads as the asserted overlap, because `Gap` does not measure lateral extent.
- Cost: it needs the overlap computed with face provenance (FORK-S5-2's per-component intersection). After that, for each bounding piece not on an asserted carrier, it needs an enclosure of the two carriers' material-side signed distances over the piece, decided margined. Any piece that is undecided, or covered only by a union of shells, stays loud. This runs only on pairs with a candidate holding assertion; every other overlap is loud with no work.
- (b) costs one name lookup but hides the sunk flange.
- Moving from (c) to (b) later only quiets more. Moving from (b) to (c) re-louds findings people have stopped seeing, so (c) is also the reversible choice.

**Worked example: the pin in the bore** (pin radius 5.01 through a 5 mm bore).
- The overlap is the shell 5 < r < 5.01 through the plate's thickness. It is bounded by the bore, the pin's cylinder, and annuli of the plate's top and bottom, and each annulus lies between the carriers.
- `Gap(bore, pin) ≤ −5 µm` holds, and the finding is quiet.
- These stay loud:
  - `≤ 0`: the bound is not Negative;
  - `≤ −20 µm`: Violated;
  - `Gap(pin, bore)`: the roles are swapped, the value is +0.01, so it is Violated;
  - an assertion over the unplaced bodies;
  - `Distance = 0`.

**Confidence:**
- recommendation overall: *likely*;
- `Gap`-only and the opposed requirement: *sure*;
- the stratum test: *sure* for `≥`, `≤ 0` and `Distance`, *likely* for b decided at rest;
- interference rule (c): *likely* on semantics, *unsure* on the enclosure cost over curved pieces (measure it on the gallery heat sink).

## For the orchestrator

- **Errors in the spec (§1, §4, test 11).**
  - "a holding `Gap = 0` or `Distance = 0`" is wrong: `Distance` on cylinders is the axis offset, so `= 0` holds for every coaxial clearance fit. Drop `Distance`.
  - §1 and §3 do not require the asserted pair to be **opposed**. Without that, an aligned `Gap` (two tops, or two pins) quiets a half-space's worth of overlap.
  - The spec writes `Gap = 0`, but no Length constant 0 exists under VR5/VR6. Say how zero is spelled, or adopt the secondary change above.
- **Interplay with FORK-S5-2.** Rule (c) needs per-component overlaps with `FromA`/`FromB` face provenance. If S5-2 falls back to witness localisation (its (c)), interference cannot be quieted at all. That is safe, but it should be stated, not left to discovery.
- **Lower-dimension contacts** (a vertex or edge of one copy on a face of another): stage 4's `Contact { a: FaceSite, b: FaceSite }` cannot spell them. Under this rule they are loud until `Gap` gains edge arms. There is no `Vertex` selection kind, so a vertex contact is quietable only by a mate. Stage 4's record shape should carry cells, not only faces.
- **Not checked.**
  - Provenance of D10's sentence: `git log -S` returns graft and merge noise in this checkout, so I relied on the brief (#4220).
  - Stage 3 and 4 shapes (unpushed). I assumed that copies are world placements' `Body` outputs and that stage 4's door can be asked "are these two faces' carriers one construction".
  - The soundness premise of (c): boundary-in-shell implies region-in-shell. It holds because, with opposed faces, each bounded complement component of the shell lies outside one copy's material. That is argued, not proved.
- I did not read §11.
- **Off-question.**
  - An assertion inside a part that is instanced several times cannot quiet its copies' findings under this rule (one assertion is needed per copy pair). This is worth a row when stage 3 lands.
