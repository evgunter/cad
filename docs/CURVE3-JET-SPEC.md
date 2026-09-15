# CURVE3-JET — the whole-curve order-1 jet door; the eval/deriv pairs go onto it

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit CURVE3-JET; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is
`work/scalar/curve3-eval-and-deriv-at-one-t-run-two-basis-passes.md`
(set its `branch:`/`pr:` fields; its `## Closed` is written by the
orchestrator at merge).

## 0. The finding, and what the tree says against it

The row: `Curve3::eval(t)` and `Curve3::deriv(t)` at one `t` each run
`t.locate_spans(&self.knots)` and a per-span basis pass on the `Nurbs`
arm, so a caller wanting a point and a tangent pays the locate twice and
the basis pass twice; the span level has `ders1_in_span` and the
whole-curve level has nothing that returns a jet. Seven production sites
write the pair.

**What the survey (2026-09-15, `origin/main` `51556b947`) corrects:**

- The whole-curve ORDER-2 jet already exists: `NurbsCurve3::ders(t)`
  (`crates/geom/src/curves/nurbs.rs:1494-1512`, row C24) — the located-span
  walk with `ders_in_span` and three hulls, sole consumer `deriv2`. The
  item's sentence is true of `Curve3` and false of `NurbsCurve3`. The
  door this unit mints is that door's order-1 sibling, and the name is
  forced by the ladder `ders`/`ders_in_span` ↔ `ders1_in_span`: **`ders1`**.
- The seven sites stand; one moved crates: the `revolve/upgrade.rs` pair
  is now `crates/geom-brep/src/dihedral.rs:373-374` (`must_carry_over_edge`).
  Current lines: `topo/src/validate.rs:4074-4075`,
  `topo/src/boolean/ops.rs:1059-1060`,
  `topo/src/boolean/contact_verify.rs:315-316`,
  `geom-brep/src/dihedral.rs:373-374`, `sweep/src/blend/battery.rs:918-919`,
  `geom-brep/src/certify.rs:1822` + `:1884`, `sweep/src/skin.rs:1152` + `:1192`.
- Six more pair sites the item did not sweep, on a bare `NurbsCurve3<f64>`
  feeding S393's `path_start_frame(origin, tangent, tol)`:
  `demos/tour/src/skinned.rs:184`, `:356-357`, `:633`, `:789` and
  `crates/pncad/examples/sweep_start_frame_and_roll.rs:34`, `:82`. The tour
  narrates the pair as what a user writes. Demos are evidence
  (`memories/demo-purpose.md`), so **they are in the unit**.
- Bit identity is inherited by construction: `ders_basis_funs`'s order-0
  row is `basis_funs`'s recursion (`geom-core/src/spline/basis.rs:72-87`
  vs `:118-137`), `rational_corrections` at `k = 0` is `eval_in_span`'s
  division, `deriv_in_span` IS `ders1_in_span(t).1`, and the multi-span
  `Interval` hull folds the same range in the same order as `eval`/`deriv`
  (as `ders` already does). Nothing committed can move.

## 1. What this unit delivers

**The doors.**
- `NurbsCurve3::ders1(&self, t: T) -> (Point3<T>, Vec3<T>)` in the
  `impl<T: SpanLocate> $Curve<T>` block of the `nurbs_curve!` macro
  (`nurbs.rs:1439`), beside `eval`/`deriv`/`ders`: the `ders` walk with
  `ders1_in_span` in place of `ders_in_span` and two hulls instead of
  three. The macro mints `NurbsCurve2::ders1` with it; say so in the doc
  and give it the same pin (§3).
- `Curve3::ders1(&self, t: T) -> (Point3<T>, Vec3<T>)` in
  `impl<T: SpanLocate> Curve3<T>` (`curves.rs:320`), exhaustive arms:
  `Line` → `(*origin + *dir * t, *dir)`; `Circle` → ONE `azimuth::frame`
  and both its fields (`(center + radial·r, tangential·r)`, the frame's
  own formulas at `azimuth.rs:72-75`, so both halves are `eval`'s and
  `deriv`'s bits by construction); `Ellipse` → one `azimuth::basis` then
  the two forms verbatim from `eval` (`:352-353`) and `deriv` (`:397-398`);
  `Nurbs` → `n.ders1(t)`.
- **Return type: the tuple**, as `ders1_in_span` and `ders` return. There
  is no curve jet type in the tree (`TangentJet` is `tangent_jet`'s
  OUTPUT, `SurfaceJet` is the surface's order-2 struct); every consumer
  destructures `(p, tau)` on the spot; a struct would be a third spelling
  beside two tuples. If a reviewer wants a named `CurveJet1`, that is a
  row, not this unit — say so once in the doc.
- `eval` and `deriv` keep their own passes; nothing is re-routed through
  `ders1` (the `Surface::jet` precedent, `surfaces.rs:404-408`: `eval` is
  not a projection of the jet). **No `ders2`**: `deriv2`'s only pair
  consumer is `topo/src/splitting/neighborhood.rs:180-184` on the
  `Circle | Ellipse` arms, never `Nurbs`; mint it the day a NURBS consumer
  of `(P, P′, P″)` exists, and say that in the PR body, not in the tree.

**The consumers.** Every site above folds onto `ders1` — the `Curve3`
sites onto the enum door, the `NurbsCurve3<f64>` sites (`skin.rs`, the
tour, the example) onto the payload door — two lines become one, the
names `p`/`tau` kept. Two need a decision, made here:
- `certify.rs:1822`/`:1884`: the derivative is CONDITIONAL (the
  `Resolved::Tangent` arm, interior samples only). Fold ONE `ders1`
  inside the `Tangent` arm's interior branch and leave `eval` for the
  `Intersection` arm and the end samples; a jet at `:1822` would compute a
  tangent the other arm discards. State the shape in one sentence at the
  site.
- `battery.rs:1545-1561`'s `pick` closure (evaluates at both ends,
  differentiates at one) and `curves.rs:562-566` `param_near`'s `Circle`
  arm are NOT sites (the item's dispositions, re-checked); leave them.
  `topo/src/boolean/rim_wedge.rs:292-296` hand-rolls the `Circle` arm's
  jet without a `Curve3` — not a site; file it as a possible consumer on
  BOOL's slate (`python3 scripts/work.py new … --kind issue --program bool`),
  do not fold it here.

**Prose.** `curves.rs:372-376` ("There is no jet door … no `CurveJet` is
minted" — the CERT-N3 fix pass's sentence, never ratified: `git log -S`
finds no Ev commit; cite the commit in the PR) is re-worded to what holds
of the `deriv`+`deriv2` pair with `ders1` and `ders` named; the nurbs
module doc's door list (`nurbs.rs:60-64`) gains `ders` and `ders1` (it is
already stale by `ders`). Present tense only, no history.

**What must not change:** every point and tangent at the thirteen sites,
bit for bit — the tour digest recipe (`work/scalar/rate-pair-in-geom-core.md`
§Digest receipt, zero-parameter, listing and narration) identical at
merge base and head; `span_bit_identity.rs` and `span_bit_identity_ext.rs`
green with their captured `DIGEST`s untouched (**do not add `ders1`
labels to the captured digest** — pin the new door with differential rows,
§3); no render cell moves; the k-lint predicate counts do not move (no
decision is added — `locate_spans` and the hull are the doors' own).

## 2. The measurement

One timing line in the PR body from the existing meter shape
(`crates/geom/tests/curves/n1r1_c24_meter.rs`, `CAD_R1_BENCH=1`): `eval`+`deriv`
vs `ders1` at degrees 2/3/5/7, reporting not gating — the row's claim is
"two passes where one answers", and the number says how much.

## 3. The pin

- `ders1_is_eval_and_deriv_bit_for_bit` at the whole-curve level: over
  `knot_and_span_params` on the knotted fixture at `f64`, and on the
  `_ext` corpus at `Dual64` and `Interval` including a multi-span
  `Interval` parameter straddling a knot (the one case where the hull fold
  could diverge) — assert equality of both halves against `eval`/`deriv`,
  do not capture a digest. Same row for `NurbsCurve2`.
- A `Line`/`Circle`/`Ellipse` row comparing `ders1` to the pair bitwise on
  the tilted fixtures (`curves.rs:584`, `:599`, `:777`).
- A pairing row beside `span_window_pairing.rs:238-239`: `ders1(t)` equals
  `ders1_in_span(t)` on the located span.
- Red first: a one-ulp mutant of the `Nurbs` arm's tangent half reds the
  differential row; with the sites folded it also reds the `topo` tier-3
  rows or the tour — put the mutant table in the PR body (which rows, at
  which mutant).
- D9: `cargo nextest run -p geom` (both digest suites; `--features
  interval` for `_ext`), `-p topo -p sweep -p geom-brep -p pncad` green
  unchanged; the tour digests identical (take the base digest at your
  merge base before the first code change, as RATE-PAIR did, and commit
  the receipt on the item).

## 4. Sweep

The class: `eval(x)` and `deriv(x)` on one receiver at one parameter
expression. Re-run the item's two passes at your merge base — the
same-identifier backreference (300-char window) and the same-EXPRESSION
pass (balanced-paren argument, 120-line window, both orders, receiver
recorded) — over `crates/*/src`, `crates/*/tests`, `demos/`, `tools/`,
`benches/`, `crates/pncad/examples`; disposition every hit in the PR body
(site / not a site and why; the test-tree hits are dumps, meters and
pairing rows that must keep calling both doors). State the blind spot: a
pair whose two calls name the parameter differently, a pair routed through
two helpers, a different receiver reached by the same expression.

## 5. Fence

This program claims no paths. This unit reaches PROPS'
`crates/geom/src/{curves.rs,curves/nurbs.rs}`, TOPO's
`crates/topo/src/validate.rs`, BOOL's AND CURVED's
`crates/topo/src/boolean/{ops.rs,contact_verify.rs}` (both claim the
glob — announce to both), BLEND's `crates/sweep/src/{blend/battery.rs,skin.rs}`,
LIB's `crates/pncad/examples/*`, TINT/TCOST's `crates/geom/tests/*`, the
unowned `demos/tour/src/skinned.rs`, and `crates/geom-brep/src/{certify.rs,dihedral.rs}`,
which are in **no program's `paths:`** (PRED's `keep_out` calls
`certify.rs` PROPS' in prose only) — the PR draws that fence and
announces it to PROPS on D306's precedent. Announced by the orchestrator
in `work/scalar/log.md` and on the PR. Merge `origin/main` immediately
before opening the PR; run `python3 scripts/work.py territory --base
origin/main` and put the output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom -p topo -p sweep -p geom-brep -p pncad`
at default features and `-p geom --features interval`; `cargo build -p
pncad --examples`; `cargo clippy --workspace --all-targets -- -D warnings`
at default AND `--all-features`; the tour (`cd demos/tour && cargo clippy
--all-targets -- -D warnings && cargo nextest run`) — the tour is outside
`--workspace` and `cargo fmt --all` does not reach it, so run `cargo fmt
--check` in `demos/tour` and `demos/wild` too (two SCALAR heads went red
on exactly that); `scripts/doc-gate.sh`. Use a private
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; disk is shared. Hosted CI is the
verification of record; poll the run to conclusion in the foreground and
report the run id. Report ≤150 lines: the doors' signatures as landed,
the thirteen sites as folded (the two decisions of §1 as taken), the
sweep hit list and blind spot, the mutant table, the timing line, the
digest receipt at base and head, deviations, findings filed outside the
fence with row names.
