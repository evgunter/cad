# SPEC: INTENT stage 4 — the coincidence door

Scope: D10's **Coincidence** and **Booleans** paragraphs (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 4 states it:

- Carriers compare in a canonical form per kind: the frame modulo the kind's symmetry, offsets as linear forms over the variables with exact rational coefficients, and derived variables read as their formulas.
- Every coincidence decided from values is recorded at one door.
- The `unproven-coincidence` lint reports each recorded coincidence that does not hold structurally.
- Booleans glue on Zero.
- These retire: declared pairs, `ContactClass` on mates, the undeclared refusals, the axis declaration channel and the stored tangent-joint flags.

**Rulings this spec follows.**

- **D10** (#3990). Ev's words behind it are in the ruling row `work/recipe/one-way-to-say-dependency-and-intent.md`.
  - Part 2: "i'm happy to build only (2) to begin with, and extend it to (3) later if necessary". Rung (2) is canonical-form equality and rung (3) is the symbolic tier's polynomial identity. Adding (3) is one more rung at the one door.
  - Part 3: a margin decided Zero glues whether or not it is structural, so the solid is the same under every lint setting. This retires DS2's "no knob".
  - Part 4: "the lint … checks every case where a coincidence is inferred from values to ensure that it holds symbolically".
- **The mitre** (`isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`, Ev 2026-10-06).
  - Stage 4 proves a box mitre by "output definitions plus rung 3, not node theorems".
  - Stage 4 measures rung 3 on box mitres.
  - A per-node theorem is only a possible cache, added where the algebra is measured too slow.
- **The zip** (`the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms`, Ev in chat 2026-10-06). The declared-REST zip retires, and the join first builds what the zip builds today.
- **VR8** as revised at stage 1 PR C. An untoleranced variable is a constant in every *analysis* lane. The lint's symbolic pass is not an analysis lane, and binds every variable as a symbol (`unproven-coincidence-lint-binds-every-variable-as-a-symbol`).
- **Stage 2's final shapes** (`docs/INTENT-STAGE2-SPEC.md` §1) are the ground this spec writes against: operations define typed outputs, operands are reads, selections are `Select` definitions, the product is the world, and mates read `Face` variables. Each unit says which stage-2 unit it needs. At this spec's writing, stage 2 A has merged (#4295), B is in progress (`intent/s2-b-reads`), and C–F are unbuilt.
- **Stage 3** makes frames and directions variable kinds, makes a placement the bundle of mates pinning one copy, and retires gauges, offsets and `Transform`-as-placement. Two units here need it (§0). Every other unit lands before it.

**Baseline.** The grep is at main `044b5eb2e9` (2026-10-08), after stage 1 closed and stage 2 A merged. Every `file:line` below is at that commit.

**The state of the mechanisms at the baseline.**

- **The coincidence ladder is the kernel's** (`crates/topo/README.md`, the CONTACT-DESIGN preamble): "structural (shared key or same `GeomSource`) is intent by construction; declared is intent plus non-contradiction; value equality never glues". The plane ladder (`boolean/plane_eq.rs`, module docs) has four rungs:
  1. same `GeomSource` (`topo/src/source.rs:139`, an opaque `(node, expr, orient)` triple);
  2. a verified declaration;
  3. definitely distinct;
  4. Zero with neither, which refuses `PlaneEqError::Undeclared` (`plane_eq.rs:325`).
  The curved ladder is `carrier_eq.rs` (`CarrierEqError::Undeclared`, `:109`).
- **The boolean refuses a value-only coincidence.** `BooleanError::UndeclaredCoincidence` (`boolean/mod.rs:2134`) is raised at `mod.rs:5044`, `reduce.rs:880`, `recl.rs:170`, `vtxfac.rs:767` and `flush_rows.rs:250`, and the document layer wraps it as `NodeErrorKind::UndeclaredCoincidence` (`eval/mod.rs:1783`) with a declare menu (`wire.rs:3795`).
- **Declarations.**
  - The node payload is `Node::Boolean.declare` (`node.rs:2383`) and `Node::Union.declare` (`:2490`), with `DeclaredPair` at `:3218`.
  - The kernel input is `BooleanDeclarations::coincident_faces` (`boolean/mod.rs:816`).
  - When the join refuses a declared union, `ops.rs:896` opens the declared-REST zip (`boolean/rest.rs`, 2693 lines).
- **The records.**
  - `ContactRecords` (`boolean/mod.rs:614`) are the verified declared contacts a result carries.
  - `BooleanNaming` (`ops.rs:230`) records what was glued (`merge_groups`, `covered`, `vertex_merges`), but only naming emission reads it.
  - The blend battery's `DecidedCoincidence::IsoscelesTurn` (`sweep/src/blend/battery.rs:2033`) is the one typed value-decided record in the tree. It does not leave the battery (`Blended`, `blend/build.rs:85`, does not carry it).
- **At rest.** The census refuses `ValidationError::UndeclaredContact` (`validate.rs:1596`) at nine sites in `census.rs`. A5 attributes each to a mate, and an unattributed one is a hard error (`assembly.rs:1297`, `:1768`).
  - A mate carries `class: ContactClass` (`node.rs:2711`), and `class_admission` (`mate.rs:649`) mints `Rest` and refuses `Tangent` as `NoAtRestRecord`.
- **Provenance channels other than `GeomSource`.**
  - `ParamSource` (`topo/src/param_source.rs:82`) is a radius token, read as `RadiusEvidence::Declared` on token equality (`:265`).
  - `AxisSource`/`AxisRecord` (`source.rs:504`, `:595`): nothing mints one, and production only re-stamps it through placement (`wire.rs:635`).
  - `CoaxialEvidence::Declared` (`geom-brep/src/intersect.rs:1804`) is reached only by tests.
- **Tangency.**
  - The profile stores `ProfileLoop.tangent_joints` (`profile/src/lib.rs:474`).
  - `judge_joints` (`validate.rs:2337`) refuses `UndeclaredTangency` (`:1090`) and `TangencyContradicted` (`:1104`).
  - `.tangent()` (`editor-core/src/program.rs:238`) constructs the tangency (the next leg inherits the incoming ray, `path.rs:2203`) and also emits the stored flag.
- **Symbolic.** `geom_core::Sym` (`sym.rs:4703`) decides a margin whose polynomial normal form is identically zero. `analysis::var_env_over` (`analysis.rs:1060`) binds an untoleranced variable as its nominal (`is_axis`, `:334`).
- **Exact constants.** `Ratio` (`editor-core/src/ratio.rs:22`) has no arithmetic; an operator over constants stays a tree node.
- **There is no lint.** `CheckId` (`checks.rs:60`) has `Connectedness`, `Separation` and `ChartCoherence`.

## 0. Ordering: nine PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Needs | Cost |
|---|---|---|---|---|
| A | `the-join-builds-what-the-rest-zip-builds` | the join's three missing arms (partner-edge chord, ring re-homing on a curved chart in aligned contact, the `mekr` `NotSameFace` cause), then `boolean/rest.rs`'s surgery and `through_the_join` deleted | nothing | H |
| B | `coincidences-are-recorded-at-one-door` | the record (each row says what was decided, where, and of which cells), emitted at every Zero that glues, merges or makes pieces touch; carried out of every op into `NodeValue`; the door `coincide::prove` with rung 1 (same source); `CheckId::UnprovenCoincidence` | nothing | H |
| C | `carriers-compare-in-canonical-form` | rung 2: `CarrierForm` per kind, `LinForm` over `VarId` with exact rational coefficients, each verb's `CarrierFlow`, the recourse ("the edit that would make it one construction"); a placement chain is one opaque pose atom until H | B; stage 2 A, B, E | H |
| D | `the-door-s-third-rung-is-the-symbolic-tier` | rung 3: re-decide an unproven record in the `Sym` lane with every variable a symbol; the box mitre measured | B | M |
| E | `booleans-glue-on-zero` | every undeclared-refusal site glues on Zero and records; the merge glues value-decided continuations; the kernel's evidence channels (`GeomSource` rung, `RadiusEvidence`, `CoaxialEvidence`) leave its decisions; DS2 and the topo README's ladder rewritten | A, B | H |
| F | `declared-pairs-retire` | `declare`, `DeclaredPair`, `BooleanCoincidence`, `BooleanDeclarations`, `SetDeclare`, the flush detector's declare protocol, DM4's declaration channel, the declared Door 1/Door 2 seats on booleans, `StaleContactDeclaration`, import's `declared_contacts`, the declare menus | E | M |
| G | `tangent-joints-are-derived` | `ProfileLoop.tangent_joints` goes: the set is derived at lowering from the constructors, and a junction decided Zero that no constructor made is recorded; `UndeclaredTangency` retires | B | M |
| H | `placed-carriers-compare-through-their-frames` | rung 2 over frames: H replaces C's opaque placement atom with the composed frame, so a mate-placed face is structural | C; stage 3 C (`a-mate-relates-two-poses`); stage 2 F | H |
| I | `mates-declare-no-contact` | `ContactClass` on mates, `class_admission`, `MintedDeclaration`, `NoAtRestRecord` and `FIT_DEFERRAL` go; at rest every contact the census finds is recorded at the door, and A5's hard error on an unattributed contact becomes an `unproven-coincidence` finding | H, F | M |

**What can start now.** A and B need nothing from stage 2 or 3. They can be dispatched today, in parallel. D, E and G need only A and B, so the whole of stage 4's kernel change lands before stage 3, and before stage 2 finishes. **Every one of the 67 rows parked on `intent-stage4-is-built` is released by A, B, C, E, F or G** (§12), so none of them waits on stage 3.

Why this order:

- **A before E.** Ev's part-3 answer had the glue reuse "the declared-Rest zip's bridged path". The zip ruling retired that path: the join builds what the zip builds. E turns every undeclared refusal into glue, and the join must already handle the scenes that only the zip handled (`RingHomingAmbiguous` 41, tangent plane×cylinder `SectionInvariant` 33, `NotSameFace` 17; measured by ZIP). Otherwise E would land a glue that refuses where the zip used to build. A deletes the zip as soon as the join builds those 91 unions, because then nothing reaches it. A needs no declaration change.
- **B before C, D, E and G.** Every later unit either produces records (E, G) or proves them (C, D). A coincidence glued with nowhere to record it breaks D10's "every coincidence the kernel infers from values … is recorded". B lands the door with the weakest sound rung, today's `GeomSource` identity. Rungs only ever prove more (D10), so C and D are pure additions.
- **E before F.** Once Zero glues, a declaration licenses nothing, and F deletes it with no behaviour change. Deleting declarations first would turn every declared scene into a refusal.
- **C needs stage 2.**
  - A carrier's canonical form reads the node's slot variables, which stage 1 made `VarId`s.
  - It also reads the frame it is built in, which stage 2 A (pose outputs of datums) and B (a profile's plane is a read) make a variable.
  - A frame read off a face is stage 2 E's `Face` variable: "a projection of a construction reduces to what it was built from".
- **H and I need stage 3.** Until frames are variables and a placement is the bundle of mates, a placed carrier's frame can only be compared as an opaque chain. H makes it a composed frame. I needs H because D10 says a mate-placed face is structural ("unless it is structural (a mate-placed face is)"). Retiring `ContactClass` before the door can prove that would turn every resting mate into a finding.
- **C and D are independent of E.** Rung 2 and rung 3 shrink the lint's findings. E only grows what is recorded. Either order is green. Landing C or D before E keeps the interim noise (§10) shorter.

Each intermediate state is a whole representation:

- after A, the declared union is built by the join alone, and declarations are unchanged;
- after B, the coincidences the kernel decides from values at the declared one-carrier rung (declared `Rest` and continuation glue), the split's pinch and the mitre are recorded and linted, and nothing builds differently. Still unrecorded after B, each with the unit that records it: declared `Tangent` and `Seam` glue (E, `booleans-glue-on-zero`); a boolean's vertex fusions decided by a margin (E, `value-decided-vertex-fusions-are-recorded-with-the-undeclared-glue`); an n-ary union's declared pair within one member (`a-unions-same-member-declared-pair-records-no-row`, after B); the blend battery's G1 and coaxiality verdicts (`the-blends-g1-and-coaxiality-verdicts-are-unrecorded-coincidences`); every `ContactRecords` row citing its decision (B2, `contact-records-cite-their-decision`);
- after E, nothing needs a declaration: a declared and an undeclared scene build the same body, and the declaration only adds a contradiction check;
- after F, the only ways to say a coincidence are a construction and (stage 5) an assertion;
- after G, the profile stores no tangency;
- after C, D and H, the door proves what D10 says it proves;
- after I, no class is declared anywhere.

**Rejected:**

- **E and F as one PR.** It would mix a geometric change (refusals becoming glue: hard to review, some goldens flip) with a ~250-file mechanical deletion. Between them, declarations are redundant but whole: the declared and undeclared paths build one body.
- **C before B.** That would be canonical forms with nothing to compare. B is also the smaller, riskier-to-defer plumbing (every op's output gains a field).
- **One rung-2 unit before stage 3, with frames compared through today's `Placement` chains.** Stage 3 deletes those chains (`Transform`-as-placement, gauges, offsets). A canonical form over them would be written twice. C compares a chain as one opaque atom (sound: it proves only what `GeomSource::Placed` proves today), and H replaces the atom.

## 1. Final shapes (after I)

**The record** (`topo::coincidence`, FORK-S4-3 for whether it is one type with `ContactRecords`):

```rust
pub struct Coincidence {
    pub cells: CellPair,          // (FaceKey, FaceKey) | (VertexKey, FaceKey) | (VertexKey, VertexKey)
                                  // | (EdgeKey, FaceKey) | (EdgeKey, EdgeKey) | (VertexKey, VertexKey) at a turn
    pub relation: Relation,       // SameOriented (continuation) | SameOpposite (contact) | OnCarrier
                                  // | Tangent { aligned } | EqualAngles (the mitre) | EqualRadii
    pub site: DecisionSite,       // a closed enum: PlaneLadder, CarrierLadder, Merge, SplitOn,
                                  // BatteryTurn, ProfileJunction, CensusAtRest, …
    pub margin: MarginDiag,       // the Zero verdict's own margin (Q1), never a synthetic one
}
```

- **Who emits it.** Every kernel op that decides a coincidence from values emits one row per decision, in decision order: a boolean's glue and merge, a split's ON that makes pieces touch, the blend battery's turn, a profile junction no constructor made, and the at-rest census.
- **How it is carried.** Rows ride out with the result (`BooleanBody`, `SplitResult`, `Blended`, `ValidatedProfile`, `AtRestBody`'s census) and are re-keyed through every graft by the graft's descendant map, as `ContactRecords` are today ("carried across seam-zip/merge mints by a descendant map, never re-derived", D1).
- **What the document layer keeps.** It names each cell by its `StableName` at the operation that decided it (the operand's table), so a row survives a later merge of its faces. `NodeValue` gains `coincidences: Arc<[NamedCoincidence]>`.

**The door** (`editor-core/src/coincide.rs`):

```rust
pub fn prove(doc: &Doc, eval: &Evaluation, row: &NamedCoincidence) -> Proof;
pub enum Proof { Structural(Rung), Unproven { residual: Residual, recourse: Recourse } }
pub enum Rung { SameSource /* B */, CanonicalForm /* C, H */, PolynomialIdentity /* D */ }
```

- The door is the one place structure is decided. Its rungs are tried in order, and a rung may only prove more (D10).
- The kernel decides margins and never structure, which retires its rung 1 in E. The kernel records; the document proves.

**Canonical forms** (C, H):

- `LinForm` is `{ terms: BTreeMap<Atom, BigRational>, constant: BigRational }`. An `Atom` is one of:
  - a free `VarId`;
  - `Turn`;
  - an opaque nonlinear subterm (a `Sin`, a var×var product, `Min`/`Max`), keyed by its `param_source` encoding.

  A `Defined` variable is expanded to its formula. A constant `Ratio` subtree folds exactly.
- `PoseForm` is a base pose and the symmetry it is taken modulo. The base is a pose variable, or the projection of a construction reduced to what it was built from.
  - In C, a placement chain is one opaque atom keyed by its placer and slot forms.
  - In H, a placement chain is the composed frame.
- `CarrierForm` is one per kind, each a `PoseForm` modulo the kind's symmetry plus `LinForm` offsets:
  - `Plane { frame mod Planar, offset }`, where a shift along the normal folds into `offset`;
  - `Cylinder { axis mod Cylindrical, radius }`;
  - `Sphere { centre, radius }`;
  - `Cone { axis, apex offset, half-angle }`;
  - `Torus { axis, major, minor }`;
  - and the edge carriers `Line { axis }` and `Circle { axis, centre offset, radius }`.
- **Equality is structural equality of the reduced forms.** It is an equivalence relation, so D10's chain of blocks closes into a loop and its brick sits on two blocks.
- **`CarrierFlow`** (FORK-S4-2) is beside `verbs::flow::ParamFlow`. Each verb states, per output role, the `CarrierForm` builder over its slot reads. For example:
  - `Extrude`: `Cap(Top)` is `plane(frame, dir, depth)`, and `Lateral(run)` is the profile edge's carrier swept along `dir`, independent of `depth`.
  - The profile's 2-D steps are cumulative, so a vertex is a `LinForm` over the step slots.

**The lint.**

- `CheckId::UnprovenCoincidence` is a report and refuses nothing. Its `CheckKind` is `Certified`: Certified in the direction it stays silent, because a row it does not report is proven.
- A finding names:
  - the two cells (as `Face`/`Edge` selections of the deciding operation's operands);
  - the relation;
  - the residual (the canonical forms' difference, e.g. `w₁ − w₂` with `w₁ = w₂ = 5 mm`);
  - the recourse: the edit that makes it one construction (Q3), or the assertion at its site that quiets it (stage 5).

**Retired.**

- No `declare` anywhere.
- No `BooleanCoincidence` and no `ContactClass` on a node.
- No `UndeclaredCoincidence`, `UndeclaredContact` or `UndeclaredTangency`.
- No `tangent_joints` stored.
- No `AxisSource`, `CoaxialEvidence` or `RadiusEvidence::Declared`.
- No REST zip.
- `GeomSource` per FORK-S4-1.

## 2. PR A — `the-join-builds-what-the-rest-zip-builds` (cost H; ~40 files, +2–3k / −4k lines)

This is the build order the zip row states (`work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`), as one PR or as three, each kept green:

1. **Ring re-homing on a curved chart in aligned contact.** It covers the 41 cylinder-bore mates.
   - `chord_join::chart_ring_side` decides a run whose azimuth window spans a full period.
   - A pierce ring on a degenerate constant-azimuth ray stops reading `Undecided` (`RingHomingAmbiguous`).
2. **The partner-edge chord.** It covers the 33 tangent plane×cylinder `SectionInvariant`s.
   - A segment along an edge of one solid takes that edge's curve as the other's chord. This generalizes `along_edge_spec`.
   - It is the zip's `Twin` (`rest.rs:1207`), moved.
3. **The `mekr` `NotSameFace` cause** (17 ring-vertex unions). Measure it first; the fix is in the join's ring placement.
4. **Delete the zip.** These go: `try_rest_union` (`rest.rs:191`), `read_segments` (`:523`), `undo_struts` (`:926`), `realize_seam` (`:1018`), `mirror_edges` (`:1151`), `mint_chord` (`:1296`), `pair_patches` (`:1518`), `glue_pair` (`:1673`), `slit_zip` (`:1801`), `zip_folded` (`:2039`), `RestZipFrontier` (`refusal_routes.rs:1200`), `BooleanError::RestZipUnsupported` (`boolean/mod.rs:2703`) and the door at `ops.rs:896`/`:918`.
   - Keep the carrier-pair doors and move them to `boolean/carrier_pair.rs`: `face_carrier` (`rest.rs:754`), `carrier_pair_relation` (`:821`), `carrier_pair_verdict` (`:844`), `flush_pair_relation` (`:599`) and `pair_extent` (`:716`). E reads them.

**Sites.**

- `topo/src/chord_join.rs`, `boolean/join.rs`, `boolean/edge_join.rs` and `boolean/insert.rs`.
- `boolean/rest.rs` (deleted, less the moved doors) and `ops.rs` (`through_the_join`, `:814`).
- `refusal_routes.rs`.
- `pncad-py` `tags.rs:1650` (`rest_zip_unsupported` goes).
- Python `.pyi`.

**Goldens and tests.**

- The zip suites become join suites: `topo/tests/crosslap_rest.rs` and `m5_s1_rest_zip.rs`, and `sweep/tests/m9_3_zip.rs`, `rest_nested_strut.rs`, `rest_zip_admission.rs`, `curved_mergedoor.rs` and `mate7a_torus_rest.rs`.
- **The 91 unions the zip builds today move to the join, so their body digests move.** The check is that each builds, passes tier 3′ with the same contact records, and has a volume equal to the zip's within its certified bound. Outside them, the tangent-site unions the join already built move too: the germ that `boolean::insert::across_tangent` reads in the sector across its tangent bound is minted there, so `join2_r1_probes` and `join2_r2_probes` bodies change bits (the volume at the last ulp), with the same face, edge, vertex and shell counts, contact records and tier 3′. No pinned digest moves.
- The torus peg-in-socket union builds through the join at every ε; main's join built it before A.

**Docs.**

- `crates/topo/README.md` C7's zip sentence ("The zip (`boolean/rest.rs`) removes conformal patches as interior … and mints each seam once") becomes a sentence about the join's finish. It is agent-written (CONTACT-DESIGN C7, labelled "Design sketch only"), so it needs no sign-off; the zip row says so.
- The code-map row (`README.md:30`) goes.

## 3. PR B — `coincidences-are-recorded-at-one-door` (cost H; ~60 files, 2.5–4k lines)

**Kernel.**

- `topo/src/coincidence.rs`: `Coincidence`, `CellPair`, `Relation` and `DecisionSite` per §1, and `Coincidences` (a `Vec` with a graft remap, written the way `remap_contacts` is).
- **Emission at every value decision that already glues, merges or touches.** B adds no glue; it records what is already decided. The sites:
  - **The declared rung of the plane and carrier ladders** (`plane_eq.rs`, `carrier_eq.rs:379` `pair_door_verdict`). A declaration's verification is a value decision ("declared is intent plus non-contradiction"), so a declared glue is recorded and linted like any other.
  - **Not the merge's or the covered pairs' own rows.** Every cross-operand pair `merge_coplanar_faces_declared` glues, and every covered pair a margin decided, is a declared one-carrier pair the declaration door already recorded, so each is one decision and one row. A vertex fusion (`BooleanNaming::vertex_merges`) decided by a margin is a coincidence too; E records it, with the margin threaded out of `reduce.rs` (`intent/value-decided-vertex-fusions-are-recorded-with-the-undeclared-glue`).
  - **The split's pinch.** `splitting/classify.rs:256`'s `Ok(Sign::Zero) => On` arm where the vertex's orbit has two or more runs on one side (D1: "one that would make pieces of one result touch"). Also the conic `ConicPlaneMeet::Parallel` arm (`:508`, `:521`).
  - **The blend battery.** `DecidedCoincidence::IsoscelesTurn` (`battery.rs:2033`) becomes a `Coincidence` row with `Relation::EqualAngles`, carried on `Blended` (`build.rs:85`). This closes `value-decided-coincidences-have-no-recording-door`.
- **Not emitted:**
  - an ON verdict that only places topology (D1);
  - a same-key or same-`GeomSource` pair the ladder settles before any margin. That is structure, not a value decision, and in B it stays the kernel's rung 1 until E moves it (E §6).
- **`k_stats`.** `Verdict` stays subject-free. The record is the subject-bearing log, so the funnel is not widened.

**Document layer.**

- `OpOut` (`wire.rs:84`) gains `coincidences`.
- The naming emitter (`names/emit_topo.rs`, `emit_union.rs`) maps each row's kernel cells to `StableName`s in the deciding operands' tables. A cell born in the op (a split's section edge) is named by the op's own role.
- `NodeValue` (`eval/mod.rs:508`) gains `coincidences: Arc<[NamedCoincidence]>`.
- **The content key is unchanged.** The rows are a function of the evaluation, as `verdicts` are.
- **`coincide.rs`: the door.**
  - `prove(doc, eval, row) -> Proof` with one rung, `Rung::SameSource`: the two cells' carriers hold one `GeomSource` base after the carriers' placements are composed. That is what N6's rung 1 proves today, so B proves exactly what the kernel already called structural, and nothing more.
  - The `Unproven` arm carries the two sources as the residual. Its recourse is the generic one (Q3), sharpened in C.
- **The lint.** `CheckId::UnprovenCoincidence` (`checks.rs:60`) walks every live node's `coincidences` and reports each `Unproven` row, with `CheckKind::Certified`. The check set is a closed enum, so the compiler finds every match site.

**Surfaces.**

- `pncad` re-exports `NamedCoincidence` and `Proof`.
- Python gets `Check.unproven_coincidence` and `Evaluation.coincidences(node)`, with the census and `.pyi` following.
- The viewer's checks pane lists the findings, each selecting its two cells.

**Goldens.** No geometry moves, and no content key or node id moves.

- The checks-report goldens gain rows: every declared scene in the corpus now reports its declared glues as unproven, because `SameSource` proves none of them. Rows are pinned per scene.
- `band_planar_mitre.rs`'s `an_isosceles_turn_is_recorded_as_a_value_decided_coincidence` reads the row through the document door.

## 4. PR C — `carriers-compare-in-canonical-form` (cost H; ~50 files, 3–5k lines)

**`editor-core/src/canon.rs`.**

- `LinForm` uses `num_bigint` rationals (already a dependency). `Ratio` stays arithmetic-free: the normalizer folds constants itself.
- **Lowering an `Expr` to a `LinForm`** (`expr.rs:979`):
  - `Var(v)` gives one term; a `Defined` variable is expanded.
  - `Ratio`, `Integer` and `Turn` are constants or the `Turn` atom.
  - `Add`, `Sub` and `Neg` are linear. `Mul`/`Div` by a constant subtree scales.
  - Anything else is an opaque atom keyed by its `param_source::encode` bytes, so two equal opaque subterms over the same variables are one atom.
- `PoseForm` and `CarrierForm` per §1. A pose variable's base is its `VarId`, modulo its kind's `Subgroup` (`VarKind::symmetry`, stage 2 A).
- **Projections reduce.**
  - A `FaceFrame` or an axis datum reading a `Face` variable is the face's carrier form, projected to the datum's kind.
  - A plane through a named edge (`cleave/a-plane-datum-through-a-named-edge`) is the edge carrier's form with the plane's symmetry.

**`CarrierFlow`** (FORK-S4-2), one per verb, beside `ParamFlow` (`verbs/src/flow.rs:230`):

- **Every surface-minting verb.** Each states the form of each output role:
  - extrude: cap, lateral, and the side of a pocket;
  - revolve: the axis-sharing walls;
  - sweep, loft, tube, fillet, chamfer, shell, offset and the blend.
- **A role whose carrier is not a function of slot forms** (a loft's interior NURBS) states `Opaque`, which compares equal only to itself, i.e. same node and same role.
- **A witness checks each statement.** At every evaluation in debug builds, and in a dedicated CI test over the corpus, the stated form evaluated at the f64 environment equals the minted carrier within its margin. This is the drift guard Ev's mitre answer asks for: "a second hand-written description of an operation can drift from its code".

**The door gains `Rung::CanonicalForm`.**

- `ParamSource`'s radius tokens are subsumed: `RadiusEvidence` asked "one radius expression?", and a `LinForm` equality answers it more strongly (`w/2 + w/2 = w`). The kernel half of `RadiusEvidence` retires in E.
- `AxisSource` rows: nothing mints one, and C's `PoseForm` is the axis identity the channel was to carry ("axis-shaped, not carrier-pair", AXIS-DECLARATION Round 3). E deletes the rows; C states the identity.

**Recourse** (Q3).

- The residual of `prove` is the two forms' difference.
- A residual that is `v − w`, over two free variables of one kind and equal value, recourses "make the slot reading `v` read `w`". That is the GUI's offer (`typing-a-value-mints-or-offers-a-variable`) in edit form.
- A residual in two pose atoms recourses "place one relative to the other". Until H that names the two placements.
- Anything else names the residual and the assertion at its site.

**Before stage 3.** A placement chain (`Transform`, an instance's gauge and offset, a mate's solved pose) is one opaque `PoseForm` atom keyed by its placer node and its slots' forms. Two carriers through one placer over one base compare equal. That is exactly what `GeomSource::Placed` proves, so C proves at least what B did. H replaces the atom.

**Goldens.** No geometry moves. Findings shrink: the corpus's flush plates, through-holes and stacked blocks built from shared variables prove. The rows a row-by-row diff lists are pinned.

## 5. PR D — `the-door-s-third-rung-is-the-symbolic-tier` (cost M; ~15 files, 1–1.5k lines)

**The all-symbols environment** (`analysis.rs`, beside `var_env_over` `:1060`): `var_env_symbolic(doc)` binds every continuous free variable as `Sym::param(var.0)` regardless of `is_axis`, binds each definition through `bind_definitions` (`doc.rs:1956`), and keeps exact constants exact. This closes `unproven-coincidence-lint-binds-every-variable-as-a-symbol`.

- Check how `Ratio::eval` (`ratio.rs:186`) lands at `Sym`. Its docs claim the exact rational, but it evaluates `num/den` in `T`, which at `Sym` is a `Lit` division. If the division is not folded exactly, D adds an exact constant constructor to `Sym`. `geom_core::sym::rational::Rat` is `pub(super)` today.

**The rung (`Rung::PolynomialIdentity`).** For a row C leaves `Unproven`:

1. Replay the nodes up to and including the deciding node in the `Sym` lane, under that environment.
2. Read the same decision's margin there. Rows are matched across lanes by `(node, site, ordinal among that site's rows, cell names)`.
3. If its normal form is identically zero, the row is `Structural(PolynomialIdentity)`.

A replay that refuses or escalates proves nothing, and the row stays `Unproven`.

**Lazy.** The rung runs only on rows rung 2 leaves, and only when the lint is asked for. One replay serves every row of one document.

**The mitre** (Ev, 2026-10-06). D measures every box mitre in the corpus and the tour: rows, replay time, and normal-form size. The cosine of an extrude's cap/wall dihedral is `dot(d, cross(d, t))`, identically 0, so each box mitre is `Structural(PolynomialIdentity)`. If the measurement is too slow, D reports it to the program, and a per-node theorem cache is the fallback Ev allowed. D does not build it unasked. This closes `isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`.

**Goldens.** None move. The mitre rows in B's goldens become proven.

## 6. PR E — `booleans-glue-on-zero` (cost H; ~120 files, 4–6k lines)

**The refusal sites become glue plus a record.** Rung 4 of each ladder (Zero, with neither source nor declaration) stops refusing:

- **Planes.** `plane_eq.rs:325`/`:332` `PlaneEqError::Undeclared` becomes `SameOriented`/`SameOpposite` plus a `Coincidence` row.
- **Curved carriers.** `carrier_eq.rs:387–397`, `:1177` and `:1302–1367` `CarrierEqError::Undeclared` become the same.
- **The boolean's raise sites.** `BooleanError::UndeclaredCoincidence` at `boolean/mod.rs:5044`, `reduce.rs:880`, `recl.rs:170`, `vtxfac.rs:767` and `flush_rows.rs:250` glue and record. Each opens the arm it would have opened under a verified declaration of the class its verdicts decide. Where no arm exists for the carrier pair (D1's frontier), the site keeps its typed per-arm refusal (`CurvedPairUnsupported` and its siblings).
- **The sliver band still refuses** (`Escalated`), and so does a definite contradiction.
- **Tangency.**
  - A seam or curve touch decided by the `Tangent` witness lane (`contact_verify.rs:231`) glues and records `Relation::Tangent`.
  - The sectors' second-order lump (C7) runs on the verdict, not on a declaration.
  - The vtxfac/recl tangent-lump arms read the decided relation.
- **The merge.** `merge_coplanar_faces` (`merge_faces.rs:1508`) and `_declared` (`:1626`) become one, which glues a cross-operand pair its margins decide a continuation and keeps operand A's description (Q4).
- **The coaxial cylinder×sphere arm.** `cs_pair_frame` (`join.rs:2164`) decides axis-to-centre distance Zero by its margin and records `OnCarrier`. `CoaxialEvidence` (`intersect.rs:1804`) is deleted. AXIS-DECLARATION's "coaxiality is never INFERRED from measurement" retires with the channel (D10's list).
- **Equal radii.** `RadiusEvidence` (`param_source.rs:265`) leaves the kernel's decisions. The equal-radius family (`tang/pinch-carrying-machinery-valence-4`) is decided by its margin and recorded `EqualRadii`.

**The kernel's structural rung leaves its decisions.**

- Rung 1 (`GeomSource` equality, `source::source_declaration`) gave a decision without a margin.
- Two same-source carriers are bit-identical (N6's theorem), so their margin is exactly zero and the Zero verdict decides the same thing. E deletes rung 1 from `plane_eq`, `carrier_eq` and `merge_faces`, and from `chart_region`'s cross-body read, which keeps its bracketed comparator.
- `GeomSource`'s remaining uses are FORK-S4-1's.

**What a result's census reads.**

- D1 (ii)/(iii) stand: a contact the result's census finds that no record backs is the op's defect, refused typed.
- E's rows back the census exactly as verified declarations did. A row is a record, so `ContactRecords` gains the Zero-glued pairs (or is replaced, FORK-S4-3).

**The document layer.**

- `NodeErrorKind::UndeclaredCoincidence` (`eval/mod.rs:1783`), `UndeclarableContact` (`:1820`), `UndeclaredCoincidenceFinding` (`:2183`) and the refusal menu's declare arm (`wire.rs:3795–3843`) are deleted.
- `class.rs:160`, `:357`.
- Python tags `undeclared_coincidence` (`tags.rs:1010`, `:1623`) and `undeclarable_contact` (`:1014`).

**Docs** (D10 governs; these retire as the program reaches them, so no fork):

- `crates/topo/README.md`:
  - The preamble's ladder: "structural (shared key or same `GeomSource`) is intent by construction; declared is intent plus non-contradiction; value equality never glues" becomes "a margin decided Zero glues; structure is decided at the document's door".
  - C2: "Conformality is thus decided structurally, never numerically" and "Invariant: no flag, mode or tolerance glues value equality".
  - C3's "carrier identity by the structural or declared rung".
  - C4's failure list.
  - C7's "an undeclared tangent pair refuses `CurvedBooleanUnsupported`".
  - C8.
  - "World-carrier Door 2 for declared planar pairs".
- `crates/editor-core/src/names/README.md` N6: "the converse is not claimed, so equal bits without a shared source stay unglued. The declared coincidence rung is this lookup". FORK-S4-1 states what replaces it.
- `docs/DISCIPLINES-DESIGN.md` DS2: "Value evidence cannot substitute for the declaration there even when definite … Carrier equality and declared contact are identification-grade; they carry no switch". It is rewritten as Ev's part-3 answer ruled: a Zero verdict is a decided verdict, the built solid is the same under every lint setting, and the family-level claim is the lint's. The grade-table row (`:568`) goes.
- `docs/AXIS-DECLARATION-DESIGN.md` gets a header line saying D10 retired the channel and pointing at C's `PoseForm`.
- `crates/verbs/README.md` §3 P1/P2 (the parameter-identity channel as coaxial evidence).

**Goldens.**

- **Refusal rows flip to builds:** every `UndeclaredCoincidence` row in the topo, sweep and editor-core suites (88 test lines), the tour's heatsink, letterforms and twopeg refusals, and Python's.
  - The guide's "interpenetrate by 4 mm to avoid flush geometry" (`docs/GUIDE.md`, the bracket) is rewritten flush.
- **Declared scenes are bit-equal** to before E: the glue path is the one the declaration opened.
- **Undeclared scenes that refused now build.** Their digests are new pins.

## 7. PR F — `declared-pairs-retire` (cost M; ~250 files, −6–8k lines, compile-driven)

After E, nothing reads a declaration except its contradiction check.

**Kernel.**

- `BooleanCoincidence` (`topo/src/contact.rs:129`), `BooleanDeclarations` (`boolean/mod.rs:816`), `FacePairDeclaration` (`:850`), `DeclaredPairs` (`:903`) and `VerifiedDeclarations` (`:1166`).
- `declares_pair` (`ops.rs:998`).
- The declared Door 1/Door 2 seats on booleans (`ContactVerdict` over a boolean's pairs, `declared_pair_overlap`, `declared_chart`).
- `ContactRefusal::Undeclared` (`contact.rs:359`).
- `ContactContradicted`, `ContinuationContradicted` and `SeamContradicted` *at use* (the at-rest ones are I's).
- The carried declared records: `CarriedContacts` (`:760`), `DeclaredContact` (`contact.rs:433`) and `StaleContactDeclaration`.
- The flush detector's offer protocol: `topo/src/flush.rs` `find_flush_candidates` (`:432`), `declare` (`:465`), `declare_all` (`:480`), `FlushFinding` and `FlushRung`. Its detection is the door's now.
- The extent pass's `Exempt::Declared`/`Rest` keys (`ops.rs`) read the records.
- `COINCIDENCE_RECOURSE` and `chord_join::UnderBoolean`'s "declare" offer.

**Document layer.**

- The node fields go: `Node::Boolean.declare` (`node.rs:2383`), `Node::Union.declare` (`:2490`) and `DeclaredPair` (`:3218`). So do `declare_rest`/`declare_continuation` (`:3269`, `:3279`) and `DeclaredSideFault` (`:3223`).
- The edit doors go: `DocEdit::SetDeclare` (`edit.rs:142`, `:5827`), `check_declared_sides` (`:5145`), and `EditError`'s `SetDeclareOnNonDeclaring`, `DeclaredSiteNotAnOperand`, `DeclaredNameNotUpstream` and `DeclareNamesMissingNode`.
- **The payload walks shrink.** `payload_names` (`node.rs:4699`), `payload_read_sites` (`:4777`), `rebind_payload_names` (`:3760`) and `declared_pairs` (`:4825`) lose their declare arm. The callers are `doc.rs:1757`, `edit.rs:4541`/`:6305`, `resolve/mod.rs:2263`, and `refactor.rs:259`, `:2047`, `:3274`.
  - If stage 2 E has landed, those walks hold only selects and the declared pairs, so F empties them to the selects. If F lands first, E's spec text "shrinks to the declared pairs" becomes "goes".
- **Evaluation** (`wire.rs`): `resolve_declarations` (`:3903`), `route_declarations` (`:3351`), `look_through_fold` (`:3440`), `drop_consumed` (`:3223`), `declare_landing` (`:4141`), `declared_step` (`:4115`), `site_operand` (`:3287`), `member_site` (`:3386`), `fold_descent` (`:3520`) and the ladder's declared-pair callers (`:3932`).
  - `judge_pairwise_contact` (`:3134`) is FORK-S4-4's.
  - `NodeErrorKind::DeclareResolve`, `DeclareSiteNotAnOperand` and `DeclareUnsupportedPair` (`eval/mod.rs:1715–1752`).
  - The content key's `feed_declared` (`eval/mod.rs:6460`).
- `names/flush.rs`: `find_flush_candidates` (`:204`), `declare`, `declare_all` and `DeclareError`.
- **Import.** `ImportOptions::declared_contacts` (`step-import/src/lib.rs:481`) goes. D1's import paragraph already says an imported assembly's contacts are `unproven-coincidence` findings quieted by an assertion, and the door records them like any at-rest contact.

**Persistence.**

- `kernel_wire::contact_class::pairs` (`:187`) and the load check (`check.rs:1879–1905`).
- `SnapshotError::DeclaredSiteNotAnOperand`/`DeclaredNameNotUpstream`.
- The `"declare"` field leaves the wire. A file holding a non-empty `declare` refuses `Unreadable` with the regenerate recourse. One holding `"declare": []` (`die_tool.pncad:526`, `die_composed_tour.pncad:2125`, `plate_param.pncad:415`) is regenerated (Q5).

**Surfaces.**

- Python: `BooleanCoincidence`, `Doc.declare`, `declare_all`, `Node.boolean(declare=)`, `Node.union(declare=)`, `DocEdit.set_declare` and `find_flush_candidates` go. That is 28 py-rs sites, 11 in the `.pyi`, and 54 test sites (`test_document.py` 19, `test_north_star.py` 10).
- Viewer: `add_boolean`'s declare list (`session.rs:2798`), `SessionOp::AddBoolean.declare` (`session/op.rs:628`), the declare-and-retry refusal (`session/refuse.rs:885`) and the create pane (`pane/create.rs:2751–2909`).
- **Tour.**
  - `find_flush_candidates`/`declare_all` sites go from booleans (4), heatsink (8), lily (4), plate (3), projectbox (2) and twopeg (5).
  - `coincident_faces` sites go from lily (7) and twopeg (4).
  - `twopeg.rs`'s 29 declarations become zero (the ruling's "ceremony").
- **Docs.**
  - REFERENCES DM4's "The declaration channel, sited at the members" (`REFERENCES.md:219–263`) and the declared clauses of "Contact is judged pairwise" (FORK-S4-4).
  - CONTACT-DESIGN C4's "Declarations live in `BooleanDeclarations::coincident_faces`", plus Continuation and Seam *as declarations*. The relations stay, as the door's `Relation`.
  - SELECT-DESIGN §3d's refusal menu.

**Goldens.** Every body digest is bit-equal to E's. Ids move where a `Boolean`/`Union` preimage spelled `declare` (it did, empty), so every node id after the first boolean moves; re-bless and state it.

## 8. PR G — `tangent-joints-are-derived` (cost M; ~45 files, 1.5–2.5k lines)

D1's profile-tangency paragraph is already ratified for this: "a junction decided Zero that no constructor made is a tangent joint all the same, recorded for the `unproven-coincidence` lint (D10) … The tangent-joint set is derived at lowering, from the constructors and the junction verdicts, never stored".

- `ProfileLoop.tangent_joints` (`profile/src/lib.rs:474`), `with_tangent_joints` (`:564`, `:578`), the `from_chain` parameter (`:651`) and `structure.rs:634` go.
- `ValidatedLoop` derives the set:
  - the constructors' joints (`.tangent()`, the fillet, the continuations, and `.cusp()` for the reverse joint), which the program lowering already knows;
  - the junctions `seg::joint_tangency` decides Zero, each recorded as a `Coincidence { relation: Tangent, site: ProfileJunction }`.
- **Refusals.**
  - `UndeclaredTangency` (`validate.rs:1090`) retires.
  - `TangencyContradicted` (`:1104`) stays for a constructor-made joint the geometry contradicts ("verified, never trusted").
  - `TangentJointOutOfRange` and `TangentJointOnFullTurn` stay.
  - `PathError::JunctionTangent`/`SeamTangent` (`path.rs:827`, `:883`) stay: they are the authoring lattice's "what did you mean by this corner?" gate (DS2's paragraph on the lattice), not a declaration.
- **Lift.** `profile::lift` (`lift.rs:19`, `:343`, `:595`, `:635`) reads the derived set, and V5's "declared junctions become `.tangent()`" reads "tangent junctions".
- **Other readers.** `editor-core/eval/anchor.rs:573`, `mesh/src/curved.rs:1422`, `path.rs:6060`/`:6082` and `validate.rs:2539`/`:2622`.
- **The record's door.** A constructor-made joint is not decided from values, so it is not recorded. A Zero-decided one is proven only by rung 3 (D) or, from stage 6, a tangency construction.
- **Goldens.**
  - `"tangent_joints"` is stored only in the pre-program `bool13_goldens/v1–v3`, which refuse at the header door today.
  - Profile digests are bit-equal.
  - Rows that refused `UndeclaredTangency` (28 test lines, tour 1, Python 2) now build and report a finding.
- **Docs.** `crates/profile/README.md` V6's "flags verified-never-trusted (`UndeclaredTangency`, `TangencyContradicted`)" becomes "constructed tangency verified, never trusted (`TangencyContradicted`); a value-decided junction recorded". DISCIPLINES DS-Q6.
- **Releases** `band/declared-joint-kind-zero-margin-reads-smooth` and, with E, `tang/declared-cusps-second-order-wedge-arm`.

## 9. PRs H and I — after stage 3

**H — `placed-carriers-compare-through-their-frames`** (cost H; ~25 files, 1.5–2.5k lines).

- `PoseForm` stops treating a placement as an opaque atom. A copy's frame is its placement's frame, which stage 3 makes a `Frame` variable, defined or solved from the copy's bundle of mates.
- **How a mate-placed face is proven is FORK-S4-5.** The recommendation: a placing mate equating two frames modulo its `Subgroup` makes the placed copy's frame, modulo that subgroup, *defined as* the partner's face frame composed with the mate's offsets. So the two faces' `CarrierForm`s reduce equal, with no special rung.
- C's opaque placement atoms, the `Placed` arm of `GeomSource` (FORK-S4-1) and `compose_placed` (`wire.rs:629`) go.
- **Goldens.** Findings on mated and patterned assemblies shrink. Poses are unmoved.

**I — `mates-declare-no-contact`** (cost M; ~150 files, 4–6k lines, mostly compile-driven).

- **The mate's class.** `Node::Mate.class` (`node.rs:2711`), `InterfaceCrossing::Mate.class` (`:1105`), `ContactClass` (`topo/src/contact.rs:47`, its re-exports `mate.rs:94` and `names/flush.rs:134`), `class_admission`/`ClassAdmission` (`mate.rs:575`, `:649`) and `MateFault::ClassNotAdmitted` (`:1016`) go.
  - So do the solve gate (`mate/solve.rs:1612`), `MintedDeclaration` (`assembly.rs:94`), its mint (`:1401`), `MintRefusal::NoAtRestRecord` (`:788`, `:1421`), `NO_AT_REST_RECORD_RECOURSE` (`mate.rs:559`), `FIT_DEFERRAL` (`contact.rs:259`, `:273`) and its steer sites (`contact_verify.rs:128`, `:338`, `:649`, `:653`).
- **The at-rest census's contacts** (`assembly.rs:1273` `verdict`).
  - The census runs over the product body.
  - Every *coincidence* between copies it finds is a `Coincidence { site: CensusAtRest }` row on the product, proven at the door (structural iff the faces' forms are equal, which is H's mate-placed case). That covers vertex–vertex, vertex on face or edge, curve touch and conformal patch.
  - `ValidationError::UndeclaredContact` *between copies* for those classes (the `census.rs` pushes, as read at rest) becomes that row.
  - **A pierce is not a coincidence.** `EdgeFacePierce`, and an `EdgeEdgeCross` the side test reads `SameSide`, stay interference evidence as stage 5 B (`interference-at-rest-is-a-finding`) leaves them.
  - `Attribution::Unattributed` (`assembly.rs:592`, `:1768`) goes, along with A5's "Undeclared contact between instances is a hard error, never blessed". An unproven contact no longer refuses `assemble`.
  - A contact *inside one op's result* that no record backs stays that op's defect, refused typed (D1 (iii)).
  - The `Separation` check's suppression reads `ContactRecords` (`checks.rs:1424`), and I deletes the mate-minted ones. I re-keys it to the door's `Structural` at-rest rows.
- **Stage 5's boundary** (reconciled with `docs/INTENT-STAGE5-SPEC.md` on `intent/stage5-spec`):
  - Stage 4 owns the recording of at-rest contacts, the lint's verdict on them, and the retirement of the class, the mint and A5's hard error.
  - Stage 5 owns the reporting:
    - its B makes interference (a vertex inside material, a pierce, a same-side crossing) a finding of its own;
    - its C dissolves `assemble`'s gate into `CheckId::AtRest`, which collects I's rows as `Contact` findings and applies the quieting rule.
  - If stage 5 B lands before I, interference is already a finding there, and I leaves it as B left it. The census's could-not-look refusals (`CensusUndecidable`, `CensusEscalated`) stay as they are; stage 5 C makes them findings.
- **Persistence.** `kernel_wire::contact_class` (`tag` `:54`, `untag` `:67`) goes. `Mate`'s wire loses `class`, so mate preimages and every later id move.
- **Surfaces.**
  - Python: `ContactClass`, `mate(class_=)`, `class_admission`, and the tags `class_admission_tag` (`tags.rs:2523`), `mate_class_not_admitted` (`:1072`), `no_at_rest_record` (`:2499`) and `unattributed` (`:2558`).
  - Viewer: `matetool.rs:104–137`, `:261`, `:453`, `:473`; `session/op.rs:24`, `:456`; `pane/features.rs:1036`.
  - Tour: `assembly.rs:694`, `:715`, `:764`, `:1465`, `:1532`.
- **Goldens.** The perf12 census goldens (`editor-core/tests/golden/perf12_census_{1e-6,1e-9,1e-12}.txt`, 473 `UndeclaredContact` lines each) are re-blessed as rows. The census's own findings are unchanged; only their spelling moves.
- **Docs.**
  - ASSEMBLY A3 ("`class` is the kernel `topo::ContactClass`" and the `class_admission` paragraph) and A5 (the hard-error sentence and *Attribution*).
  - CONTACT-DESIGN C4's `ContactClass` list and C6 (`Fit` as a class). C6's interference fits are stage 5's question.
  - `docs/MATE-7-TANGENCY-DESIGN.md`'s declared-`Tangent` mate arm.

## 10. What moves and what retires

| | A | B | C | D | E | F | G | H | I |
|---|---|---|---|---|---|---|---|---|---|
| Body digests | the 91 zip unions (volume-checked) | — | — | — | refused scenes now build; declared scenes bit-equal | bit-equal | bit-equal | — | — |
| Node ids, pinned hex | — | — | — | — | — | after the first boolean | — | — | after the first mate |
| Wire | — | — | — | — | — | `declare` gone | — | — | `Mate.class` gone |
| Content keys | — | — | — | — | — | `feed_declared` gone | — | — | mate class gone |
| `unproven-coincidence` findings | — | **appear** (declared glue, split pinch, mitre) | shrink (rung 2) | shrink (box mitres) | grow (every former refusal) | — | grow (Zero junctions) | shrink (mate-placed) | grow (at-rest contacts) |
| Refusals retired | `RestZipUnsupported` | — | — | — | `UndeclaredCoincidence` (all spellings) | the declare and contradiction-at-use families | `UndeclaredTangency` | — | `UndeclaredContact` between copies, `NoAtRestRecord`, `ClassNotAdmitted` |
| Analysis axes, MC draws, stackup | — | — | — | — | — | — | — | — | — |

**Ratified text this stage retires or rewrites**, each quoted. D10's last paragraph names each as retiring "as the program that builds it reaches them", so none needs a fork except where marked.

- **D10 itself.** "the declared-contact seats (CONTACT-DESIGN C4's `BooleanCoincidence` on booleans, a mate's `ContactClass`, C6's `Fit` as a class); the undeclared-coincidence, undeclared-contact and undeclared-tangency refusals, which become `unproven-coincidence` findings, with ASSEMBLY A5's hard error on an unattributed contact; DISCIPLINES DS2's identification grade; the axis declaration channel; `ParamSource`'s literal tokens; PARAM-LINT's declared-distinct record; the profiles' stored tangent-joint flags".
  - `ParamSource`'s literal tokens are already retired: stage 1 PR D, `param_source.rs:785` `RETIRED`.
  - PARAM-LINT's declared-distinct record was never built (`docs/PARAM-LINT-SPEC.md:3`, "DRAFT … awaiting Ev's sign-off"). E strikes it from the spec and from DISCIPLINES (`:263`, `:384`).
- **topo README (CONTACT-DESIGN) C2–C4, C6–C8 and the preamble** (E, F, I), as quoted in §6. Ratified #178, #965. D10 governs.
- **N6** (E, H): "Same source is syntactic identity of the triple … so equal bits without a shared source stay unglued. The declared coincidence rung is this lookup". The gluing half retires under D10. The identity half's fate is **FORK-S4-1**.
- **DISCIPLINES DS2** (E): "Value evidence cannot substitute for the declaration there even when definite, as a matter of principle … Only intent can assert the family-level claim, and gluing on the point-fact makes topology a function of a coincidence nobody stated." D10 retires it by name. The family-level claim is now the lint's.
- **AXIS-DECLARATION Round 3** (C, E): "Declared intent, structural invalidation. A declaration supplies what nothing can infer". Also "Coaxiality is never INFERRED from measurement" (Ev, 2026-09-12). D10 retires the channel by name. Coaxiality is one `Axis` read twice (structural), or a Zero verdict recorded.
- **REFERENCES DM4** (F): "The declaration channel, sited at the members" and "A pair that touches with the contact undeclared refuses `UndeclaredContact`". The pairwise judgement itself is **FORK-S4-4**.
- **ASSEMBLY A3** (I): "`class` is the kernel `topo::ContactClass`" and "`class_admission` is the one table both the solve and the mint door read". **A5** (I): "Undeclared contact between instances is a hard error, never blessed."
- **profile README V5, V6** (G), as quoted in §8.
- **`crates/verbs/README.md` §3 P1/P2** (C, E): the parameter-identity channel as coaxial and parallel evidence.

## 11. Test plan (each row names the runtime value that breaks it)

1. **(A) The join builds the zip's 91.** With `through_the_join`'s zip door deleted, each of the 91 unions builds, passes tier 3′, and has a volume within the certified bound of the zip's. *Breaks if* a ring on a constant-azimuth ray reads `Undecided` (the 41 refuse `RingHomingAmbiguous`), or a tangent chord is computed instead of read from the partner edge (the 33 refuse `SectionInvariant`).
2. **(A) Nothing else moves but the tangent sites.** Outside the 91, the digests that move are the tangent-site unions the join built before A, whose germ A mints in the sector across its tangent bound (`join2_r1_probes`, `join2_r2_probes`; the volume at the last ulp), each with the same counts, contact records and tier 3′; no pinned digest moves. *Breaks if* a moved carrier-pair door changed a verdict (a digest moves in the crosslap suite).
3. **(B) A declared glue is recorded.** A flush plate pair joined under a declared `Rest`: `coincidences(node)` holds one `SameOpposite` row naming the two cap faces by their operand names, and the lint reports it `Unproven` with residual "two sources". *Breaks if* emission keys rows by arena key (the name is absent after the merge), or the declared rung is treated as structural (no finding).
4. **(B) Same source is proven.** Two faces of one extrude glued back through a split-and-union proves `Structural(SameSource)`. *Breaks if* the door compares sources before composing placements (a transformed copy fails).
5. **(B) The mitre leaves the battery.** A filleted box's eight corners give eight `EqualAngles` rows on the blend node, in vertex order, each `Unproven` in B. *Breaks if* `Blended` drops them (zero rows).
6. **(B) The split's pinch.** A split through a vertex whose orbit has two runs on one side records one `OnCarrier` row. A split crossing an edge transversally records none. *Breaks if* every ON vertex is recorded (the transversal row gets one).
7. **(B) Nothing builds differently.** Every corpus digest, content key and node id is bit-equal pre/post B. *Breaks if* rows enter the content key.
8. **(C) Rung 2: linear forms.** Two blocks of heights `h` and `h/2 + h/2`, stacked flush: proven `CanonicalForm`. Heights `h` and a typed `10 mm` with `h = 10 mm`: `Unproven`, residual `h − v`, recourse "make the slot reading `v` read `h`". *Breaks if* a defined variable is read as an atom (the first row fails), or two typed equal values compare equal (the second is silenced).
9. **(C) Ev's loop and brick.** A ring of four blocks, each floor on its neighbour's (a `Plane` variable read four times), plus a brick across blocks 1 and 3: every row is proven. *Breaks if* equality is pairwise-syntactic rather than of reduced forms (the closing pair is unproven).
10. **(C) Modulo symmetry.** Two planes from one frame, one translated in-plane by `x`: proven. One translated along the normal by `d` and its offset by `−d`: proven. *Breaks if* in-plane motion is kept in the form, or the normal shift is not folded into the offset.
11. **(C) The witness.** A mutant `CarrierFlow` stating an extrude's cap at `depth/2` fails the corpus witness test. *Breaks if* the witness compares forms only to forms.
12. **(C) A projection reduces.** A `FaceFrame` datum read off face `f`, and a pad built on it flush with `f`: proven. *Breaks if* the datum's output is an opaque atom.
13. **(D) All symbols.** Two separately typed `5 mm` lengths with no tolerance: the analysis lane decides their difference Zero (VR8), and the lint reports the coincidence. *Breaks if* rung 3 reuses `var_env_over` (it proves the pair).
14. **(D) The box mitre is proven.** Every corpus and tour box mitre is `Structural(PolynomialIdentity)`, and the measured replay time is recorded in the PR. *Breaks if* `Ratio` constants are inexact at `Sym` (the normal form has a residue), or rows are matched by arena key across lanes (a row is unmatched).
15. **(E) Undeclared flush glues.** Two boxes built from distinct typed variables, touching flush, unioned with no declaration: the result is one solid whose volume is the sum, with one `SameOpposite` row, `Unproven`. Pre-E this refused `UndeclaredCoincidence`. *Breaks if* any of the five raise sites survives.
16. **(E) The sliver band still refuses.** The same boxes offset by 2ε refuse `Escalated`. *Breaks if* rung 4's replacement glues in-band.
17. **(E) Declared and undeclared are one body.** For every declared corpus scene, the body digest with the declaration equals the digest with it removed. *Breaks if* the undeclared path keeps a different description (operand B's), or merges in a different order.
18. **(E) Coaxial by margin.** A sphere centred on a cylinder axis by two separately typed equal offsets builds through the coaxial arm and records `OnCarrier`. Pre-E it posed `FrameError::NoArm`. *Breaks if* `CoaxialEvidence` still gates the arm.
19. **(E) The census backs Zero glue.** The result of test 15 passes tier 3′ with its rows as records. *Breaks if* the rows are not fed to `ContactRecords` (`UndeclaredContact` inside the result).
20. **(F) Nothing reads a declaration.** Python `Node.union(..., declare=...)` is a `TypeError`. Every corpus digest is bit-equal to E's. A pre-F file with a non-empty `declare` refuses `Unreadable`. *Breaks if* a load path silently drops a non-empty `declare`.
21. **(G) The tangent set is derived.** A `.tangent()` joint is in the derived set and not recorded. A line meeting an arc tangent by value, with no constructor, is in the set and recorded `Tangent`. A `.tangent()` joint the geometry contradicts refuses `TangencyContradicted`. *Breaks if* `UndeclaredTangency` survives (the second refuses), or constructor joints are recorded (the first reports).
22. **(H) A mate-placed face is structural.** A plate resting on a base by a planar `Rest` mate: its contact row is proven `CanonicalForm`. The same plate placed by a world pose equal in value: `Unproven`. *Breaks if* H reads the solved pose's value (the second is proven).
23. **(I) No class at rest.**
    - `twopeg` with its mates' classes removed assembles, and every at-rest contact row is proven (every contact is mate-placed).
    - A product with two unmated copies touching flush assembles with one `Unproven` `CensusAtRest` row. Pre-I that was `AtRest` / `Unattributed`.
    - A tilted brick piercing a plate between two copies is not a coincidence row: it is interference as stage 5 B leaves it.
    - *Breaks if* A5's hard error survives, or a pierce is recorded as a coincidence.
24. **(B–I) Python.** `Check.unproven_coincidence` rows equal the Rust report, and the census, `.pyi` and `tags.rs` agree.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, the checks registry's exhaustive matches, `refusal_routes.rs`, `scripts/gates/*.sh` (the `bit-identity-consumer` gate's allowlist stays empty) and `payload-rung-sweep`.

## 12. The rows each unit releases

The 67 rows parked on `intent-stage4-is-built` (the 2026-10-08 re-homing, `work/intent/log.md`), by the unit that removes what each waits on. A row needing two units waits on the later one.

| Unit | Rows |
|---|---|
| **A** (14) | `cleave/the-rest-lane-reads-a-nested-struts-site-at-its-holders-tip`, `join/peg-in-socket-union-refuses-join-desync-at-a-coarse-eps`, `reach/a-sharp-plate-offset-over-a-rounded-one-refuses-unpaired-loose-ends`, `zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate`, `zip/a-rest-lane-slit-zip-kills-seam-edges-with-no-substitution-row`, `zip/a-vertex-of-one-solid-inside-the-rest-contact-has-no-twin`, `zip/blind-shaft-in-a-full-turn-bore-revisits-the-seam-vertex`, `zip/rest-zip-drops-the-euler-operators-refusal`, `zip/rest-zip-frontier-refusals-reached-by-no-row`, `zip/rest-zip-seam-chord-on-cylinder-wall`, `zip/slit-zip-band-run-across-two-loops-is-reached-by-no-row`, `zip/the-rest-lane-zips-no-pinch-apex`, `zip/the-rest-lanes-glue-reads-its-correspondence-unfused`, `zip/unclaimed-half-edge-read-as-a-minus-half-in-zip` |
| **B** (2) | `cleave/split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence`, `tang/germ-takes-the-span-bounded-face-reach-alone` |
| **C** (2) | `cleave/a-plane-datum-through-a-named-edge`, `cleave/split-refusal-detector-names-a-shared-parameter-coincidence` |
| **E** (31) | `carvetail/half-revolve-caps-are-never-an-operand`, `carvetail/loft-walls-keyed-per-segment-on-a-declared-carrier`, `cleave/topo-mints-indeterminates-outside-the-funnel`, `emit/a-pair-boolean-names-a-declared-covered-pair-by-operand-order`, `fuse/a-kissing-convex-corner-result-ships-an-undeclared-vertex-on-face`, `fuse/a-pinch-line-crossing-a-face-interior-drops-the-pinchs-records`, `fuse/a-two-pinch-union-ships-a-pinch-its-records-do-not-declare`, `fuse/coincident-shell-has-no-fixture-for-its-unpaired-and-mixed-arms`, `join/a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts`, `join/a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys`, `join/the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone`, `reach/a-box-corner-on-a-declared-tangent-ruling-refuses-curved-boolean-unsupported`, `reach/a-settled-declared-coincidence-crosses-a-tight-volume-bound`, `reach/a-union-over-a-declared-continuation-keeps-its-walls-split`, `reach/an-uncovered-edge-tangent-to-a-fillet-at-the-curved-operands-vertex-refuses`, `reach/covered-endpoint-arms-read-a-non-convex-touch-at-the-ends-only`, `reach/maximal-faces-curved-arm-cannot-tell-a-licensed-curved-skip`, `reach/rounded-stack-subtract-and-intersect-refuse-fallback-extent`, `tang/a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent`, `tang/a-torus-seam-graze-needs-the-rim-root-deflated`, `tang/a-turned-lens-keeps-the-door`, `tang/decided-coincidence-carries-a-synthetic-invalid-margin`, `tang/pinch-carrying-machinery-valence-4`, `tang/the-tangent-offer-drops-for-a-face-with-null-scaffolding-mid-op`, `tang/torus-declared-rest-lane-banked`, `topo/boolean-in-band-arms-read-ahead-of-the-declaration`, `topo/coincidence-tangent-locus-contact-section-escalate-without-their-rung`, `topo/curved-pierce-frontier-tells-one-story-for-several-decisions`, `topo/restatement-derives-each-moved-edges-kind`, `topo/torn-hops-read-as-absent-across-the-boolean`, `zip/a-boolean-reports-one-undeclared-contact-per-refusal` |
| **F** (16) | `cleave/coincidence-intent-has-too-many-spellings`, `emit/a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names`, `emit/union-refuses-in-some-member-orders-and-publishes-in-others`, `fuse/a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`, `fuse/a-carried-vertex-on-face-row-at-a-pinch-is-unprobed`, `fuse/a-vertex-on-face-row-follows-its-face-not-the-part-it-rests-on`, `tang/a-flush-pair-with-no-readable-extent-has-no-typed-finding`, `tang/declared-cylinder-pair-offsets-read-off-the-reach`, `tang/flush-detector-offers-disjoint-coplanar-pairs-as-continuations`, `tang/lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces`, `tang/the-rim-routings-sense-guard-has-no-finished-fixture`, `topo/boolean-coincidence-route-still-holds-join-and-self-check-decisions`, `topo/boolean-declared-doors-still-offer-the-declare-menu`, `topo/declared-pair-verdict-answers-an-unreachable-distinct`, `topo/plane-orientation-offers-no-tolerance-at-a-declared-rest-door`, `topo/recl-membership-tangent-lump-arm-is-unreachable` |
| **G** (2) | `band/declared-joint-kind-zero-margin-reads-smooth`, `tang/declared-cusps-second-order-wedge-arm` (with E) |

H and I release none of the 67, so **every parked row is released before stage 3**. The umbrella `intent-stage4-is-built` parks on all nine units, as its body says. **Recommendation:** re-point each row from the umbrella to its unit above, so that a row's trigger fires when its unit merges, not when I does. The re-pointing is the orchestrator's call (`work/intent/log.md`, the re-homing), and this PR does not make it.

The in-scope `work/intent/` issues:

- `value-decided-coincidences-have-no-recording-door` closes with B;
- `unproven-coincidence-lint-binds-every-variable-as-a-symbol` closes with D;
- `isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box` closes with D;
- `the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms` closes with A.

## 13. Risks

- **E's breadth.** E touches every arm that read a declaration (vtxfac, recl, reduce, sectors, the seam lane, the merge). The test that keeps it honest is test 17: a declared scene's body is bit-equal with and without its declaration. A scene where they differ means the declared path did something the Zero path does not. That is a finding about the declared path, to be stated in the PR, not hidden.
- **The interim noise.** Between B and C, every declared glue is an `Unproven` finding, and between E and C every flush boolean is one. Findings refuse nothing (D1: "an `unproven-coincidence` finding refuses nothing"), but a demo or test asserting "no findings" goes red. Dispatching C (and D) as soon as stage 2 allows shortens the window. Tour rows pin findings by count per scene, never as zero, until C.
- **A's geometry.** The three arms are hard topology (curved-chart ring homing, a full-period azimuth window). ZIP measured the 91 and their scenes, and the risk is the `mekr` cause, which is unmeasured. A may split into three PRs; only the last one deletes the zip.
- **Rung 3's cost.** One `Sym` replay per document per lint run. D measures it on box mitres before anything depends on it. If it is slow, the per-node theorem cache is the fallback Ev allowed, and D does not build it unasked.
- **`CarrierFlow` drift** (FORK-S4-2). A stated form that disagrees with the built carrier proves a falsehood, which is a silent wrong "proven". The witness (test 11) is the guard, and it must run in CI over the whole corpus, not only in debug.
- **The stage-3 boundary.** H and I assume stage 3 defines a placed copy's frame from its mates. If stage 3 instead stores a solved pose as a free value, a mate-placed face can never reduce equal, and FORK-S4-5 must be answered in stage 3's representation. H waits on stage 3's C (`a-mate-relates-two-poses`), not on `intent-stage3-is-built`: stage 3's placement unit B gives the bundle, and C makes its mates read poses off geometry, which is what H replays at `Sym` (FORK-S3O, fork log row 96). Stage 3's F waits on this stage's C, whose rungs prove the contacts F's refusals leave (`docs/INTENT-STAGE3-SPEC.md` §12).
- **The stage-5 boundary** (reconciled, §9). I records at-rest coincidences and retires the class and A5's hard error. Stage 5 B makes interference a finding, and stage 5 C makes the census a check that reports I's rows and quiets them. Stage 5 C's `blocked_on` should name `mates-declare-no-contact` as stage 4's last unit it needs.
- **Collisions with stage 2.** B and E rewrite `wire.rs`'s boolean path while stage 2 B (`intent/s2-b-reads`) retypes its operands. F and stage 2 E both shrink `payload_names` and the ladder's declared callers. Whichever lands second merges main and restates.

## 14. Open questions

### FORKs, for designer pairs

Each changes ratified text or turns on a choice D10 does not make. None is resolved here.

**FORK-S4-1 — What a carrier's provenance is read from, and N6's fate.**

- **The problem.** D10 puts structure at one door in the document. The kernel today carries an opaque `GeomSource` stamp per description (N6, ratified), and composes it through placements. After E no kernel decision reads it: same source implies a zero margin. The door needs, for each recorded cell, the construction that built its carrier.
- **Options:**
  - (a) **Keep `GeomSource`** as a pure identity stamp, and key a document-side table of `CarrierForm`s by it. N6 keeps its identity half and loses "stay unglued".
  - (b) **Read it from the names.** Each row's cells are named in the deciding operands' tables. A `StableName` already traces to its minting node and role (`FromA`/`FromB`, `Merged`, `Instance`), and the role is what `CarrierFlow` is keyed by. `GeomSource`, `GeomOrigin`'s `Recipe` arm, `stamp_minted` and `compose_placed` retire. N6 becomes "a carrier's identity is its construction, read at the door", and the bit witnesses become C's witness.
  - (c) **Stamp the canonical form itself** on the body in place of `GeomSource` (the agent survey's "side table on `Body`"), computed at mint from the node's slot forms.
- **Recommendation: (b).** It has one provenance carrier (the name table) instead of two kept in step, and naming is already exhaustive over node kinds (`names::lift`'s precedent). (c) puts document variables into the kernel, which N6's layering note forbids. (a) keeps a second identity that only restates the name. Confidence: likely. It needs a designer pair, because N6 is ratified (#74) and (b) deletes its mechanism.

**FORK-S4-2 — How a construction states its carriers.**

- **The problem.** D10: "Each construction states which of its inputs each output carrier is a function of". Ev's mitre answer warns that "a second hand-written description of an operation can drift from its code".
- **Options:**
  - (a) **A stated `CarrierFlow`** per verb and role, beside `ParamFlow`, checked by a witness against the built carrier at every corpus evaluation.
  - (b) **Derived.** Run the verb at a symbolic scalar with each slot a symbol, and normalize the carrier it mints. There is one description (the code), but rung 2 then becomes rung 3: canonical-form equality of symbolic carriers is polynomial identity. That inverts Ev's "build only (2) to begin with".
- **Recommendation: (a)**, with the witness in CI. It matches D10's "states", keeps rung 2 cheap and deterministic, and the witness bounds the drift Ev named. Confidence: likely. The designer pair should weigh whether (b) restricted to *which inputs* (a dependency set, not a form) is a cheaper guard.

**FORK-S4-3 — Is the record a new type, or is it `ContactRecords`?**

- **The problem.** D1 (ii) already says a result's contact records "each [are] checked by the `unproven-coincidence` lint". `ContactRecords` (`boolean/mod.rs:614`) is today the verified-declaration currency the census reads to excuse a contact.
- **Options:**
  - (a) **One type.** `ContactRecords` becomes the coincidence record: each row gains its relation, decision site and margin, and glue pairs become rows.
  - (b) **Two types.** A `Coincidence` log beside `ContactRecords`, which keeps only what backs the census.
- **Recommendation: (a)**, since one currency means nothing to keep in step. It rewrites C3's granularities (`VvContact` … `PatchContact` stay as cell shapes). Confidence: likely. It is weighed with FORK-S4-1 by the same pair.

**FORK-S4-4 — A union's pairwise judgement without declarations** (REFERENCES DM4, "Contact is judged pairwise, in member space, before the fold").

- **The problem.** The judgement exists so that an *undeclared* contact refuses in every member order, and that refusal retires. What is left is order-freedom of the sliver band's `Escalated`, and of the records: a fold records only what it meets, and a contact a third member covers is met in one order and not another.
- **Options:**
  - (a) **Keep the pairwise judgement** as the union's coincidence census. Its records and in-band refusals are order-free, at today's n(n−1)/2 cost.
  - (b) **Drop it.** A union records what its fold meets, and its findings may depend on member order.
  - (c) **Keep it for refusals only.** The records come from the fold, filtered to cells that survive in the result.
- **Recommendation: (a)**, because order-free findings follow DM4's own reason for the rule, and the cost is today's. Confidence: unsure. It changes DM4's text beyond D10's list.

**FORK-S4-5 — How a mate-placed face is proven structural** (with stage 3).

- **The problem.** D10: "contact between copies is an `unproven-coincidence` finding unless it is structural (a mate-placed face is)".
- **Options:**
  - (a) **Through the frame.** Stage 3 defines a placed copy's frame from its mates, so the two faces' canonical forms reduce equal with no special rung.
  - (b) **A mate rung.** The door proves a contact structural when a placing mate names its two faces, whatever the frame representation.
- **Recommendation: (a).** It is one door, and a face resting by a mate on a face *derived* from the named one (a fillet's flank on the mated cap) is proven by (a) and not by (b). It depends on stage 3 defining, rather than storing, a placed frame, and that is the reconciliation item with the stage 3 spec. Confidence: likely.

### Questions with a recommendation (not forks)

1. **What B records that the kernel decides by structure today.** **Recommendation:** nothing. A same-key or same-source pair is not decided from values. E moves the source rung out of the kernel, and the margin then decides it Zero; E records it then.
2. **The lint's setting** (Ev, part 3: "if the lint is turned off, the user can always just use numbers"). **Recommendation:** the check set is a per-run selection, as today's checks are; there is no document flag. Turning the lint off removes reports, never changes a body (DS3).
3. **The recourse.** **Recommendation:** C's residual decides it (§4). The forms of the recourse are one substitution edit, "place one relative to the other", or the assertion at the site (D1: "its recourse is one construction or an assertion at its site"). A recourse that names an edit must be one the edit door accepts: test 8 applies it and re-proves.
4. **Which description a glued face keeps** (D10: "keeping one fixed operand's description"). **Recommendation:** operand A's, a union's earliest member's, as `outermost_survivor` (`merge_faces.rs:2030`) already picks the arena-first face. Test 17 pins it.
5. **Pre-F files.** **Recommendation:** a file with `"declare": []` loads under a one-release tolerance only if the wire version says so. Otherwise it refuses `Unreadable` with the regenerate recourse, as stage 2's pre-B files do. The three fixtures holding it are regenerated in F.
6. **Tangency in stage 4.** A Zero-decided tangency is proven only by rung 3 or (stage 6) a construction. **Recommendation:** record it in G and E, and let D's rung try it. Stage 6's tangency constructions shrink these findings, and nothing in stage 4 builds a tangency rung.
7. **The analysis lanes.** D10: "The analysis lanes, which run over a parameter box, see such a coincidence as the point it is and escalate there." **Recommendation:** nothing to build. A Zero glue at a box straddles in the `Interval` lane and escalates through the existing path. E adds a row asserting that a value-decided flush union over a toleranced height escalates in the box, and a structurally shared one does not.
8. **`k_stats`.** **Recommendation:** leave its `Verdict` subject-free. The record is the subject-bearing log, and the K-report needs no cells.

### Inconsistencies found

- D1's merge-stage paragraph already says "a boolean merges a cross-operand coplanar pair its margins decide a continuation, records it for the `unproven-coincidence` lint" (`DESIGN.md:521`), while the topo README C2 and the preamble still say value equality never glues. The README is behind D10, and E rewrites it.
- `cs_pair_frame`'s doc and AXIS-DECLARATION's "never INFERRED from measurement" stand against D10's Zero glue. D10 names the channel, so it retires (E).
- `crates/sweep/README.md:276` and `battery.rs:468` call `BatteryVerdict::coincidences` "D10's record; no reader yet". B gives it its reader.
- `docs/PARAM-LINT-SPEC.md` is a never-dispatched draft written against `ParamName`/`DocParam`, which no longer exist. D10's retirement of its declared-distinct record is a doc strike (E).
- `step-import`'s `ImportOptions::declared_contacts` is a declared-contact seat D10's list does not name. It is a seat all the same, and F retires it.
