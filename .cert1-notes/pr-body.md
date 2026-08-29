**S-CERT unit CERT-1.** The two sphere polar acceptance defects — issue 723 (extent from endpoints) and issue 893 (the rim lever's near-polar collapse) — both accepting-direction, both sphere-only, fixed at the arms, with the import-door reproduction committed as fixtures. (Keyword hygiene per issue 723's own comment thread: issue numbers are spelled out in this body; nothing here is an instruction to GitHub to change any issue's state.)

## Mechanism 1 — the extent (issue 723)

`curved.rs`'s sphere arms took `(lo, hi)` from `min_max` over meridian **endpoint** latitudes. A meridian's carrier is a great circle, which contains both poles, so an arc whose stored span crosses a pole reaches latitude ±1 in its interior — where an endpoint fold never looks. Executed: −47.187% certified volume at `pad = 0.0`, tier 3 green, through `import_step`; −29.29% on the rimless two-band arm (issue 723, comment 4).

**The fix is the issue's option (2)** — carry the torus's own extent derivation to the sphere — taken over option (1) (refuse pole-crossing arcs) because only (2) covers the rimless arm (no rim to hang a refusal on) and it removes the unstated premise rather than guarding it. New fn `sphere_meridian_span_levels`: from the stored anchor endpoint and the stored span, the latitude along the arc is `λ(θ) = sa·cosθ + ca·sinθ` (`ca` read off `dP/dt = n_c × (P − center)`); whether a pole's angular offset lands inside the span is decided through the funnel as **`props_meridian_pole`** — signed angular distance from the pole to the nearer span end, levered at R (a length, per D4). `Positive`/`Zero` fold the pole latitude `±√(sa²+ca²)` into the level list; the fold is continuous across the decision because a latitude is quadratic at its extremum (the two choices at a `Zero` differ by ~band²/2). The `atan2` locates one fixed point against the *stored* interval — not the forbidden wedge-unwrap shape, which is two endpoint inversions differenced.

Both sphere arms get it (flux lane and `boundary_material_sign` share `sphere_boundary`'s parse).

## Mechanism 2 — the rim lever (issue 893)

`sphere_boundary`'s rim arm minted `RimLevel::Unit(sin v, 0)`, so the level chord was the axial separation `R·|Δ sin v|`, which vanishes as `cos v̄ → 0`: two genuinely distinct near-polar rims decided `Zero` and a non-rectangular domain passed `props_rim_level`.

**The candidate the spec named first is the fix that landed** — the second channel. The rim arm now mints `Unit(sin v, cos v)` with `cos v = r_c/R` (stored data, exactly as the torus reads its pair), and the scalar-extreme lift completes a sine with its nonnegative cosine, `√(max(0, 1 − s²))` (latitudes live in [−π/2, π/2]; the clamp keeps the interval-scalar sqrt in-domain). The chord `level_coincides` meters is then the direction separation at R — ~`R·Δv`, the geodesic scale — everywhere on the sphere. The refusal fallback was not needed: the candidate survived measurement (all suites, all local ε points, no definite row flipped). No refusal is minted by this change; the near-polar staircase now refuses through the **existing** `props_rim_level`.

## Red-first evidence (each row committed red, then the fix)

At default ε, before the fix (`crates/geom-brep/tests/cert1_sphere_polar.rs`, commit e3192e0c):

- half-cap with split pole-crossing meridian arc: `ACCEPTED, area 1.137399314059163e-4 != exact 1.635432903567499e-4 (rel 3.045e-1)` — the issue's own digits;
- rimless hemisphere split at ±π/4: `ACCEPTED, area 4.442882938158366e-4 != exact 6.283185307179587e-4 (rel 2.929e-1)`;
- near-polar staircase, rims 10·escalate apart: ACCEPTED (panic: "two near-polar rims 10·escalate apart must not pass as one level") — issue 893's ask 1, the row no suite had.

All values asserted are closed forms (`R²π(1 − sin b)`, `2πR²`, typed refusal by predicate name), never regression captures. The near-polar rows derive their offsets from the run's own `Band` (`10·escalate`, `0.5·zero`), so each row holds one honest outcome at every ε of the hosted matrix; a within-band accepting control keeps the lever fix from being a blanket near-polar refusal.

## Fixtures (issue 723's reproduction, re-derived from the issue text)

`crates/step-import/tests/fixtures/halfcap/gen_halfcap.py` (the original artifacts died with their machine) writes the twins: `halfcap.step` (meridian arc split by one ordinary vertex at t = 1.0; 3 V / 4 E / 3 F, χ = 2) and `halfcap_nosplit.step` (one arc; 2 V / 3 E / 3 F). Parameters recovered from the issue's numbers: R = 10 mm, base latitude 0.5 rad — the generator reproduces the issue's exact volume `3.518158565e-7 m³` and the executed face area `1.635432903567499e-4 m²` digit for digit. `halfcap_pole.rs` imports both, requires tier 3 and certifies the exact closed-form volume at `volume_pad = 0`; `tier_gate.rs` gets the two corpus rows with censuses.

**The no-split twin's disposition changed, deliberately.** It used to refuse `VolumeUncomputable { DegenerateFace }` — an artifact of the endpoint fold seeing `lo == hi`, not a fact about the geometry: the face is an honest half-cap with positive extent. Under the span-derived extent it certifies the same exact volume as its split twin, which is the honest answer — the alarm shape the issue reported (one vertex of pure topology flipping a refusal into a different answer) is gone in both directions. Classification: the old refusal was a D2-addendum row-2 posture (valid input, lane unbuilt — DESIGN.md's addendum, re-derived at line 1255); this unit builds the lane, so the refusal retires rather than being reclassified. A pole-crossing arc and a near-polar rim pair are valid inputs and are now **served**, not refused — the only refusal in this unit's ground is the pre-existing `props_rim_level` iso-rectangle refusal, still row 2, whose honest serving alternative remains the certified-quadrature lane at the cost of a `pad > 0` enclosure instead of an exact number.

## Audit document

`docs/predicate-dimension-audit.md` (cited by target name per its own discipline): the `props_rim_level` row's verdict moves from `OK` — which contradicted its own N7 prose, issue 893's ask 3 — to `FIXED — N7 RETIRED`; note N7 itself is retired with the resolution recorded in N1's style; a new row records `props_meridian_pole` (margin: angular distance × R, m). The stale premise notes in `props/mod.rs`, `curved.rs`, `s58_iso_rectangle.rs` and `rim_dim_scale_twins.rs` are rewritten to the invariant.

## Margin populations that moved (no baseline is a target)

- `rim_dim_scale_twins.rs`'s sphere pin asserted the margin **is** the axial separation — a pin on the defect. It now pins the direction chord `2·sin(Δv/2)·R`, and its population gains a second honest cluster: a rim sitting at its own extreme records a rounding-scale second-component residual (the lift recomputes the extreme's cosine from its sine; the rim reads its own off stored data) instead of bitwise 0. The suite now asserts the sharper shape: every margin far inside the band or decisively past escalation, nothing in the ambiguity band; the scale twin compares the chord margins.
- `props_rim_level` margins on sphere faces with distinct rim levels grow (chord ≥ axial) — the direction that refuses more, never accepts more; every definite decision in the tree decided the same way (full batteries green with zero test edits beyond the pins named here).
- `props_meridian_pole` is a new recorded name (2 samples per sphere meridian arc). If the K census floor flags the roster change, the K-REPORT runbook re-derivation is the follow-up — not a geometry change.

## Sweep receipt — every `min_max`-over-levels consumer in `curved.rs`

Pattern: `grep -n 'min_max(' crates/geom-brep/src/props/curved.rs`, re-run after merging main. Six hits, one-line dispositions:

- `:161` (`boundary_material_sign`, cylinder) — safe: cylinder meridians are axial **lines**; `v = (p−o)·â` is linear along them, monotone, endpoint extremes exact.
- `:172` (`boundary_material_sign`, cone) — safe: generators are lines through the apex; the signed slant level is linear along them; a both-nappe span is refused by `props_cone_nappe` before the closed form integrates.
- `:185` (`boundary_material_sign`, sphere) — **fixed here**: reads `sphere_boundary`'s levels, which now carry the span-derived pole extremes.
- `:780` (`cylinder` flux) — safe, same argument as `:161` (same parse).
- `:921` (`cone` flux) — safe, same argument as `:172` (same parse).
- `:1113` (`sphere` flux) — **fixed here**, same parse as `:185`.

The torus does not appear because it never used `min_max` — its extent is the stored-span derivation this unit generalises. **What the pattern could not match:** any extent derivation not spelled `min_max` (the torus's `torus_ends` — audited by reading, sound; `props/quad.rs`'s quadrature bounds — out of this unit's scope and untouched); folds added after this merge; and endpoint-shaped premises outside `curved.rs` (e.g. the mesh `closing_column` assert the issue's Related section names — mesh ground, another stream's).

## C-m's three recorded questions (from issue 723's first comment)

1. **Which quadrature engine is authoritative after the fix:** unchanged — this fix lives entirely in the closed-form lane (`curved.rs`) and touches no engine in `props/quad.rs`. The closed-form lane remains authoritative for the analytic iso-rectangle inventory; the quadrature lane for its own. No engine became redundant; no consolidation was done here (deliberately — C-m is gated behind this unit precisely so it consolidates arithmetic that is now correct).
2. **Does a convergence-block change imply the same change in the other copies:** no convergence block was touched, so no implication transfers. C-m inherits the triplication question exactly as it stood.
3. **Was `QUAD2_AREA_PIECES = 64` load-bearing for the fix:** no. Nothing in this fix consumes it; the half-cap and every new row go through the closed forms, not quadrature.

## Verification

Hosted CI is the verification of record; the gate's drawn point and run id are reported in the PR conversation once the run concludes. Note the draw rule: this change is in ordinarily-named files (`curved.rs` is not interval-pathed), so the gate's compile mode is the SHA draw, not forced to the interval lane — the interval point was therefore run locally.

Local (all through `with-build-slot.sh`, worktree-own target):

- `cargo test -p geom-brep` — 261/261 at default ε (red-first: 3 rows red pre-fix, output quoted above);
- `cargo test -p geom-brep --features interval` — 284/284;
- `cargo test -p geom-brep --features probe --test all rim_dim_scale_twins` — 6/6;
- `cargo test -p step-import` — 159/159 (includes the new fixture suite and tier-gate corpus);
- `cargo test -p topo` — green (398 + doc/aux binaries);
- `cargo fmt --all --check` clean; `cargo clippy -p geom-brep -p step-import --all-targets` clean.

No `Co-Authored-By` trailer appears in any lane commit.
