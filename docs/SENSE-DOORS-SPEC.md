# SENSE-DOORS — the five bare-T sense-sign doors take the bit; sphere's mixed sign splits

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit `sense-sign-doors-take-the-bit`; deleted at
merge per `docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md`
in full first. The item is `work/scalar/sense-sign-doors-take-the-bit.md`;
the ruling is `work/scalar/D6.md` §RATIFIED (PR 2457).

## 0. The ruling this executes

The sense is a bit (`Face::sense`), and the typed thing is what the bit
selects: `OutwardNormal<T>` where a normal is meant (`from_chart(chart_normal,
sense: bool)` is its only constructor, and its doc says why a `T` sign is
the wrong parameter), `bool` at the doors that compute their own gradient.
Never a ±1 type. `Face::sense_sign<T>()` retires in the SECOND unit
(`sense-sign-multiplies-fold-onto-outward-normal`), not here; this unit
takes the doors.

## 1. What this unit delivers

**The five doors take `sense: bool`** instead of a `T` ±1:

- `geom_brep::classify_material_pairing(s_plus, sense_plus, s_minus,
  sense_minus, p, arm, band)` (`crates/geom-brep/src/dihedral.rs`) — it
  multiplies the sense into the gradient it computes itself; the multiply
  becomes a conditional negation of the gradient. Callers:
  `topo::census::ee_cross_backed`, `topo::validate` check 4,
  `topo::boolean::rim_wedge` (forwarding).
- `geom_brep::material_kappa_rel(kappa_rel, sense_plus)` (`dihedral.rs`)
  — `-(kappa_rel * sense_plus)` becomes `if sense { -kappa_rel } else
  { kappa_rel }`; exact, so bit-identical. Callers: `rim_wedge`,
  `validate` check 4.
- `topo::boolean::rim_wedge::classify_shared_rim(s_plus, sense_plus,
  s_minus, sense_minus, rim, extent, band)` — pure pass-through; its
  caller `verify_tangent_declaration` (`crates/topo/src/boolean/mod.rs`)
  has a `senses` closure that defaults a MISSING face to `T::one()`.
  With a `bool` door that default would read `true` — do not write it:
  the enclosing function refuses a missing face two lines above
  (`surface_of(..)?`), so the missing-face arm refuses through the same
  vocabulary, and `work/bool/missing-face-in-verify-tangent-declaration-reads-as-sense-true.md`
  closes in this PR (BOOL's row and ground — say so; the unit closes the
  class it is in, not an unrelated defect).
- `geom_brep::curved_face(surface, outer, sense_sign, band)` and its
  private `sphere(.., sense_sign, ..)` (`crates/geom-brep/src/props/curved.rs`)
  — caller `topo::props::face_flux`. **`sphere`'s `s_f` is two things
  spelled as one `T`**: the face sense on one branch and a decided rim
  side `t_sign(linear_rim_side(..)?)` (a `Sign`, which can be `Zero`) on
  the other. Split them: the sense arrives as `bool`, the rim side stays
  a `Sign` (or the `T` it already is, minted from the `Sign` at the one
  place it is consumed) — and check whether anything relied on the zero
  arm; say what you found.

**Every caller** passes `face.sense` (the bit) instead of
`face.sense_sign::<T>()`. `Face::sense_sign` itself stays (the second
unit retires it with the ten multiplies). The `dihedral.rs` tests that
pass `1.0`/`-1.0` literals into the bare doors pass `true`/`false`.

**What must not change:** every verdict and every margin these doors
produce, bit for bit — a conditional negation is exact in IEEE, so a
D9 differential over the callers' suites (`topo`, `geom-brep`, `sweep`
where it reaches) is green unchanged, and the k-lint gate's predicate
counts do not move. No refusal vocabulary changes except the
missing-face arm above.

## 2. Comment style, docs

Comments state the invariant: a door's doc says the sense is the face's
bit, not a sign to multiply by, and points at `OutwardNormal::from_chart`'s
doc for why (do not restate the argument). No history.

## 3. The pin

- The existing callers' suites, green unchanged (the D9 differential;
  say which suites and that nothing moved).
- One row per door that the `true`/`false` arms produce the same
  classification as the retired `1.0`/`-1.0` did on a fixture where they
  differ from each other (anti-vacuity: the two senses must classify
  differently).
- `sphere`: a row at the rim-side branch that a `Sign::Zero` rim side
  behaves exactly as before the split (or, if nothing can reach zero,
  the argument in the doc and a `debug_assert`).
- The missing-face arm: a row that a `FaceKey` resolving to no face
  refuses typed rather than classifying.

## 4. Sweep

The class: a function taking or returning a sense as a scalar `T` (±1)
rather than the bit or an `OutwardNormal`. Sweep `crates/*/src` (and
the excluded roots) for `sense_sign` reads that cross a function
boundary and for `T` parameters named `sense*`/`s_f`/`sign` that are
minted from `Face::sense`; the ten hand-multiplies are the SECOND
unit's — list them as not-this-unit with the pointer. Hit list with
dispositions and the blind spot in the PR body.

## 5. Fence

This program claims no paths. This unit reaches `crates/geom-brep/src/{dihedral.rs,props/curved.rs}`
(PROPS'), `crates/topo/src/{census.rs,validate.rs,props.rs,boolean/mod.rs,boolean/rim_wedge.rs}`
(TOPO's and BOOL's). Announced by the orchestrator; merge `origin/main`
before opening the PR; `python3 scripts/work.py territory --base
origin/main` output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-brep -p topo` at default features, plus
the interval lane for `topo` if a touched row is `cfg(feature = "interval")`.
Hosted CI is the verification of record; poll to conclusion in the
foreground. Report ≤120 lines: the five signatures, what `sphere`'s
split found, the differential's receipt, the sweep hit list and blind
spot, deviations, rows filed or closed and where, PR number, head SHA,
CI run id and conclusion.
