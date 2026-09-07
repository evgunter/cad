# MSOLVE-6 — The mate's lever is the mated parts' own extent (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-6.md`. Answers
`work/msolve/mate-lever-needs-the-parts-extent.md`, which Ev ruled on
`[ev]` PR 2086 (2026-09-07): **option B — the extent is resolved from
the mated part's own evaluated body**, not authored beside the datum
and not left at the session box. This spec is the ruling made
precise. Read the item, the PR's comments, ERROR-DESIGN E3's
amendment (`docs/ERROR-DESIGN.md`, "E3 amendments (revision E12)"),
and `crates/editor-core/ASSEMBLY.md` A11 before the code.

## What the tree says now

- `Alignment::lever_arm` (`crates/editor-core/src/mate.rs`) answers
  three ways: a datum that names no scale gets `SESSION_SCALE`
  (one metre, "D4 ¶4's session box, not a lever"); a datum naming a
  scale at or above `MIN_LEVER_ARM` (one micron) gets that scale
  (`authored_extent`: the larger frame-origin norm folded with every
  authored length); a scale below the micron refuses
  `LeverRefusal::DatumTooSmall`, which the solve carries as
  `MateFault::Unleverable`. Its own doc says the metre survives
  "exactly where D4 ¶4 put it" because "the full amendment needs the
  mated parts' extent to reach this door, which is issue
  `mate-lever-needs-the-parts-extent`". That door is this unit.
- The lever is read at two sites in `crates/editor-core/src/mate/solve.rs`:
  `mate_coset` (the clocking-redundancy decision on a frame
  coincidence, `Margin::levered(theta, arm)`) and `fold_pair`, where
  `arm` starts at `0.0` and takes the max over the pair's mates before
  `coset::intersect(held, coset, band, arm)` decides the fold. Both
  reach the lever through `alignment.lever_arm()`, with no body in
  hand: `solve_document(doc, tol)` takes a document and a tolerance
  and runs in `eval/mod.rs` BEFORE any node, right after the
  `PartCache` is built and before it holds anything.
- The sibling ships the amendment whole: `eval/measure.rs::arm` is
  `reach(a) + reach(b) + ‖ref(b) − ref(a)‖`, an upper bound on the
  extent of the two operands together, with no floor and no constant;
  `reach_of(body, face, origin)` walks the face's boundary and bounds
  every edge from the origin by `curve_reach` (line ends, circle and
  ellipse centre plus radius, NURBS control hull), refusing `None`
  where an edge's carrier has no bound it can state. Its doc argues
  the rim bound for the v1 carrier table (a plane patch and a cylinder
  patch are both bounded by their rims).
- A part's body reaches the evaluation through
  `eval/parts.rs::PartCache::get(doc_ref, tol) -> PartValue { body:
  Arc<Body<T>>, .. }`, evaluated once per `(DocRef, ε)` key under a
  shielding verdict bracket; `InstantiatePart` is a leaf with no
  frame (A11 rule 2 puts placement on the cluster), so its body does
  not depend on the solve. `MateFrame` origins are authored in the
  part's own coordinates (`mate_coset`: "b's part coordinates into
  a's").
- ERROR-DESIGN E3's amendment (ratified at E12) ends: "The
  `max(·, 1 m)` sibling in `mate.rs` takes the same lever." A11's text
  says "Nothing in the walk is evaluated" and lists the numbers the
  solve reads; it says nothing about the lever's source.
- MEASURED (M10-7, recorded in `lever_arm`'s doc): removing the metre
  with no part extent turns twelve rows of
  `crates/editor-core/tests/asm_r2a_mate_solve.rs` into refusals —
  every `Coaxial`/`FrameCoincidence`/`Clocking` on `z_up()` frames and
  every `PlanarRest { offset: 0.0 }` at the origin.
  `crates/editor-core/tests/m10_7_r1_probes_interval.rs::r1_mate_lever_is_discontinuous_at_zero_extent`
  pins the three-way answer by value.

## What the unit builds

**The lever a mate's angular decisions turn on is an upper bound on
the extent of the two mated parts together, from the datum, taken
from each part's own evaluated body.** No floor, no constant. In
symbols, for a mate between members on instances `a` and `b`:

```
L  =  (R_a + ‖a.origin‖)  +  (R_b + ‖b.origin‖)  +  Σ |authored lengths|
```

where `R_x` is an upper bound on the distance from instance `x`'s
part-local origin to any point of its part's body. The three terms
mirror the sibling's `reach(a) + reach(b) + ‖ref(b) − ref(a)‖`: by the
triangle inequality `R_x + ‖x.origin‖` bounds the part's reach from
its mate frame's origin, and the authored lengths (a `PlanarRest`
offset) are the separation the datum itself names between the two
frames once mated. Over-refusal is the safe direction, so every term
is an upper bound and none is dropped. A pattern copy or a transform
on the member's chain moves the part rigidly and changes no reach.

1. **The body's reach.** One function, in `crates/editor-core/src/mate/reach.rs`
   (new) or beside `reach_of` in `eval/measure.rs` — the lane picks
   the home and says why — answering `R` for a `Body<T>` from the
   part-local origin: the maximum over the body's faces of the face's
   reach, computed by the SAME `reach_of` the measure site uses
   (widen its visibility; do not copy it), read as an `f64` upper
   bound (`Bounds::hi` on an interval scalar). A face whose reach
   `reach_of` cannot bound is a typed refusal, never a guess.
   **Check the rim argument per surface kind the kernel can build**
   (`topo`'s surface enum): a plane or cylinder patch is bounded by
   its rims; a spherical, toroidal or free-form patch is NOT in
   general (a hemisphere's rim is its equator). For each such kind
   either bound the carrier itself (centre distance plus radius for a
   sphere; state the bound for the others) or refuse typed. A part
   the repository's own fixtures or demos build whose reach refuses
   is the stop clause below.

2. **The reach reaches the solve through one trait.**
   `solve_document` gains an argument, `&dyn MateReach` (name yours),
   with one method: the reach `R` of an instance node, `Result<f64,
   ReachRefusal>`. The evaluation's implementation (in `eval/mod.rs`,
   beside where `solve_document` is called) answers by reading the
   instance's `doc_ref` and asking the `PartCache` it already built —
   lazily, on first ask, so a part no mate names is not evaluated
   early and a mated part is evaluated exactly once (the instantiate
   node hits the cache afterwards). A resolver refusal (`PartFault`)
   is carried into the reach refusal's own variant, typed, unaltered.
   The document's own `solve_document(doc, tol)` shape goes away;
   every caller passes a reach. Callers in the tree
   (`grep -rn 'solve_document(' --include=*.rs .`): the evaluation,
   the viewer's mate tool (`crates/viewer/src/matetool.rs`), the
   Python door (`crates/pncad-py/src/py/mate.rs`), `demos/tour`, and
   ~120 test call sites. The viewer and Python doors have a workspace
   or resolver in hand; give them the evaluation's implementation
   through a public door (e.g. `eval::mate_reach(doc, &opts, tol)`)
   rather than a second one. Tests get a fixture helper that builds
   the reach against the stub store through that same public door.

3. **The lever moves to the solve, with the datum's terms kept.**
   `Alignment::lever_arm` becomes the datum's own contribution
   (`‖a.origin‖`, `‖b.origin‖`, the authored lengths — pure, no
   refusal) and the solve adds the two reaches. `SESSION_SCALE`,
   `MIN_LEVER_ARM`, `names_a_scale` and `LeverRefusal::DatumTooSmall`
   retire: with a real part on each side the lever is never zero and
   never vacuous, so the constant has no case left and the floor has
   nothing to guard. If the lane finds a case the floor still guards,
   state it with the document that reaches it — that is a deviation
   to argue in the PR, not a silent keep. `MateFault::Unleverable`
   keeps its shape and carries the new `LeverRefusal` variants: a
   reach the module cannot bound (naming the instance, the part and
   the face kind) and a part that does not resolve (naming the
   instance and carrying the `PartFault`).

4. **Blast radius, stated and pinned.** Today an unresolvable part
   fails its instance and poisons its subtree while the mate stays
   `Determining`. After this unit the mate faults too, in the
   resolver's voice, and a mate fault poisons the document. That is
   the honest reading (a mate on a part that does not exist has no
   pose) and the assembly gate needed the part anyway; pin it with a
   row and say it in the PR.

5. **Docs.** A11 (`crates/editor-core/ASSEMBLY.md`) gains the sentence
   Ev ruled on: *the solve reads no geometry except each mated part's
   own extent, an upper bound taken from its evaluated body, which
   enters only as the lever a parallelism verdict is decided over.*
   The two code-doc phrases that quote A11 as geometry-free
   (`mate.rs` "no geometry inspection, no numerics beyond decided
   predicates"; `mate/solve.rs` "no geometry is inspected") take the
   same qualifier. `lever_arm`'s long doc is rewritten to what is
   true now (the metre paragraphs go; the nanometre story stays as
   the record of why a floor was once needed and why it is not now).
   ERROR-DESIGN's E3 amendment gains one status sentence at the
   sibling line: shipped whole at the mate site by this unit.

6. **Nothing else moves.** The pose answer is untouched: the lever
   enters predicate margins and the `Contradictory { lever, clash }`
   payloads only. `SolvedPoses` on every corpus document whose
   verdicts do not flip must be bit-identical to main; the
   correctness arm measures it.

## Acceptance

- **A1 — the twelve rows, no constant.** Every row of
  `asm_r2a_mate_solve.rs` passes with `SESSION_SCALE` and
  `MIN_LEVER_ARM` deleted; `grep -n '1\.0\b\|1e-6\|1\.0e-6' crates/editor-core/src/mate.rs crates/editor-core/src/mate/solve.rs`
  finds no lever constant (cite the grep in the PR).
- **A2 — the lever is an upper bound, measured.** Rows in
  `crates/editor-core/tests/msolve6_part_extent.rs` (registered in
  `tests/all.rs`): for a box part the reach is at least the far
  corner's distance; for a cylinder part at least `sqrt(r² + h²)`;
  the lever of a mate on two such parts equals the formula above to
  the bit (compute both sides in the row).
- **A3 — the lever decides at the parts' scale.** Two rows on the
  measure site's own example: a 10 mm part tilted by 1e-8 rad is
  PARALLEL across its own extent (deviation 1e-10, a tenth of the
  default ε) where the metre lever refused it; a 10 m part tilted by
  the same angle is refused (`Contradictory`, `lever: Some((θ, L))`
  carrying the part-scale `L`) where the metre passed it. Run at the
  three CI eps values; derive the band edges from the run's own
  `Band`, never from literals (MSOLVE-3's lesson).
- **A4 — refusals typed.** A part that does not resolve faults the
  mate `Unleverable` carrying the `PartFault`; a body with a face
  whose reach cannot be bounded faults `Unleverable` naming the
  instance and the face kind (build one if the kernel can; if no
  door builds such a face, say so and pin the arm at the unit level
  with a hand-built body).
- **A5 — the evaluation and the memo.** The document's evaluation
  calls the solve with the cache-backed reach; a mated part is
  evaluated exactly once (`PartCache::evaluations()` before and after
  a run with a mate); a part whose content changes so that a verdict
  flips changes the mate's memo key (MSOLVE-4's `SolveAnswer` is fed,
  so this holds by construction — pin it with one row).
- **A6 — the doors.** The viewer's mate tool and the Python
  `solve_document` door pass the evaluation's own reach; the Python
  row `test_the_lever_is_the_parts_extent` (in the existing mate test
  file) drives a 10 mm document and asserts the row A3's answer
  through Python. `demos/tour` builds.
- **A7 — the record.** A11's sentence, the two code-doc qualifiers,
  ERROR-DESIGN's status sentence; `r1_mate_lever_is_discontinuous_at_zero_extent`
  moved to pin what is true now (named in the PR's moved
  expectations); every suite green on hosted CI at job level,
  interval lanes included.

## Constraints, binding

- **Fence.** `crates/editor-core/src/mate.rs`, `mate/solve.rs`, a new
  reach module, `eval/mod.rs` (the reach adapter and the call),
  `eval/measure.rs` (visibility of `reach_of`/`curve_reach` only),
  `eval/parts.rs` only if the cache needs a reader it lacks,
  `crates/editor-core/ASSEMBLY.md` A11, `docs/ERROR-DESIGN.md` one
  sentence, `crates/viewer/src/matetool.rs`, `crates/pncad-py`
  (`py/mate.rs`, `.pyi`, the census if a tag moves), `demos/tour`,
  tests and fixtures. Not `topo`, not the predicates, not
  `coset.rs`, not `product.rs`.
- **Upper bound or refusal.** No term of the lever may under-estimate;
  a reach the module cannot bound refuses typed. No `unwrap_or`, no
  `.ok()` discarding a `PartFault`, no `_` arm.
- **One reach.** The body's reach uses the measure site's `reach_of`;
  do not write a second boundary walk. Widen visibility rather than
  copy.
- **Bit discipline.** Poses do not move where verdicts do not flip;
  the correctness arm compares `SolvedPoses` on the corpus. A row
  whose expectation moves is listed by name with the reason (a
  verdict that flips at the parts' scale is the unit's purpose, not a
  cost).
- **Ordering.** The reach is asked lazily inside the solve; a part
  evaluated from there runs under `PartCache::get`'s own shielding
  bracket. If any k-lint census row or decision-log row moves because
  a part is now evaluated before the first node, STOP (below).
- **Process.** Implementer discipline by path
  (`docs/prompts/implementer-discipline.md`); merge-only git; the
  four workspaces built; both clippy configurations; hosted CI is the
  record.

**Stop clauses.** (i) A part the repository's own fixtures, demos or
Python guide build has a face whose reach `reach_of` cannot bound and
no carrier bound can be stated for it — push what you have, mark the
PR draft, state the part and the face kind. (ii) Evaluating a mated
part from inside the solve moves a k-lint census or decision-log row
— same. (iii) A caller of `solve_document` that has no resolver or
workspace in hand and cannot be given one without a design change —
state it; do not invent a reach.

## Out of scope

The measure site; `docs/DESIGN.md` D4 ¶4 (the session box remains the
general principle; only the mate site stops borrowing it); the
predicates' margins other than the lever; authoring an extent beside
the datum (rejected on PR 2086); the split-root tie in the gather
(`work/issues/product-gather-refuses-a-split-root-whose-tie-spans-both-halves`).

## Review

Style review by `docs/prompts/reviewer-style-lane.md`. Correctness
arm, claims:

- **C1** The lever formula is an upper bound on the two parts'
  extent together from the datum, for every fixture part in the
  corpus (compute the true maximum from the body's vertices and
  curved rims and compare); no term under-estimates.
- **C2** A2/A3 on the stated documents at the three eps values; the
  verdict flips in the stated direction and nowhere else in the
  corpus (`SolvedPoses` diffed against main over every suite that
  calls the solve).
- **C3** Refusals: an unresolvable part, a face with no bound —
  typed, carried unaltered; no `unwrap_or`/`.ok()` on the road.
- **C4** Exactly-once evaluation of a mated part; no k-lint or
  decision-log row moved; the memo key flips with a verdict.
