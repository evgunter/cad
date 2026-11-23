## For Ev

**Recommendation (likely).** Code outside `conic.rs` asks a conic where it is in three ways only, and takes no centre and no frame from it:

1. **Its point** at a parameter (`point_at` and the jets, as now).
2. **Its point as a harmonic**: one new named value in `geom`, `Conic::harmonic()`. It states the point as `stored + U·ξ(t) + V·η(t)`: `stored` is the one point the conic holds, `U` and `V` are two vectors, and `ξ`, `η` are the placement's own two functions of the parameter (`cos t`, `sin t` about a centre; `cos t − cos t₀`, `sin t − sin t₀`, formed without cancellation, about a point of the arc). It is closed under what readers do to a position (seen from a point, dotted with a direction, sent through a chart's linear map, multiplied by another harmonic), and it answers what they then ask: value, range over a window, extremes, roots, enclosure over an interval of `t`, at every scalar. A reader never learns which placement it has.
3. **A point against it**: `azimuth`, `param_in` and the point's unit coordinates, each read from the stored point.

`Conic::frame` is deleted, with `Circle::frame`, `Ellipse::frame`, the conic arms of `Curve3::frame`, `Curve3::circle_at(frame, …)` and the public `Cover::arc(frame, …)`. `Conic::center` stays as a **datum** (below). A conic is made from a conic only through the conic's own doors.

**Premise check.** The question asked was "what interface for readers that each do their own arithmetic on the position". Three corrections:

- **The arithmetic is one thing, built many times.** Almost every remaining read forms the conic's point as centre + two vectors × (cos, sin) and then composes, bounds or solves it. The kernel has no type for that. It has at least ten private ones: `ConicHarmonics`, `ConicTorusHarmonics`, `CircleSphereHarmonic`, `ResidualHarmonics`, `FirstHarmonic`, `Harmonic3`, the `Harmonic` structs of `row_contact.rs`, `carrier_net.rs`, `injective.rs` and `pose.rs`, plus two range functions (`ArcWindow::extent`, `geom_brep::harmonic_range`). `carrier_net.rs`'s is already "P₀ + U·α(t) + V·β(t)" with the functions an enum. The trait already hands out two thirds of it (`Conic::along` gives the two amplitudes; `semi_axis_vectors` gives `U`, `V`), and each reader fetches the remaining third, the constant, from the centre. So the centre is read 208 times because the harmonic is handed out without its constant.
- **Giving readers the right constant is not enough** (measured, sure; a scratch probe at f64 against 80-digit arithmetic, not kernel code). An ellipse held on its arc with semi-axes 1e6 and 3e5 m, and a plane crossing it within 3 m of the stored point. The root's miss along the arc is:
  - 3.4e-10 m read from the derived centre, as readers do today;
  - 1.7e-10 m with the constant taken from the stored point and the root solved the readers' way (phase ± half-chord), because the root's parameter is then a difference of two angles of order 1;
  - 5e-16 m solved about the stored point, and the same at semi-axes from 1e3 to 1e7.

  A sphere's residual along the same arc reads 1.8e-4 m off as Fourier coefficients about the centre (the form `quadric_harmonics` builds) and 7e-16 m as a polynomial in `ξ`, `η` about the stored point. The solve and the basis differ by placement, so they belong with the harmonic, not with each reader.
- **The 112 "direction only" reads are not all directions.** A `u_ref` or `v_ref` read that is multiplied by a semi-axis and paired with the parameter is a harmonic's amplitude (`ring_path.rs` `plane_roots` is one). Those are the sites the general-anchor row says read a phased ellipse wrong (about 60). They are the same population as this row's 208, seen twice, and converting them to the harmonic discharges both.

**Each family.**
- *The certified lane's harmonics* (about 30): the harmonic. Against a plane it is first degree; against a wall, sphere or cone second (the harmonic times itself, as `quadric_harmonics` and `conic_cone_harmonics` do by hand); against a torus fourth. The surface's algebra stays in `geom-brep`; the basis, range and root posing are the harmonic's.
- *The symbolic scalar*: `point_at`. `swept.rs` restates the evaluator through `circle_at(&circle.frame(), …)`.
- *The cone section's chart image*: the harmonic through the cone's frame. The slant is the axial height over `cos α`, first degree. The azimuth channel is already stated about the vertex (`FocalImage`'s `u0`, `t0`). The pcurve's stored harmonic channel is then held as its carrier is.
- *The rebuild doors* (about 20): conic doors. The image under an isometry (`placed`, `mapped`), `reversed`, a re-seam and `map_scalar` exist; owed are a homothety, a concentric offset and a circle re-seamed at a point. Records that hold a circle destructured (`Rim`, `MeetSegment::Arc`) hold the `Circle`. `Circle::new` and `Ellipse::new` stay for a mint that is handed a centre; with no frame reading, nothing can feed them from a conic.
- *`mesh/sizing.rs`*: a first-degree harmonic's range (it rebuilds one from two evaluated points less the centre). *`torus_region.rs`*: a point against the conic.
- *Circle datum reads* (79 functions): see next.

**The centre as a datum.** By sample (about a dozen of the 79), most of these decide something about the arc: it lies on this surface, it is a parallel of that one, it is the same circle as another. That is the harmonic's range against the surface, not a centre question (a circle's centre off an axis is the swing of its distance from that axis). The rest need the centre as such: the file writer, a surface of revolution made about a circle's axis, concentricity of two whole circles. They are handed `center()`.

For a conic held on its arc that is the stored numbers' centre, one short subtraction: rounded at the semi-axes' scale at f64, an enclosure that wide at `Interval`. It is as good a datum as the centre the same construction would have stored (`work/pcert/the-cone-sections-image-is-built-from-its-derived-centre.md` measures the two within 1e-15 m at semi-axes 3 to 13 m), so a datum verdict reads what it reads on main. What changes is that the centre no longer defines the curve, so it may not go back into a point.

I lean to returning it wrapped, so that `centre + vector` does not compile and the bare coordinates come out through one greppable door (unsure; a plain `Point3` is defensible and differs only there).

**The answers as final states.**
- **A, recommended: the harmonic.** Makes true: every positional computation starts from the stored numbers on both scalars; moving the anchor to a general point changes `U`, `V` to the conjugate pair at parameter zero and nothing outside; nothing hands out or accepts a restated conic; a conic from a conic keeps its placement. Leaves possible: restating by hand from the centre's bare coordinates. Reversible: the harmonic is additive, and `frame` could be restored.
- **B: one reading per question on `Conic`** (PR 7510's two, then a third, …). True for the linear readers. The fifteen or so that are not linear either move each surface's algebra into `geom` or keep a derived centre under an `#[expect]` for good. Every B reading is one line over A's harmonic (`extent_along` is its range, `cover_over` its ball; the PR's private `offsets` and `offset_extent` are its internals), so B is A's first step, not a rival end.
- **C: publish the stored point and its phase.** The easy path is the wrong one: the middle number above, with nothing red. Every reader changes when the anchor moves.
- **D: a conic held on its arc has no centre; asking is a panic.** Whether a reader works would then depend on which construction made the conic it is handed, which is the cutoff you ruled out, moved to the reader. The writer needs the number regardless.

**Ratified text.** The clause "Every reading of where a conic is goes through the conic" (`crates/geom-core/README.md`) is agent-written (commits `32b2ae4ced`, `35b1559d50`), approved in passing on PR 7258 ("ok sounds good!"). It does not say what "through" means, which is what stopped the implementer. I would replace it with:

> Every reading of where a conic is goes through the conic: its point, its point as a harmonic of its parameter about the point it stores, or a point read against it. Nothing takes a centre or a frame from a conic to restate it; its centre is a datum, stored or derived, and never a term of one of its points.

The `conic.rs` module doc changes with it: the first bullet (the placement "read at the conic's centre (`Conic::frame`)") and the paragraph "a position is read as the centre" become the three readings and the datum.

**Size, kept out of the ranking.** 142 functions in 57 files, the same sites the general-anchor row owes, done once.

**Confidence.** A over B, C, D: likely. Constant-only is not enough; Fourier about a far centre loses digits: sure (measured). Every non-linear reader is a polynomial in the point: likely (wall, sphere and cone read; the torus builders by their docs). Most circle datum reads are a harmonic's range: likely to unsure (a dozen of 79 read). Identity of two conics as one's level along the other's harmonic: unsure.

## For the orchestrator

**Assumed or not read.** Read: `conic.rs` whole; `implicit.rs` wall, sphere and cone builders; the listed sites of `pose.rs`; five sites of `ring_path.rs`; about twelve circle datum sites; `FocalImage`. Not read: the torus builders' bodies, `derive_class_zero`, `certify.rs` `pcurve_map_residual`, limb 2's memo, most of the 79 circle functions. Nothing was built or run in the kernel.

**Probes** (Python, f64 against `decimal` at 80 digits; they model the arithmetic and are not kernel code):
- `/tmp/claude-0/-home-user-cad/55faae27-5a2c-5e42-af60-1b9ce7691468/scratchpad/designer-cri-c/probe2.py` (roots)
- `/tmp/claude-0/-home-user-cad/55faae27-5a2c-5e42-af60-1b9ce7691468/scratchpad/designer-cri-c/probe3.py` (sphere residual)

The Fourier loss in `probe3.py` is the `a²` cancellation of the coefficients. It would be the same with an exact centre, so main has it for every far-centred conic; `ConicHarmonics::terms` charges for it.

**Errors in the brief.**
- The 112 direction reads include harmonic amplitudes (above).
- 79 expectations carry the circle reason, not about 70.
- `mesh/sizing.rs` (`sphere_face_widest_cos`) is not "two statements compared by bit"; it is a first harmonic's range. The by-bit site is `mesh/src/curved.rs`, per the row.

**Defects off the question** (ONEROUND's; they belong on `one-stored-form-waits-on-the-readers-of-a-conics-centre.md` or a new row; I filed nothing, the checkout being read-only):
1. `Conic::unit_in_plane`'s default reads `self.frame().origin()`, and `Ellipse` does not override it, so an ellipse held on its arc is read through its derived centre. `conic.rs`'s module-wide `allow` hides it from the gate. Production callers: `pose/window.rs` `foot_seed`, `splitting/section_loops.rs` `conics_clear`, `blend/open/end_face.rs`.
2. Gate holes: `Curve3::circle_at(frame, r, t)` and the public `Cover::arc(frame, semis, window)` accept a restated conic and are not on the clippy list. The closures of `Circle::placed` and `Ellipse::mapped` hand out the stored frame.
3. `Cover::arc_of` still stores the derived centre frame in its `Arc` piece (listed, expected).
4. Two homes for one operation: `ArcWindow::extent` and `geom_brep::harmonic_range`.

**On PR 7510.** Nothing in the recommendation asks for it to change before it lands: its two readings become sugar, and its gate is the ratchet until `frame` is deleted, when the clippy list, the expectations and the `.counts` retire. I have not reviewed it as code.

**Suggested unit order**, each with the general-anchor row's sites in the same files:
1. the harmonic with its first-degree questions, and the linear readers;
2. degrees two and four, and the certified lane;
3. the chart image, with the pcurve channel held as its carrier;
4. the conic doors, and `Rim` holding a `Circle`;
5. the datum split, and deleting `frame`.

The harmonic must live inside the conic's module tree for `Held` to stay private.

**For Ev's sign-off.** The README sentence binds future work, so it waits. The wrapped-or-plain centre and the reading of two conics' identity are the two places I would expect pushback. On the second: for two whole circles the comparison of centre, axis and radius is a legitimate datum read and may be the better statement; over a window in use, the level of one along the other's harmonic is.

**Not weighed.** How the certified lane's noise meters (`FirstHarmonic`'s `lo_noise`, the root slack) consume a harmonic posed about a stored arc point. I assumed the meters stay in `geom-brep` and take the harmonic's posed polynomial; nothing I read contradicts it, and nothing I read confirms it.