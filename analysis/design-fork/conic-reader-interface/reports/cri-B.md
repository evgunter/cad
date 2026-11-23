## For Ev

**Recommendation (likely).** A conic hands out one reading of where it is: its **position**, `P(θ) = X + U·κ(θ) + V·σ(θ)`. Here X is the point the conic stores, `U = a·u_ref`, `V = b·v_ref`, and `(κ, σ) = (cos θ − c₀, sin θ − s₀)` is the unit circle's offset from the stored point's phase `(c₀, s₀)`. The phase is `(0, 0)` for a conic held at its centre and a unit pair for one held on its arc. The phase and the way κ and σ are evaluated stay private to the conic. The kernel names **one trigonometric polynomial along a conic** (`geom::Trig`, written in the same basis about the same stored point). Readers compose their reading with the position through Trig's own arithmetic (dot, part across an axis, square, product) and then solve, bound or enclose it through Trig's own operations.

The rest follows from that:
- `Conic::frame` is deleted.
- `Conic::center` is replaced by a datum door that returns a `Centre` type, which cannot be added to a vector.
- No "rebuild door" exists: a conic made from a conic is a map of it.
- When the doors are gone, the clippy list, the `#[expect]`s, `scripts/gates/conic-centre-reads.sh` and its `.counts` all retire. The type does their job.

### Premise check (sure unless marked)
1. **The ~30 harmonic readers do not each do their own arithmetic. They all do the same thing.** Each composes a polynomial in position with the conic and writes the result as Fourier coefficients about θ = 0, then hands those to a solver (`ConicHarmonics`, `ConicTorusHarmonics`, `CircleSphereHarmonic` in `implicit.rs`; `Trig`/`square`/`trig` and `Harmonic` in `topo/injective.rs`; `Harmonic` in `carrier_net.rs`; `ArcWindow::extent(c, p, q)`; `harmonic_sup`).
   - A Fourier constant is the polynomial's mean over the turn, which is the reading of the centre. So a polynomial written that way is centre-shaped wherever its numbers came from.
   - The flaw is therefore one level down. The kernel has no representation of a polynomial along a conic, and has seven ad hoc ones, all centre-based.
   - No interface on `Conic` can fix that while the representation stays Fourier. This is why PR 7510's two readings (`extent_along`, `cover_over`) each had to reach into the private placement: each was a one-off Trig.
2. **Three of the six families are not families.**
   - **The rebuild doors** are maps the conic already has: `mapped`, `reversed`, `translated`, `seamed_*`, `map_scalar`. Example: `pole_form::half_turn_curve` rebuilds a circle from `frame().origin()` where `circle.placed(half turn)` is meant.
   - **The symbolic scalar** (`swept.rs::register_placed_carrier_end`) is a restated evaluator, `Curve3::circle_at(&circle.frame(), r, t)`. It is `point_at(t)`.
   - **The bit comparison** (`injective.rs::one_stated_conic` and its kin) is `Curve3::same_content`, which already compares the placement, phase included.
3. **The module doc contradicts the README clause.** `conic.rs` says "a position is read as the centre ([`Conic::center`], the origin of [`Conic::frame`])". The README says "Every reading of where a conic is goes through the conic". Both cannot hold. This design keeps the README.
4. The census reproduces: 142 listed functions in 57 files (`.counts`).

### Ratified text
- The README sentences were agent-written (`32b2ae4ced`, `35b1559d50`) and approved by you in passing on PR 7258 ("ok sounds good!"). I keep their substance and make the last clause concrete. Replace "Every reading of where a conic is goes through the conic." with:

  > Where a conic is, is read as its position, a first harmonic held about the point it stores; a reading composed with it is a trigonometric polynomial held about the same point, never a sum about the centre. A conic's centre is a datum, read by name, and never a position.

- The "consumed extent" clause says "a datum fixed by a carrier's form (an apex, a centre) is read there". For an arc-held conic, that puts a reach's datum megametres off and makes its lever useless. It should say "the point a conic stores". (unsure: I did not trace which reaches read a conic's centre.)

### The final state of `conic.rs`
- **`Conic` keeps:** `axis`, `u_ref`, `v_ref`, `semi_axes`, the evaluators, `running_point`, `azimuth`, `param_in`, the speeds, `curvature_hi`, `major_frame`, `along` (direction-only).
- **`Conic` gains:** `position() -> Position<T>` and `centre() -> Centre<T>`.
- **`cover_over` stays as sugar**, because `Cover`'s piece walk needs the conic itself.
- **`extent_along` becomes sugar** over `position().height(origin, n).extent(window)`.
- **Removed:**
  - `frame`, `center`, and `unit_in_plane` (it is centre-relative; `azimuth` is the arc-aware reading);
  - `Circle::frame`, `Ellipse::frame`, and the conic arm of `Curve3::frame`;
  - `centred_at`, a centre used as a position. Its one outside caller is a test in `pcurve_cache.rs`.
- **`Circle::placed` is renamed `mapped`**, so both kinds have one map door.
- **Module doc:** "Where a conic is held" stays. The paragraph that says a position is read as the centre is replaced by one stating `position()` and the `Centre` datum.

**`Position` / `Trig` (likely).** Both are opaque.
- `Position` gives `height(o, n)`, `across(axis)`, `minus(o)`, and from them `norm_squared`.
- `Trig` offers:
  - `+`, `−`, `×` (degrees add) and scaling;
  - evaluation, plus a running value with its rounding bound;
  - its extent over an `ArcWindow`;
  - the jet a Taylor bound reads;
  - re-basing `about(θa)`, with its rounding charged;
  - a magnitude ledger that replaces each reader's hand-written `terms`.
- The basis is powers of (κ, σ). Because `κ² + σ² = −2(c₀κ + s₀σ)`, a degree-N polynomial has 2N+1 coefficients, as a Fourier one does: it is a change of basis, not a bigger object.
- Every operation is the scalar's own, so at `Interval` it encloses.
- It meets your four conditions:
  1. Both placements and both scalars state the stored curve.
  2. Moving the anchor to a general point of the arc is a general unit phase inside `Trig`'s κ and σ. No reader sees it.
  3. No door hands out a centre as a position.
  4. A conic made from a conic is a map, so it keeps its placement.

**Worked example (by hand, likely).** A cone section with `a = 1e6` m, arc-held at its vertex A, against a 1 m sphere near A:
- Today's Fourier constant is `(|C−o|² + (a²+b²)/2 − r²)/2r ≈ 7e11`, so its rounding is about 1e-4 m in a residual that is O(1) m near the arc.
- In the base form, the constant is `(|A−o|² − r²)/2r`, which is O(1). Over a 1 m arc, κ ~ 5e-13 and σ ~ 1e-6, so `(A−o)·U·κ` ~ 5e-7 and `U·U·κ²` ~ 2e-13. Each term rounds at its own size, and the residual holds to about u·1 m.

**The families:**
- **Certified harmonics** (`implicit.rs`, `curve_pair/pose*.rs`, `ring_path.rs`, `injective.rs`): Trig over `position()`. The torus implicit is a degree-4 Trig. The axis-distance floor is the square root of a lower bound on a degree-2 Trig. The solvers in `half_angle.rs` / `quartic.rs` read Trig's evaluations, jets and ledger, and re-base to their anchor. (likely)
- **Cone chart image** (`derive_class_zero`, `pcurve.rs`, `certify.rs`): built by the cone's chart from `position()`, its constant the stored point's chart coordinates. This is the "to try" in the pcert issue. (likely)
- **Rebuild doors:** the conic's maps. (sure)
- **Symbolic scalar:** `point_at`. (sure)
- **Bit comparison:** `same_content`. (sure)
- **Circle-leg chart frames** (`torus_region.rs`): the chart is the torus's, so read the surface's frame plus `position()`. (unsure: not traced)
- **Datum reads** (coaxiality, rims, STEP):
  - A decision posed on a centre is re-posed as a predicate on the position: "coaxial with L" means the degree-2 Trig `|⊥(P−o)|²` about L is constant (its non-constant coefficients vanish), and the plane is the same. Stated from the stored point, this is well-conditioned at any radius.
  - `Centre` is for building a new datum, such as a STEP surface's axis. It has `axis_line()` and an explicit `point()`, and no `Add<Vec3>`. For a centre-held circle (every circle today) it is the stored centre, to the bit.
  - For an arc-held conic it is derived at the semi-axes' rounding, and the type's docs say so.
  - (likely on the predicates; unsure that no datum reader needs a centre point at f64 to decide.)

### Alternatives rejected
- **(A) One `Conic` reading per family** (PR 7510's path, extended). `geom` would absorb `geom-brep`'s solvers, each reading would match on the placement, and each would change when the anchor generalises.
- **(B) Publish the stored point and its phase.** Readers would see the phase (fails condition 2), and expanding to Fourier is one line (fails condition 3).
- **(C) A derived centre with its rounding charged.** Sound, but it loses the digits the arc form exists for, and it contradicts "no reading replaces it with a rounded copy".

All are reversible. Trig is the larger commitment and the easier one to keep, because the ad hoc forms fold into it.

### Cost (kept out of the ranking)
- The certified lane's ~30 functions and the half-angle solvers are rewritten onto Trig.
- The cone lane's hand-tuned per-component levers in `conic_cone_harmonics` are re-derived by the ledger. There is a real risk that the ledger starts out looser and some rows refuse until it is tuned. (unsure)
- A centre-held conic's numbers move by product-to-sum rounding: it is still read from its centre, but not to main's bits.

## For the orchestrator

- **Not measured.** I built nothing. The worked example is hand arithmetic. Of the 142 listed functions I read `implicit.rs` (`quadric_harmonics`, `conic_cone_harmonics`), `injective.rs` (`conic`, `one_conic`, `one_stated_conic`), `on_carrier.rs` (`clearance`, `distance_within`), `pole_form.rs`, `swept.rs`, `cover.rs::arc_of`, `carrier_net.rs` and `row_contact.rs`. The family assignments for the rest are the implementer's, checked only by sample.
- **Biggest unknown:** whether `half_angle.rs`'s 32-anchor pole rule and its noise bounds can consume a base-form Trig without loss. Re-basing to a far anchor costs what the centre form costs there. I assumed the roots that matter lie near the stored point, which an arc-held construction guarantees.
- **Off-question defects to file:**
  - The module-doc vs README contradiction (premise 3) belongs to this unit's PR.
  - `on_carrier.rs::clearance`'s ellipse arm runs `ellipse_level_clearance` in centre coordinates. For a far-centred arc-held ellipse it cancels at the semi-axes' scale. It is live today for the cone section wherever clearance is read; I did not trace its callers.
  - `row_contact.rs`'s `Harmonic::of_turning` forms `a = f0 − b`, the same mean-form cancellation, on a fitted harmonic (not a conic reader).
  - `Cover::arc_of`'s piece stores the derived centre frame beside `on: Some(ellipse)`. I did not check whether anything reads that frame rather than the ellipse.
- **Brief:** accurate on the counts. "About 30 readers each do their own arithmetic" is the framing to correct: it is one arithmetic in one centre-shaped basis.
- **Provenance:** `git log --all -S'goes through the conic' -- crates/geom-core/README.md` gives `35b1559d50` and `32b2ae4ced`, both agent commits. Fork-log row 296 (2026-11-21) records Ev's approval as "ok sounds good!".
- The proposed README sentence and the "consumed extent" wording change bind future work, so that PR waits on Ev.
