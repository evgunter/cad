## For Ev
2	
3	**Recommendation.** One rule: **a direction or frame is built once, as a witness, where its construction knows it, and is carried unbroken into the stored surface or curve. No door re-derives it from points it placed, and nothing at rest re-measures it from its numbers.** This is your `[scalars]` sentence from PR 6912 ("a claim (a frame, which asserts that it is a rotation…) is built at the lane scalar, from its own inputs") applied to the one place still exempt from it, the stored carriers. Two changes follow:
4	
5	1. **The sweep doors keep the sketch frame they are handed.** Not a fork: plainly right, and buildable now. `sure`
6	2. **A carrier stores the witness, and check 1 stops asking a frame for its length.** This is the fork. I lean to it over the alternative 2′ below. `likely`
7	
8	Terms. *Box scalar*: `Interval` or `Sym`, a number held as a bracket over the parameter box (`Sym` also carries its expression). *Carrier*: a stored `Surface` or `Curve3`. *Door*: an operation that returns a body. *Witness*: a type whose only constructors establish a fact, here `UnitVec3` (a unit direction) and `RigidMotion` (an origin and an orthonormal frame); at a box scalar it means "an enclosure of one unit vector, or one rigid motion, per point of the box". *Check 1*: tier 3's first check, which among other things reads each stored frame's length and squareness back from its numbers. *Newell*: fitting a plane to a loop's vertices. *M1*: this program's measurement with the switch off. *The aimed-ray row*: the unit already being built that keeps the side reader's ray exact. *Fork N*: row N of the design-fork log.
9	
10	### Premise check
11	
12	- **It is one act, not three data.** `extrude` is handed `profile.plane()`, which is a `RigidMotion` witness (`SketchPlane`: "the plane is its frame"). Its first statements take `.placement()` and `.normal().get()`, which drop the witness. Everything after rebuilds directions from points placed with that frame: `newell_plane` for the caps and line walls, `rim.normalize()` for the cylinder's `u_ref` and again for each rim circle's, `w.normalize()` for the struts, `(q_to − q_from).normalize()` for line rims. `cap_plane` already holds the sketch normal and uses it only to decide which way Newell's fit faces. `sure` (read)
13	- **It is not one door.** The revolve keeps its witness: `AxisFrame` holds the axis as a `UnitVec3`, and every revolution surface takes `frame.axis` and `frame.u3` as they are. But its caps and the loft's go through the same `cap_plane`, and its latitude circles through the same `rim.normalize()`. `sure` (read)
14	- **Two different widths are tangled.** The slab's is width in a *value*: the cap's normal is (0, 0, 1) and the stored bracket is not. The tour's is a *claim* read back from numbers: check 1 computes `‖u_ref‖ − 1` from the bracket's ends (`analytic_datum_verdicts`, a bracket read that never goes through the decision funnel (`decide`), so `Sym`'s identically-zero theorem cannot reach it). An enclosure of unit vectors is not a box of unit vectors, so that question fails for any frame that is wide over the box, however it was minted. A fix at the extrude reaches the first and not the second. `sure`
15	- **The second is already on record.** Fork 259's case (a): a cylinder moved by a boxed motion refuses "axis not unit length" at a later finish and finishes only at a box half-width of 1e-11. It was left reported as undecided, with no fix.
16	
17	### Change 1: the sweep doors keep the sketch frame
18	
19	- A cap is the sketch plane's own frame, moved by the sweep: its normal is ± the sketch normal, the sign read from the profile's winding (structure), and its origin is the sketch origin plus the sweep. `newell_plane` and the `cap_plane_orientation` decision lose their production callers and go. A door that placed its vertices with a frame does not fit a plane to them and then measure which way the fit faces (`[edges]`: "What is true by construction is not measured").
20	- A line wall is the sketch frame turning the segment's in-plane direction, decided once in the sketch. That is the revolve's `AxisFrame::build` ("a rigid motion carries a unit vector to a unit vector, so nothing is decided in space"). An arc wall's and a rim circle's frame is the sketch frame turned to the arc's start radius. A strut runs along ± the sketch normal.
21	- **Makes true:** the slab's top cap normal is (0, 0, 1) at every depth and its walls' normals are exact, at `f64` as well as over a box. The only width left in a cap is its depth's own.
22	- **Leaves possible:** a door can still mint a fresh direction from points, but only through a decided mint, which is a named decision a reviewer sees.
23	- **Consequence:** with the aimed-ray row, this is what M1's hand substitution stood in for on the slab and the derived-frame cube. `likely`: the substitution replaced the normal where the side reader reads it, not where it is stored.
24	- **Reversible:** yes, local to `sweep`.
25	
26	### Change 2: a carrier stores the witness
27	
28	- `Plane`, `Cylinder`, `Circle` and the rest hold their frame as a witness, as `SketchPlane` does. Today `geom::Frame` is "stated data certified at rest" and `RigidMotion` is "a proof certified at its mint", joined by one bridge (`Frame::of_motion`) that nothing calls. They become one description.
29	- Check 1 keeps poison and the scalar conventions (a positive radius, a half-angle in range). Its six frame margins, and the `Length` and `Tilt` refusals built on them, go at every scalar.
30	- Import and the section code mint through the decided mints that exist (`from_axis_and_reference`, `gram_schmidt`). Import's verbatim adoption of a near-unit direction goes.
31	- **Makes true:** the tour's refusal and fork 259 (a) cannot be constructed, by any door. A plane's normal, a line's direction and a cone's frame, which nothing certifies today while readers take them as unit, become unit by type. (Open row `unlevered-frame-conventions-are-uncertified-at-rest`; PR 3238 measured a demo plane normal 0.26 off unit.)
32	- **Gives up:** the `f64` at-rest measurement that would catch a wrong mint, or rounding drift over a long chain of exact mints, on the cylinder-like kinds. `RigidMotion` already accepts that for every placement.
33	- **Needs your eye:** a carrier is read at another scalar by a structural lift (`map_scalar`; tier 3 at `f64` reads the body as `[x, x]`). That lift would carry the witness, restricted to maps that keep each value. It is the body's own data read where it is certified, not a lane taking a frame from `f64`, but it is an upward crossing of a witness, which PR 5068's text says does not exist.
34	- **Reversible:** mechanically, but it touches every reader of a carrier's direction.
35	
36	**2′, the defensible alternative: keep bare fields, and check 1 asks through the decision funnel.** Frame length and squareness become zero-decisions like every other identity tier 3 certifies. At `Sym` an identically-zero deviation is then discharged by theorem, and the leaf's row matches the `f64` witness's measured Zero.
37	- **For:** small, easy to reverse, keeps the `f64` measurement, and uses the mechanism every other identity over a box uses.
38	- **Against:** whether a body certifies then rests on the symbolic tier proving a rigidity the construction already knew, and fork 266 measured chained frames' identities freezing at `Sym`. Plain `Interval` never certifies a wide frame. The fields stay "unit by convention", so the open row above stays open. It adds decisions to every finish.
39	
40	### Worked example (the case change 1 cannot reach)
41	
42	A 5 mm-radius hole whose body is turned about z by θ = 30° ± 1° and then cut from a plate. The stored `u_ref` is (cos θ, sin θ, 0) with two independent brackets, so `‖u_ref‖` reads [0.985, 1.015] and check 1's margin is ±7.6e-5 m against ε = 1e-9. It decides only once the leaf is narrower than about 1e-5 of a degree, though every frame in the family is exactly unit. Under 2 nothing asks. Under 2′ the `sin² + cos²` rule discharges this one, and a chain of such turns may not.
43	
44	### Ratified text I would change
45	
46	- `[scalars]` (yours, PR 6912, 2026-11-14): unchanged. Carriers become an instance of it.
47	- `geom`'s crate docs ("Unit-vector and orthogonality fields are conventional data, unchecked here … at rest, check 1 certifies …") and the "What is NOT this type" paragraphs of `UnitVec3` and `RigidMotion`: replaced under 2.
48	- **Provenance.** Check 1's frame margins are PR 3238, an agent unit merged with no question to you. The "NOT this type" text is from PR 5068, where your words were about crossings ("is there any way to remove 1 also?", "A … sounds good!") and never about carriers. The `git log -S` searches end at the shallow checkout's graft (2026-11-06), so I read both PRs instead.
49	- **These conflict in spirit, and the later one is yours.** `RigidMotion`'s docs since 5068: "nothing re-derives rigidity from the entries … at a box scalar no door re-derives a motion's rigidity from its entries". Check 1 does exactly that to the same numbers once they sit in a carrier. I lean to changing the older text.
50	- "Crossing scalars" ("never up") gains the carrier's lift, if you accept it.
51	- `newell.rs`' "Plane equations are derived caches" (agent text) is reversed for a swept face: the plane is the description and the vertices are the cache.
52	
53	### Considered and not recommended
54	
55	- **Repair the three sites in the extrude.** It leaves the revolve's and loft's caps, every rim circle, and every frame that turns over a box.
56	- **Have `Sym` tighten a value from its expression.** Its expression for the slab's cap normal should already reduce to (0, 0, 1) (inferred). This compensates downstream for a datum the door should not have widened, works at one scalar, and ties certification to the symbolic budget.
57	- **Skip the frame margins at the box scalars only.** A box-certified answer would then rest on a question asked at `f64` and not over the box.
58	
59	### Confidence
60	
61	- Change 1 is right: `sure`.
62	- Change 1 with the aimed-ray row closes the slab and the derived-frame cube in one leaf: `likely`, unmeasured at the mint.
63	- Change 2 over 2′: `likely`.
64	- Change 2 closes the tour's stop: `likely` for check 1 itself; `unsure` what the next reading makes of a `u_ref` whose bracket is still a quotient. A measurement is named for it.
65	
66	## For the orchestrator
67	
68	**Assumed or unverified.**
69	- I ran nothing. Every measured number is M1's or a fork row's; the 30° example is arithmetic.
70	- Not verified: that the tour's refusal list holds only the two surface findings. Check 1's carrier half reads the same margin on each rim circle, whose `u_ref` is the same `rim.normalize()` (`swept::placed_segment_spec`), so `CurveDatumUndecided` should stand beside them or come next.
71	- Inferred, unmeasured: the `u_ref` refusal is not behind the switch. With the switch on, `topo::take` runs the same tier 3 on a Boolean's operand, so plate-minus-hole over a macroscopic box should refuse the same way on main. The tour's certified document escapes only because it never subtracts its holes (`demos/tour/src/tolerance.rs`, header).
72	- Unknown: how an imported or `f64`-built body enters a box lane, which bears on change 2's crossing sentence; and whether any `map_scalar` caller passes a map that does not keep each value.
73	- Provenance: `git log --all -S` for `geom`'s at-rest paragraph, `UnitVec3`'s "What is NOT this type" and `newell.rs`' module doc all end at grafts (`45cae9d7`, `c58a72e3`). The `[scalars]` clause traces to `be9f5573`. I read PR 3238's body and PR 5068's and 6371's comments through GitHub.
74	
75	**Errors and gaps in the brief and the row.**
76	- "Three data" undercounts. The re-derivation sites are: both `newell_plane` calls (`swept::cap_plane`, `extrude::side_surface`); `rim.normalize()` in `extrude::side_surface`, `swept::placed_segment_spec`, `revolve/surfaces.rs`, `revolve/full.rs` and `revolve/turn.rs`; `strut_carrier`'s `w.normalize()`; the line rim's `(q_to − q_from).normalize()`; and `cap_plane`'s `place.linear.c2.normalize()`. `loft.rs` and `revolve/partial.rs` reach Newell through `cap_plane`.
77	- The revolve is the precedent, not a sibling defect: its surfaces are exact off `AxisFrame`.
78	- M1's `BOXFINISH_PLANE=mid` is applied in `side_reader.rs`, where the reader reads the normal. It does not show what an exact stored normal does to the other checks or to cost, and it leaves the plane's origin at Newell's centroid. Change 1 moves the origin to the sketch origin plus the sweep, which changes cap chart coordinates and so the caps' pcurve rows.
79	- The row's inference ("a vector divided by its own length over a box is not recognisably unit") is right and is half of it: `Vec3::normalize` is `self / self.norm()`, and check 1 never asks `Decide`.
80	
81	**Measurements that would decide something** (probe branch; none run).
82	1. Mint the extrude's caps from `profile.plane().frame()` and its line walls from the frame-turned 2-D normal; rerun the slab ±0.05 and both derived-frame rows with the aimed-ray row's ray. Decides the first `likely`.
83	2. Skip check 1's `Length` and `Tilt` margins at the box scalars, as a stand-in for change 2, and rerun the tour's certified stop; read the next refusal. If a later check trips on the quotient-wide `u_ref`, the direction mint itself needs work: `UnitVec3::new` at `Interval` divides a bracket by its own norm, so ([0.9, 1.1], 0, 0) becomes ([0.82, 1.22], 0, 0) though every point of the box normalizes to (1, 0, 0). The candidates are a mint that returns the range of the unit vector, or the profile keeping the direction its construction made (`Arc2::from_chord` computes `unit` and drops it; the path layer's `Dir` is dropped at validation). I did not recommend either unmeasured.
84	3. `cut_plate` on main at a macroscopic box: the first refusal's payload (the inferred claim above).
85	4. A count of carrier-direction read sites, for change 2's ergonomics.
86	
87	**Off the question** (not filed; I may not write).
88	- `Frame::of_motion`, documented as the one bridge, has no caller (grep over `crates`, `demos`, `probes`, `benches`).
89	- `Extrusion::Vector` admits an in-band obliquity, so its walls are ruled along `w`, not the sketch normal. Change 1's exact wall frame needs `work/strut/extrude-arc-walls-are-ruled-in-n-not-w.md` settled, or a decided mint on that arm.
90	- The registered-identity sites (`register_rigidity`, `register_rim_identity`) exist to tell `Sym` what `rim.normalize()` discarded. If an arc's start direction were data, the rim identities would be polynomial (inferred). A follow-up for E12's allowlist.
91	- Rows change 2 would likely close or re-scope (names read, bodies not): restdatum's `unlevered-frame-conventions-are-uncertified-at-rest`, `cone-and-torus-box-extents-read-the-carrier-axis-as-unit-by-prose`, `dist-line-triangle-takes-an-unchecked-unit-direction`, `sector-face-cylinder-normal-normalizes-an-undecided-radial`, `tessellation-chart-frame-is-a-hand-rolled-undecided-triple`. ONEHOME's `a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs` loses its "axis not unit" findings.
92	- Sequencing is yours: change 1 needs no ruling and unblocks the slab class; change 2 is the `[ev]` question and holds the tour's stop.
93
