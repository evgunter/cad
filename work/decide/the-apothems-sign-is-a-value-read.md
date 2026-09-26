---
id: the-apothems-sign-is-a-value-read
kind: issue
title: the apothem's sign: six arc-family decisions on a parameter bulge are zero exactly where the apothem L(1-b^2)/(4b) is positive, which only a value read reaches
status: dispatched
opened: 2026-09-25
priority: P2
cost: H
refs: [rule-d-reaches-the-unit-bulge-only, 3186]
parent: DECIDE-4
needs_ev: true
---

**Filed from DECIDE-4's fork, on Ev's ruling on `[ev]` #3186 (2026-09-25):**
this becomes its own item, not a widening of DECIDE-4.

## What stands

DECIDE-4's Phase 1 counted these on the `0.5` parameter control of R2's
D-tab (`r2_d_tab_parameter_dyadic`):

- `line_span`, `arc_span` and `contact_at_shared_vertex`, two each;
- `line_span`'s two on the `0.4` parameter D-tab.

None of them is frozen. The early form is
`u·(|2b|·2(1 − b²) − |4(b² − 1)|·b) / (|2b|·2b)`: the magnitude
`sqrt(r² − (L/2)²)` of the centre's offset against the signed offset
`L(1 − b²)/(4b)`. Evaluated at points in every sign region, it is zero
exactly on `0 < b < 1` and on `b < −1`, which is where the apothem is
positive. So these decisions stand on the apothem's sign, not on `b`'s.

The uncut forms are in `crates/editor-core/tests/m10_bulge_renders.txt`
(on `props/sign-hull` from DECIDE-4's merge), and the attribution is the
section "What stands on DECIDE-3's and SYM-9's tree (DECIDE-4)" of
`rule-d-reaches-the-unit-bulge-only`.

## Why neither sign route reaches it

- **The turn `σ` (route B, ruled for `b`'s sign)** relates `|b|` to `b`.
  The apothem's sign is `sign(1 − b²)·σ`, which the profile program does
  not decide.
- **Rule C on as the dial stands (route A)** does take all six. But it
  also re-labels theorems as `sign_gated` on every document, loses the
  door on `pcurve_map_residual`, and raises `numeric` everywhere. It was
  rejected on #3186.

## The candidate

A NARROWED rule C: a read that folds only `abs`/`sqrt` atoms whose
argument has a certified sign over the leaf's box, ordered behind the
door and behind every value-free fold, as the decision read is. Its
zeros are `sign_gated`, which is the honest label for a fold that holds
on the box and not identically. It is unmeasured. The first step is to
build it behind a dial and count what it takes and what it re-labels,
on the six documents and the bulge controls.

A structural alternative, owed a look before the read is built: whether
the sweep's arc construction already decides which side of the chord
the centre lies on. If it does, that decided sign could be stated the
way route B states the turn.

## What DECIDE-8 found (2026-09-26)

**Nothing upstream decides the sign.** The profile decides `sign(b)`, which
is `σ` (`path_arc_bulge`, `segment_straightness`). It decides nothing that
separates a minor arc from a major one. `path_junction_turn` equals
`sign(1 − b²)` on the D-tab only because the incoming line is
perpendicular to the chord, which is a property of that joint.

**The six are not the sweep's, and they are not a fact about the arc.**
They are asked in `Profile::validate`'s pair pass, before any sweep runs,
on ADJACENT pairs:
- `judge_pair` knows the pair's shared vertex and does not pass it to
  `seg::pair_contacts`;
- so `line_arc` recomputes it as one of the two roots
  `tc ∓ sqrt(r² − h²)`, and then asks the tier whether a root is that
  vertex.

Which root it is, is the apothem's sign. The decisions stand on the
profile re-deriving a point it already holds.

**A certified-sign read measured** (draft #3282,
`SymRules::signed_root_last`, shipped off): rule C's fold asked as the
ladder's last rung.
- It takes the six, and 28 more on the bracket.
- It re-labels no theorem and no door answer on nine documents.
- It misses the `0.4` parameter D-tab's pair, which is frozen on
  `fl(0.4)`'s ring.
- It costs +39% on the bracket's leaf and +7–12% on four others.

The per-document tables are on #3282's branch, `decide/8-apothem-sign`.

## Where the six are answered

In the profile's pair pass, not the tier. `pair_contacts` takes the
pair's shared vertices:
- the shared vertex is a contact by construction and is never asked;
- the other intersection is spelled from it with no square root;
- nothing decides `sign(1 − b²)`, because nothing needs it.

The implementation row is PATHS':
`work/paths/an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root.md`.

The certified-sign read does not ship for these six. Whether it has a
case of its own waits on the bracket's 28 having the same structural
look: `the-brackets-fillet-decisions-owe-a-structural-look`.
