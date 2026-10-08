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

## Round 2

I adopt B's framing: a holding assertion quiets a finding when it **entails** it, meaning everything the assertion admits lies in the finding's stratum. "Entails" is the right word for the principle my report called "commits to the stratum". Taking that principle seriously settles point 1 in my favour, and I move on point 3.

1. **Which measures can quiet: hold (`Gap` only, over an opposed pair).** The argument is B's own entailment test, applied per arm.
   - **`Distance` on two cylinders.** It is the axis offset, so `Distance = 0` holds for a coaxial clearance fit as much as for a contact. It no more entails contact than `≥ 0` does, which B rejects for exactly that reason.
   - **`Gap` over an aligned pair** (two tops, two pins). The sign of the gap does not mean overlap. A block sitting with clearance in a pocket 1 mm below the plate's top gives `Gap(A.top, B.top) = −1 mm`, which would entail "interference" where nothing overlaps.
   - So B's stratum table, filled in honestly, has a single row: `Gap` over an opposed pair. B's argument that "the verdict carries the role check" covers swapped roles, not aligned faces.
   - I keep B's home for it: a stratum entry on the primitive, beside `dim()`, keyed by arm and face sense. Any future signed primitive adds a row there.
   - **Is this a choice for Ev?** No. Each side's own principle decides it.
2. **An interference's site: hold, but this one is a choice for Ev.** B's rule (the assertion names one bounding face of each copy) hides the sunk flange: the bore/bushing assertion quiets an overlap that also holds a 0.1 mm flange sink nobody stated. B's answer is that "the faces are listed". But a quiet finding is not read, and keeping such overlaps visible is what the finding is for. There are three candidates, from loosest to strictest:
   - **B-base** (one face of each copy): hides the flange. Cost: name lookups.
   - **B-strict** (the tightening B itself names: every opposed `Gap`-measurable pair bounding the overlap must be asserted). It catches the flange at the same cost as B-base. It still misses an incursion with no `Gap` arm, such as a pin's chamfer cone biting into the bore's edge.
   - **Mine** (every bounding piece lies between the carriers of one holding asserted pair). It catches both. Cost: enclosures over the bounding pieces, measured, and only on pairs that have a candidate assertion.
   - **I would accept B-strict as the build** if mine measures too slow on the heat sink. **I would not accept B-base.**
   - Reversal costs are asymmetric. Loosening later quiets findings deliberately, at the cost of a line of text. Tightening later turns loud, in authored documents, overlaps that were real all along.
3. **A contact's site: move to B (exactly the two cells the record names).** B's objection was to incident faces, which I never proposed. My structural-carrier widening only quiets more, so it is the safe one to add later as a separate step. Starting exact costs one extra assertion per coplanar face that is split by a feature. That is not worth bringing stage 4's door into stage 5.
4. **An overlap the kernel cannot intersect: hold (a loud interference that no assertion can quiet, naming the intersection's refusal); a minor choice for Ev.** The census did look, and it decided the overlap exists; only localization failed. Calling that "could not look" is wrong in fact. It would also refuse at Warn a decided overlap that the user cannot fix: for example, a press fit whose carrier pair has no join arm. Either side is a one-line change to reverse, and the difference is only which severity refuses the finding.
5. **Spelling zero: hold, as a separate small proposal.** It does not block this fork: until it lands, "zero" is a free `0 mm` variable, and the stratum test reads its sign at rest. B did not argue against it.

**The D10 sentence I would now propose:**

> A finding is quiet exactly when holding assertions entail it: each reads a measure's output directly, over cells of the finding's two copies, and admits only values that mean what the finding observed — today only C5's `Gap` over an opposed pair has such values: `= 0` for a contact, `≤ b` or `= b` with `b` negative for an interference. A contact's cells are the two the census found coincident; an interference is one connected overlap of the two copies' material, quiet when every face bounding it lies between the carriers of an asserted pair; an overlap the kernel cannot bound is loud.

## Round 3

### 1. Where one site splits into several regions, and what the converged rule does

The census decides one pair of cells at a time and never counts regions:
- curved conformal contact is one `ConformalPatch` per face pair ("trim regions definitely overlap");
- planar contact is vertex- and edge-level records;
- interference is the first witness vertex, or a pierce.

The intersection (point 4's localizer) gives one lump per connected overlap, with `FromA`/`FromB` face names.

| Case | Census today | Intersection | Converged rule |
|---|---|---|---|
| a. U-shaped block resting on a plate: one face pair, two contact patches | vertex/edge records on the face pair | — | one site; `Gap = 0` quiets both patches |
| b. pin = bore radius, bore crossed by a drilled hole: one face pair, two conformal regions | one `ConformalPatch` | — | one site; quiets both |
| c. press-fit pin, bore face split into two loops by a cross-slot | witness vertex | two lumps, both bounded by the same bore and pin faces | both between the asserted carriers; quiets both |
| d. clevis: two lug bores, two faces on one carrier, pin pressed through both | witness vertex | two lumps, bounded by `bore1` and `bore2` | **both quiet under an assertion on `bore1`**: the rule tests carriers, and `bore2` lies on `bore1`'s carrier |
| e. pin also cuts a rib | witness vertex | two lumps | the rib lump is outside the shell: loud |
| f. curved × planar or NURBS faces touching twice (a wavy cam on a plate) | `CensusUndecidable` within reach | may refuse | no `Gap` arm; loud, or could-not-look |

- **Rows a–c:** the regions share both cells and both carriers. Every measure we have reads the carriers, so it gives the same value for every region. The second region is not a second fact the assertion fails to see; it is the same fact, repeated where the faces extend. Telling the regions apart is a statement about **extent**, which no current primitive measures.
- **Row d:** the converged rule quiets too much. The regions there are different cells, and the contact rule (point 3) already treats different cells as different sites. Interference should do the same.
- **Row f:** this is where the multiple regions Ev has in mind become real. It is also where no measure exists yet, and any future measure there is *local*: two NURBS faces touching twice have no single signed gap, only a local minimum near each touch.

### 2. Ev's witness, as a final state

**What a witness is.**
- A witness is a `Point` variable, free or constructed, that the part of one of the two copies defines in its own frame. It is carried by that copy's placement.
- "Unplaced" can only mean the frame of the copies' **space**, never the world (export alone reads the world, D10).
- The space's frame is the kernel's internal choice (its earliest member), so a person cannot write a point in it. A point on one of the bodies is what a person can write, and it moves with the parts.

**What the predicate reads.**
- The intersection of the two copies, computed in the space's frame.
- An assertion with positive witnesses quiets only the components containing one of them, decided strictly inside, margined.
- A negative witness turns loud any component that contains it.
- **Default with no witness:** every component the assertion otherwise matches is quiet. "Required only when there are several components" would make an edit that splits one overlap into two suddenly invalidate an assertion. As a fail-loud warning that is acceptable, but it is a requirement that switches on with geometry.

**How it interacts with the rest of the converged rule.**
- **Points 1–2 (opposed `Gap`, stratum) and point 5 (an overlap the kernel cannot intersect):** unchanged. The witness picks a component and does not replace the measure, and it needs components to test against, so a refused intersection is still loud.
- **Point 4's carrier test:** still needed. The sunk flange lies in the *same* component as any witness of the press fit, so the witness cannot see it. A witness distinguishes components; the carrier test distinguishes within one.
- **Staleness:** a witness that leaves its overlap when dimensions change quiets nothing, and the assertion reports why. Fail-loud, but a free `Point` drifts across a parameter family where a constructed one tracks the geometry.

**Cost.** One point-in-solid test per witness per lump, after the intersection that point 4 already needs. Cheap. The real cost is one more slot on `Assert`, and a third variable kind (`Point`) in a statement that has been purely scalar.

**Weighed.**
- For rows a–c, a witness narrows an assertion to one region of a single carrier relation. It says "the overlap should stop here" without saying it in geometry. That is a real intent, but it is an intent about extent, and the honest spelling of it is geometry: split the face, or assert the extent with a `Distance` requirement.
- For row d, cells already separate the regions, at no cost to the user.
- For row f, the witness has a natural home: **inside the measure, as the localizer of a local signed-gap arm**. That is a `Gap` over two general faces "near" a point, which is the only way such a scalar is well defined. That arm does not exist yet. When it lands, its point makes the measure's site and the finding's site coincide, and the quieting rule needs no change.

### Recommendation (likely)

Keep points 3–4. Close row d by having point 4 require the asserted faces themselves. Hold Ev's witness for the future local-`Gap` arm, as part of that measure, not as a slot on `Assert`.
Replacement for point 4:

> An interference finding is one connected overlap of the two copies' material. An assertion quiets it when both of the assertion's faces bound the overlap and every face bounding it lies between the carriers of an asserted pair. Regions of one overlap that share both faces are one site: no current measure tells them apart.

Point 3 stands as is (exactly the two cells the record names). It distinguishes everything except row a/b-style regions of one cell pair, which are one fact.
If Ev wants per-region intent anyway, the coherent form is the witness sketched above: an optional positive point, defaulting to all components, with no negative witnesses. A negative witness only says "this component should not exist", which a component without a witness already reports as loud.

### 3. Zero

**Where a distinguished constant is needed today.**
- **Mates** spell zero by absence: `MateFrame.offset: Option<Placement>` (`node.rs`), and `None` is the exact coincidence.
- **Angles** already have an exact zero: `0·turn` is a rational fraction of a turn.
- **Lengths** are the only dimension with no constant at all. A length zero is needed by contact assertions (`Gap = 0`), coincidence requirements such as `Distance(p, q) = 0`, and an occasional formula clamp (`max(x, 0)` refuses on dimensions today).
- **The GUI** would offer every `0 mm` free variable to share with every other `0 mm`.

So the problem is confined to length zeros, mostly in assertions. By Ev's own test, a change is worth making only if it is small and does not reshape `Assert`.

**Ev's alternatives.**
- **Every relation against zero** (`Gap ≤ 0` and so on): this loses requirements on magnitude (`MinClearance ≥ 2 mm`, a press fit's minimum interference). Adding a constant to `Gap` brings the bound back under another name.
- **Disallowing `Gap ≤ −b` only:** a press fit could then state no minimum interference. To quiet one it would need `Gap < 0`, a new strict relation that is just a stratum assertion, and `≤ 0` is already ruled out.

Neither avoids a zero anywhere else, so neither passes Ev's test.

**Recommendation (likely): the exact constant `0` stands in a slot of any dimension.**
- A lone `0` in a Length or Angle slot is the constant. `0 mm` stays a free variable, as `90 deg` does next to `turn/4` (VR6: the spelling decides).
- It does not change `Assert`, fills the one dimension without a constant, and covers formulas too.

A smaller alternative that does pass the test: make `bound` optional on `Assert`, with absence meaning exact zero, mirroring the mate offset. It is local to assertions, but it gives zero two spellings once formulas want one, so I prefer the constant.
