# PROPS sign-hull — (c′): the zero is canonicalised at f64, the `Interval` point-zero arm narrows

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Lanes,
the linalg lane; the item is
`work/props/interval-orthonormal-basis-sign-hull.md` — read it in full:
§What was measured, §Sized, §Question and its **RULED (c′)** section,
and the §Rider; difficulty logged at spec: **M**, task-class
**NUMERIC** — f64 bits move on the twelve boolean-reversed corpus walls
and eight STEP golden records re-derive). Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/sign-hull`, cut from `main`. Ev's ruling on PR 1944 is settled:
you implement (c′); you do not re-open (c), option 1 or option 2.

## The ruling, made concrete

`Vec3::orthonormal_basis` (`crates/geom-core/src/linalg/vec.rs:~405`)
starts with `s = T::one().copysign(self.z)`. Under (c′) the sign is
taken from the CANONICALISED zero — at f64, `copysign(1, n.z + 0.0)`,
which maps `−0.0` to `+0.0` and leaves every other value alone — and
at `Interval` a point-zero `n.z` answers `s = [+1, +1]` instead of the
two-sided hull, sound because the f64 program can no longer answer
`−1` at a zero.

**Shape: one `Real` door, not a change to `Interval::copysign`.**
`Interval::copysign`'s two-sided hull at a zero-containing sign is
correct for every caller whose f64 program keeps the zero's sign, so it
stays as it is. Add to `Real` (`crates/geom-core/src/real.rs`) a door —
name it for what it does, e.g. `copysign_zero_plus(self, sign)` — whose
contract is *"`copysign(self, sign + 0)`: the sign of `sign` with a
zero read as `+0`"*, implemented on every `Real` impl:

- `f64`: literally `self.copysign(sign + 0.0)` (IEEE round-to-nearest
  gives `−0.0 + 0.0 = +0.0`); pin that `−0.0` and `+0.0` both give
  `+|self|` and that every non-zero `sign` gives what `copysign` gives.
- `Interval`: `sign.lo > 0` → `|self|`; `sign.hi < 0` → `−|self|`;
  `sign` a POINT zero (`lo == hi == 0`, either bit) → `|self|` with the
  decoration capped as the sign-definite arms cap it; a zero-containing
  `sign` of NONZERO width → the two-sided hull as `copysign` gives (the
  f64 program may still answer either sign there). State at the door
  why the point-zero arm is sound: the door's OWN f64 semantics
  canonicalise the zero, so no backend invariant about sign bits is
  needed — the item's measurement 1 is exactly why (c) was rejected and
  this door is not (c).
- `Dual`, `Sym`, `Probe` (and any other `Real` impl the grep finds): the
  same contract, derived from each impl's `copysign` with the sign
  canonicalised; the derivative arm follows `copysign_deriv`'s
  convention (a point zero is not a jump for THIS door — the jump is at
  a nonzero-width straddle only). `crates/geom-core/src/dual.rs` and
  `sym.rs` are M10's ground: **announced by this spec; the orchestrator
  posts the seams**; your edit there is one impl body each.

`orthonormal_basis` calls the door; nothing else in its arithmetic
moves. Measure and state: at f64 the ONLY inputs whose frame changes
are those with `n.z = −0.0` (the existing bitwise row
`orthonormal_basis_matches_the_duff_spelling_bitwise` sweeps signed
zeros — it goes red on exactly the `−0.0` rows and is re-cut to say
so, not widened around); at `Interval`, `orthonormal_basis_at_a_vertical_plane_is_bounded_and_certified`
(which pins the hull as a decision) is re-aimed to pin the NARROWED
frame at a point zero and the hull at a nonzero-width straddle.

## The rider — `s` named twice in `b2`

`b2 = (−(s·br), s − s·(n.y²·r), −n.y)` names `s` twice in its `y`
component; the site (`vec.rs:~449`) keeps both for f64 bit identity
because a single-mention `s·(1 − n.y²·r)` would flip a signed zero at
`n = (0, ±1, −0.0)`. Under (c′) that `−0.0` is exactly the class this
unit re-baselines, so the objection may be moot: MEASURE the
single-mention spelling over the same f64 sweep; if the only rows that
move are `n.z = −0.0` rows (already moving) take the single mention on
the same golden pass and retire the site's argument; if any other row
moves, keep the double mention and record the measured reason at the
site. Either way one sentence at the site says which and why.

## Blast radius, each handled by name

1. **STEP goldens**: eight `DIRECTION` records in
   `crates/step-export/tests/fixtures/die.step` and `kiss_assembly.step`
   (#1939's census names the walls) re-derive; four are half-turn
   flips of `u_ref`. Re-cut the fixtures with the repo's own
   re-baseline tooling (whatever `step-export`'s golden tests use to
   bless — never by hand), and record in the body per record: the wall,
   old `u_ref`, new `u_ref`, and "boolean-reversed wall, `n.z = −0.0`"
   as the reason. **`crates/step-export/tests/fixtures/*` is EXCH's
   ground — announced; the orchestrator posts the seam.** No other
   golden moves: assert it (the wild and Band 4 corpora carry no
   `−0.0` wall — #1939's census rows are the pin; keep them green).
2. **Frame-bit tests**: §Sized counts ~19 tests pinning frame bits.
   Each that goes red is re-cut with its reason in a comment; none is
   deleted or widened. List them in the body.
3. **`Datum::FaceFrame`** (`crates/editor-core/src/node.rs:~704`, the
   variant doc): one doc line naming the class — a `FaceFrame` on a
   boolean-reversed vertical wall authored before this change rotates
   by a half-turn at its next evaluation; no committed document carries
   one (#1939's census), the format has no migration channel, and Ev
   accepted the cost on PR 1944. **`node.rs` is DOCM/SEAT ground —
   announced; the orchestrator posts the seam.** Doc only.
4. **M10-5's workaround** (`crates/editor-core/src/clearance.rs::{in_plane_axis, chart_frame}`):
   NOT retired here (M10's ground, and the two `[−2.2e-16, 2.2e-16]`
   walls still need it). File
   `work/m10/chart-frame-workaround-retires-for-the-point-zero-class.md`
   (`--program m10`) with the measurement below as its trigger.
5. **Payoff, measured**: #1939's instrument
   `crates/geom-brep/tests/onb_c_payoff_interval.rs` on M10-5's 12-gon
   prism — 6 of 12 walls' `u_ref.z` narrow from width 2 to ≤ 7.4e-15
   and the cell's z-enclosure halves; 4 exact; 2 stay hulled (honest
   width). Re-run it at this head and quote the table; it is the
   acceptance.

## Posture

- Red-first: the bitwise row's `−0.0` rows and the vertical-plane
  hull row, both red against `main`'s spelling, quoted; the payoff
  table before and after.
- ε posture: none — no tolerance read moves. No `CI-Config:` trailer
  (hosted CI runs the full matrix; the `Interval` lane is where the
  point-zero arm lives, so both lanes matter).
- D2-addendum: nothing retired; the new door's zero-width straddle arm
  is the hull, stated as the honest answer, not a refusal.
- Sweep obligation (discipline §5): every `copysign(` call in
  `crates/*/src` — which are sign transfers whose f64 program keeps the
  zero's sign (they keep `copysign`) and which canonicalise or could
  (candidates for the new door, NOT converted here — listed with a
  disposition each); `Interval::copysign_deriv` and any other
  sign-transfer door in the backend; what the pattern cannot match.
- Territory: `python3 scripts/work.py territory --base origin/main` in
  the body with every owner (`vec.rs`, `real.rs`, `interval.rs` are
  PROPS'; `dual.rs`/`sym.rs` M10's; `step-export/tests/fixtures`
  EXCH's; `node.rs` DOCM/SEAT's; test files tcost's).
- Review: standard v6 dual (block PROPS-B2 slot 1; ordinal claims at
  review dispatch). Reviewers' first target: soundness of the
  point-zero arm — construct an f64 program whose zero is `−0.0` at the
  door and show the `Interval` answer still encloses it; second: that
  no golden outside the twelve walls moved.
- **Landing: the item gets `pr:` and `status: review`. DO NOT MERGE —
  the orchestrator lands after the dual and its fix pass; the fix pass
  closes the item (its RULED section stands), deletes this spec at
  merge with its `## Per-merge deletion` section in
  `docs/DOC-LEDGER.md`.** No `Co-Authored-By`; push early to
  `props/sign-hull`.

## Acceptance

The door on every `Real` impl with its contract stated; `orthonormal_basis`
on it; the `Interval` point-zero frame narrowed and pinned, the
nonzero-width straddle still hulled and pinned; f64 moves only on
`n.z = −0.0` inputs, measured; eight STEP records re-derived with
reasons and no other golden moved; the `FaceFrame` doc line; the M10
follow-up filed; the payoff table quoted; hosted CI green on the full
matrix.
