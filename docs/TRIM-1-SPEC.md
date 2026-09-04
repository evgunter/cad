# TRIM-1 — the de Boor collapse extractor for interior iso-curves (spec)

Item `work/trim/interior-iso-curve-de-boor-extractor.md` (#1195), TRIM
plan order 3. Branch `trim/1-de-boor-extractor`. Charter input
`docs/PCURVE-P2-SPEC.md` (§6 "do not disturb these rows", §7 "widen the
schedule, do not downgrade"); P-2's record is PR #1177 and commits
`e58df98d7` (rim arms widened) / `3c219381b` (wall–seam half reverted).
**Difficulty pre-logged L; task class NUMERIC** (argued under "PR
shape"). Survey run 2026-09-04 against main `f23d373d5`, by symbol;
every line cite below was re-derived on that head.

**One sentence.** An interior iso-curve `u = u*` of a described NURBS
chart is an EXACT curve in the chart's own `v` spline space whose
control polygon is the de Boor collapse of the net at `u*`; the seam
class's control-difference hull applies to it verbatim once the
collapsed row replaces the boundary-row COPY, and every primitive the
collapse needs already exists. This unit adds the extractor beside
`boundary_iso_u`, teaches check 4's seam class the interior route, and
lets `nurbs_iso_derive`'s wall–seam arm measure the column it is
missing. Nothing is downgraded; `9v1`'s `General` route is untouched.

---

## What the survey refuted (binding premise corrections)

**(1) `an_interior_column_still_refuses` is NOT the row this unit
flips, and it keeps refusing untouched.**
`crates/geom-brep/tests/imported_chart_arc_rim.rs:156-177` certifies a
`Pcurve::IsoArc` with `p0 = (1, 0)`, `pd = (5.196…, 0)` against
`rim(1.0)`. Its FIXED channel is `v = 0`, a genuine boundary row; the
"interior" thing is the ARC's `u`-start, and the image `u ∈ [1, 6.196]`
leaves the `[0, 3√3]` domain. That is a wrong-locus rim of the ARC-RIM
class (`run_iso_arc_checks`, `pcurve_cache.rs:3425-3497`: lane →
interval → **schedule residuals** → class), and the residual at `t = 0`
is millimetres, so it refuses at check 3 before `side_of` is reached
(derived from the check order — UNMEASURED, see the probe note). P-2 §6 keeps the row — rightly, but
the row exercises the arc class, not the interior-column refusal, and
the arc class is untouched here. Likewise
`a_seam_column_certifies_on_a_non_unit_chart`'s second half
(`:217-222`): `u = 1` claimed for the `u = 0` row's carrier is a
different LOCUS and refuses at the residual, before and after.

**(2) The wall–seam revert's stated reason was a test's NAME.**
`3c219381b` and the comment it left (`crates/topo/src/pcurves.rs:561-588`)
say the measured column "handed the certifier exactly the image
`an_interior_column_still_refuses` requires it to refuse". Per (1) that
row never reaches `side_of`. What refused the widened mint was
`side_of` itself (`pcurve_cache.rs:3738-3764`) from the seam arm
(`:3882-3890`) — the certification-side refusal the item file quotes.
The revert's conclusion stands (the certifier could not certify it);
its evidence did not. The reverted derivation code (`git show
3c219381b -- crates/topo/src/pcurves.rs`, the `-` lines) is the right
derivation-side shape and comes back essentially verbatim.

**(3) No new `geom` primitive is needed.** Every ingredient is public
and `T`-generic: `geom_core::spline::basis::basis_funs<T: Real>`
(`basis.rs:79`, the Cox–de Boor row on one span, structure-denominator
form, D9 fixed association); `SpanLocate::locate_spans` +
`enclosure_hull` (`geom-core/src/spline/locate.rs:86-106`, the
per-span/hull idiom `NurbsSurface::eval` uses, `nurbs.rs:1042-1062`);
the homogeneous evaluation form (`eval_in_span`, `nurbs.rs:521-549`);
`NurbsSurface`'s accessors. `Decide: SpanLocate` (`predicate.rs:813`),
so the extractor runs at every `Decide` scalar, `Dual` included.
Fence 7 is met with **no announced seam into `crates/geom/src/`**.

**(4) On the P-2 fixture the collapse is a control-row COPY.** The
widened chart is degree 1 in `u`, knots `[0,0,1,2,3,3]`
(`crates/sweep/tests/m8_4_intersection_iso.rs:371-393`): at `u* = 1`
the basis row is the Kronecker delta on control row 1, `Q_j = P_{1j}`
bit for bit, hull identically zero. Same on dm1's wall (double knots,
degree 2). The collapse's arithmetic must be exercised at the
certifier's door on a `u*` strictly between knots (rows A1, A2).

**(5) `16v1`'s description names the bowed wall's OWN pre-widening
chart, not the neighbour.** The loft describes seam `j` as wall `j`'s
`u = 0` iso (`crates/sweep/src/loft.rs:518-543`); `rechart`
(`m8_4…:408-418`) mints a NEW key and `set_face_surface` keeps the old
one alive because that description references it
(`crates/topo/src/attach.rs:52-53`). `16v1` reaches the wall–seam arm
by key inequality (`pcurves.rs:548`); the arm's logic is the same
either way. Row A3 must NOT re-describe `16v1` against the new key —
that would route it through the own-chart arm and stop testing the
measurement.

**(6) S394's count is three, not two.** `map_err(|_|` on a
`boundary_iso_*` call at `pcurve_cache.rs:3523` (arc), `:3891` (seam)
and `:3961` (cap).

**Probe note — two measurements this survey could not take.** The
machine's build mutex was held by other lanes for the whole survey
(2.5 h; a queued run of the two rows in (1) never acquired it). So
(1)'s "refuses at check 3" is DERIVED from `run_iso_arc_checks`' check
order and the geometry, not read off a payload, and the P-2 body's
behaviour past `16v1` (STOP 3) is likewise unmeasured. **The
implementer's first act, before any edit:** (i) run the two rows in (1)
with their refusal printed and quote the payloads in the PR body — if
either names `INTERIOR` rather than `ResidualExceeded`, premise (1) is
wrong and the row IS on this unit's path: STOP and report; (ii) run
`mint_pcurves` on A3's fixture and quote `16v1`'s payload and raising
site. A ready-made scratch (the reverted derivation restored plus a
polynomial-only collapse in the seam arm, and A3's tail printing
mint/validate/props/tessellate outcomes) is at
`/home/evan/.local/share/cad-work/trim-1-spec-stub.py`, lane-private,
never to be committed.

---

## The math, stated for Ev

**Setting.** A described chart is `S(u, v) = Σ_ij R_ij(u, v) P_ij`,
`R_ij = N_i(u) N_j(v) w_ij / Σ_kl N_k(u) N_l(v) w_kl`, weights `w_ij > 0`
stored as `f64` STRUCTURE (C6: `NurbsSurface::weights() -> &[f64]`,
`nurbs.rs:322`; control points at `T`). An *iso-curve* is the
restriction to a parameter line; a *pcurve* is an edge carrier's 2-D
image in `(u, v)`. The seam class (`Pcurve::IsoLine`, `pl.x`
banded-zero) claims `P(t) = (u*, v(t))`, `v` affine in the carrier
parameter.

**The collapse.** Fix `u*`. Homogeneously
`Σ_i N_i(u*)·(w_ij P_ij, w_ij) = (W_j Q_j, W_j)` with
`W_j = Σ_i N_i(u*) w_ij` and `Q_j = Σ_i λ_i(j) P_ij`,
`λ_i(j) = N_i(u*) w_ij / W_j` — a convex combination. EXACTLY in ℝ,

    S(u*, v) = Σ_j N_j(v) W_j Q_j / Σ_j N_j(v) W_j ,

a rational curve in the chart's own `v` space (`knots_v`, degree `q`)
with control polygon `Q` and weights `W`: de Boor's triangle at `u*`
collapsed one step short of a point, the `v` direction left symbolic.
It is the row that inserting `u*` to multiplicity `p` would expose as a
control-net copy — which is why the boundary case (multiplicity already
`p + 1`) is a copy with no arithmetic (`nurbs_iso.rs:38-55`).

**The hull, unchanged.** With the carrier
`C(v) = Σ_j N_j(v) w^c_j c_j / Σ_j N_j(v) w^c_j` in the SAME space and
`w^c_j ∝ W_j`: `S(u*, v) − C(v) = Σ_j R_j(v)(Q_j − c_j)`, `R_j ≥ 0`,
`Σ R_j = 1`, hence `sup_v |S(u*, v) − C(v)| ≤ max_j |Q_j − c_j|`.
That is check 4's seam argument (`pcurve_cache.rs:3845-3856`,
`EnvelopeStatement::MapResidualIsoHull`, `:971-989`) with the copy `B`
replaced by `Q`. The argument never used that `B` was a copy; it used
that `B` IS `S(side, ·)` in the same space, and `Q` IS `S(u*, ·)`.

**"Exact" in floating point.** `Q` is never STORED; it is computed at
certification time at the certifying scalar `T` and compared with the
carrier's stored structure. At `Interval`, `basis_funs` returns
enclosures of `N_i(u*)` (denominators are pure `f64` knot differences
lifted once, `basis.rs:42-56`), products and sums round outward, and a
straddling `u*` is evaluated per overlapped span and hulled
(`locate_spans`/`enclosure_hull`, the evaluator's own idiom). So each
computed `Q_j` ENCLOSES the exact one, `hull = max_j |Q_j − c_j|` has
`hull.hi ≥ sup_v |S(u*, v) − C(v)|`, and `decide("pcurve_envelope", …)`
returns `Zero` only when the WHOLE enclosure sits inside the band
(`interval.rs:588-598`). The collapse's rounding lives INSIDE the
certified number; it is not a separate envelope term. At `f64` the
same arithmetic rounds to nearest and the band absorbs it, as for every
other f64-lane predicate. **No new interval capability; `Dual` runs the
arithmetic and certifies nothing, as today (D1).**

**The rational case, and the class limit.** `W_j` is COMPUTED at `T`,
and a `NurbsCurve3<T>` carries `f64` weights as structure
(`geom_core::spline::algebra` docs: "weights stay `f64` forever").
The hull needs `w^c_j ∝ W_j` exactly, so the exact class admits an
interior column of a rational chart exactly when `W` is structure —
when the weight net factors in a way an exact `f64` test can see:

- **(a) weights constant along `u`**: `w_ij == w_0j ∀i` (bitwise).
  `W_j = w_0j` in ℝ; `λ_i = N_i(u*) / Σ_k N_k(u*)` (exact in ℝ, centres
  the enclosure); the row is wrapped with row 0's weights and the
  carrier must carry them bitwise — the existing check (`:3897-3906`).
- **(b) weights constant along `v`**: `w_ij == w_i0 ∀j`. `W_j` is
  constant in `j`, so the curve IS the polynomial `Σ_j N_j(v) Q_j` with
  `λ_i = N_i(u*) w_i0 / Σ_k N_k(u*) w_k0`; the row is wrapped with
  weights `1.0` and the carrier must carry a CONSTANT weight vector
  (entries bitwise equal — any constant cancels in `R_j`). This is
  every arc-profile loft/sweep wall this kernel builds (profile weights
  vary along `u`, constant along the sweep) and dm1's imported wall
  (`imported_chart_arc_rim.rs:29-66`).
- Both at once: the polynomial chart (the P-2 fixture, every weight 1).
- **Anything else refuses typed**: a non-separable net has no
  structure weight vector for any carrier to share; the seam class's
  hypothesis genuinely fails. The honest route is a composite bound
  (`MapResidualComposite`, `compose::tensor::surface_curve_residual`,
  consumed only by `ssi/certify.rs:502-516`) WITHOUT a uniqueness
  tube — a new certificate statement, filed as a residue, not built.

The existing slacks stay and stay weight-aware: `stretch_u/v`
(`nurbs_stretch_bounds`, `:2869`, carrying `weight_ratio_factor`)
meter the fixed channel's DRIFT (`du_extent`) and the domain overshoot
as today. What the interior arm drops is the boundary SNAP term
`|u_start − side|·stretch_u` (`side_of`'s `(w − lo).abs() * arm`): the
row is collapsed at `u_start` itself, so there is nothing to snap.

---

## The mechanism

### 1. The extractor — `crates/geom-brep/src/nurbs_iso.rs`

Beside `boundary_iso_u`, per the module's placement rule (`:19-31`:
extraction is the EdgeDescription layer's) and its banked promise
(`:13-18`). Signature:

```rust
pub fn interior_iso_u<T: SpanLocate>(s: &NurbsSurface<T>, u: T)
    -> Result<NurbsCurve3<T>, IsoRowError<T>>
```

- Structure precheck, `f64` exact: case (a)/(b) above; neither →
  `IsoRowError::WeightsNotSeparable { .. }` (new variant, data only).
- `let su = u.locate_spans(s.knots_u())`; per nonempty span in `su`
  (`knots_u().span(i)`, the evaluator's skip): `basis_funs(s.knots_u(),
  span, u)`, `λ_i` per case, `Q_j = Σ_i λ_i P_{(span−p+i) j}`; hull the
  rows across spans with `enclosure_hull` per coordinate (point scalars
  see one span).
- Wrap `NurbsCurve3::new(s.knots_v().clone(), q, weights)` with row 0's
  weights (a) or all `1.0` (b); `SplineError` → `IsoRowError::Structure
  { source }` (`:167`'s spelling).
- Total on the arithmetic (a poisoned `u` poisons the row, D4); no
  comparison on `T` — the only comparisons are the weight test on `f64`
  structure and the sealed locator.
- `iso_boundary_row` (`:145-175`; consumer `replace_face.rs:1484`) keeps
  its behaviour: it re-states a domain-end float, which an interior `u`
  has none of. Its `Interior` doc and the module docs lose the "not
  built" sentences. `boundary_iso_u/v` untouched.

### 2. The certification arm — `pcurve_cache.rs`, `run_iso_checks` check 4, seam class

`side_of` (`:3738-3764`) becomes a boundary DECIDER:
`Result<Option<(bool, T)>, PcurveCertifyError>`, `None` = definitely
neither boundary (both `pcurve_iso_boundary` verdicts definite and
non-zero; escalations still escalate). This is `side_pick`'s
`e58df98d7` refactor transposed, for the same reason: the shared text
named no class. Its four call sites decide for themselves:

- **seam class** (`:3882-3890`): `Some((end, slack))` → `boundary_iso_u`,
  slack as today; `None` → `interior_iso_u(payload, u_start)`,
  `slack_u = du_extent.value()` (drift only). Both feed the SAME
  shared-space check and hull; the weights half of that check reads the
  row the extractor wrapped (case (a): `== w_0j`; case (b): carrier
  weights constant), and for a boundary row is the existing bitwise
  comparison. `WeightsNotSeparable` → `IsoUnsupported { what: "an
  interior column of a chart whose weight net varies in both directions
  — the collapsed row's weights are computed, not structure, so no
  carrier shares its rational space; the composite route is banked" }`.
- **cap class** (`:3952-3960`): `None` → `IsoUnsupported`: "a LINE cap
  rim on an interior ROW — the exact class certifies interior COLUMNS
  (the seam class) only; the cap class's interior row arrives with its
  first minting construction".
- **arc class**, fixed channel (`:3513-3521`) and moving-channel start
  (`:3667`): `None` → their own texts (interior row; a `u`-start on
  neither domain end is not a full-domain traversal).
- `EnvelopeStatement::MapResidualIsoHull` keeps its name; its doc
  (`:971-989`) states both rows (copy or collapse) and the enclosure
  argument. No new statement: the sup bounded is the same, by the same
  hull.
- **Ledger** (`docs/predicate-dimension-audit.md:310,312`): no new
  metered name. The interior branch is reached on two DEFINITE
  `pcurve_iso_boundary` verdicts (m) and its hull enters
  `pcurve_envelope` (m). Each row's prose gains one clause naming the
  collapsed-row hull; the header's contract is met with zero new rows.

### 3. The derivation arm — `crates/topo/src/pcurves.rs`, wall–seam arm (`:553-593`)

Restore `3c219381b`'s removed lines: `side_pick(&column, &[cu0, cu1])`
→ `None` → `derive_chart_foot(carrier.eval(t0), surface, half_edge)?`
(`:914`, `PcurveFittedLane::chart_foot`, one certified sample) →
`side_pick(&column, &[T::from_f64(foot.x)])` → `Zero` mints
`IsoLine { p0: (u, p0.y), pl: (0, pl.y) }`. Fall-throughs, each with
its own text (the `e58df98d7` rule):

| after                                      | outcome |
|--------------------------------------------|---------|
| boundary pick `Zero`                       | mints the boundary column (unchanged) |
| no boundary, foot `None` (`Dual`: no lane)  | `no_boundary()` verbatim — a dual body answers what it answered before |
| foot refuses (`FootPointInconclusive`)     | that `FittedCertificate` refusal propagates (unchanged door) |
| foot `Some`, metre check `Zero`            | **mints the interior column** (new) |
| foot `Some`, metre check definite non-zero | NEW text: the start has a chart foot but the neighbour's `v` map does not place it there — the two walls do not share the seam's parameterization |
| any `side_pick` escalation                 | `Escalated`, as today |

No snap of the measured `u` to a knot: the certifier collapses at the
value it is handed and any error in it is metered by the hull (D4 ¶1).
`Intersection` arm (`:761-853`): **untouched** — `9v1` keeps `General`
at the Fitted grade;
`r1_a_partial_column_restatement_takes_general_and_certifies`
(`r1_p2_probes.rs:553`) keeps its teeth both ways. Cap-rim, arc-rim
and catch-all arms untouched. The `:561-588` comment block is replaced
by the invariant (the fixed channel is collapsed at certification; the
derivation measures only its position).

---

## Widening vs new variant — census, ruling

Every non-test `Pcurve::IsoLine` match in `crates/*/src` (grep
`Pcurve::IsoLine\|IsoLine {`, 2026-09-04): `pcurve_cache.rs` ×10
(`eval :404`, `chart_box :476`, `shift :562`, `certify :1866`,
`recertify :2046`, checks, docs); `topo/pcurves.rs` ×6 (mints);
`topo/chart_region.rs:2409` (straight by variant, endpoints only);
`topo/replace_face.rs:1425` (extraction via `iso_boundary_row`), `:1777`
(transport); `topo/props.rs:1563` (admits both iso classes, then
requires every boundary vertex on the chart rectangle at `:1217` — a
RUNTIME check, TRIM-2's, which an interior column now reaches and
refuses typed as filed); `mesh/trimmed.rs:979,993` (admits `IsoLine`;
the trim walk is TRIM-2's); `mesh/chords.rs:524` (`|pl|` as UV speed);
`geom-brep/description.rs:294` (the `iso` mint door);
`geom-brep/certify.rs:2621` (a D2 bit pin);
`step-import/recognize_curve.rs:35` (doc). **No consumer reads
"boundary" off the VARIANT**: the hypothesis lives in check 4 via
`side_of`, plus TRIM-2's region-shape refusals, which are about the
loop. **Ruling: `Pcurve::IsoLine` gains interior columns; no new
variant.** A variant would duplicate every site above to say nothing
new, and would turn P-2 §7's principle (an exact description beats a
fitted one) into a type-level fork for a property the certificate
already records.

---

## Fences

- `crates/mesh/*`, `crates/topo/src/props.rs`: OUT (TRIM-2's, by
  announced seam with S-MESH and Track M). This unit REPORTS which of
  the six filed refusals the P-2 body reaches (row A3) and edits none.
- `crates/geom/src/*`, `crates/geom-core/src/*`: S-CERT's ground; read,
  not touched (premise 3). A primitive found missing is STOP 1.
- `topo/src/replace_face.rs` not edited: an offset of a chart carrying
  an interior-column DESCRIPTION keeps refusing `IsoRowError::Interior`.
- The `Intersection` arm, `certify_general`, the `IsoArc` class (beyond
  the `side_of` shape and its texts), `Pcurve`'s variants,
  `PcurveFittedLane`'s methods and `EnvelopeStatement`'s variants do not
  change.

---

## Riders and residues

- **S394 — CARRY.** All three sites (premise 6) are on lines this unit
  rewrites (every `side_of` caller). One variant
  `PcurveCertifyError::ChartRow { source: SplineError }` with a Display
  arm; `SplineError` is not generic, so the prose census
  (`pncad-py/src/prose_census.rs`) needs no row.
- **`fitted-magnitude-nan-schedule-parameter` — CARRY.** The `t:
  f64::NAN` site is `chart_foot_lane` (`pcurve_cache.rs:1232-1250`), the
  door the wall–seam arm now calls for every interior seam, so its
  refusal becomes reachable from a seam derivation. The item's second
  option: `FittedMagnitude::EndpointFootDistance { last_distance }` and
  its Display arm. ~15 lines in a file already open.
- **D36, S83, D305 — LEAVE.** 22 sites across crates not opened; `ssi.rs`
  not opened; a census disposition whose `pcurve_cache.rs` entry is the
  arc class, untouched.
- **Residue this unit files in `work/trim/`** (README: a disclosed
  residue gets its file in the disclosing PR): the non-separable
  rational interior column — the composite-bound statement without a
  tube. Not scheduled.

---

## STOP conditions (pre-registered)

1. The collapsed row cannot be formed from `basis_funs` +
   `locate_spans` + `enclosure_hull` at `Interval` (e.g. a straddling
   `u*` needs a hull the sealed trait does not expose). Then a `geom`
   primitive IS needed: STOP; name the one function, its file and its
   tests as an announced seam into S-CERT's ground; do not build it.
2. The re-sweep of the SHAPE (`IsoLine`, `IsoArc`, `MapResidualIsoHull`,
   `side_of`) at the merge base finds a consumer that reads "boundary"
   off the variant. STOP and report.
3. The P-2 body has a SECOND blocker after `16v1` mints (UNMEASURED,
   probe note): `mint_pcurves` refuses elsewhere, the loop walk refuses
   (`pcurve_loop_continuity`, `pcurve_loop_closure`), or
   `validate_pcurves` reports on the widened face. Row A3's own
   assertion is the instrument; if it fires, STOP, quote the payload
   and the raising site, and do not widen a second arm in this unit
   (the `e58df98d7` ruling).
4. Row A1 cannot reach envelope `≤ 1e-12` at the `interval` lane on a
   genuine convex-combination row of a millimetre fixture. That is a
   measurement about enclosure width: report the number; never scale
   the fixture to pass.

---

## Acceptance rows — red first, each naming the mutant it kills

Every row asserts a DEFINITE outcome at every cell of `{1e-6, 1e-9,
1e-12}` with no ε-conditional return (P-2 §5). Fixtures are static
witnesses (test-suite-cost). New geom-brep rows go in one module
aggregated by `tests/all.rs`; sweep rows are edits.

**A1 — the certifier's door, a genuine collapse (NEW,
`geom-brep/tests/interior_iso_column.rs`).** A degree-2 × degree-1
polynomial chart with interior `u` knots at thirds, millimetre scale;
carrier := `S(u*, ·)` at `u* = 0.5` (strictly between knots) derived
INDEPENDENTLY of the extractor — `insert_knot_u(0.5, 2)`
(`nurbs.rs:839`, evaluation-invariant in ℝ) and the exposed control row
at Greville abscissa `0.5`. `PcurveCache::certify(IsoLine { p0: (0.5,
0), pl: (0, 1) }, …)` certifies with `MapResidualIsoHull` and
`envelope <= eps` at every cell (`.hi()` at the `interval` lane); also
`envelope < 1e-12` absolutely, so no cell passes on a loose ε.
**Mutants:** collapse at the wrong parameter (nearest knot instead of
`u_start`: hull = row-to-row distance, mm → refuses); the boundary row
for an interior column (today's behaviour → refuses); `λ` without its
normalisation (invisible on a polynomial chart — which is why A2 exists).
**A2 — rational case (b).** dm1's wall copied as a fixture
(`imported_chart_arc_rim.rs:29-66`), `u* = √3/2` (mid-span, the
weight-½ column fully live); carrier: the meridian segment as a
degree-1 polynomial `NurbsCurve3` from `S(u*, 0)` to `S(u*, H)`
evaluated by the chart. Certifies, `envelope <= eps` at every cell.
**Mutants:** `λ_i = N_i` without the weights (the point leaves the
cylinder by O(R·(1−w)) ≈ mm → refuses); wrapping the row with the
chart's weights instead of `1.0`. **A2b:** one weight perturbed so the
net is non-separable → `IsoUnsupported` naming the weight net, at every
cell (structural, definite).
**A3 — the P-2 body validates at rest (EDIT,
`m8_4_intersection_iso.rs::an_interior_column_intersection_mints_a_general_image`,
`:463-577`).** Its final `match mint` (`:543-573`) PANICS with "the rim
arms learned to read a trimmed chart — good news… fold the mint back"
the moment `mint_pcurves` succeeds: red-first by its own text. Fold:
`mint_pcurves` is `Ok(())`; the bowed face's cache set is complete;
`body.pcurve(16v1)` is `IsoLine` with `pl.x == 0.0` and `p0.x` within
`1e-9` of `1.0`; `body.pcurve(9v1)` is still `General` (no downgrade);
`validate_pcurves(&body, band)` is EMPTY. Then assert which of the six
filed refusals `topo::mass_properties` and `mesh`'s tessellation
actually raise on this body (`mesh` is a sweep dev-dependency), quoting
the payload — TRIM-2's opening measurement. Keep `INTERIOR_COLUMN_SCALE`.
**Mutants:** minting the boundary column for `16v1` (residual mm → the
mint refuses); a certifier that still refuses interior (red as today);
any second blocker (STOP 3, visibly).
**A4 — the derivation pin flips (EDIT,
`r1_p2_probes.rs::r1_wall_seam_arm_still_refuses_the_interior_column`,
`:470-510`).** Re-expressed positive: every `Chart`-described
spline-carrier half-edge on the widened face other than the seam
derives `Ok(IsoLine)` with `pl.x == 0.0` and `p0.x` strictly inside the
`u` domain, and `PcurveCache::certify` of it against the face's chart
certifies at every cell. Rename to what it now pins. **Mutant:** the arm
re-offering `cu0`/`cu1` (`posture()`, `m8_4…:249-284`, still demands a
domain end on UNWIDENED charts — unchanged).
**A5 — `Dual` answers what it answered before.**
`r1_dual_scalar_still_reaches_the_mint` (`:517`) untouched, plus one
assertion in A4's file: at `Dual64` the wall–seam arm on the widened
body still refuses with `no_boundary`'s text (foot `None`). **Mutant:**
a lane that fabricates a foot at a scalar with no certified projection.
**A6 — nothing else moved.** `an_interior_column_still_refuses`,
`a_seam_column_certifies_on_a_non_unit_chart`,
`an_adopted_iso_column_is_a_knot_domain_end` (P-2 §6),
`r1_a_partial_column_restatement_takes_general_and_certifies` and
`m6_loft_body.rs:84` are not edited and stay green.

---

## PR shape, difficulty, task class

**One PR.** Mechanism in three files (`nurbs_iso.rs`, `pcurve_cache.rs`,
`topo/pcurves.rs`); two test files edited, one added; the two riders;
the ledger prose. Order inside the PR: extractor + A1/A2 red→green
(testable without a body), then the `side_of` shape and the seam arm,
then the derivation arm and A3/A4.

**Difficulty L, argued.** Against: one convex-combination row and a
hull argument already in the tree; the derivation code exists in git;
no `geom` primitive. For: the `side_of` shape change touches four
certifier sites, each owing an honest text; the rational class table is
where a subtle wrong (a too-clever bitwise test, `λ` without weights) can
hide — A2's mutants exist for exactly that; and A3 is the first
whole-body mint of a trimmed chart at rest, where STOP 3 may fire. L at
the high end; if STOP 3 fires it is M and a second unit.

**Task class NUMERIC.** The deliverable is metered quantities certified
at three ε cells and two scalar lanes; the structural edits serve them.

---

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. Measure first: run
A3's fixture through `mint_pcurves` before any edit and quote the
`16v1` payload and raising site in the PR body. Own `CARGO_TARGET_DIR`
outside the worktree. Hosted CI at all twelve `test (…)` jobs is the
verification of record, and no narrowing dispatch. (This clause used to
forbid a `CI-Config` lane/eps trailer; that spelling was deleted on
2026-09-04.) The census
above is as of `f23d373d5`: re-sweep the SHAPE at the merge base and
put the hit list in the PR body (§5). Findings outside the fence go in
the PR body, not another program's slate; the residue above gets its
file in `work/trim/` in the same PR. Merge origin/main before opening;
watch CI to completion in the foreground; do not merge — full v6 dual
review per `work/trim/plan.md`.
