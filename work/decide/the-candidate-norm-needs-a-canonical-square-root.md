---
id: the-candidate-norm-needs-a-canonical-square-root
kind: issue
title: the sign-hull candidate's norm is a sqrt over a quotient the tier cannot meet with the normal's own sqrt atom: the fourth piece SYM-10's Phase 1 found, hand-planted green, and what it costs the plate
status: open
opened: 2026-09-21
priority: P1
cost: H
---


## What was measured (SYM-10 Phase 1, 2026-09-21)

`docs/SYM-10-SPEC.md` named three pieces the sign-hull frame needs of
the tier — manifest order, a manifest bound for the conditioning
floor, and rule C's read of a `Select` — and said the unit stops after
Phase 1 if a fourth is needed. A fourth is needed, and it is the one
the SYM log's answer to PROPS set aside as "an atom-identity question
the render answers": **`sqrt(1/X)` against `1/sqrt(X)`**, and its
sibling **`sqrt(P²/X)` against `|P|/sqrt(X)`**.

The construction is `b1 = select(d, cz, cy)` with `cy = vy /
max(‖vy‖, s/4)`, `vy = (n.z, 0, −n.x)`. On the tilted document
`n = (0, −(¼+t), 1)/S`, `S = sqrt(17/16 + t/2 + t²)` — the normal's
own `sqrt` atom, which rules A/E/A0 cancel out of `‖n‖`. But the
CANDIDATE is not unit: `‖vy‖ = sqrt(n.x² + n.z²) = sqrt(1/S²)`, which
rule A turns into `sqrt(1/X)` for `X = S`'s argument, and rule E's
scale canonicalisation then spells as
`sqrt((16/17) / (1 + 8t/17 + 16t²/17))`. That is a `sqrt` atom over a
QUOTIENT, keyed on a form no other atom is keyed on, and `S · sqrt(1/X)`
— the number one — stands in every denominator of the refused residual
(`sym10_phase1_the_tilted_rows_residual_rendered` with the three
pieces planted: the early form's census is `sqrt 4547`, every other
atom gone, `S` and `sqrt((16/17)/(…))` side by side in each term).
`‖vz‖ = sqrt((¼+t)²/X)` is the same shape with a perfect square above.

The spec's fold 2 (the manifest bound) rests on "once `‖v‖` folds to
`1`". It does not: `‖e_k × n‖ = sqrt(1 − n_k²)`, unit only for
`n ⟂ e_k`. So the floor's `max(‖v‖, k·s)` is not manifest either — it
needs a READ, the same read as the decision (`max(A, B)` IS
`select(B − A, A, B)`).

## Hand-planted, green (measurement only; the plants are in the branch's history at `2a479c267` and `2d4c986bd`, not in the tree)

| plants on the tilted row (`ε/8` and `1e-3`, `Guided`) | result |
| --- | --- |
| any subset of {1 manifest order, 2 manifest bound, 3 Select read, 4 min/max order read, z equal-arm select, u the UNSOUND floor fold} | red, the same enclosure `[−1.42e-8, 1.42e-8]`; with 3 on, the refusal moves one face earlier and 45% wider (`[−2.06e-8, 2.06e-8]`, side plane) — PROPS's option-E measurement reproduced |
| 3 + 4 + q (canonical root, every `sqrt` re-keyed) | **green**, both halves, both lifts, 0.2 s; `sign_gated` 0 → 416 |
| 3 + 4 + r (narrow root: split only against an EXISTING atom's argument up to a scalar, content split only over a perfect square) | **green**, identical counts; `r` fires TWICE |
| 1 + 3 + 4 + r | green; fold 1 turns 16 gated into theorems (`symbolic_zero` 556 → 572) |

`q`: `sqrt(N/D) → sqrt(N)/sqrt(D)` where `D ≥ 0` is known (a
positive constant, manifestly non-negative, the argument of an existing
`sqrt` atom, or certified positive — a read), each half `p = c·p'`
split into its rational content `c = s²·f` (the square part `s` taken
out exactly, `sqrt(f)` a constant atom) and the primitive integer
polynomial `p'`, and `sqrt(R²)` read as `|R|`. `r`: the same split
only where `D = λ·A` for an existing atom `sqrt(A)` (`constant_ratio`,
rule E's), so the only new atoms are constant `sqrt(f)`s and `sqrt(N)`
over the numerator alone. Both are identities of reals wherever the
value exists (`sqrt(ab) = sqrt(a)·sqrt(b)` for `a, b ≥ 0`; `D > 0`
where it is a denominator with a value).

## What it costs, measured on the plate and the slab

Neither shape is free, and `r` is not narrower where it matters:

- `m10_sym_profile_interval`'s pinned ledger: the plate's largest early
  form **288 → 28**, `Early/Decision` frozen 8 → 0, `Early/Assertion`
  104 → 0, `Door/Decision` 104 → 0 — identical under `q` and `r`. The
  slab's early digests move too.
- `m10_10_pins_interval`'s nominal split: `carrier_matches_mapped_source`
  **`[180, 0, 8, 64] → [180, 0, 0, 72]`** — the door's eight
  REGISTERED decisions become numeric, under `q` and under `r` alone.
  The registrant's `‖q − c‖ = r` forms no longer meet the walk's: the
  same class as `coefficient-ring-width-is-not-monotone-in-reach`
  (opening an atom the walk was cancelling OVER costs the walk a
  theorem), here paid by the registry.
- Under `3 + 4 + r` six of `m10_10_pins_interval`'s seven rows red,
  among them the plate's bound at ceiling + δ, `assert_bound →
  carrier_endpoint_start`, and the eps-relative ceilings.

So the fourth piece is a CANONICAL FORM for `sqrt` atoms, and a
canonical form is a decision about every atom the tier keys — the
walk's, rule D's hand-built `sqrt(1 + X²)` (`trig::sqrt_atom`), and
what a registrant's forms meet. Applied to the walk alone it takes the
tilted row and pays with the plate's door. What it would take: the
canonicalisation applied uniformly (walk, `trig`, registry), the
`D ≥ 0` side condition argued once beside rule E's four denominator
sources, and the six documents re-measured. That is a design
conversation, not a lane's call (#2728: the fork comes back to Ev).

## A second hazard the plants showed: a read re-labels a theorem

Plant 4 (the certified order read at `min`/`max`, rule C's shape)
fires on the slab: `m10_8_pins_interval::m10_8_the_shipped_set_is_inert_on_straight_geometry`
reds with `numeric 263 → 255, sign_gated 8` — eight numeric decisions
became gated reads (a gain), and every early digest on the slab and
the plate moves. Rule C is ordered after rule F at the `abs` node so a
theorem is never counted gated; a read at `Select`/`min`/`max` that runs
BEFORE the atom is minted has the same relabelling power over anything
the atom would have cancelled against. Phase 2's read must sit behind
every value-free fold, or in a walk of its own asked only after the
early walk declines (the door's shape).

## Home

`crates/geom-core/src/sym.rs` (`combine`'s `Sqrt` arm, the `Select`
and `Min | Max` arms), `sym/signed.rs`, `sym/quotient.rs`'s scale step
(which is what spells `1/X` as `(16/17)/(…)`), `sym/trig.rs`'s
`sqrt_atom`. The rows: `editor-core/tests/m10_derived_frame_tilted_interval`'s
`sym10_phase1_*` (evidence, `#[ignore]`d). Filed by SYM-10's lane at
its Phase 1 stop; the unit's PR carries the full tables.

## Decision for Ev (2026-09-21)

Three shapes, put on PR #2970's comment thread (the Phase 1 record
with the tables this row summarises):

1. **The full canonical root as Phase 2** — every `sqrt` atom the tier
   keys re-keyed uniformly (the walk, `trig::sqrt_atom`, what a
   registrant's forms meet), the `D ≥ 0` side condition argued once,
   the six documents re-measured. Takes the tilted row; as planted it
   costs the plate's door eight registered decisions and moves its
   ledger and ceilings.
2. **The narrow form `r` as Phase 2, on the condition that the plate's
   split and ceiling do not move down** — the orchestrator's
   recommendation. Re-keys no existing atom; fires twice on the tilted
   row; the plant as cut still loses the plate's eight registered
   decisions, so the condition is a measurement to pass, not a fact
   in hand.
3. **Stop after Phase 1** — the row stays red, #2468 holds, and the
   fork stays with Ev.

## Ruled (Ev, 2026-09-21, on PR #2970): shape 1 — the full canonical root, at the mint site

Ev's ruling, with the orchestrator's recommendation that preceded it
(the narrow form withdrawn as re-baseline avoidance; "having to
re-baseline is never a reason to skip a code change for the better"):
a `Sqrt` atom's key becomes a function of its argument's value class —
`sqrt(N/D)` minted as `sqrt(N)/sqrt(D)` where `D ≥ 0` is known, the
rational content split out (`s` exactly, `sqrt(f)` a constant atom), a
primitive integer polynomial under the root, `sqrt(R²)` read as `|R|`
— in `mint_atom`'s `Sqrt` arm, the one door the walk, `trig::sqrt_atom`
and the registry mint through, so uniformity is structural. Dialed like
every rule, shipped on. With it the two reads (the `Select` read and
the same read at `min`/`max`, behind every value-free fold); fold 1
only as the theorem upgrade it measured; fold 2 dropped (its premise
is false — the candidates are not unit). Acceptance: no decision LOST
on the six measured documents once everything mints through the one
door — the plate's eight registered decisions in particular meet again
by construction, and if they do not that is a defect the unit finds
and fixes, or stops on and says why; every ledger digest, freeze
count, ceiling and split that moves is re-baselined with what moved
said. Taken by unit `DECIDE-3` (`docs/DECIDE-3-SPEC.md`), cut from
`sym/10-decision-door`'s head; this row stays open until DECIDE-3
lands or stops.

## DECIDE-3 Phase 1 (2026-09-21, branch `decide/3-canonical-root` at `e88987915`)

### The mint sites, enumerated

Every place a `Sqrt` atom is minted, and whether it reaches the one
door rule G lives at. Line numbers ride along and are allowed to go
stale; the names are the citation.

| site | where | the argument it passes | reaches the door |
| --- | --- | --- | --- |
| `combine`'s `atom1`, from the `Sqrt \| Abs` rule arm | `sym.rs`, after A0, rule F and rule C (`:2207`) | the node's kid form, EARLY walk | **yes** — rule G's call is this site |
| `combine`'s `atom1`, from the op-list arm | `sym.rs` (`:2230`) | the node's kid form, PLAIN walk (and the early walk with every dial off) | **no, by design**: rule G is early-only like every other rule, so the plain form stays M10-7's and no theorem is re-labelled |
| `combine`'s `atom1`, from rule D's `Sin`/`Cos` fallback | `sym.rs` (`:2219`) | never a `Sqrt` — only `Sin`/`Cos` reach it | n/a |
| `trig::sqrt_atom` (rule D's hand-built roots) | `sym/trig.rs` (`:305`) | `1 + X²` for `S`, and `(D + C)/(2D)` for each half-angle `c₂`, after rule E's cancel | **yes** — routed through `root::mint` |
| the registered-identity door | `sym.rs`'s `form_in`, the `registry` arm | mints nothing of its own: an aliased node takes the registered node's form, built by the same EARLY walk | **yes, by construction** |
| `algebra::reduce`'s rule A | `sym/algebra.rs` (`:82`) | reads `AtomInfo`, mints nothing | n/a |
| `manifest::magnitude` | `sym/manifest.rs` (`:298`) | mints an `Abs` atom, never a `Sqrt` | n/a |
| `signed::fold` (rule C) | `sym/signed.rs` (`:287`) | folds a root to a polynomial, mints nothing | n/a |

So the door is reached from every site that mints a `Sqrt` except the
plain walk's, which is excluded on purpose.

### The side condition, argued once

`sqrt(N/D) = sqrt(N)/sqrt(D)` needs `D > 0` where it is used, not
merely `N/D ≥ 0`: at `N ≤ 0, D < 0` the left side is real and neither
root on the right is. `D ≠ 0` is rule E's four-source denominator
argument. Non-negativity comes from, in this order: (1) `D` a
non-negative rational constant; (2) `manifest::nonneg`, rule F's own
predicate; (3) the session already holding `sqrt` of `D`'s PRIMITIVE
part — a `Sqrt` atom exists only because a node of this DAG computes
that root, and a real root has a value only where its argument is
non-negative, and `D` differs from its primitive by the positive
content; (4) a certified bracket over the leaf's box, which READS a
value, rides rule C's dial and gates the form. `N ≥ 0` then follows
from `N/D ≥ 0` with `D > 0` and is never tested on its own. The
argument lives in `crates/geom-core/src/sym/root.rs`'s header, beside
what it governs.

Source 3 depends on minting ORDER, and that is said rather than
hidden: whether `sqrt(D')` is already in the session is a fact about
the walk's traversal. What does not depend on order is the KEY — an
atom this rule mints is keyed on the primitive polynomial and nothing
else — so what order can change is whether a root splits at all, never
which atom it splits into.

### The baseline, re-taken on this branch's head

`e88987915` (the spec and the unit on `sym/10-decision-door`'s closing
head, with `origin/main` of 2026-09-21 merged; `props/sign-hull` was
already in). `--features interval`, dev profile, this box.

| row | at head |
| --- | --- |
| `m10_8_pins_interval` (5) | green. Plate ceiling, plain tier and A0 alone, `eps=1e-9`: certifies `7.738768852289407e-7`, refuses `7.84388558145157e-7`. Bracket, plain tier: `3.701343361154913e-8` / `3.75161920154464e-8`; shipped tier: `3.8716012820540445e-7` / `3.924189758484525e-7`. Bracket at `1e2·ε`: shipped `sym0 1083 / gated 0 / registered 144 / numeric 794 / frozen 1637`, certified 1, share 0.536; plain `3360 / 0 / 0 / 10787 / 1637`, share 0.238. Slab inert: `shipped` serializes what `none` does, byte for byte. |
| `m10_9_pins_interval` (7 + 1 ignored) | green. At `eps=1e-9`: plate `803/0/140/470`, frozen 1044; annulus `328/0/140/209`, frozen 1056; link `515/0/90/497`, frozen 1060; bracket `1083/0/144/794`, frozen 1637; pad `854/0/128/971`, frozen 2750. Rim registrant on the plate: door shut `518/0/0/303` refusing `carrier_endpoint_start`, door open `518/0/6/303` refusing `carrier_matches_mapped_source`. |
| `m10_10_pins_interval` (7) | green — including the nominal split `carrier_matches_mapped_source [180, 0, 8, 64]` on the plate (the door's eight registered decisions), the bound at ceiling + δ per document, and the eps-relative ceilings. |
| `m10_bulge_interval` (3) | green — the boss's and both `d_tabs` splits at the nominal. |
| `m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does` | **RED**, `half = ε/8`, `Guided`: cap plane `newell_plane_residual`, enclosure `[-1.4199563802552717e-8, 1.4199563941330595e-8]` against `escalate = 1e-8`. |
| `m10_the_derived_frames_refusal_is_not_a_freeze` | **RED**, `none` rung at `5e-2`, `Pinned`: the boss's SIDE plane `newell_plane_residual`, `margin is invalid (NaN or a poisoned enclosure)`, where the row asserts `carrier_endpoint_start`. |
| `the_forms_the_walks_build_are_pinned_per_eps_row` | **RED** on the plate's ledger: every count identical, every digest moved (SYM-10's Phase 1 measured it; the re-baseline is DECIDE-3's). |

The six measured documents are the plate, the annulus, the link, the
filleted bracket, the rounded pad and the slab; the pad's shape report
OOMs on this box and is measured on the ceiling instrument only.
