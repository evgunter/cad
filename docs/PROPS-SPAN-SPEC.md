# PROPS span — `Span<'a>` carries its `KnotVector`; the unbranded pairing becomes unrepresentable (ruling A)

**Binding at dispatch** (PROPS program; the item is
`work/props/span-carries-its-knot-vector.md` — read it in full, Ev's
ruling is **A** (in-chat, 2026-09-05); difficulty logged at spec: **L**,
STRUCTURAL — a lifetime threaded through one layer and its consumers,
no arithmetic moves). Read `docs/prompts/implementer-discipline.md` in
full. Branch `props/span-knot-vector`, cut from `main` AFTER CERT-N3
(#1879) — its `spline/algebra.rs` and `knots.rs` edits are in your base.

## What A means, exactly

`Span` (`crates/geom-core/src/spline/knots.rs:242`) proves "in range and
nonempty for the vector it was drawn from" and travels without that
vector, so `basis_funs(kv, span, t)` accepts a span from A against B —
silently wrong if B is longer, an index panic if shorter. Under A the
span carries the borrow:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span<'a> { kv: &'a KnotVector, index: usize, first_control: usize, degree: usize }
```

and every entry point that took `(kv, span)` drops `kv`:
`basis_funs`, `ders_basis_funs` (`basis.rs:79,122`), `span_hull`,
`span_hull_rational`, `derivative_span_hull`, `sup_norm_bound_span`
(`hull.rs:131,201,305,351`), the private `span_indices` and
`span_weights_positive`, `geom-brep/src/props/quad.rs:1077`'s
`bspline_eval_ring_in_span`. `KnotVector::admits(span)` is deleted —
the state it tested is unrepresentable — and `span_indices` keeps ONLY
its `coeff_len != kv.control_count()` half (see §Residue). `SpanSet<'a>`
carries the lifetime; `SpanLocate::locate_spans` becomes
`fn locate_spans<'a>(self, knots: &'a KnotVector) -> SpanSet<'a>` — a
method-level lifetime, the trait itself unchanged — with its five impls
(`locate.rs`, `dual.rs`, `interval.rs`, `k_stats.rs`, `sym.rs`) edited
by signature only. `Span` stays `Copy` and `Eq` (a `&` is; equality
now includes the vector by address — say so at the derive, or derive
`PartialEq` by hand on the indices and argue which is right).

**The mutation-hold check comes FIRST** (the ruling's condition): the
census at `main` today found every span `let`-local within one
immutable borrow and every knot-algebra door `&self -> Self`; a span
held across an `insert_knot`/`refine_knots`/`elevate_degree`/
`refine_to_union`/`apply_plans` would not compile under A. Re-derive
that at your base by attempting the change and reading the borrow
errors: if any site genuinely holds a span across a mutation of its
own vector, STOP, report it in the PR body with the site, and do not
restructure it — the answer then falls to C and the orchestrator
re-asks. Expect none.

**`SurfaceWindow` (#468) closes fully**: `crates/geom/src/surfaces/nurbs.rs:199`
holds two spans plus `base`/`stride` derived from `knots_v`; under A it
holds `&'a NurbsSurface` and derives the rest, its public mints
(`window`, `window_at`) return `SurfaceWindow<'_>` tied to `self`, and
`eval_in_span`/`ders_in_span`/`ders3_in_span` take it — a window from
surface A passed to surface B is then a type-level mismatch only if the
methods read the window's surface; make them, and delete
`NurbsSurface::admits`. State at the type which of the two pairings
(u-vector, v-vector) it closes: both, through the one borrow.

**The curve doors** (`curves/nurbs.rs` `eval_in_span` and siblings):
the span borrows `self.knots`, but a span from curve A still typechecks
against curve B's method. Read the basis from the SPAN's vector (never
from `self.knots` beside it), so the only remaining mismatch is
coefficients-against-vector — §Residue — and say so at each door.

## Residue — file it in this PR, do not merely disclose it

A closes the span↔vector pairing. It does NOT close the
coefficient↔vector pairing (`span_indices`' surviving `coeff_len` half,
`hull.rs:85-96`'s own doc: a same-length array from another curve
passes) nor `InteriorKnot`'s (`knots.rs:191`, deliberately
crate-private for exactly this reason — CERT-N3's decision, not yours
to reopen). Give the coefficient pairing its own item under
`work/props/` in this PR (`work/README.md`'s residue rule; your own
program's slate), with the guard's site and what it does and does not
catch. `InteriorKnot` gets one sentence at its doc pointing at `Span`'s
new shape as the precedent, nothing more.

## Fence

`crates/geom-core/src/spline/{knots,locate,basis,hull,algebra,compose}.rs`
and the five `SpanLocate` impl sites (`crates/geom-core/src/{dual,interval,k_stats,sym}.rs`
— `dual.rs` is M10's by its program header; the edit is one impl
signature forced by the trait, announced in this spec and named in the
PR body); `crates/geom/src/{curves,curves/nurbs,curves/fit,curves/projection,surfaces/nurbs,surfaces/projection}.rs`;
`crates/geom-brep/src/{patch_bound,pcurve_cache,props/loop_area,props/quad,ssi,ssi/certify,ssi/enclose}.rs`;
`crates/mesh/src/chords.rs:398`; `crates/step-import/src/recognize.rs:393-402`;
every test file the signatures reach (~112 sites). The census counted
roughly 82 `src` and 112 test sites across five crates; the PR body
carries the per-crate table of what actually changed. Other programs
own several of these files (`work.py territory --base origin/main`
lists them; it warns, it does not block): every edit outside `geom-core`
is the mechanical consequence of a `geom-core` signature, no subject
edit on anyone's ground — say so per crate.

## Posture

- **No arithmetic moves.** Every basis, hull and evaluation reads the
  same vector it read before, through the span. Bit identity is the
  acceptance for every numeric output: state it, and pin it once — a
  row that evaluates a curve and a surface at a literal corpus through
  the new doors and compares to the retired spellings' values captured
  before the change (the CERT-N3 D31 shape: measure before, compare
  after, quote in the PR body).
- **Red-first, honest per S216**: `compile_fail` doctests with the
  error code — a span from one vector fed to a door on another cannot
  be written (E0597 or E0499 as the borrow dictates; verify the code by
  compiling the row and quote what stable rustdoc does and does not
  check, as CERT-M3 did); one twin per row differing in one identifier
  that resolves. The retired panic path (`B` shorter than `A`) gets a
  row showing it is now unwritable, not a row showing it still panics.
- **D2-addendum classification**: `admits` and `NurbsSurface::admits`
  retired at row 0 (the state is unrepresentable); per retired guard,
  name every consumer that matched on its `false`/`None` and what it
  became. No new refusal, no new panic: a `Span` is an in-repo value,
  and D9's "never panic on input" is untouched.
- **ε posture:** none; say so in one line.
- **Sweep obligation** (discipline §5): the shape is *a proof about one
  value carried beside a different value of the same type* — after
  `Span` and `SurfaceWindow`, read `geom-core/src/spline/` and
  `geom/src/{curves,surfaces}` for any other such pair (a `usize`
  index proved against one container and used on another; `InteriorKnot`
  is the known member); hit list with disposition; what reading cannot
  match.
- **Companion note** beside the code, present tense, with a clause id
  in the file's existing convention: `crates/geom-core/README.md`'s
  spline clause states the new invariant — a span is a borrow of the
  vector it indexes; `SurfaceWindow` a borrow of its surface — and
  names the one pairing left open. `docs/DESIGN.md` is not edited
  (the ruling is Ev's in-chat and lands as the README clause per
  `CLAUDE.md`'s convention for finished work).
- **Review:** standard v6 dual (block PROPS-B1 slot 1; ordinal claims
  at review dispatch). Reviewers' first target: the borrow does what A
  claims — construct the mismatch every way the old API allowed and
  show each is now a compile error or, where it is not (the curve
  doors, the coefficient pairing), that the door says so; second: bit
  identity on the corpus, both lanes.
- **Landing**: the item gets `pr:` and `status: review`; the residue
  item filed; the spec deleted at merge. No `Co-Authored-By`; no
  `CI-Config:` trailer; push early to `props/span-knot-vector`;
  re-merge `main` before every push (the `spline/` layer is live).

## Acceptance

`Span<'a>` and `SpanSet<'a>` carry the borrow; every `(kv, span)` door
lost its `kv`; both `admits` deleted; `SurfaceWindow<'a>` holds its
surface; the five `SpanLocate` impls compile with the method-level
lifetime; the mutation-hold check reported (expected: no site);
`compile_fail` rows honest; bit identity pinned; the residue item
filed; the README clause written; hosted CI green on the full matrix.
