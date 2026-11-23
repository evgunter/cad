ISO-RUN-DENSITY: the reviewer's probe, run as given (debug, default ε). Measured.
1. **The head refuses exactly the reviewer's 8 cells**, the same bounds to the last digit: δ 1e-2, h 1.0 and 1.9, gap 1.5 and 1.6, phase 0.5; δ 1e-3, h 1.33; r = 1e-3 the same. My earlier miss was the placement: my pair sat at c = 0.5 col, beside the seam; theirs at 9.5 col.
2. **Merged main 032e8b71c7 refuses 2 cells of the same probe, with its rows**, at the r = 10, δ = 0.1 row (which the reviewer's run did not reach, scale 1e3 having panicked): h 1.0 and 1.33, gap 1.5, phase 0.5, CertificateExceeded bound 0.10823 against 0.1. That is the same bound ratio (1.0823) as the head's δ 1e-2 cells. So the class exists on main too; the rows only move which cells it lands in.
3. Width sweep: gap 1.0, 1.1, …, 2.0, 2.25, 2.5, 2.75, 3.0 × phase 0, .25, .5, .75 × h 1, 1.33, 1.9 × the five scale/δ rows, 900 cells. The head refuses the same 8 and no others (only gaps 1.5 and 1.6 at phase 0.5); main refuses its same 2. My earlier 1,216 cells near the seam (holes, notch beside a hole, two notches from opposite rims; three hole sizes) refused nothing on either tree.
4. Cures on these bodies:
   - (a) the lane's correction switched on for the cylinder row (a point inside each refused triangle, rebuild, under the existing 6-attempt budget): **0/180 refuse.**
   - (b) lattice nodes laid only inside a triangle spanning more than √2 columns: **0/180 refuse.**
   - (c) rows back is main: 2 refuse.
   Both cures on the 900-cell width sweep, and their cost on the 391 + 1,361 moved faces, are running now.
5. The differential against MERGED main 032e8b71c7, CAD_FUZZ_SEED pinned, default ε:
   - **mesh**: 58,353 faces paired. Only cylinder faces move: 390, 1,013,168 → 31,686 triangles (median 0.16× per face, max 2.22×). Every other kind is bit-identical.
   - **sweep's 61 meshing modules**: 32,235 faces paired. **Every key paired this time, 0 unpaired:** the 416 were the two fuzzers' unpinned seeds. 1,361 cylinder faces move, 2,189,525 → 172,795 (max 1.018×). Every other kind is bit-identical.
   - Unpaired in mesh: the head's new rowless_cylinders rows (268 keys), the rod-sliver row's old δ 5e-3 against its new δ (3), and issue555_subfloor_cap::the_written_zero_keeps_the_rebuild_bit_identical (6 each side). I have not yet read why that body's digest differs between the trees; it is plausibly a body rebuilt from mesh output.
   - bisector_nodes is red on both trees.
