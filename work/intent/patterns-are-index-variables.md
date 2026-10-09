---
id: patterns-are-index-variables
kind: issue
title: Patterns are index variables: PatternKind, Node::Pattern, Node::PlacedUnion and Instance(i) retire into index(N), families and Member names; mirror is a construction; presets are the façade's
status: parked
opened: 2026-10-08
priority: P0
cost: H
refs: [interference-at-rest-is-a-finding, a-placement-is-the-bundle-of-mates, intent-stage3-is-built]
blocked_on: [transform-retires-into-a-placement, a-union-member-is-keyed-by-its-read]
---

FORK-PAT (fork log row 99). Ev, 2026-10-08, on interference between a
pattern's outputs: "for pattern interference, i am wondering if we
should rethink patterns more broadly to all just be built from programs
and having no special presets"; "i am asking if we can get rid of these
special cases as fundamental kernel entities and replace them with maps
over numbers + some facade helper functions"; "we should have a map
higher order function to allow declaring over all the outputs of the
pattern. (will this suffice?)". The designer pair converged; the
recommendation is in `docs/DESIGN.md` D10 **Repetition** on the `[ev]`
PR. This row supersedes
`a-map-over-a-patterns-bodies-asserts-once-per-member` (opened on PR
4339), which is closed when this lands.

## What is built

- **The index.** `k = index(N)` (and `index(N) within j`) is a `Count`
  definition over `0..N`, `N` any `Count` expression. A definition or
  operation whose reads reach `k` is evaluated once per value; its
  outputs are **families**, flat, keyed by the tuple of indices reached,
  outer first. No node holds a template.
- **The kind rule.** A variable defined by reading `k` is its member at
  the same value to a reader reaching the same `k`, and the whole family
  to every other reader. Lockstep is one index read twice; two indices
  meet in one reader only when one is `within` the other (implied when
  its count reads it), else the reader refuses.
- **Reads.** `xs[i, j]` reads a member (DM3's pick becomes this); an
  index out of range leaves the reader unresolved and typed. `union` and
  `subtract` read a family directly (DM4: a member may be a family; the
  two-body floor moves to evaluation). A definer refusing at one index
  refuses the whole family, naming the index.
- **`Count` arithmetic** gains exact `mod` (with the existing operators);
  `scalar(k)` (today's `CountToScalar`) puts an index into a length or
  angle. No list literal.
- **Names.** `RoleSeg::Instance { i, of }` becomes
  `Member { (i, j, …), of }`, keyed by the index variables' ids and the
  integers. P5's placement-major layout (`j·M + i`) goes; the `SegPat`
  index predicate matches `Member` by tuple.
- **Mirror.** `Mirror { body, plane }`, a construction defining a new
  `Body` in its source's root, the plane a `Plane` pose whose reads
  reach that root alone (a face read as a plane, or a plane constructed
  from the body's geometry) (MIRROR-DESIGN P1–P4, P6). A symmetric part
  is `union(body, Mirror { body, plane })`, its seam the plane's face
  read twice, so the glue is structural. Improper poses become
  unrepresentable in the pose kinds rather than refused at each door
  (`Frame::admission_fault`, `EditError::ImproperPlacement`).
- **Group boolean.** `PlacedUnion` retires; its `Separation` certificate
  becomes `union`'s fast path when its bodies are rigid images of one
  body, visible in the reads; an uncertified pair goes through DM4's
  pairwise pass instead of refusing `PlacementsUncertified`.
- **Evaluation.** Copies of one body are built once and mapped; a
  construction reads no frame, so a feature at several positions is one
  body placed per member, and a construction is built per member only
  when its scalar inputs read the index (D9 convention 4: equivariance
  audited per site). D9 caching keys a
  node reading an index by its content key plus the index values.
  Orbit structure, if a reader ever wants it, is derived from a
  placement's values being affine in the index, never declared.
- **Façade and GUI.** `linear_pattern`, `circular_pattern`, `grid`,
  `bolt_circle` and `mirror` write the program, as mates and values,
  never a pose: `circular_pattern` an `Axis` mate, a `Plane` mate and
  `spin = scalar(k)·turn/N` (a free `Angle` step only when one is
  typed); `linear_pattern` a mate leaving one slide and
  `slide = scalar(k)·pitch`; `grid` a `Plane` mate and two slides over
  `i` and `j within i`; bolts into holes `bolt.axis ≡ holes[k].axis`. A display-only recogniser in the façade, matching a placement under an
  index whose values are affine in it, reads a stored program back as "circular, 12 about A" for the GUI's forms
  (`PatternKindChoice`, `PatternRuleSpec` become its output and input);
  no preset tag is stored.

## Unchecked requirement

Two members under one `Axis` mate differ only by their spin values, so
ring closure is an `Angle` comparison modulo a turn, not a matrix
identity: it is structural only if the symbolic tier (E12) reduces a
spin value's chart modulo a turn (cos(turn/12) is irrational, so a
polynomial identity over ℚ in matrix entries does not prove
`N · turn/N = turn`). Check this before the
façade's full-ring default is relied on to quiet the
`unproven-coincidence` lint.

## Sites

- `crates/editor-core/src`: `node.rs` (`PatternKind`, `Node::Pattern`,
  `Node::PlacedUnion`, `PartSelect::Instance`, the placement-rule slot
  table, `placement_rule_fault`, `CountMismatch`); `eval/wire.rs` and
  `eval/stepped.rs` (`stepped_map`, `wire_pattern`, `wire_placed_union`,
  `place_each`); `eval/mod.rs` key tags 12/13/19–22; `names/emit.rs`
  and `names/role.rs` (`placer_axis`, `name_pattern`,
  `name_placed_union`); `mate/member.rs`; `refactor.rs`;
  `persist/check.rs`; `assembly.rs`; `placement.rs` (`Frame`'s improper
  arm). About 60 editor-core test files.
- `crates/viewer`: `forms.rs`, `session/author.rs`, `session/op.rs`,
  `combine.rs`, `matetool.rs` and about 17 tests.
- `crates/pncad` re-exports and `guide.rs`; `crates/pncad-py`
  `place.rs` (`PatternKind`), `doc.rs` (`pattern`, `placed_union`,
  `part`), `pncad.pyi`, about 11 Python tests; the `pncad` façade.
- The tour: `assembly.rs` bench layout, `bool_bodies.rs`,
  `diefillet.rs`.
- Corpus: `die_tool` (and `tests/corpus/die_tool.pncad`, the one
  persisted document holding a pattern), `heatsink_union`, `sink`,
  `part_select`.
- Docs that follow: INTENT-STAGE2 §Pattern; INTENT-STAGE5 B's
  one-node overlap sentence (an overlap between members is an at-rest
  finding between copies); ASSEMBLY A11 (5)'s member walk (its
  `Pattern` level); NAMES N1 (`Instance` → `Member`).

## Migration

A one-time regenerate. `Linear` → `k = index(N)` and a placement of
the master whose mates leave one slide along the stored direction, read
off geometry (the anchor #4326 requires), with
`slide = scalar(k)·spacing`; `Circular` → an `Axis` mate on the stored
axis with `spin = scalar(k)·step`, keeping the stored step (never infer
a full ring; the `unproven-coincidence` lint proposes `turn/N`).
`PlacedUnion(Explicit(frames))` → one placement per frame, a `Plane`
mate against the face with the frame's in-plane offsets and spin as
values, computed from the stored frame once at migration, gathered by
`union`. `Part { Instance(i) }` → `xs[i]`. Names and ids move
(`Instance` → `Member`; explicit-list members by node); geometry is
bit-equal for copies; re-baseline
and say what moved.

## Sequencing

Stage 3 PR G (`docs/INTENT-STAGE3-SPEC.md` §8). It waits on stage 3 D,
which follows B (a placement), C (values: `slide = scalar(k)·pitch`)
and D itself (no construction reads a frame, and `Transform` is gone).
It also waits on FORK-DM4 (`a-union-member-is-keyed-by-its-read`):
`PlacedUnion` retires into a placement and a union reading a family,
and how a union keys its members' names is what that fork decides. G
closes `placed-union-places-and-fuses-in-one-node` and
`explicit-placement-frames-hold-floats`, which ride with it.
