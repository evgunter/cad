# MSOLVE-8 — A levered clash names its arm; the coset's directions carry the witness; `MateFault` names its consumers (spec)

Unit of the `msolve` program. Item `work/msolve/MSOLVE-8.md`. Answers
three rows on `work/msolve/`, read them in full before the code:
`levered-clash-margins-hide-their-arm` (S-MATE's display residue,
re-homed by FIX), `subgroup-directions-are-unit-by-prose` (SCALAR's
class sweep) and `mate-fault-subject-spelled-in-three-crates` (CHROME,
with its 2026-09-15 evidence). **Track:** a refusal's payload and
sentence gain what they measured; a precondition becomes a type; one
doc sentence — no verdict moves. One style review plus a correctness
arm (§Review). No A/B row. Sequenced after MSOLVE-6 because the arm
these margins carry is the one it changed, and after MSOLVE-7 because
that unit is in `solve.rs` now.

## What the tree says now

1. **Three levered clashes arrive with their arm invisible.**
   `MateFault::Contradictory` (`mate.rs`) carries `clash: f64` and
   `lever: Option<(f64, f64)>`, documented and rendered as
   `(radians, arm)`: "measured a roll of {radians} rad on a {arm} m
   arm, a deviation of {product} m". One site fills it — the clocking
   rider's `mate_clocking_redundant` in `solve.rs::mate_coset`, whose
   disagreement IS an authored angle. Three sibling margins in
   `coset.rs` are a pure number times the arm and reach the variant
   through `FoldStop::Clash { predicate, margin }` with `lever: None`,
   so the sentence prints a levered product as a bare metre figure the
   reader cannot re-derive: `member_of`'s `mate_member_axis_fixed`
   (`‖Q·a − a‖ · arm`) and `mate_member_rotation_identity`
   (`rotation_residual`: `‖Q − I‖_F · arm`), and `candidate_rotation`'s
   `mate_rotation_two_axis_reachable` (`reach · arm`). The five other
   `member_of` predicates measure lengths outright and correctly carry
   no lever (two `asm_r2a_mate_solve` rows pin that). The item records
   the trap: a sine, a Frobenius departure and a reach are NOT radians,
   and an earlier `unit: &'static str` beside the value was rejected as
   a free string on a public surface. **The `lever` field's doc is
   also stale since MSOLVE-6** ("floored at one metre"): the arm is
   the mated parts' own extent plus the datum term now, and its
   sentence at that field must say so.

2. **Directions are unit by prose.** `coset.rs::Subgroup`'s `normal` /
   `direction` fields are `Vec3<f64>` with "UNIT by construction of
   every constructor here" in the doc; `parallel` (`‖u × v‖`) and
   `perpendicular` (`u · n`) lever a sine or a cosine by `arm` into a
   decided margin, so an unnormalized direction would scale a DECIDED
   margin silently — the failure `geom_core::UnitVec3<T>` exists to
   make unrepresentable (SCALAR's ruling, `work/scalar/unit-vector-
   invariants-carried-as-prose.md` §RATIFIED: the witness lives in
   `geom-core`; its mints are the normalizing constructor `UnitVec3::
   new(v, site, band)`, exact negation, `sin_cos`, and the decided
   frame ladders; the frame witness `OrthoFrame<T>` carries three
   `UnitVec3` axes and converts to an `Affine3` by `to_affine`). The
   solve's directions come from ONE read: `mate_coset` takes
   `MateFrame::placement(tol)` — `frame::point_at`, a decided ladder —
   and reads `.linear.c2` off the `Affine3`, where the witness the
   ladder had was already dropped; `invert` transports them by the
   representative's rotation. SCALAR's rule: a function in the class
   takes the witness the day its caller holds one. The caller can hold
   one at the frame read.

3. **Which mate a `MateFault` is about has no home on the enum.** Two
   consumers re-derive it per arm — `viewer::tree::blamed_mates`
   (exhaustive) and `pncad_py::MateFaultPayload` — and neither cites
   the other. CHROME's evidence is decisive against a bare
   `subject() -> Option<RecipeNodeId>`: `Band` names no mate and
   reaches EVERY row of the document (`solve_document` records the one
   fault against every `Node::Mate` and `Node::InstantiatePart`
   before reading a mate), `PosesOfAnotherDocument` names no mate and
   reaches NO row (raised by `SolvedPoses::placement`, never inserted
   in a fault map) — an asymmetry a bare `Option` erases. **Ruled
   (plan.md item 15):** no `subject()`; the row closes by the form it
   sanctions — one sentence on `MateFault` naming its two consumers
   and stating the asymmetry once, so it has one home instead of two
   comments.

## What the unit builds

**1. The lever is typed by what it measured.** Replace the
`(f64, f64)` socket with a closed enum beside `MateFault`:

```rust
/// What a levered clash measured, and the arm that carried it to a
/// length. The product is the deviation; the halves are what the
/// sentence prints, because a stored product beside them would assert
/// an identity nothing enforces.
pub enum Lever {
    /// An authored roll, in radians (the clocking rider).
    Roll { radians: f64, arm: f64 },
    /// A dimensionless residual — a sine, a cosine, a Frobenius
    /// departure from the identity, a reachability defect — named by
    /// the predicate that measured it.
    Residual { value: f64, arm: f64 },
}
```

`Contradictory.lever: Option<Lever>`; `None` only for a predicate that
measured a length outright or decided structurally (`MATE_MEMBER_EMPTY`),
which `predicate` settles as today. `Display` gains the second
sentence: "measured a residual of {value} (`{predicate}`, dimensionless)
on a {arm} m arm, a deviation of {product} m where the cosets would
have had to meet". No free string, no unit knob: the two arms are the
two kinds of number the solve levers, and a third kind is a new arm
with its own sentence. The `lever` field's doc says what the arm is
now (MSOLVE-6's, by pointer to A11 rule 5), not the floor.

**2. The plumbing carries the halves.** `member_of` returns the failing
predicate with a `Clash { margin, lever: Option<Lever> }` (or the
equivalent named pair) instead of `(name, f64)`; `FoldStop::Clash`
carries it; `candidate_rotation`'s reach site fills `Residual { value:
reach, arm }`; `rotation_residual` returns the bare Frobenius norm
and the caller levers it, so the value the sentence prints is the
number the predicate decided on. `solve.rs`'s `FoldStop::Clash` arm
forwards the lever. The `member_of` predicates that measure lengths
keep `lever: None` and the two `asm_r2a` rows keep pinning it.

**3. The witness enters at the frame read.** `MateFrame` gains
`frame(&self, tol) -> Result<OrthoFrame<f64>, FrameError>` through the
`OrthoFrame` ladder that matches `point_at`'s construction (aim along
`axis`, roll by `reference`), and `placement` is `frame(tol)?.
to_affine()` so the affine every other reader takes is unchanged bit
for bit — pin it. `mate_coset` reads the axis as `frame.w()`
(a `UnitVec3<f64>`), and `Subgroup::{Planar.normal, Cylindrical.
direction, Prismatic.direction, Revolute.direction}` become
`UnitVec3<f64>`; `parallel` and `perpendicular` take `UnitVec3` and
their doc drops "unit" as a precondition because the type carries it;
`Subgroup`'s doc drops "by construction of every constructor here".
Where the solve derives a direction from another by a rotation (the
`spin(theta)` target's `c2`, `invert`'s transport by the
representative's rotation, `clocking_about`'s axis), take the witness
where `geom-core` admits one (a rotation applied to an `OrthoFrame`
yields an `OrthoFrame`, if that door exists) and otherwise re-mint
under the run's band with `UnitVec3::new(v, "<named site>", band)`,
mapped to `MateFault::Indeterminate` on refusal like every other
decision in the fold; say in the PR which sites took which road. A
re-mint on a direction rotated by a proper rotation decides a length
within rounding of one and never refuses on a document the doors
build; a refusal there is the witness doing its job, not a regression.
The `f64`-only status of `coset.rs` is unchanged (the row: the
generic-scalar half of the witness does not arise here).

**4. One sentence on `MateFault`.** The enum's doc names its two
consumers by path (`viewer::tree::blamed_mates`, `pncad_py::
MateFaultPayload`) and states the asymmetry once: `Band` names no
mate and reaches every row of the document; `PosesOfAnotherDocument`
names no mate and reaches none. The two consumers' comments on that
asymmetry become one-line pointers to the enum. No method.

**5. Consumers.** `pncad-py`: `MateFaultPayload.lever_tilt` /
`lever_arm` become `lever_kind: Option<&'static str>` (`"roll"` /
`"residual"`), `lever_value`, `lever_arm` (or the names the payload's
own vocabulary prefers), the presence array and the `.pyi` and the
Python row on a levered clash updated; the viewer renders `Display`
and needs no code. `docs/ERROR-DESIGN.md` E3's sentence on the levered
clash, if it names the pair, names the enum.

**6. The rows.** `crates/editor-core/tests/msolve8_levered_clash.rs`
(registered in `tests/all.rs`): each of the three margins reached
through ordinary doors with a document that trips it (the item's
`asm_r2a` rows and `mate1_r1_probes` already build contradictory
pairs — reuse their fixtures), asserting `lever: Some(Lever::Residual
{ value, arm })` with `value * arm` equal to `clash` bit for bit and
the sentence naming the predicate; the clocking rider still
`Lever::Roll`; a length predicate still `None`; `placement(tol)` bit-
identical to `frame(tol)?.to_affine()` over the mate frames of every
fixture document; a direction re-minted from a proper rotation never
refuses over the same fixtures. `display_contract.rs` gains the new
sentence.

## Acceptance

- **A1** Every `Contradictory` raised in the workspace carries a lever
  iff its predicate levered one, pinned per predicate (the three, the
  roll, one length, the structural one).
- **A2** `Subgroup` directions are `UnitVec3<f64>`; no `Vec3` direction
  reaches `parallel` / `perpendicular`; the affine every other reader
  takes is bit-identical.
- **A3** No verdict moves: every mate row in the workspace passes
  unchanged, and the PR names any expectation that had to move (a
  moved `Contradictory` payload is expected; a moved verdict is a
  finding).
- **A4** The Python payload, `.pyi`, census and one Python row agree;
  the two consumers' comments point at the enum's sentence.
- **A5** The three items closed on the branch with `## Closed`
  sections citing the rows; `work.py lint` clean.

## Constraints, binding

- `docs/prompts/implementer-discipline.md` in full, by path. Hosted CI
  is the verification of record; poll it in the foreground; never end
  a turn with background work active. Four Cargo workspaces; a public
  signature change (`Contradictory.lever`, `Subgroup`) is checked in
  all of them before the push; `pncad-py`'s clippy with `--features
  python --all-targets` too.
- Merge-only; push early and often; the PR through the GitHub MCP
  tools. Private `CARGO_TARGET_DIR` outside the worktree; `git status`
  before every `git add`; never `git add -A`.
- Fence: `crates/editor-core/src/mate.rs`, `mate/coset.rs`,
  `mate/solve.rs`, `crates/pncad-py` (payload, tags, `.pyi`, tests),
  `crates/viewer/src/tree.rs` (the one comment only), `docs/ERROR-
  DESIGN.md` (one sentence, if it names the pair), tests, the three
  items. Nothing in `geom-core` (if a door is missing there, take the
  re-mint road and file the door on SCALAR's slate), nothing in the
  edit door, the walk or the evaluator.
- Comments state the invariant (discipline §4).
- **Stop clause.** If a mate row's verdict moves under the witness
  (a direction the solve used was not unit — that is a finding to
  file with the document that shows it, and the unit stops there); if
  `OrthoFrame` cannot reproduce `point_at`'s affine bit for bit for
  every fixture mate frame; or if a third kind of levered number turns
  up that is neither a roll nor a dimensionless residual — STOP, write
  what you measured in the PR as a draft, and end your turn.

## Out of scope

The `FromFace` frame (MSOLVE-9); the static clocking refusal at
`AddMate` (MSOLVE-10); a `subject()` method (ruled against); the
carrier fields of `geom` (SCALAR's at-rest rule); the generic-scalar
half of the witness in `coset.rs`.

## Review

One style review plus a correctness arm, claims verbatim:

- **C1** A1: for each of the three predicates, a document that trips
  it yields `Residual { value, arm }` with `value · arm == clash`
  bit for bit and `value` equal to the number the predicate decided
  on (re-derive it from the fixture's frames).
- **C2** A2: `placement(tol)` and `frame(tol)?.to_affine()` agree bit
  for bit over every fixture mate frame; every direction the fold
  compares is a witness; the re-mint sites never refuse over the
  fixtures.
- **C3** A3: enumerate the mate suites and confirm no verdict moved.
- **C4** The enum's consumer sentence is true: `Band` reaches every
  row and `PosesOfAnotherDocument` none, measured, not read.
