(iso-run-density implementer) The first reviewer's three doc fixes are held locally as a242c622b5, on top of d6c35cefd7. Nothing is pushed.

1. `crossing_rods_keep_no_side_a_neighbour_can_make` doc:
   - It now says the equality is true of any triangulation of the face's points, whatever inserted them, so rows laid again pass it. It holds only that the patch triangulates its points.
   - "No rows" is held by `rowless_cylinders`: p = 0 on the walls, p = 4 on the two-hole cells.
   - The cure's cost is measured and pinned by nothing: n 28 → 86 triangles, n 60 → 182, n 130 → 408, up to 2.2× the rowed face.
   - The cure retires with PR 7573's unit. No cap was brought back.
2. The `rowless_cylinders` header and `Count::n` doc now name the case the identity does not cover: a pinch, or a vertex met again on one branch, is one CDT vertex counted twice in n. No body of these rows has one.
3. The cone/spline-cylinder issue now reads "1.2 to 223 times".

The pre-push list is running detached on a242c622b5 (pid 1932; it records HEAD and whether the tree is dirty), one step after another:
- work.py lint, run-gates
- fmt, as ci.yml's lint job spells it, benches included
- `clippy --workspace --exclude viewer --all-targets --all-features`
- `doc-gate.sh --workspace-pass --skip-viewer-toolkit`
- the budget clippy and `cargo test -p mesh --features budget`
- viewer app clippy, the viewer's slow rows, sweep's meshing modules
- mesh at 1e-9, 1e-6 and 1e-12

I'll report when it is done. If the second final adds a commit, I'll re-run the list on that tree.
