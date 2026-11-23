(join-split implementer) A1: the coarsest δ, both trees, default ε. Each cell gives per-patch triangles, positions, signed volume and its ratio to the truth (topo::mass_properties), and check_mesh.

**The base is NOT flat at δ 1e9.** It meshes the snowman cap exactly as it does at δ 0.04 to 0.01.

| body | δ | base dd8e70198d | head 24ee9a28fb |
|---|---|---|---|
| snowman cap lat +60 (h 1e-3) | 1e9 and 1 | [18,6], 14 pos, +2.2794e-6 (0.726) | [14,14], 16 pos, +1.8649e-6 (0.594) |
| snowman cap lat −60 | 1e9 and 1 | [18,6], 14 pos, +2.2794e-6 (0.726) | **[6,6], 8 pos, −7.33e-20, FLAT** |
| lens of two unit spheres, off-axis, t 0.01 | 1e9 and 1 | [18,18], 20 pos, +1.1234e-4 (0.716) | [16,16], 18 pos, +1.0524e-4 (0.671) |
| the same lens, axial (each face holds a pole) | 1e9 and 1 | [4,4,4,4], 10 pos, +9.4045e-5 (0.600) | identical |
| unit ball | 1e9 | [24,24], 26 pos, +3.2190 (0.768) | identical |
| unit cylinder r 1, h 2 | 1e9 | [6,8,6,8], 16 pos, +5.6569 (0.900) | identical |
| ball less the box x ≥ 0.5 | 1e9 | [23,12,23], 31 pos, +2.8989 (0.820) | identical |

- check_mesh is Ok in every cell.
- Every cell at δ 1 is the same as at δ 1e9: the meshes have reached their floors (the minimum chord counts), which hold whatever δ is.
- **The head is flat at the viewer's scale probe on the snowman cap at lat −60, and the base is not.** Every thick body is identical on both trees and positive well above rounding.

A2 (check_volume over the 435 cells, with volume and area from topo::props) and A3 (the foot table) are next. The cure differentials are paused for these.
