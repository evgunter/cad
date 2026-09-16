# BOOL-4 — issue 750: material containment separates the extent box from the material

**Binding at dispatch** (S-BOOL program, `work/bool/plan.md`; difficulty
logged pre-draw: **L**). Read `docs/prompts/implementer-discipline.md`
in full before starting. The primary specification is the issue item
`work/bool/containment-examination-is-extent-box-coarse.md` (issue 750)
and the slate entry in `work/bool/plan.md`; this document binds the
unit.

## Situation

`crates/topo/src/census.rs`, the conservative loudness backstop
(`sweep_cross_solid_backstop`), arm 2 — **instance containment**:
for every pair of solids whose padded extent boxes are candidates, it
takes one solid's vertex-extent box against the other's REACH box and
decides six margins under `census_backstop_containment`. All six
definitely positive ⇒ `CensusUndecidable { what: "one instance's
extent box inside another's — the interference class (recorded
gate-skips do not exist yet)" }`; any definitely negative ⇒ clear;
anything weaker ⇒ `CensusUndecidable` "in band". Since PR 737 deleted
the record deferral (a declared TRUE contact used to switch this
examination off — a live wrong answer), the arm's verdict is
independent of declarations, which is the intended property, and the
cost is the issue: **a part sitting in a concavity has its box inside
the container's box while sharing no material**, so no L-bracket,
blind-bore, pocket or cavity assembly can pass the certifying door by
any declaration. The refusal is honest — the arm genuinely cannot tell
the L-bracket from the embedded cube — and it is honest because the
arm reads boxes where the question is material.

The reproduction (issue 750, verbatim geometry; the red-first row):
container `common::prism_z` over the L profile `(0,0), (3,0), (3,1),
(1,1), (1,3), (0,3)`, `z ∈ [0,1]`; part cube `x ∈ [1,2]`,
`y ∈ [1.2,2]`, `z ∈ [0.2,0.8]`, resting flat on the inner wall
`x = 1`, four v-on-f records all confirming; the part is wholly
outside the bracket's material. Declared: one error,
`CensusUndecidable{Solid, Solid, "one instance's extent box inside
another's"}`. Undeclared: 9 errors including that one.

**Two falsifications bind, verbatim** (issue 750; `work/bool/plan.md`
§Ratified ground):

1. **No C6 gate-skip.** The assembly design
   (`crates/editor-core/ASSEMBLY.md`, C6) scopes recorded gate-skips
   to interference fits — deliberately overlapping shells. The
   L-bracket has no overlap at all. A gate-skip would suppress the
   refusal rather than fix it, and suppressing it re-creates exactly
   the class PR 737 removed: a declaration that turns a check off.
2. **No separating plane derived from the container's own face
   planes.** The obvious record-free narrowing that reuses data
   already in `Geo` is unsound exactly on non-convex containers:
   extend the L's `x = 1` plane and points at `x > 1, y < 1` are
   outward of it and inside the material.

The question, then, is a **material** test: is the contained
instance inside the container's material, or inside its box and
outside its material? The kernel already owns the door that answers
"is this point in this body's material" — `point_in_solid`
(`crates/topo/src/boolean/solid_contain.rs`): trilean, closest-hit,
grazes retry on the fixed schedule, exhaustion typed, planar plus the
issue-1011 curved arms BOOL-2 and BOOL-3 landed (cylinder, sphere,
cone, torus). The coupling the plan flagged is real and is the design:
**this unit consumes those arms; it derives no ray casting of its
own.**

## FIRST, before the build — three measurements, reported

1. **The per-solid door.** `point_in_solid(body, q, band, tol)` sweeps
   every face of every shell of the BODY; the census asks about ONE
   solid of a multi-solid body. Report how the restriction is
   expressed (the faces of the shells of one `SolidKey`, in arena
   order), whether the closest-hit core is reusable as-is behind a
   face selection (it must be: ONE core, two entries — no second ray
   sweep), and the `Tol` plumbing: `census_with` holds `band` only,
   `point_in_solid` takes `band` and `tol`, and the census's caller
   `pseudomanifold_certificate_via` (`validate.rs`) holds `tol`.
   `Tol::witness()` is banned in kernel `src`; name the signatures
   that widen (`census_and_certify`, `census_with`, the
   `census_traces*` doors re-exported from `lib.rs`) and their
   callers.
2. **The pair invariant the clear rests on.** A single witness point
   decides a whole instance only if the instance's interior lies in
   ONE component of space minus the container's boundary — i.e. the
   two solids' boundaries touch at most in coincidences and never
   cross. State where `census_with` establishes that before arm 2
   runs (the exact sweeps — vertex/edge/face pairs — and backstop
   arm 1 run first and push every crossing or unexaminable pair into
   `errors`), and what arm 2 must check before trusting a witness:
   the pair has no error already pushed against either solid's
   entities for this pair, or the arm reads the trace. If the
   invariant cannot be read off the census's own state without
   re-sweeping, STOP and report the shape you would need.
3. **The refusal vocabulary for a DECIDED interference.** With a
   material test, "inner definitely inside outer's material" is no
   longer undecidable — it is an interference fit, decided. Reporting
   it under `CensusUndecidable` would be a false sentence. List the
   consumers a new `ValidationError` variant reaches
   (`crates/editor-core/src/assembly.rs`'s attribution match,
   `crates/pncad-py/src/tags.rs`'s census arm and `tests.rs`'s tag
   list, the display contract) and report the reach; the shape is
   stated under deliverable 2, not asked.

## Deliverables

1. **The material test, in arm 2.** The six-margin box test stays the
   GATE (the "too BIG is wrong" paragraph stays true: over-width in
   the reach box still costs an answer, now by sending a separated
   pair into the material test rather than into a refusal). When all
   six margins are definitely positive, instead of refusing: probe
   the contained instance's vertices in arena order against the
   CONTAINER's material through the per-solid point-in-solid door at
   the run band —
   - the first vertex answering definitely **`Out`** ⇒ **clear**
     (inside the box, outside the material — the L-bracket, the
     pocket, the cavity);
   - definitely **`In`** ⇒ the decided interference (deliverable 2);
   - **`OnBoundary`** ⇒ skip to the next vertex (a coincidence is the
     confirm pass's business, not this arm's);
   - an escalation, `RayExhausted`, or a face kind the door does not
     serve ⇒ `CensusUndecidable` with a `what` naming the cause
     (the existing in-band wording stays for the box-in-band case);
   - every vertex `OnBoundary` ⇒ `CensusUndecidable` with a `what`
     saying so (an instance with every vertex on the container's
     boundary is not a placement this arm decides).
   The clear is sound by measurement 2's invariant: a boundary point
   of the contained instance strictly outside the container's material
   has interior points of the instance beside it that are also
   outside, and with no crossing between the two boundaries the whole
   interior is in that component. State the argument once, at the
   arm, and point at it from the audit row. Both directions of the
   pair are examined as today. **Neither falsification is retried**:
   no record consulted, no plane derived from the container's faces.
   The per-solid door lives in `solid_contain.rs` beside
   `point_in_solid`, both entries over the one closest-hit core.
2. **The decided interference.** `ValidationError::InstanceInterference
   { outer: SolidKey, inner: SolidKey, witness: VertexKey }` (or the
   `EntityId` spelling the enum's neighbours use — match them), with
   a Display that says one instance's material contains a vertex of
   another's — an interference fit — and that recorded gate-skips do
   not exist. `Attribution::Unattributed` in `assembly.rs` beside
   `CensusUndecidable`; a census row `instance_interference` in
   `tags.rs` with `tests.rs`'s list moving (disclosed as the census
   move it is); the display-contract row. The embedded cube (1 m in
   4 m, flush at `z = 0`, four v-on-f) refuses under it.
3. **Rows** (red-first, each pinned at the merge base first with the
   message copied verbatim into the PR): the L-bracket declared →
   `Ok(())`; the L-bracket undeclared → the remaining errors with NO
   containment refusal among them (state the count you measure — the
   issue says 9 before; say what the 8 are); the embedded cube →
   `InstanceInterference` (was `CensusUndecidable`); a cavity
   assembly (a two-shell hollow container with a part in its void) →
   clears; a part in a planar pocket → clears; a part in a blind BORE
   (a cylindrical wall face) → **measure** whether backstop arm 1's
   proximity class refuses the curved face pair first, and pin what
   happens — do not claim the bore clears if arm 1 refuses it; say
   so, and the bore stays on the slate as arm 1's (the exclusion
   ring's) case; a part with every vertex on the container's
   boundary → the typed refusal; a container carrying a face kind the
   door does not serve → `CensusUndecidable` naming the cause; the
   witness escalating at the band edge (a vertex within ε of the
   container's boundary, not on it) → the typed in-band refusal, at
   three ε rows; `--features interval` on the whole suite.
4. **`Tol` threaded, not witnessed.** From `pseudomanifold_certificate_via`
   into `census_and_certify` and down to the door; the `census_traces*`
   doors widen with it and their callers move; no `Tol::witness()`
   under `crates/topo/src` outside `#[cfg(test)]`.
5. **Docs re-recorded**: arm 2's doc in `census.rs` (the box is the
   gate; the material test decides; the invariant); the
   `CensusUndecidable` doc's arm-2 sentence; `crates/editor-core/ASSEMBLY.md`
   lines 230–231 — interference is now DECIDED by the material test
   and refused typed; recorded gate-skips are still not implemented.
   S-MATE has exited (`docs/DOC-LEDGER.md` sweep 6, design of record
   at `ASSEMBLY.md`), so the handoff the plan recorded "to S-MATE at
   landing" is recorded IN `ASSEMBLY.md`, as a sentence a future
   gate-skip unit reads; say so in the PR.
6. **D9 / behaviour**: no boolean result moves — this is a validation
   arm, not an op; the tour, the die corpus and the document gallery
   render bitwise against the merge base; every test, pncad-py row
   and demo whose refusal was the retiring class is enumerated (grep
   `census_undecidable` and `CensusUndecidable` expectations) and each
   move is listed with its new verdict.
7. **ε posture**: no new comparand — the witness decides through
   `point_in_solid`'s existing predicates at the run band; the arm's
   `census_backstop_containment` audit row is unchanged; the per-solid
   door mints no new predicate key (if it must, name it and add the
   audit row — it should not).
8. **Class sweep** (discipline §5): other census arms that answer from
   boxes alone where the question is material — backstop arm 1's
   proximity class (curved pairs; the exclusion ring is the certified
   excluder) — measure, report, do not act; the `declared.faces`
   cross-solid observation from the issue is filed at spec time as
   `work/bool/declared-faces-has-no-cross-solid-check.md` — cite it,
   do not act on it.

## Acceptance

Three measurements reported first; the L-bracket declared certifies;
the embedded cube refuses as a decided interference; the invariant
stated at the arm and its precondition checked in code; neither
falsification retried; `Tol` threaded; hosted CI green; gate record
per head.

## Hard rules

- NO `Co-Authored-By`, no model names; no closing keywords; "issue 750"
  spelled out (the orchestrator closes the item).
- Scope fence: `crates/topo/src/census.rs` (arm 2 and its docs; the
  precondition read), `crates/topo/src/boolean/solid_contain.rs` (the
  per-solid entry over the one core), `crates/topo/src/validate.rs`
  (the variant; the `Tol` hop), `crates/topo/src/lib.rs` (the
  re-exports' signatures), the topo census/validate test suites,
  `crates/editor-core/src/assembly.rs` (the attribution arm only) and
  `crates/editor-core/ASSEMBLY.md` (the two lines), `crates/pncad-py`
  census rows. NOT: the exact sweeps, backstop arm 1, `Candidates`,
  the ray schedule and the closest-hit core's decisions, the boolean
  ops, C6's design, any record deferral.
- STOP and report if: measurement 2's invariant cannot be read off the
  census's state; the `Tol` hop reaches beyond `validate.rs`'s doors
  and their direct callers; or the only route you find is a gate-skip
  or a face-plane separator.
- Merges on green after the dual (the approach is ruled; the variant's
  shape is stated, not asked).
- Re-merge main before opening the PR.
