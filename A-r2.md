# Fork `clear-enclosure` — designer A, round 2

Shown: designer B's round-1 report.

## For Ev

**Recommendation (likely): designer B's cell-bound type, plus one rule added to it.** Neither half fixes the other's fault.

1. **The engine's bound over a cell becomes its own type, `CellBound`.**
   - It wraps `Interval` and delegates the arithmetic, so there is still one arithmetic (your PR 2701 unification stands).
   - It has no `Decide`, so it can never be the scalar a body is written in.
   - It is decided at one funnel door of its own (the shared decision site every margin reading goes through). That door never writes the node's verdict log.
   - It begins where a cell is put into the face's chart. The lifted carrier before that point stays `Surface<Interval>`, the set of surfaces the stored face stands for (one surface at f64).
2. **Readings at points are made at the body's own scalar and logged.** A finding is checked at two stations, one point on each face, giving one distance. At `Interval` such a reading straddles the threshold when the parameter box is too wide, so it is the drive's cue to narrow the box. This replaces my earlier "every refusal names its lever".
3. **The rule I add: a cell bound never establishes an equality.** Where the engine needs "these meet" or "this is zero", it takes the answer from one of three places:
   - topology: a shared edge or vertex, or a recorded contact;
   - a tier-3 verdict: the edge lies on its carrier, the apex is a vertex, the wedge class;
   - a point reading at the body's scalar, where `Sym` can prove it at any box width.

   The cell door's Zero answer means only "a candidate for a point reading", never "accepted".

**What stays agreed (sure).**
- An f64 value enters as itself.
- The K·ε band is how a reading is decided, at every scalar. It is not added to the values as they enter.
- One crossing door, held by the lane value that carries the right to certify; the shell's `lift` parameter goes.
- `Sym` fits the engine but would buy nothing over cells.
- `[topology.tiers]` is reworded to "the question decides with the band at every scalar; a stored value enters as the reals it stands for: itself at a point scalar, its bracket at `Interval` and `Sym`".

**Why the type is right after all (likely).** The tree already keeps the two meanings of an undecided `Interval` apart by hand: "narrow the parameter box" (the drive's) and "split the cell" (the engine's).
- `docs/predicate-dimension-audit.md` rows F28, F29 and F30 are three scopes whose decisions are kept out of the log.
- F30 records the cost when they were logged: drives that certified at once "ran past ten minutes".
- F30 also hand-picks the one reading that belongs to the drive (`BoxDependent`, the upper-bound case the box decides only in part).
- The shell's cavity check runs the same sweep with no such scope, so the convention has already been missed once.

A type with no `Decide` makes "a cell reading in the drive's log" fail to compile, which is the kernel's rule that a bad state should be unrepresentable.

**Why the added rule is needed (likely).** Under the type alone, four engine readings still decide Zero by cell interval on facts the body has already decided:
- a shared edge lies on the other face's carrier (`self_intersection_on_shared`);
- a vertex is the cone's apex (`self_intersection_apex`);
- the wedge at an edge is 0 or 2π (`self_intersection_cusp`);
- a meeting's separation (already read off topology).

`[one-way.readings]` forbids a second reading of one fact. Example: a capped cylinder whose height ranges over [9.99, 10.01] mm, at `Sym<Interval>`.
- Tier 3 proves by identity that the rim lies on the cap plane.
- The engine's cell reading of the same fact is about ±0.02 mm wide, so it refuses a body tier 3 accepted.
- Under the rule, only the departure sign is read across the rim (whether the faces leave their shared edge to opposite sides). That sign is macroscopic, so it decides over the whole box.

**Other options, as final states.**
- **`Interval` everywhere, plus a token gate** (my earlier lean, and B's second option). It keeps the logging split by hand, and it is the shape the shell already broke. Rejected.
- **A `Sym`-native engine.** It pays a symbolic expression node per operation per cell, for decisions `Sym` cannot change. Rejected.
- **The type without the rule.** It leaves the second readings in place, and at `Sym` it keeps discarding identity proofs. Rejected.

**What changes for the at-rest gate unit.**
- The yes/no embedding verdict is a function bounded on certification rights. Only the minimum-clearance measure keeps its `None` at f64.
- Cell readings are unlogged by type, and each finding's point reading at the body's scalar is the one logged row.
- For each local reading in `topo::injective`, the spec says either that it passes by the band, or which topology or tier-3 fact it takes in place of a cell Zero.
- The shell's cavity check loses `lift` and becomes a reading of the same gate.

**Confidence:** type and point readings likely; added rule likely; the four Zero places as second readings likely (read from code, not run); rewording sure.

## For the orchestrator

1. **What moved designer A.**
   - F28–F30 (checked in the clone, audit doc lines 1458–1575) show the logging conflation is real, has been paid for by hand, and was measured costly.
   - `cavity_embedded` (`topo/src/shell.rs`) calls `lane.sweep` with no `k_stats::unlogged` scope. `eval_node` opens a verdict bracket per node (`editor-core/src/eval/mod.rs`), so at `Interval` or `Sym<Interval>` a shell node logs the sweep's cell rows. That is F30's pathology, and B's defect stands.
   - B's "point readings at the body's scalar" is a better channel for the drive's cue than designer A's generalized `BoxDependent`, because it uses the drive's own log. Designer A drops R3 for it.
2. **Designer A's "it fixes neither (a) nor (b)".** It was wrong about (b), the one-width-two-levers fault: the type, together with point readings at the body's scalar, is (b)'s fix. It was right about (a): the type leaves the four Zero places as cell readings.
   - On the cost: `Vec3<T: Real>` and `Point3<T: Real>` (`geom-core/src/linalg`) mean `CellBound` must implement `Real` by delegation, against B's assumption that it need not. That is harmless, because the safeguard is "no `Decide`", not "not a `Real`". The unit should measure the compile cost.
3. **Whether B's point readings cover (a).** Only the meeting and violation coincidences. `on_shared`, `cusp` and `apex` are local-certificate readings on presented rectangles, sides or vertices, and would become unlogged cell readings with the family's width. B's own sentence, "the equalities the question needs come from topology", is (a)'s principle, but B's design does not apply it to those three places.
4. **Open for the spec.**
   - Which point readings log. Designer A suggests only those whose undecided answer a narrower box resolves, so a Holds logs nothing and the f64 witness's log cannot diverge on it. B flagged the row pairing as unchecked.
   - Station placement at the body's scalar (F28's region readings) is neither a cell bound nor a finding's check. The spec must say it stays unlogged.
5. **To file** (designer A cannot push): `work/clear/`: the shell's cavity sweep runs outside an unlogged scope (`topo/src/shell.rs` `cavity_embedded`, against `editor-core/src/eval/wire.rs` `wire_clearance`'s F30).
6. Nothing was built. There is no target directory, and the clone is unmodified.
