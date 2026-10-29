---
id: patterns-are-index-variables
kind: issue
title: Patterns are index variables: PatternKind, Node::Pattern, Node::PlacedUnion and Instance(i) retire into index(N), families and Member names; mirror is a construction; presets are the façade's
status: closed
pr: 5954
opened: 2026-10-08
priority: P0
cost: H
refs: [interference-at-rest-is-a-finding, a-placement-is-the-bundle-of-mates, intent-stage3-is-built]
branch: intent/g-patterns-are-index-variables
design: true
closed: 2026-10-29
---

**Spec:** `docs/INTENT-STAGE3-SPEC.md` §8, restated against main at `4a4bb23608`; where this body and §8 differ, §8 governs. G waits on D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`): a pattern is a placement over an index into the target space it names, its copy a `Bodies` family there, combined by `⊔` (members kept as pieces, each its own arrival) or `∪` (fused). It waits on E's core (`an-operation-computes-in-a-frame-of-its-reads`) too: a member is `Framed` over its source's one body (the copies ruling), so §6 Q1, once shared with E, is answered. Its `Mirror` (§8 (c)) waits on nothing. Riders: `placed-union-places-and-fuses-in-one-node`, `explicit-placement-frames-hold-floats`, `a-map-over-a-patterns-bodies-asserts-once-per-member`. One design question stays open, measured first: whether a ring closes structurally (§8 Q1); PR 5555's mate rung does not reach it, since neighbouring members touch where no mate states it.

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
  index out of range leaves the reader unresolved and typed. `union`,
  `intersect` and `assemble` read a family directly (DM4 and PR 5494;
  a subtract's tool is one body, so cutting by a family is
  `subtract(a, union(holes))`). A definer refusing at one index
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
  never a pose: `circular_pattern` an `Axis` mate (a `Pivot` mate where
  the copy also seats, after `flag-and-pivot-pose-kinds`; an axis and a
  plane mate together overconstrain) and `Spin(Z) = scalar(k)·turn/N` (a free `Angle` step only when one is
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
  `eval/wire/stepped.rs` (`stepped_map`, `wire_pattern`,
  `wire_placed_union`, `place_each`); `eval/mod.rs`'s key tags (re-grep
  the numbers); `names/emit.rs` and `names/role.rs` (`name_pattern`,
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

A regeneration from each document's generator (DESIGN Band 4), never a
conversion of stored frames. The generators write: `Linear` as
`k = index(N)` and a placement of the master into its target, whose
mates leave one slide, with `slide = scalar(k)·spacing`; `Circular` as
an `Axis` mate on the axis with `Spin(Z) = scalar(k)·step`, keeping the
written step (never inferring a full ring); `PlacedUnion` as that
placement and a `union` of its family; `Pattern` as that placement and
an `assemble` of its family where the copies are kept apart;
`Explicit(frames)` as one placement per frame, each a `Plane` mate
against the face plus its in-plane slides and spin written as turns
and lengths, combined by `union` or `assemble`. `Part { Instance(i) }`
is `p[i]`. Names and ids move (`Instance` → `Member`; explicit-list
members by node); copies' geometry is bit-equal or moved by rounding;
re-baseline and say what moved.

## Sequencing

Stage 3 PR G (`docs/INTENT-STAGE3-SPEC.md` §8, §0). It waits on D′3
(one source per placement, a named target, `⊔`) and on E's core
(`Framed`). It closes `placed-union-places-and-fuses-in-one-node` and
`explicit-placement-frames-hold-floats`, which ride with it, and it
ends D′3's interim in which each body of a `Pattern` or `PlacedUnion`
operand of a `⊔` is its own arrival.

## §8 Q1, the ring: design fork (2026-10-28, INTENT)

G's lane measured it (`crates/editor-core/tests/g_ring_closure_measure.rs` on `intent/g-patterns-are-index-variables` @ c871332e). The door proves a ring of N planar wedges only at N=4; at N=6/8/12 no side seat proves at any budget. A designer pair (blinding on `analysis/design-fork/intent-g-ring-closure`, fork-log row 207) converged in round 2. The symbolic tier learns angle arithmetic: a product of sines and cosines becomes a sum over the angles' sums and differences, with arguments reduced modulo a turn, run per node. Every seat of a `turn/N` ring then proves, the closing one included; a free step leaves only the closing seat a finding; nothing is refused. Ev approved it on PR 5905 (2026-10-28, "i agree with the idea"). `[repetition]`'s ring sentence is sharpened; `[coincidence.structural]` needs no change, since its polynomial-identity criterion already admits the tier knowing angle arithmetic. G builds the tier rule: measure its budget per node first (a 12-ring at the shipped 256 terms, and the die's different-axis chain), give it a per-rule table and a dial like rules A–D, and make `g_ring_closure_measure` the regression row (N ∈ {5,6,8,12} all proven; a free step leaves the closing seat alone unproven; a free member-angle literal leaves every seat unproven).
