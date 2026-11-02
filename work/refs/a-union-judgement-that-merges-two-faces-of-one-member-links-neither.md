---
id: a-union-judgement-that-merges-two-faces-of-one-member-links-neither
kind: issue
title: A union whose judgement merges two faces of one member refuses as an emission bug
status: closed
opened: 2026-10-17
priority: P1
cost: M
pr: 6342
closed: 2026-11-02
branch: reads/union-same-member-links
---

## What

A union's end pass (`emit_union`'s `Parents::of`) links member faces
by the pairs its pairwise judgements recorded (`Links::judged`). For
each merged face of a judgement, `judged` pushes a pair for every
member-`a` face and member-`b` face the merge holds, and nothing for
two faces of the SAME member it holds. When a judgement's merge glues
two faces of one member into one face, those two member faces stay
two parents, the finished face descends from both, and `Parents::of`
refuses `NamingError::Emission { what: "a union's face descends from
member faces no judged pair links" }`: a kernel-bug framing on a legal
recipe.

`crates/editor-core/src/names/README.md`, "A union's face is named for
its PARENT", says two member faces are linked "when the pairwise
judgement of their two members (DM4) merged or covered them"; it has
no case for two faces of one member, so what such a pair is (one
parent, as the pair boolean names it, or something else) is the
question this row owns.

## Witness

`wire_legal_union_refusals::a_bar_through_a_sunk_boss_refuses_at_the_unions_parents`
pins it (wire, 2026-10-17, branch `wire/merged-chord-rule`):
- plate `[0,3]² × [0,1]`; boss of radius 0.3 about (1.5, 1.5), z 0.5
  to 1.5, its wall the two semicircular pieces of
  `circle_split(.., 2, ..)`; the two joined by a `Union`;
- a bar x 1.4 to 1.6, y 0.5 to 2.5, z 0.8 to 2.0, joined to that
  union by a second two-member `Union`.

The bar cuts each boss piece in two, and the second union's merge
glues the two cosurface pieces on each side of the bar: each merged
wall face holds both pieces, both faces of the first member. Before
the chord rule
(`work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md`)
the fold step's naming refused `MergedChordConstituents { several: 2 }`
first, which masked this arm; with it, the fold step names its seam
chords and the end pass refuses here. The subtract of the same slab
names (`emit_union_borders::a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls`):
a subtract is named by the pair emitter alone (`eval/wire.rs`'s
`wire_subtract`), and only a union or intersect reaches `name_union`.

## Decided by the designers; the spec for the unit (2026-11-02, REFS orchestrator)

Three designers (fork `union-same-member-merge`) agreed after one reconciliation round.

**What is wrong, and why it is wider than the title.**

- `Links::judged` records a pair only across the two members. A merge that glues two faces of ONE member therefore links nothing (the witness, the nested `Union[Union[plate, boss], bar]`, and a two-member intersect).
- The end pass also assumes that whatever the fold glues, some single pair also glued (`Parents::of`'s comment). That is false. A union covers material, so the fold's exposed run can hold fewer faces than any pair's, and a closed curved run that no single pair opens is glued by the fold alone. Each designer built a witness independently:
  - flat `Union[plate, boss, bar]`, refusing in every member order;
  - two half-cylinder members plus a full-height bar;
  - `[Union[plate, boss], lo, hi]`, where only the two blocks together open the ring;
  - two notches severing a wall piece only in the fold.
- The edge and vertex passes rest on the same assumption. With faces linked by lineage, `Flush::meeting` reaches its `unreachable!` in 2 of 6 orders.

**The rule (names README N2).** Member faces are linked, transitively, when any of these holds:

1. a pairwise judgement merged them, whether they are faces of two members or of one;
2. a judgement held one face's region through the other;
3. the finished body holds area of both in one face, **read exactly**: by area held, never by the fold's lineage, which only grows and would make names depend on member order.

A parent is `Merged` of its class. It is held as one face, or as `Borders` pieces separated by other members' walls, as the subtract already names it.

**What this unit lands:**

- (1) in `Links::judged`;
- a typed missing-rule `NamingError` variant for the faces that only (3) would link. It names the member faces, replaces the `Emission` refusal in `Parents::of`, and fires before the edge and vertex passes, so `Flush::meeting`'s `unreachable!` cannot be reached;
- the false comments in `Parents::of` and on `Links` come out;
- N2 states rules 1–3. It also says that until descent is read exactly, a face only rule 3 links refuses as that missing rule.

**Done when:**

- the witness `a_bar_through_a_sunk_boss_refuses_at_the_unions_parents` names, and its names equal the subtract's sunk-boss slot modulo node ids;
- a two-member intersect names;
- the fold-only witnesses each refuse with the typed variant, in every member order tried;
- plain `plate ∪ boss` still names piece 0 and piece 1.

Review tier: **single, FULL** (a naming rule change with correctness at risk; the design is already weighed). The README change goes through an `[ev]` PR: it changes the N2 link rule Ev approved on PR 3734.

## Closed (2026-11-02, PR 6342)

A judgement's merge links every member face it holds, two faces of one member included. The witness names exactly as the subtract does. A fold-only glue refuses as the typed `UnsupportedUnjudgedGlue`. The review's panic, a seam merged in a later fold step, is fixed. Ev approved landing this as the interim. Ev's questions on the class reading led to a second fork, which recommends per-face exact parents. That is the parked unit `a-merged-face-is-named-for-exactly-the-member-faces-it-holds`, whose rule goes to Ev on its own `[ev]` PR. Follow-ups:

- `a-union-whose-fold-glues-a-run-no-judgement-opened` (P1, exact descent);
- `pair-boolean-keys-a-merged-parent-by-the-exact-face-set` (P3);
- TOPO's `the-closed-run-skip-is-described-as-unfinished-and-as-the-design`;
- a witness on BORDERS' order-dependent refusal row.
