ISO-RUN-DENSITY — STAGE 1 REPORT, part 2/2.

Correction to part 1: the rows-off, no-refine control meshes 194/210 cylinder cells, not 196: the 16 refusals are CertificateExceeded at δ 1e-3 (8), 1e-2 (2), 3e-3 (2) and 0.1 (2), plus the holed and squat ones.

RECOMMENDATION: cure 1, restricted to the class, with the widest-edge-split refinement rule (1b). Concretely, for a face the route reads as no-rectangle whose every loop image is on an iso line (and, I would argue, the holed sibling):
 (a) size at δ, not UNPROVED_HEADROOM·δ;
 (b) on a cylinder, lay no lattice rows (and so no interior lattice at all);
 (c) where a triangle's certificate refuses, insert the midpoint of its widest side that is no constraint (centroid where it has none), and rebuild; one round has always sufficed.
It reaches the reference at 1.00–1.20× (worst on the coarsest δ, where a +2 refinement point is a large share), from 4–223× today, at all three ε, and lane time falls about 3×. It is the triangulation left generic and the bound's method improved, which is the preference given. Each ingredient is necessary: δ alone leaves the cylinder at 11× median and refusing; rows off without refinement refuses 16/210; refinement with rows on stays at 12× median.
Not recommended: cure 2 (held lines) buys nothing measured over cure 1. Cure 4 is the best on spheres only at coarse δ (1.02× worst against 3.0× in one δ 0.1 cell; medians are equal) and is cure 1 itself on cylinders: no mechanism is earned. Cure 3 is not needed.

WHAT I AM UNSURE OF / WHAT STAGE 2 MUST FACE
1. Scope. The switches as written apply to EVERY chart-lane analytic face. Run over the whole mesh suite (1b at the default ε, 6 passes), 19 tests fail. Most are pins that move legitimately: d9 goldens, route_rows' bytes and k-stream, and refusal rows. But real NEW refusals appear OUTSIDE the class: cone faces (section ellipse, through the apex, funnels, a cone with a ring: CertificateExceeded in the development, where the edge rule falls back to centroid) and m5_pr11's tilted cylinder. So the cure must be keyed to the class (a still-channel test on the loop's images), not to Trim::Read. No class face failed in that run.
2. [mesh.sizing]: sizing a chart-lane face at δ without a written theorem, with refinement as the backstop, is a change to what the clause says ("a lane without its theorem keeps UNPROVED_HEADROOM"). Per the spec that text goes to Ev as its own [ev] PR with these numbers. Is there a theorem? Read, not proved: on a cylinder with no rows, every CDT triangle has its corners on the boundary; the risky triangles are those spanning many columns, and the certificate catches those. I have no closed-form bound for them, so the refinement step IS the guarantee.
3. Coverage. The class in the suites is thin: 7 mesh tests, and nothing seen yet in sweep or step-import (census at ~1300/4848; tour not run). Retiring the unit is defensible on frequency alone. But the density cost per class face is large (median 22× on walls, 223× on a slender one), and the cure is three small changes, so I lean build.
4. The sphere gap is only the headroom (2×). If you want the smallest change, (a) alone on spheres plus (a)+(b)+(c) on cylinders is the same recommendation split by chart.
5. Not measured: a cone or torus notch (no door builds one); a notched wall with a split ruling under the cure (split_rulings' row passed in the suite run, with the switch lane-wide); release-build timing on a quiet box.
6. A side two faces can both make: 1b's refinement points are interior points, so it cannot make a new shared side. The shared-side cure ran unchanged and fired nowhere new in the corpus.
Filed: nothing yet. Candidate filing: Trim::Read takes a tilted-section Harmonic wall as well as an iso-bounded one. That is not a defect (the face belongs to the chart lane either way), so I have not filed it unless you want a row.

Waiting for your ruling. Meanwhile I am only finishing the census (sweep/step-import, then the tour), and will send its counts.
