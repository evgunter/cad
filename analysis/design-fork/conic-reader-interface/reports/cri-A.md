## For Ev

**Recommendation (likely).** Give the conic one public reading of where it is, its **stored form**: `Conic::stated() -> Stated<T>`. It holds the point the conic stores (its *base*), the stored unit directions and semi-axes, and the parameter's **offsets** from the base, `(κ(θ), σ(θ))`, so that
`P(θ) = base + u_ref·(a·κ(θ)) + v_ref·(b·σ(θ))`.
On a centre-held conic the base is the centre and `(κ, σ) = (cos θ, sin θ)`. On an arc-held one the base is the anchor and `(κ, σ) = (cos θ − cos θ₀, sin θ − sin θ₀)`, formed without cancellation. Which of the two a conic is stays private: a reader never sees the phase, only the base, the vectors and the offset functions.
- `Conic::point_at` and `running_point` become this form's evaluation. They already have exactly this shape on both placements, so the bits do not move.
- Every reader of position, on f64 and on `Interval`, states what it reads as a polynomial in `(κ, σ)` about the base.
- `Conic::frame` and `Conic::center` leave the public surface.
- A conic made from a conic is made by a map in `conic.rs` that carries the stored form.
- A circle's centre, as a datum, is handed out only by a circle door named for it.

**Premise check: the problem is the harmonic, not the centre read (sure, by arithmetic).** The question as asked is "what interface lets the readers reach the position". The ~30 certified-lane readers do not lose digits because they *read* a derived centre. They lose them because they write their residual as a harmonic about the centre, `c₀ + c₁cos θ + s₁sin θ + c₂cos 2θ + s₂sin 2θ`. For a far-centred arc those coefficients are huge and cancel wherever the result is decided, even if the centre were exact.

Worked example: a cone section with `a = 1e6 m`, `b ≈ 1e3 m`, against a 1 m sphere beside its anchor.
- Today `quadric_harmonics` forms `c₀ ≈ |C − o|²/2r ≈ 5e11`, which carries about `1e-4 m` of rounding against an ε of 1e-7. The noise meter (`terms = (|C − o| + a)²`) sees this, so the reader refuses rather than answering wrongly.
- The same quadric in offsets about the anchor has coefficients `F(A) ~ 1`, `∇F·(a û) ~ 1e6` and `a²|⊥û|² ~ 1e12`. Near the arc they multiply `κ ~ φ²/2 ~ 5e-7`, `σ ~ φ ~ 1e-3` and `κ² ~ 2.5e-13`, so every term and its rounding sits at the arc's scale.

So handing these readers a clean centre, or adding a `Conic` method per reader shape, buys nothing on the cases the arc-held form exists for. They are owed a **re-posing** in the README's own sense ("a formula that loses digits where its result is decided is re-posed"), about the stored point. That is one answer for all of them, not thirty.

**The families, re-sorted (likely).** Of the six groups in the census, three are not families at all:
- **The symbolic scalar** (`swept.rs::register_placed_carrier_end`) rebuilds a point with `Curve3::circle_at(&circle.frame(), r, θ)`, which by its own doc is the node the certifier evaluates. It should call the carrier's own evaluator. Nothing is left to design.
- **The cone section's chart image** (`derive_class_zero`, `pcurve.rs`, `certify.rs`) is a position reader. It should be built from the base's chart coordinates (the anchor's azimuth and height on the cone, which are well-conditioned) plus the offsets. That is what `work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md` already proposes, and it is family 1.
- **Comparing two conics** (`injective::one_conic`, the mesh pair) and the chart-frame readers are a comparison of stored content (`Curve3::same_content` and the stored-data walk) and direction reads (`axis`, `u_ref`). Some direction reads still go through `Curve3::frame` (`section_loops::whole_conic_principal` reads `u_ref` that way), so the census's 112 direction reads undercount.

What remains is three real answers:
1. **Position readers** (the harmonics, distances about an axis, the torus residual, extents, covers, the chart image): through `Stated`.
   - A linear reading `(P − o)·n` is `k + p·κ + q·σ`, and `Stated` bounds it over a window. That is today's `offset_extent`, moved to `Stated`, so `extent_along` and `cover_over` become sugar over it.
   - Quadratic and quartic readers (`ConicHarmonics`, `conic_torus_harmonics`, `conic_axis_distance_floor`) form their coefficients in the `(κ, σ)` basis.
   - Bounds over a window come from enclosing `(κ, σ)` over it, which is one sine each, so it is tight on `Interval`.
   - The half-angle root ladder takes its expansion from the base. On the arc, `κ = −2t²/(1+t²)` and `σ = 2t/(1+t²)` with `t` the half-angle about the anchor, so `F·(1+t²)²` is still a quartic. This part is **unsure**: it is a real rework of `half_angle.rs`'s frame and has not been built.
2. **Rebuilding a conic from a conic**: maps on `conic.rs` that carry the stored form, never `Circle::new` or `Ellipse::new` fed another conic's centre.
   - What exists stays: `mapped`/`placed` (isometries), `reversed`, `seamed_*`, `map_scalar`.
   - Add `scaled_about(c, k)`, a homothety: base to `c + (base − c)·k`, sizes times k, phase kept. It serves `replace_face::homothety`.
   - Add `Circle::concentric(r′)`: the base moves radially by `r′ − r`, and on a centre-held circle it stays put. It serves the cylinder and cone transports, with `mapped` for the slide.
   - Constructors then take only a construction's own data.
3. **Datums**: covered in its own section below.

**Why the reader never sees the phase (sure).** You asked for (2): the anchor can later move to a general point of the arc without each reader changing. With the general anchor, `(κ, σ)` gets new formulas, e.g. `cos θ − cos θ₀ = −2·sin((θ + θ₀)/2)·sin((θ − θ₀)/2)`. Readers are written in `κ` and `σ`, so none of them changes. Your rule that "nothing outside asks which point a conic holds" is kept.

**What it makes true.**
- (1) Every position reader starts from the stored numbers, on both placements and both lanes.
- (3) No public door hands out an ellipse's centre, and a circle's centre comes only from a door named as a datum.
- (4) Rebuilds keep the placement by construction.
- **What it leaves possible:** a reader can still expand the offsets back into cos and sin by hand and cancel. Only review catches that, as it catches any badly posed formula today.
- **Reversible:** yes. `Stated` is a view of what is already stored.

**Alternatives weighed.**
- **(B) One `Conic` method per reader shape**, as PR 7510 did for `cover_over` and `extent_along`. This moves about thirty readers' algebra into `conic.rs`. Each method still needs the phase inside it, and there is a new method each time a reader appears. Rejected.
- **(C) Make the placement public** (the frame at the stored point plus `Held`). Every reader then matches on the placement, which breaks (2) and your "nothing outside asks" rule. Rejected.
- **(D) A local form about a window's middle point** (`P(θm)` plus the conjugate diameters there). It suits the root ladder, but it restates the curve through a rounded `P(θm)`. That is a rounded copy, against "no reading replaces it with a rounded copy". Rejected.

**Datums: what a reader that really wants a centre is handed (unsure).** Coaxiality, rim reads and the STEP import want a circle's axis of revolution.
- Hand them `Circle::axis_line() -> Line`: the stored centre and axis.
- Answer it only on a centre-held circle, and **panic on an arc-held one**. A construction holds the arc exactly where the centre runs off, so reading that centre as a datum is choosing the wrong form, which is your "kernel bug if we choose the wrong one".
- The ellipse gets no centre door: none of the reads sampled wants an ellipse's centre as a datum.
- When the STEP import mints arc-held circles, its datum reads must be posed through the arc ("this arc lies on that cylinder" is a quadric reading). That is the honest question for a far-centred rim anyway.
- Alternative: a typed refusal instead of the panic, if a user file can reach it. I lean to the panic while no arc-held circle exists.

**Final public surface of `conic.rs`.**
- **`Conic`:** `axis`, `u_ref`, `v_ref`, `semi_axes`, `stated`, the evaluators (`point_at` defined through `stated`), `azimuth`, the speed and curvature bounds, `major_frame`, `param_in`, `along`, `unit_of`, and `extent_along`/`cover_over` as sugar.
  - **Removed:** `frame`, `center`, and `unit_in_plane`, which becomes `Stated::offsets_of(p)`.
- **`Stated`:** `base`, `offsets`, `running_offsets`, `offsets_of`, the windowed linear extent, and the half-angle form.
- **`Circle` and `Ellipse`:** lose `frame` and `centred_at`. `centred_at`'s only callers are tests, so today it is non-test code that exists for tests.
- **`Circle`:** gains `axis_line`, `concentric` and `scaled_about`.
- **`Curve3::frame`:** stops answering conics.
- The clippy list shrinks to `Circle::axis_line`, each datum reader carrying an expectation and its reason, or it goes entirely if the panic holds the line.

**Ratified text I would change (§2).** The clause "Every reading of where a conic is goes through the conic" was agent-written and approved in passing in PR 7258 ("ok sounds good!"). Changing it needs your sign-off, since it binds future work.

Proposed README text, replacing that sentence:
> Every reading of where a conic is is stated from the point it stores: the conic hands its stored form (that point, its semi-axis vectors, and its parameter's offsets from that point), and a reader poses what it reads in those offsets, never as harmonics about a centre the conic does not store. A centre is a datum only of a conic held at it.

The module doc's "Where a conic is held" changes the same way: "a position is read as the centre (`Conic::center`, the origin of `Conic::frame`)" becomes "a position is read through `Conic::stated`, whose evaluation every evaluator is".

**Confidence.**
- One answer (`Stated`) serves every position reader: likely.
- The harmonic basis, not the read, is where the digits go: sure.
- The root ladder can run in the offsets: unsure.
- The datum door panics on an arc-held circle: unsure.
- Rebuilds as maps: likely.

## For the orchestrator

- **What I read.** `designer.md`, the README's Float rounding section, `conic.rs` in full except its tests, the gate's `.counts` and its header, `clippy.toml`, and these readers: `implicit.rs` (the eight listed functions), the `conic_quadric.rs` module doc, `replace_face::{transport_curve, homothety}`, `swept::register_placed_carrier_end`, `mesh/sizing.rs::sphere_face_widest_cos`, `section_loops::whole_conic_principal`, `injective::{conic, one_conic}`. Also the pcert row and fork-log row 296.
- **What I did not check:**
  - the remaining roughly 120 listed functions; my re-sorting of families 2, 3 and 5 is from samples;
  - whether any production reader wants an ellipse's centre as a datum (I found none);
  - `half_angle.rs`'s frame in detail.
  - I built nothing and measured nothing. The worked example is arithmetic.
- **Assumed:** the ladder's expansion point is free to choose per placement. If `SubdivisionFrame` fixes it at the seam, the half-angle part of family 1 is a larger rework of the ladder.
- **Errors in the brief or census:**
  - "two statements of one circle compared by bit (`mesh/sizing.rs`)": the listed function there, `sphere_face_widest_cos`, is a linear height harmonic (`harmonic_range` from `frame().origin()`), i.e. `extent_along`.
  - The direction-read count misses reads made through `Curve3::frame` (`whole_conic_principal`).
- **Defects found off the question, worth rows:**
  - `Circle::centred_at` and `Ellipse::centred_at` are public with only test callers (`pcurve_cache.rs:13550`, conic.rs tests).
  - `register_placed_carrier_end` evaluates through `Curve3::circle_at` over a read frame instead of the carrier's own evaluator.
  - `injective::one_conic` compares conics frame by frame, not through `same_content`, and ignores the placement.
- **Measurement that would settle the main claim before Ev rules:** a far-centred arc-held section (`a ≥ 1e5`) against a sphere beside its anchor through `conic_quadric_roots`. Today I expect a noise refusal; with `ConicHarmonics` formed in offsets, a certified verdict.
