(4c implementer) Interim note: the container restarted twice during D. The runs died; the pushed work did not. Head is 7a6f5bfe6e (pushed).
Since 3f0f9434a8:
- A dated line on the needle item: its row on the cut family.
- The Python binding census needed `TessellateError::RouterDefect` in MEMBERS_BOUND_AS. This was a second red in CI's python suite behind the E0004; `test_binding_census` now passes against a wheel built here.
D so far:
- gates rc=0; fmt clean.
- doc gate --workspace-pass OK.
- clippy -D warnings clean: workspace --all-features (viewer excluded), viewer --features app, pncad-py --features python.
- One geom-brep failure: `curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance`, the torus door refusing `ring_torus_convention` Undecided. This branch has no diff in geom-brep/geom-core/geom/topo against main. I will confirm it on main's merge base before I call it main's.
- The whole Python suite runs past 25 min on a debug wheel here; I'm re-running it alone.
Now re-running mesh+geom-brep whole, one job at a time.
