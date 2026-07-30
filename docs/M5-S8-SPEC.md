# M5 S8 — fillet branch selection: nearest-the-authored-corner (binding spec)

Executes Evan's ruling (in-chat, 2026-07-30, on the
ambiguous-fillet plot): since the far tangent circle is always
deliberately authorable as the NEAR fillet of the other corner
(the second carrier intersection), the two-survivor case resolves
by **picking the candidate nearest the authored corner**; only a
genuine near-tie escalates. Branch `ev/m5-s8-fillet-branch` from
main. Profile-only; small.

## 1. The rule

- Among candidates surviving the existing corner-side + extent
  classification (unchanged), compare by **total setback** (the
  sum of the two legs' arc-length setbacks — the fillet_leg_fit
  datum; near pocket ⇒ both small, far pocket ⇒ both large; any
  monotone combination agrees, sum is the symmetric choice).
- The comparison is a named Q1 predicate (suggested
  `fillet_branch_proximity`, lever arm in meters), K-funnel
  registered, run in fixed order after the four per-candidate
  classifications. Definitely-nearer wins; **in-band tie ⇒ F6
  escalated typed error** (never guess) with honest recourse
  (adjust r or the leg extents so one pocket wins; margins as
  payload). Trio rows (definite/exact/in-band) required — note
  the exactly-tied case IS constructible (symmetric lens with the
  corner equidistant is measure-zero but representable; if no
  exact-tie fixture exists at f64, the interval-lane hairline row
  is the exact arm, per the S2 precedent).
- `AmbiguousFilletBranch` RETIRES (the two-survivor case now
  resolves; the tie is the standard escalated form naming the new
  predicate). Remove the variant; update its tests; report any
  ripple beyond profile.

## 2. Reachability (the ruling's premise, pinned)

A committed test authors the far pocket's fillet deliberately:
the S2-review vesica configuration, corner authored at the
SECOND carrier intersection with legs swept from there — the
construction yields exactly the circle that was the far
candidate of the original corner, with clean tangency. One more
row: the original vesica fixture now PICKS the near candidate
(exact tangency asserted) instead of refusing.

## 3. PATHS-DESIGN amendment (rides this unit; ruling = sign-off)

The §2 Fillet DOF note ("exactly determined") is amended, dated:
`.fillet(r)` selects the tangent circle nearest the authored
corner among valid candidates; near-ties are refused typed
(escalation); the far circle is authorable via its own corner.
This RESOLVES recorded divergence 2 in
`AmbiguousFilletBranch`'s… now-retired rustdoc — move the
divergence note's resolution into the PATHS amendment. (The cusp
variant split, divergence 3, stays open — not ruled.)

## 4. Acceptance

- Vesica near-pick row (exact tangency, both junctions declared
  and verified); far-author reachability row; tie-escalation trio
  incl. interval-lane hairline; existing refusal classes
  (NoCornerForFillet both sub-kinds, FilletDoesNotFit,
  already-tangent/cusp, degenerate) bit-unchanged; line×line
  fillets bit-identical (delegation untouched).
- Messages compose the shared carrier per the two-tolerance
  shape.
- Local: -p profile both lanes, fmt, clippy -p profile. CI gates
  the matrix.

## 5. Process

Standard rules (foreground, one row per call, push per unit,
OUTPUT DISCIPLINE ≤30 lines, numbered deviations). Review: one
adversarial pass — attack the tie predicate's lever arm and the
monotone-combination claim (construct a case where sum and max
would DISAGREE on the winner if one exists — the spec claims they
cannot among surviving candidates; verify or refute), plus the
retirement ripple.
