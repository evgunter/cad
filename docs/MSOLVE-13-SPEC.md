# MSOLVE-13 — A mate reads its face where it says (spec)

Unit of the `msolve` program. Item: `work/msolve/MSOLVE-13.md`.

It answers `work/msolve/a-mate-read-at-a-transform-under-a-union-refuses-read-below-a-root.md`, which Ev ruled a defect on `[ev]` PR 3695: "it should be possible to do that". Two designers weighed it on 2026-10-03 and converged in one round. This spec states their final state.

**Track:** the at-rest gate's face resolution, the member walk's union step, member identity, and the gather's recourse.

**Review:** single, full.

## What the tree says now

1. **The gate reads a mate's face by name alone.** `assembly::resolve_face` looks the bare name up in the product's table. It consults the operand only to word a refusal (`operand_answer` → `RefusedRef::ReadBelowARoot`). The solve, by contrast, reads the same reference as a name at its operand (A11 (5)). This produces three symptoms:
   - **(a) Two transforms under a union refuse.** Take `U = Union { t1, t2 }` over two transforms of one mated instance. The union re-mints the names (`FromMember`), so each mate refuses `ReadBelowARoot`, although the document is what the gather's own recourse asked for.
   - **(b) A transform above the operand gives a false refutation.** Take `t3 = Transform(t1)` as the root, with a mate read at `t1`. The member walk starts at the operand and walks down, so `t3` is never composed and the solve seats the member at `t1`'s placement. The product holds the face where `t3` moved it, under the same names (N1). The gate then verifies the contact there and reports a geometric refutation against a mate that is right.
   - **(c) A fused body cannot be mated.** A pick on it lands on `U` with the name "member `t1`'s face `f`", and the member walk refuses it (`NotAnInstancePick`).
2. **The member key includes the operand, which is only a proxy for the placement.** A11 (5) keys a member by (instance, copy chain, operand). Once a union can be read through, one placement has two spellings: at `t1`, and at `U` as "member `t1`'s `f`". Under today's key these are two members. A second mate on the pair is then not folded with the first, and the first refuses UNDER. Main already has a smaller instance of this: a mate read at `Part { P, 1 }` and one read at `P` naming copy 1 are two members.

All of this was read from the code, not run. The unit's first commit is red rows for the three symptoms (a)–(c) and for item 2.

## What the unit builds

**1. Resolve at the operand, then lift.** Put one function beside `names::verbatim_edge`, exhaustive over node kinds with no wildcard, so a new kind does not compile until it is classified. For a consumer of a node, it answers how that consumer carries an entity of the node:
- **Carried with a spelling:**
  - a `Union` carries it as `member_name(union, member, name)`;
  - a pair Boolean carries it as `FromA` or `FromB`;
  - a `Split` and a `Part` selection carry it verbatim. A `Split` may still fragment the face; that is read off its evaluated table, and a fragment is `Vanished`.
- **Moved:** `Transform`, `Pattern` and `PlacedUnion` place the geometry again.

`resolve_face` then works in four steps:
- The name must be in the operand's own table, or the mate refuses `Vanished`.
- Walk the consumers up to a root, composing lifts.
- **Exactly one** product face is reached: mint the contact on it.
- Otherwise, refuse:
  - the face was merged or fragmented on the way up: `Vanished`, naming the node that consumed it;
  - two routes reach two product faces: `Ambiguous`, never broken by picking;
  - a **moved** node on the only route: `RefusedRef::MovedAbove { at, by }`, reading "read at `at`, but `by` places it again before the product holds it; read it at `by`". This replaces `ReadBelowARoot`. The pattern-master pin keeps refusing, now for this stated reason.

When the operand is a root, or joined to one by verbatim edges only, the lift is the identity and nothing changes. Do not build the upward step from `face_descends_from` or `seam_pair::head`: they pass through `FromMember` without checking the member. Build the expected name per edge with `member_name`, and reuse `look_through_fold`'s merge and fragment handling (`eval/wire.rs`).

**2. The walk descends a union.** `mate/member.rs`'s walk continues through a `Union` at the member that `FromMember { member, of }` names, exactly as it descends a pattern through `Instance { i, of }`. `crates/viewer/src/matetool.rs`'s pick path and `member_of`'s "a union is a different body" docs follow.

**3. Member identity.** A member is its instance plus the CHAIN of placing nodes the walk passed, outermost first: pattern copies with their indices, and transforms. The operand drops out of the key.
- References through different placements remain different members.
- Two spellings of one placement become one member: at `t1` and at `U` naming member `t1`; at `Part { P, 1 }` and at `P` naming copy 1.

A document whose references are all read at their mint or at a transform keys identically to today. Measure that: no spanning tree in the mate suites may move except where two references name one placement two ways.

**4. The gather's recourse.** `ProductError::PlacedUnderTwoRoots` (`product.rs`) branches on what the recipe places twice:
- a body: "union the two to fuse them, or pattern it to keep the copies apart";
- an instance: "instantiate it again or pattern it to place it twice; union the two to fuse them".

**5. Ratified text.** These sentences are agent-written: A5's comes from MSOLVE-5 (`5104af42e`), A11 (5)'s member-identity sentence from MSOLVE-2 (`0c5bdc789`). No Ev ratification was found. Ev ruled the defect, and the unit's PR states the check. Rewrite:
- `ASSEMBLY.md` A5's minting sentence, which today says the gate "asks the operand … refuses `ReadBelowARoot`";
- A11 (5)'s member-identity sentence;
- A11 (5)'s walk sentence, to add union members.

## Acceptance

- **A1:** The three red rows go green, each with the outcome above:
  - (a) the union of two transforms mints both contacts;
  - (b) the `t3` case refuses `MovedAbove { t1, t3 }`, where it used to refute geometrically;
  - (c) a pick on `U` makes a valid mate.
- **A2:** The two-spellings row folds. A coaxial mate at `t1` beside a planar mate at `U` on the same pair determines; it does not refuse UNDER.
- **A3:** Every row that pins `ReadBelowARoot` today still refuses, as `MovedAbove` or for its stated cause. The PR lists each one.
- **A4:** No verdict moves in the mate suites except the rows the PR names.
- **A5:** The item closes. Also close `work/issues/product-table-answers-a-tie-before-kind-the-operand-the-reverse` if it dissolves (the operand lookup becomes the first rung), saying so.

## Constraints, binding

- **Discipline:** `docs/prompts/implementer-discipline.md` in full. Hosted CI is the verification of record. Work merge-only.
- **Fence:**
  - `crates/editor-core/src/assembly.rs`, `names/` (the lift), `mate/member.rs`, `mate.rs` / `mate/solve.rs` (member identity only), `product.rs` (the recourse) and `ASSEMBLY.md`;
  - `crates/viewer/src/matetool.rs`, with the viewer's rows;
  - `pncad-py`, wherever `ref_read_below_a_root` and the `at` getter cross (tag, payload, `.pyi`, census);
  - tests and the items.
- **Stop clause:** STOP and write what you measured into the PR as a draft if any of these holds:
  - the member-identity change moves a spanning tree in the corpus beyond the two-spellings case;
  - a consumer kind cannot be classified as carried or moved without evaluating a slot;
  - the `t3` row does not reproduce the false refutation on main.

## Review

One full review. These are the claims to falsify:

- **C1:** The gate resolves at the operand. No consumer kind is unclassified, and the lift is the identity where the operand is root-joined by verbatim edges.
- **C2:** A gate success means the solve's seat holds in the product. In particular, no route through a placing node succeeds.
- **C3:** Member identity: two spellings of one placement are one member, two placements are two, and nothing else moves.
- **C4:** Every former `ReadBelowARoot` pin still refuses, for a stated cause.
- **C5:** The recourse is true at its raise site for both kinds.
