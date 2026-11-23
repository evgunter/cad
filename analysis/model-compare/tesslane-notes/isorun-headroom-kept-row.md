ISO-RUN-DENSITY: the row you asked for (point 5), measured, the whole corpus at three ε (identical at 1e-9, 1e-6, 1e-12). HEADROOM KEPT, rows off on the cylinder:

| | cyl meshes | cyl ratio to r_dec min/med/max | sph meshes | sph ratio | worst cert/δ | passes |
| headroom + rows off, NO refine | 210/210 | 1.00 / 1.40 / 1.50 | 60/60 | 1.65 / 1.96 / 4.4 (= today) | 0.689 | 1 (sph ≤2, the existing shared-side cure) |
| headroom + rows off + 1b refine | 210/210, identical counts: the refinement never fires | same | same | same | 0.689 | same |

Plainly: **with the headroom kept, no cell refuses without refinement. (c) exists only for (a).**
So, split by ingredient:
- (b) alone takes the cylinder walls from 4–223× to 1.00–1.50× with no refusal and no new mechanism.
- (a) is worth the remaining ~1.4× on cylinders (1.00–1.20× with (c)) and the whole 2× on spheres (1.65–4.4× → 0.75–1.0×, with one δ 0.1 cell at 3.0×). On cylinders it needs (c): without it, 16/210 cells refuse.
- On spheres (a) needed no refinement in any cell (cert ≤ 0.986δ, 60/60 mesh at δ without (c)).
This changes what I would build if Ev says no to (a): (b) alone, with no refinement at all, since (c) would be dead code. If Ev says yes: (a)+(b)+(c), where (c) is reached only on cylinders. Proceeding with stage 2 as ruled: (a) at one named site, (b), (c) with a named termination constant. Say if you want (c) built only behind (a)'s answer instead.
