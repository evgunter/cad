---
id: a-declared-tangent-seam-flips-invalid-when-eps-tightens
kind: issue
title: A declared conventional edge valid at one ε refuses TransverseNotIntrinsic or TangentNotIntrinsic at a finer one, against D2's "never flip"
status: closed
opened: 2026-10-14
priority: P2
cost: E
closed: 2026-10-15
---


## Finding

Filed by TANG's `OnPair` build (PR 4998), from its second and third full
reviews; for Ev.

D2's prefer-intrinsic paragraph, as main's #4856 (`ac971b225`) left it
(`docs/DESIGN.md:805-809`):

> The check refuses a declared conventional description on a
> definitely-transverse edge. Escalated dihedrals, seam edges and
> NURBS/`Approx`-adjacent edges are exempt, so ε-tightening can escalate
> but never flip a valid body to invalid.

The first sentence is what the code does. The second does not hold for a
**declared** conventional edge: tier 3's check 4 refuses one at a finer ε
than it was built at, on both arms (`crates/topo/src/validate.rs:7095`,
the transverse arm; `:7260`, the tangency arm, both `is_declared()`).

**Witness (still flips on PR 4998's head).** A 2 × 2 polygon whose
corner at `(1, 0)` turns by `θ = k·ε`, extruded `c·√(2ε)` at ε = 1e-9:
its strut there is a near-straight plane × plane corner, which the
extrude declares. Read in-process at a finer band
(`topo::test_support::tier3_at`):

| k | c | strut | ε | ε/10³ | ε/10⁶ |
|---|---|---|---|---|---|
| 20 | 0.25, 0.5 | `OnPair`, declared | `[]` | `[]` | `[TransverseNotIntrinsic]` |
| 100 | 0.25, 0.5 | `OnPair`, declared | `[]` | `[SliverDihedral]` | `[TransverseNotIntrinsic]` |

Main flips identically, its strut a declared `Chart` (the third review's
probes on `fb8fd18b`). The tangency arm has the same shape: a slot
`(0,0)→(2,0)`, a `+1` bulge to `(2,2)`, `→(0,2)`, extruded `c·√(2ε)`,
has declared tangent wall seams that read `[]` at 1e-9 and
`[TangentNotIntrinsic × 2]` at 1e-12 on main.

**Why it flips.** The edge was built where the must-carry rule read it
under-determined: first order smooth, or second order zero-side. A
zero-side reading turns definite as ε tightens, so a finer band reads the
corner, or the tangency, and the declared description refuses.

**What PR 4998 does, and leaves.** A body is always validated at the ε it
was built at in production: `Tol` is committed once per process and the
kernel is serde-free (the comment at `finer`, `validate.rs:7061-7086`).
So the flip shows only on a re-read at a finer band. There, a *derived*
`OnPair` escalates on both arms (`PairCornerDetermined`,
`PairTangencyDetermined`). A *declared* conventional edge, `Chart` or
`OnPair`, refuses as on main, per the orchestrator's ruling for this PR.
A derived `OnPair` that reads the rule determinate at its own build ε is
a wrong description, and refuses (`TransverseNotIntrinsic`,
`TangentNotIntrinsic`).

## What it needs

An `[ev]` ruling on D2's "never flip" sentence for declared edges. Either:

- a declared conventional edge escalates at a finer ε as a derived pair
  does (a code change: drop the `!is_declared()` from `escalates`); or
- the sentence is scoped to what holds: derived descriptions escalate,
  while a declaration is the modeler's claim and is refused where a
  finer ε contradicts it (a wording change).

## Ev's decision (PR 5150, 2026-10-15)

Approved by Ev after three reconciliation rounds (row 148 of
`docs/DESIGN-FORK-LOG.md`), with D2's paragraph reworded on the PR to
state the rule's scope as one principle. Neither option above: the
framing dissolved. What to build:

- Tier 3 validates a body only at its build ε. Delete the finer-ε
  branch in check 4 (`finer`, `escalates`), the two escalation variants
  `PairCornerDetermined` and `PairTangencyDetermined`, and the band
  parameter `topo::test_support::tier3_at` passes; tier 3's band comes
  from `tol`.
- Check 4 keys its demand on the description's form, not its
  authority: a conventional description (`OnPair`, or a `Chart` that is
  not a wrap) on an edge the must-carry rule reads determinate refuses
  (`TransverseNotIntrinsic`, `TangentNotIntrinsic`), declared or
  derived.
- The offset arm's `held_neighbour_restated` stores what the rule reads,
  not the neighbour's declaration (the designer measured 8 rows that
  change, all hand-stated fixtures or duplicate reports).
- No standing door audit; the landing PR sweeps once for a door that
  mints a conventional form the rule would read determinate.

## Closed

Built as Ev's decision says (branch `tang/d2-form-keyed-check4`).

- Tier 3 reads a body at its build ε only. Check 4's finer-ε branch,
  `PairCornerDetermined`, `PairTangencyDetermined` and
  `topo::test_support::tier3_at` are gone, and the battery takes no
  band: `tier3_local_checks(_marked)` derives `Band::linear(tol)`.
  `geom_brep::classify_dihedral_reported` and the sagitta on
  `SecondOrderWalk::Determinate` existed only for those findings and
  went with them.
- Check 4's demand keys on the form: `OnPair`, or a `Chart` that is
  not a wrap, refuses `TransverseNotIntrinsic` / `TangentNotIntrinsic`
  on an edge the rule reads determinate, declared or derived. A
  `Scaffold` is not a conventional form, so a scaffold at rest draws
  only `ScaffoldAtRest`.
- The offset doors store the rule's reading for a declared image in a
  held neighbour's chart too (`offset_restate::restate`, and
  `replace_face`'s chart arm, which now checks the neighbour section
  for every such edge).

The witness above no longer has a finer-ε reading to flip at. The
rows that moved are listed in the PR body.
