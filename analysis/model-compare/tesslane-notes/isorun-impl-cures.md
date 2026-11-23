ISO-RUN-DENSITY: the cures on the reviewer's bodies, 900-cell width sweep (gap 1.0–3.0 × phase 0/.25/.5/.75 × h 1/1.33/1.9 × five scale/δ rows). Measured, debug, default ε, nothing pushed.
- head: 8 refuse (the reviewer's 8); 892 mesh.
- main 032e8b71c7: 2 refuse (r = 10, δ 0.1, gap 1.5, phase 0.5; the head meshes both). Its other cells take 1.2–17× the head's cylinder triangles (median 3.6×).
- **(a) the lane's correction on the cylinder row** (centroid of each refused triangle, rebuild, existing 6-attempt budget): 0 refuse. The 8 cells mesh at 76 triangles (δ 1e-2, all three heights; δ 1e-5 at r 1e-3 the same) and 192 (δ 1e-3). **The other 892 cells are bit-identical to the head**: it fires only where the head refuses.
- **(b) lattice nodes only inside a triangle wider than √2 columns**: 0 refuse. The 8 cells take 80–92 and 236 triangles, and the other 892 are again identical to the head (no triangle there is that wide).
- (c) rows back = main: 2 refuse, and 3.6× median density.
So on these bodies (a) is the cheapest. For scale, the plain two-hole wall takes 68–72 at δ 1e-2, so (a) costs about 4–8 triangles on a refusing face. Both (a) and (b) leave every non-refusing cell untouched.
Still running: (a) and (b) over the mesh suite and sweep's 61 modules (seed pinned), to show what each does to the 391 + 1,361 moved faces. I expect (a) to be bit-identical there, since nothing in those suites refuses on the head; (b) may fire on wide triangles that certify, which is exactly what that run is for.
On issue555's 6 unpaired keys (from the differential): a direct dump of that body's vertex points AND of its mesh positions is bit-identical on head and main. Only my digest log keys them differently, which I have not explained. The mesh itself does not differ.
