# PROPS mignitude-floor — `‖E‖` bounded below through the sign witness, not componentwise

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Lanes,
the offset_fit lane's second unit; the item is
`work/props/offset-fit-mignitude-floor-on-norm-e.md` — read it in
full; difficulty logged at spec: **H / NUMERIC**, block PROPS-B2 slot
2, dual review). Read `docs/prompts/implementer-discipline.md` in full.
Branch `props/mignitude-floor`, cut from `main`.

## The defect, measured (CERT-7, issue 1320)

`Composite::cell_bound` (`crates/geom-brep/src/offset_fit.rs:~1733`)
bounds `‖E‖` from below by assembling the componentwise mignitudes of
`Ẽ`'s three cell hulls (`e_mig_sq`, `:~1767`). On a patch whose normal
rotates across the cell every component of `E ≈ d·n` straddles zero,
so the assembly collapses: at `d = 1e-6` on the quarter cylinder the
sup cell reads `e_lo = 1.581e-8` where `‖E‖ ≈ |d| = 1e-6`, and the
certified bound is 96% its `τ²/‖E‖` term (`3.103e-4` of `3.222e-4`).
The module doc (`:~136-146`) already says this is not a rounding
problem and that "a lower bound that saw the components together
rather than one at a time is what would move this row". That bound is
this unit.

## The construction

`|E·n| ≤ ‖E‖` for any `E`, and the composite already carries
`D = Ẽ·M̃ = w̃·w³·(E·m)` as the sign witness, so

```text
‖E‖ ≥ |E·n| = |E·m| / ‖m‖ = |D| / (w̃ · w³ · ‖m‖) = |D| / (w̃ · ‖M̃‖)
```

with `M̃ = w³·m` the composite's own homogeneous normal. A lower bound
on `|D|` is the cell hull's mignitude, which is POSITIVE on every cell
that passes the sign witness (the witness is `D` definite); an upper
bound on `‖M̃‖` is the sup of the cell hulls of `M̃`'s three channels
(`sqrt_up` of the summed squared magnitudes, the shape `y_mag`
already uses). The composite computes `M̃` in `build` on the way to
`Y` and `D` and discards it; keep its three channels as `m: [PatchSpans; 3]`.
Then

```text
e_lo = max( mig-assembly (today's), mig(D) / (w̃_hi · sup‖M̃‖), |d| − dist )
```

— the max of three sound lower bounds is sound; the new middle term is
the one that reads `‖E‖ ≈ |d|` where `E ∥ n`, which is exactly the
sup cell's situation. It feeds BOTH places `cell_bound` uses a lower
bound on `‖E‖`: the divisor of `dist` (`e_lo_iv + |d|`) and the
`τ²/‖E‖` term (`e_floor`). Every intermediate stays in the ring; `.hi()`
read once at the end, as now.

What the module doc's `‖R‖` decomposition needs of `‖E‖` is unchanged;
no side condition is added or removed. A cell whose `D` hull has
mignitude zero cannot pass the sign witness anyway, so the new term
never divides by a zero the witness did not already refuse on.

## Deliverables

- `Composite` carries `M̃`'s channels; `cell_bound` takes the max of
  the three lower bounds; the module doc's "what it did not buy"
  paragraph (`:~136-146`) is rewritten in the present tense for what
  the bound now reads (discipline §4 — no history).
- **Red-first, the micron row**: `crates/geom-brep/tests/offset_fit.rs`'s
  `a_micron_scale_offset_certifies_and_names_its_limit` (quarter
  cylinder, `d = 1e-6`): today the `1e-9` arm refuses
  `SampleCapReached { achieved: 3.79e-7 }` (post budget-faces), and
  the sup cell's `e_lo` reads `1.58e-8`. Quote the red; then the
  measured green — the row asserts the new `e_lo` at the sup cell
  within a factor of 2 of `|d|` (the `r2_probe_micron_sup_cell_decomposition`
  instrument at `:~1800` is the model; make it a row, not an
  `eprintln`), and whatever the `1e-9` arm now does: if it certifies,
  the row asserts the certificate; if it still refuses, the row
  asserts the new `achieved` with the per-term decomposition showing
  which term now carries the sup and why. State the factor the bound
  moved by at the first grid and at the certified/refused grid.
- **Every certificate tightens or holds**: `hull_sup` can only go down
  (a max of lower bounds), so every row that pins a certified bound's
  digits (`offset_fit.rs`, `offb_r2_probes.rs`, `cert7_*_probes.rs`,
  `approx_surface.rs`, `census_g2_carrier.rs`, the sweep crate's rows
  — `grep -rn "hull_sup\|achieved" crates/*/tests`) re-baselines with
  its old and new digits and the reason in the PR. That is the
  deliverable, not a cost (discipline §3). A row whose bound got
  LARGER is a MAJOR finding: stop and report it.
- **Face changes measured**: budget-faces' four faces are on main;
  report which rows change face (a cap stop that now certifies; a
  `BoundNotFinite` that now has a finite bound), with the reason.
- The D2 addendum: none — the admission set only grows where the bound
  tightens, and every admitted input is still certified by the same
  decomposition; say so at the enum in one sentence if a face row
  moves.
- Sweep obligation (discipline §5): the shape is *a lower bound on a
  vector norm assembled from componentwise mignitudes where a
  dot-product witness of the same vector is already in hand* — every
  `mig(` caller in `crates/geom-brep/src/{offset_fit,offset_meters,patch_bound}.rs`
  and `crates/geom-core/src/ring_interval.rs`'s consumers, classified:
  the component straddles zero on a rotating normal (this unit's
  shape — list, fix only inside `offset_fit.rs`), or a genuinely
  scalar mignitude (keep). `offset_meters`' meter 1 uses the same
  inf-side shape on the cross product `‖S_u × S_v‖` — read whether it
  has a witness to read together; if it does, file, do not fix (the
  regularity floor is its own unit).

## Seams

`crates/geom-brep/tests/offset_fit.rs` is tcost's (consumer rows only);
`crates/topo/tests/census_g2_carrier.rs` names certificate digits
(topo's — re-baseline only). Nothing else outside PROPS's paths.

## Posture

- ε posture: none — no tolerance read moves; say so. No `CI-Config:`
  trailer (inert); no empty commits.
- Bit identity: **every certificate's `hull_sup` changes by design**
  (tightens); the receipt is the re-baseline table with old and new
  digits per row and the factor at the micron row. The fit itself
  (structure selection, knots, control bits) is untouched: state that
  the fitted surface's bits are identical before and after (`r2_bytes`
  or the module's own bit test).
- Review: dual, in the experiment.
- **Landing: the item gets `pr:` and `status: review`. DO NOT MERGE —
  the orchestrator lands after the dual and its fix pass; the fix pass
  closes the item, deletes this spec at merge with its
  `## Per-merge deletion` section in `docs/DOC-LEDGER.md`.** No
  `Co-Authored-By`; push early to `props/mignitude-floor`.

## Acceptance

`e_lo` at the micron row's sup cell within a factor of 2 of `|d|`; the
`1e-9` arm's outcome measured and pinned with its decomposition; every
certificate row re-baselined tighter or unchanged with its digits; the
fit's bits unchanged; the sweep's hit list filed; hosted CI green on
the full matrix.
