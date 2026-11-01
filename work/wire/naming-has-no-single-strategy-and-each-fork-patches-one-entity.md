---
id: naming-has-no-single-strategy-and-each-fork-patches-one-entity
kind: issue
title: Names have no single strategy: each naming fork patches one entity kind, and the same leak (build order, slot, index, frame) keeps recurring
status: closed
opened: 2026-10-20
priority: P1
cost: H
design: true
pr: 5455
branch: wire/ev-naming-strategy
closed: 2026-10-20
---


Ev on PR 5426 (2026-10-20): "it seems like we are still in general
missing a good consistent strategy for names in a way that keeps causing
problems".

## The recurrence (WIRE's naming forks, 2026-10-17 to 20)

| fork-log row | entity | what leaked into the name |
|---|---|---|
| 158 FORK-CIRCLEPHASE | a split circle's pieces | the circle's phase, a frame choice |
| 160 FORK-SPANCHORD | a chord across merged constituents | no name: the rule covered one constituent |
| 163 FORK-INPLANE | an in-plane operand edge | its own name died: build classified it as a chord |
| 169 FORK-ROWPRESENCE | an n-ary union's coincidence rows | fold (member) order |
| 178 FORK-PAIRSEAM | a pair boolean's seams and crossings | operand slot; bare names collided |
| 179 FORK-SECTIONID | a split's section faces | completion (sweep) order, a plane frame |

Each fork was settled for its own entity kind. The shared shape: a name
read something the author did not write and the operands' names do not
fix (an order, a slot, an index, a frame) or failed to read something
they do.

## What is owed

1. A survey of every naming defect in `work/` (open and closed), grouped
   by what leaked, to test whether the shape above covers them.
2. A strategy stated once, as a principle the N-clauses instantiate rather
   than a per-entity rule, if the survey supports one.
3. A mechanical check that finds the class before an issue does. For
   example, a property test that perturbs what a name must not read (member
   order, operand order for symmetric ops, the plane frame, a tilt within
   one topology, a Transform copy of an operand) and asserts the name
   table is invariant up to declared ties.

The strategy is Ev's to ratify (an `[ev]` PR); the survey and the check
are not.

## The survey (WIRE lane `wire-namesurvey`, 2026-10-20, main at `3b687e1652`)


**Method.** I scanned all 4,369 live `work/` rows. I also scanned the 1,196 rows deleted when closed programs left the tracker, read from git history at the commit before each deletion: `emit` (123 rows) is the main closed naming program, plus `docm`, `blend`, `recipe` and `fuse`. I filtered by title and body (emitter paths, `RoleSeg`/`NamingError`/N1–N5, order/tie/rank words) and then sorted by hand. I left out rows about refusal wording, display words, perf, Python bindings, and member-order effects on geometry rather than names. The counts are **likely within ±15%**: the sorting is manual and some rows fit two groups. The worktree and target dir are deleted; nothing was committed.

### 1. Census (about 120 naming defects)

| group (what leaked) | count | open |
|---|---|---|
| Order (fold, member, completion, build, mint) | 29 | 3 |
| Slot or operand position (A/B, which output) | 14 | 5 |
| Index, ordinal or sibling count | 14 | 6 |
| Frame or coordinate (normal sign, chord, carrier parameter, phase, chart seam) | 21 | 6 |
| Absent case (no rule; refusal on a legal body) | 20 | 5 |
| Other (collision, kernel lineage records, growth, retirement) | ~22 | ~8 |

Three exemplars per group, by id with title quoted. `emit/…` rows are closed and live only in git history.

- **Order.**
  - `emit/union-face-names-follow-fold-order`: "merged-then-cut is Merged(set)#SideOf, cut-then-merged is the merge plus a bare constituent".
  - `wire/a-pinch-vertex-is-named-by-the-fold-step-that-mints-it`: "its name depends on the member order".
  - `wire/a-section-faces-index-is-its-completion-order-not-its-identity` (FORK-SECTIONID 179): "a held section name silently denotes another face".
  - Also in this group: fold-row spellings ×5 (wire), `three-member-coincidence-rows…` (169), `emit/name-order-was-insertion-order-under-the-counter` (mint order), `wire/a-section-faces-division-root-is-the-operand-face-its-cut-ran-in` (build order: whichever face the kernel's `mef` cut ran in).
- **Slot.**
  - `wire/the-pair-verbs-declared-merge-is-asymmetric-in-its-operands`: "which member's rims fragment follows the A/B assignment".
  - `emit/seam-chain-ranks-are-oriented-a-first-so-an-operand-swap-may-reverse-them`.
  - `refs/a-name-is-scoped-by-the-variable-holding-its-body`: the split's half segment restated the output.
  - Also: `should-pair-seams-take-name-ordered-sides` (178), `intent/a-union-member-is-keyed-by-its-read`, `seamrank/seam-line-ranks-keep-a-sided-axis…`.
- **Index.**
  - `borders/section-face-and-hole-rim-are-ordinals-over-their-group`: "a new section or hole renumbers the others".
  - `emit/a-second-crossing-by-one-face-renames-the-first-and-its-pieces` (row 73).
  - `borders/borders-and-keeps-spell-a-lone-piece-bare`: "it gains Borders or Keeps when a sibling appears".
  - Also: `emit/a-param-jump-that-swaps-the-outer-loop-renumbers-names-unreported`, `emit/loft-anchors-every-section-with-section-zeros-map`, `intent/a-split-name-picks-a-pattern-copy-by-its-evaluated-index`.
- **Frame.**
  - `wire/a-revolved-pinch-split-refuses-with-its-normal-flipped`.
  - `emit/union-partners-are-the-planes-that-tell-its-pieces-apart` (rows 14 and 18: Ev called the half-space reading "fragile").
  - `emit/edge-dir-is-a-chord-so-curved-edge-pieces-misrank`.
  - Also: `seamrank/seam-rank-extents-compare-carrier-parameters-as-metres`, three NURBS no-parameter rows, `vseam/a-picked-half-rim-blends-half-and-refuses` (`BandRim`/`BandRimPi` split at the chart seam), `band/lamina-plane-annulus-keeps-its-slit`, `wire/a-split-circles-phase-edit-re-aims-its-piece-names-silently` (158, ruled not a defect).
- **Absent case.**
  - `wire/a-chord-spanning-two-merged-constituents-has-no-name` (160).
  - `emit/a-touch-and-an-edge-edge-crossing-have-no-ruled-spelling`.
  - `seamrank/a-conventional-vertex-mints-no-edge-derived-name`.
  - Also: `emit/shared-rim-several-is-a-missing-rule…`, `refs/a-pinch-carried-through-a-later-fold-step-refuses-naming`, `tang/a-lone-unread-partner-has-no-class-rule`.
- **Other.**
  - Collision: `wire/a-split-through-a-pinch-names-both-tip-copies-alike`; `wire/a-subtract-of-a-transform-copy-ties-its-seam-crossings` ("bare sides collide").
  - Kernel lineage: `wire/emit-topo-reads-a-fusion-chain-one-hop`, `honesplit/a-split-half-loses-the-lineage-of-a-twice-crossed-edge`, plus 6 more.
  - Growth: `emit/a-crossing-cites-its-edge-by-a-name-that-holds-that-edges-ends-so-names-grow-exponentially` (row 77).
  - Misclassification: `wire/a-split-names-an-operand-edge-in-its-plane-as-a-section-chord` (163).

### 2. Does it reduce to one principle?

The candidate is "a name is a function of the authored inputs' names, the recipe's structure and recorded verdicts only". **Likely no, not as worded.** It fits the frame group: a name that reads a value (normal, chord, parameter units, phase) breaks it, and FORK-CIRCLEPHASE was resolved by keeping the name on structure (`Piece(k)`). Five things do not fit:

- **Order and slot are recipe structure.** DM4 says member order is "the list's, the author's statement" (D9). The candidate therefore allows the biggest group. It needs a clause like: *invariant under each operation's declared symmetries* (any permutation of union/intersect members, swapping the operands of a symmetric pair).
- **Locality.** The index rows are functions of structure and verdicts (a rank along a carrier, a sibling count, `SectionFace{k}`) and are still defects. The missing rule is that a name reads nothing outside its entity's own derivation and boundary. N2 states this piecemeal ("nothing in a piece's name lies beyond its own boundary", said twice).
- **"The inputs' names" is too weak.** Two operands with identical names (a Transform copy, two split halves, pinch copies) collide or tie. DM4's fix keys by the *read*, which is minted identity, not a name. The candidate should say "the inputs' reads and names".
- **The absent group leaks nothing.** It needs a separate totality and injectivity requirement (`check_total` plus `NameTable::insert`). No invariance principle covers it.
- **The lineage rows** are cases where the kernel's birth records (D5) are incomplete. The name reads the right thing, but the record it reads is wrong. Birth records belong on the candidate's input list.

### 3. The ratified clauses (main's text, which includes PR 5426)

- **N1** has both forms. It states general principles: "derivation path… a construction role, not a point set", "no floats and no arena keys", and "authored things are named by minted ids… what the kernel made by the verdicts that decided it" (the nearest statement of the candidate). Then it gives per-entity rules: poles, profile pieces, loft walls, run walls, `circle_split`.
- **N2** is framed as a principle ("covariant margined predicates") but is in fact a per-entity list: face pieces get `Borders`, Split pieces get `Keeps`, section faces get `Borders` over the plane, edge pieces get `Ends`, vertices get sense and rank, corners get `Fragment`. Its one general invariant is buried mid-clause: "No rule reads a plane or a direction… Name order spells a set".
- **N3** is a general merge policy that also carries per-entity seam and chord rules.
- **N4** states the candidate almost verbatim: "the table is a function of (recipe structure, structural parameters, predicate verdict vector) only". CI pins it only as f64 vs Interval invariance, a different perturbation, and "recipe structure" admits order and slot. N4 also states the tie principle: "a candidate number is storage order, not something the author wrote".
- **N5** is about resolution, not naming.
- **DM4** gives a general naming principle ("keyed by the read, not by depth… No position is recorded") and per-node rules for rows.
- **DM5** is per-case (union, subtract, loft).
- **DM6** is a general rule about edits; it bears on naming only through stranded names.

Places where the same idea is stated twice or differently:
- Member-order invariance is stated three times: DM4 ("a name is defined over the finished body"), N2 flush ("the same in every member order") and the N2 union-parent paragraph.
- "A placement adds no name segment" appears in N1, DM4 and N6, and N4 states it differently. N4 says a world placement qualifies names by `RoleSeg::Placed`. N1's scope clause says `Place [P1,P2]` would be refused `DuplicateName`. `intent/a-placement-copy-is-its-own-table-in-the-product` (open) retires `Placed`. **Likely a live inconsistency.**
- Seam sides in name order: N2 and N3.
- A `Merged` of one element is that element: N3 and DM5.
- Section faces: the new N2 bullet and the N3 merged clause.
- The candidate principle itself is stated twice in different words: N1 (minted ids plus verdicts) and N4 (structure, parameters and verdicts).

### 4. A mechanical check

Run evaluate → perturb → evaluate. Compare each table as a set of (name, rounded geometry) pairs (the `named_geometry` shape in `emit_boolean_vertex_keys.rs`), with ties compared as sets of candidates. `union_member_orders_decide_alike.rs` already does this for perturbation 1 on a planar corpus; `test_utils::fuzz` provides seeded, replayable sampling.

| perturbation | catches (likely) | misses |
|---|---|---|
| 1. Member order of n-ary ops | about 25 of 29 order rows (fold rows, pinch, stepped plate, flush, rim and seam ranks) | completion order, build order, mint order |
| 2. Operand order of symmetric pairs (2-member union/intersect) | slot rows on union/intersect: declared merge, covered pair, B-arena chase, intersection into a void, A-first ranks (about 8) | subtract-side rows, output-scope rows |
| 3. Rigid frame change of the split plane (normal sign with halves swapped, in-plane rotation, origin shift along the plane) | revolved normal flip, mirrored pinch, `SideOf` planes, `n_a × n_b` axis (about 6) | anything not on a split |
| 4. Tilt within one topology | chord direction, parameter as metres, curved ranking direction, completion order (about 8) | ordinals that change only when topology changes |
| 5. A structurally identical copy as an operand | Transform-copy subtract, split halves in one union, pinch copies, keyed-by-operation (about 5) | — |

**Missed by all five:**
- The index and locality rows (about 14). They need a sixth perturbation: add an unrelated feature (a far hole, a second crossing) and assert that surviving names keep their geometry.
- `name-order-was-insertion-order`. It needs a seventh: replay the same recipe with a different insertion history, so the minted ids differ.
- The absent and lineage rows (about 30). Only coverage catches these: a fuzzer over legal arrangements that asserts no `Naming(*)` refusal.

**D10 hold:** a whole-input placement or Transform perturbation (rigidly moving or tilting the model) is held ground. It is named here and not built. Perturbation 5 built via `Transform` is under the same hold; the variant that does not touch it is two separately authored identical nodes.

**Probe run (one perturbation, on main at `3b687e1652`).** I reversed every spelled Union/Intersect list in `corpus::documents()` and diffed every node's per-port table as (name, geometry) pairs. That covered 23 lists across 9 documents, including the 21-member `die_composed_tour` (1,655 rows), with about 14k rows compared. The result was **0 differences and 0 new failures**. So the registry corpus finds nothing today. **Likely** it is the wrong corpus for this class: its unions are mostly disjoint or simple, while the census's order rows come from contact-heavy fixtures (pinches, flush faces, tilted wedges). The check needs those fixtures, which partly sit in `union_member_orders_decide_alike.rs` already.

