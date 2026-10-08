---
id: union-refuses-in-some-member-orders-and-publishes-in-others
kind: issue
title: A union refuses in some member orders and publishes in others, over PR 3112's review corpus and the #3168 review fixtures
status: parked
opened: 2026-09-24
blocked_on: [intent-stage4-is-built]
priority: P1
cost: H
refs: [a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names, a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission]
---


## What

The same union, with the same members and declarations, refuses in some
member orders and publishes in others. A document that fuses as written
can therefore refuse after the author reorders its members. The refusal
is loud, so there is no wrong answer, but the order still decides
whether there is an answer.

## Measured (EMIT, 2026-09-24, on `emit/rim-piece-ranks`)

Source: `crates/editor-core/tests/emit_union_rim_piece_ranks.rs`, whose
rebind row now checks this. It runs PR 3112's review corpus (as
`probe_corpus`) and the #3168 review fixtures, every order.

23 cases refuse in some orders and publish in others, down from 25
before the pairwise contact rule (`union-contact-is-judged-pairwise-before-the-fold`),
which took `row` and `rowids` out. The row pins them in `KNOWN_MIXED`,
so a new one, or a change in any of these, turns it red.

(2026-10-06: 22 cases now, all `DeclareResolve`; `r4tri` has left and
`RayExhausted` is gone. The current table and owners are under
"Re-measured on main (2026-10-06)" below.)

| refusal | cases | owner |
|---|---|---|
| `DeclareResolve` (Vanished): a declared face the fold has consumed | `abg`, `abgids`, `abglow`, `abgg2`, `fam0{00,01,02,12,22}`, `fam1{00,01,02,12,22}`, `fam2{00,01,02,12,22}`, `r1flush`, `r2endsg`, part of `r4trig` | the refusal GATHER's member-space look-through unit designed, which closed with GATHER's departure (its row was deleted); the cut-and-partly-merged orders of `r2endsg` and `r4trig`: `a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names` |
| `UndeclaredContact` | none: `row` and `rowids` now refuse it in all 24 orders (measured below) | built by `union-contact-is-judged-pairwise-before-the-fold` |
| `Boolean(Containment(RayExhausted))`: `c` (x 0.8..2.0), flush with both `a` and `b`, folded between them | `r4tri` (`[0,2,1]`, `[2,0,1]`), part of `r4trig` | `work/cleave/a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted.md` (P0) |

`UndeclaredContact` is a fold-order dependence of the contact check
itself: whether a contact is flush depends on whether another member
has covered it yet. `RayExhausted` is REACH's: the containment
fallback's vertex probe runs out when every vertex of the joining
member lies on the accumulation's boundary. It shares that shape with
`work/fuse/two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted.md`
but is not the same defect, because the operands here are distinct and
legal.

## The contact rule: pairwise, before the fold (Ev's direction on the `[ev]` PR; measured 2026-09-25, origin/main `2139eefa8e`)

**Where the check runs.** `wire_union` (`crates/editor-core/src/eval/wire.rs`)
hands each step's two operands, the accumulation and the joining
member, to the kernel pair verb (`run_pair`). The contact check is that
verb's census. The union adds no check of its own, so a contact is
judged against whatever the fold has accumulated. DM4 as ratified before this rule described exactly
this: "evaluates as a fold of the kernel's pair verb in member order",
and "two members that touch refuse `UndeclaredContact` exactly as a
pair boolean's operands do". Every contact the fold can see is between
two members, because the accumulation's boundary is made of member
faces. So a pairwise member-space check would see everything the fold
sees, plus the contacts another member covers. The dependence on order
comes from those covered contacts.

**Measured on `{a, b, h}` with `(a, b)` declared flush, all six orders.**
Scratch probe; the same pattern holds across the 24 orders of `row`.

| `(a.x1 wall, h.x0 wall)` | `[a,h,b]` `[h,a,b]` | `[a,b,h]` `[b,a,h]` | `[b,h,a]` `[h,b,a]` |
|---|---|---|---|
| undeclared | `UndeclaredContact` | fuse | `Vanished`: a face of `b` split by `h` (GATHER's row) |
| declared | `Emission` (seam-vertex parentage; `two-emitter-refusals-a-legal-declared-union-reaches`) | `Vanished`: `a`'s wall, consumed by containment in `b` | `Vanished`: a face of `b` split by `h` |

`a ∪ h` alone refuses `UndeclaredContact` in both orders and fuses in
both once the pair is declared.

**Ev's direction.** Every pair of members that touch must be declared,
even where the fold has already merged a third member over the
contact. Contact is judged pairwise in member space before the fold.
A declared contact that another member covers is satisfied, not
refused. DM4 in `crates/editor-core/REFERENCES.md` states this. The
wording waits on Ev's confirmation (`needs_ev`).

(2026-10-06: Ev ruled this on #3200, under "Ruled" below. The
`Emission` in the table above belongs now to
`a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission`;
the two-emitter row is closed. GATHER's row is deleted.)

Rule 2, "a contact another member covers is not a contact", is
rejected. Ev's objection: it lets a set get out of declaring a contact
just because no single pair is blamed for it.

**Measured pairwise (scratch probe, every member pair of every flat
case in `emit_union_rim_piece_ranks.rs`, 153 two-member unions, with
the case's declarations for that pair).** Five pairs refuse
`UndeclaredContact`: `row` (a, h), `rowids` (a, h), `cross` (g, cross),
`r1three` (s1, s3) and (s2, s3). All the others fuse. `cross` and
`r1three` already refuse in every order today (`cross` with a mix of
four refusal kinds, `r1three` 24 of 24 `UndeclaredContact`). So the
pre-pass adds new refusals only to `row` and `rowids`, and turns
`cross` into a uniform `UndeclaredContact`.

**What moved once it was built** (measured on `emit/pairwise-contact`;
pinned by `emit_union_rim_piece_ranks.rs`).
- `row` and `rowids` with (a, h) undeclared refuse `UndeclaredContact`
  in all 24 orders, naming a face of `a` and a face of `h`, and they
  left `KNOWN_MIXED`.
- With (a, h) declared, they fuse in the 6 orders that fused
  undeclared before the rule (`[0,1,2,3]`, `[0,1,3,2]`, `[1,0,2,3]`,
  `[1,0,3,2]`, `[1,2,0,3]`, `[2,1,0,3]`); before the rule, those 6
  refused `DeclareResolve` declared. The other 18 are unchanged:
  2 `Emission` (`[0,3,1,2]`, `[3,0,1,2]`) and 16 `DeclareResolve`.
- So none of the 10 orders that refused `Vanished` undeclared was a
  face contained whole: all 10 are splits, and they stay GATHER's.
- `{a, b, h}` refuses `UndeclaredContact` in all 6 orders undeclared.
  Declared, `[a,b,h]` and `[b,a,h]` fuse (the face `b` consumed whole
  is satisfied); `[a,h,b]` and `[h,a,b]` refuse `Emission`, and
  `[b,h,a]` and `[h,b,a]` `DeclareResolve`, as before.

**Cost.** There is no contact-only door in the kernel. The
cross-operand `UndeclaredCoincidence` is raised inside the boolean
(`crates/topo/src/boolean/rest.rs`, `vtxfac.rs`, `recl.rs`). The
detector `topo::flush::find_flush_candidates` compares carriers over
every face pair with no region test, so it reports cosurface faces
that never meet. The pre-pass is therefore the pair verb run on each
member pair: at most n(n−1)/2 two-member unions, with pairs whose
closed boxes are disjoint skipped. The kernel's BVH prunes within each
of those pair booleans, but `wire_union` has no member-level box
pruning today, so that has to be added.

## Ruled (2026-09-25)

Ev ruled the pairwise contact rule on #3200. The `UndeclaredContact`
arm of this row is built by `union-contact-is-judged-pairwise-before-the-fold`.
The row stays open for the arms other programs own:
- `DeclareResolve` is the refusal GATHER's closed look-through unit designed; its cut-and-partly-merged orders belong to `a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names`;
- `RayExhausted` belongs to REACH's row.

(2026-10-06: GATHER's row is deleted and `RayExhausted` is gone, PR 3655.
Current owners: "Re-measured on main (2026-10-06)" below.)

## Measured (EMIT, 2026-09-30, on `emit/cut-and-merged-pair`)

A pair step no longer publishes a face under a constituent its merge
retires (`a-face-cut-and-merged-in-one-step-publishes-a-piece-under-the-name-its-merge-retires`,
PR 3526). The declaration door had bound flush pairs to that bare
piece. Where one fold step cuts a face and merges part of it, the door
now refuses `DeclareResolve` (`ConsumedByFold { by: Split }`).

| case | orders refusing before | after |
|---|---|---|
| `r2endsg` | 8 | 12, all `DeclareResolve` |
| `r4trig` | 12 | 14 (`Boolean:2/DeclareResolve:12`) |

- `r2endsg` `[1,3,0,2]`, `[2,3,0,1]`, `[3,1,0,2]` and `[3,2,0,1]` bound
  the correct face. The `ALONG`~`CEND` contact at x 0..1 lies on the
  x 0..1.4 piece. Their correct tables are now lost to totality.
- `r4trig` `[1,3,0,2]` and `[3,1,0,2]` bound the wrong face silently.
  Their contact at x 0.8..1 lies in the merged face.
- `r4trig` `[2,3,0,1]` and `[3,2,0,1]` refuse `DeclareResolve` before
  reaching `RayExhausted`.

No names-only rule recovers both, so the refusal is the fail-loud
choice for now. The lost orders are owned by
`a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names`.
The pins now count each refusal kind, so a swap of kinds at the same
total turns them red.

(2026-10-06: `r4trig` is now 12, `DeclareResolve:12`, and `[2,3,0,1]`,
`[3,2,0,1]` are plain `Split` refusals. See the next section.)

## Re-measured on main (2026-10-06)

Measured on main at `35e4ac1829` by an analysis lane; nothing on the
union path moved between that and this branch's base. One refusal
class is left. Every case that refuses in some orders and fuses in
others refuses `DeclareResolve` (`Vanished`), diagnosed
`ConsumedByFold { by: Split | FragmentedMerge }`. `RayExhausted` left
with PR 3655 (`r4tri` left `KNOWN_MIXED`), and `UndeclaredContact` is
uniform in every order.

`KNOWN_MIXED` (`emit_union_rim_piece_ranks.rs`) holds 22 cases, all
`DeclareResolve`, and passes. Orders are block indices: b0 = `a` and
b1 = `b` in the corpus cases, b0 = `ALONG` in `r2endsg`.

| case | fuse | Split on b0's face | Split on b1's face | FragmentedMerge |
|---|---|---|---|---|
| `abg`, `abgids` (b0 Cap(End)) | 4 | 2: 021, 201 | – | – |
| `abglow` (b0 Cap(Start)) | 4 | 2: 021, 201 | – | – |
| `abgg2` (Cap(End)) | 8 | 8: 0213 0231 0321 2013 2031 2301 3021 3201 | 8: 1230 1302 1320 2130 2310 3102 3120 3210 | – |
| `fam000`, `fam001`, `fam002`, `fam012`, `fam022` | 4 | 2: 021, 201 | – | – |
| `fam100`, `fam101`, `fam102`, `fam112`, `fam122` | 2 | 2: 021, 201 | 2: 120, 210 | – |
| `fam200`, `fam201`, `fam202`, `fam212`, `fam222` | 4 | – | 2: 120, 210 | – |
| `r1flush` (Cap(End)) | 6 | 10: 0213 0231 0312 0321 2013 2031 2301 3012 3021 3201 | 8: 1230 1302 1320 2130 2310 3102 3120 3210 | – |
| `r2endsg` (b0 Cap(End)) | 12 | 8: 0312 0321 1302 2301 3012 3021 3102 3201 | – | 4 on b0: 0132 0231 1032 2031 |
| `r4trig` (Cap(End)) | 12 | 8: the same orders as `r2endsg` | – | 2 on b1: 0132 1032; 2 on b0: 0231 2031 |

`row`, `rowids`, `cross` and `r1three` refuse `UndeclaredCoincidence`
in all 24 orders, so they are not mixed.

**Order-mixed unions outside this row's old scope**, same class:
- `emit_union_flush_names.rs` `KNOWN_REFUSING`, `near` (the ZIP
  document): `DeclareResolve` (Split) in `[0,2,1]`, `[2,0,1]`.
- `emit_union_rim_piece_ranks::an_undeclared_covered_contact_refuses_in_every_order_and_declared_fuses_where_b_covers_it`,
  `row`/`rowids` with `(a, h)` declared: 6 fuse, 2 `Emission`, 16
  `DeclareResolve`.
- `emit_union_rim_piece_ranks::a_contact_b_covers_refuses_undeclared_and_is_satisfied_declared_where_b_consumed_the_face`,
  `{a, b, h}` declared: 2 fuse, 2 `Emission`, 2 `DeclareResolve`.
- `wire_legal_union_refusals.rs`, the area-overlap fixture `{a, big, p, s}`:
  14 `Fused`, 8 `SeamVertex`, 2 `Split`.
- `wire_legal_union_refusals.rs`, the split fixture `{a, big, c, s}`:
  2 `Fused`, 18 `SeamVertex`, 2 `MergedChord`, 2 `Split`.

**Root cause.** `look_through_fold` (`crates/editor-core/src/eval/wire.rs`,
~:3407) refuses `ConsumedByFold { by }` for a name that has fragment
descendants (its `split` binding; `fold_descent`). Declarations are
routed to the fold step that joins their two sites
(`route_declarations`), and the fold calls
`drop_consumed(look_through_fold(..))` at each step, so whether a
declared face is already split by that step depends on member order.
This is ratified DM4 (`crates/editor-core/REFERENCES.md`, "Consumed
faces" and "Merges and order"; ruled on PR 2677).

**Owners.**
- Pure split by an earlier step (`abg*`, `fam*`, `r1flush`, `abgg2`,
  `near`, and `r2endsg`/`r4trig` orders 0312 0321 3012 3021): this
  row. GATHER's `member-space-look-through-stops-at-splits-containment-and-fragmented-merges`
  built the refusal and was deleted with `work/gather/` (68e8072a6).
- Cut-and-partly-merged orders (1302, 2301, 3102, 3201 of `r2endsg`
  and `r4trig`):
  `a-declaration-on-a-face-one-fold-step-cut-and-partly-merged-cannot-be-routed-by-names`
  (parked on the same hold).
- `FragmentedMerge`: this row.

**Parked on the D10 hold.** The one class left is a refusal of a
declared pair, which is ground the 2026-10-03 hold covers
(`work/emit/log.md`). The row waits on
`d10-one-way-to-say-intent-is-unbuilt`. INTENT's stage 4
(`work/intent/plan.md`, "The coincidence door": "booleans glue on
Zero; declared pairs … retire") removes the class by construction:
with no declarations to route, `route_declarations`,
`look_through_fold`, `drop_consumed` and `Diagnosis::ConsumedByFold`
(`resolve/mod.rs`) have no job. The pairwise pre-pass
(`judge_pairwise_contact`, `fold_step_refusal`) already certifies every
contact before the fold. When the row fires, re-measure: the class
should be gone, and what it was hiding (below) is what remains.

**Evidence for stage 4: the fan-out experiment** (scratch, deleted;
not built, because it changes ratified DM4 on held ground). Where
`look_through_fold` refuses, feed the pair to every accumulation face
row descending from the name (`names::face_descends_from`), crossed
with the other side. This is sound because fragments inherit surface
keys, and a declared pair whose surfaces never meet at an edge is a
no-op (`crates/topo/src/merge_faces.rs`, ~:1519-1527). Measured:
- `KNOWN_MIXED` empty: all 22 cases fuse in every order;
- no name rebinds; `KNOWN_ABSENT` and the digests unchanged;
- `emit_shared_rim_several` fused cells 148 → 210;
- `wire_legal_union_refusals`' 4 `Split` orders become `Fused`;
- `{a, b, h}` declared: its 2 `DeclareResolve` orders fuse;
- `r4trig` 1302 and 3102 fuse (the orders where a names-only route
  bound the wrong face before PR 3526).

The only failing tests pin the old refusal: `docm8_flat_merged` 5,
`emit_union_rim_piece_ranks` 3, `emit_union_flush_names` 3,
`emit_seam_junction` 1, `emit_shared_rim_several` 1,
`wire_legal_union_refusals` 2.

**What the refusal hides**, surfaced by the fan-out and owned
elsewhere. Whoever builds stage 4 meets these in more orders than
today:
- `Emission("seam vertex parentage underdetermined from incident edges")`
  (`names/emit_topo.rs`, the seam-vertex pass's catch-all): `row`/`rowids`
  with `(a, h)` declared go from 2 orders to 8 (0231 0312 0321 2031
  2301 3012 3021 3201); `emit_seam_junction` from `{ahbg, habg}` to 8
  orders; `{a, b, h}` stays at 2. Owner:
  `work/wire/a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission.md` (P1).
- `NamingError::SeamVertexParentage` (`names/emit_topo.rs`): `near`
  `[0,2,1]`, `[2,0,1]`, and
  `emit_union_flush_names::a_seam_a_leftover_vertex_splits_is_published_twice_under_two_names`
  `[0,2,1]`. Owners:
  `work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md` (P0)
  and `work/fuse/a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made.md` (P1).

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the class left is DeclareResolve/ConsumedByFold on routed declarations (wire.rs look_through_fold), removed when stage 4 retires declared pairs. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)
