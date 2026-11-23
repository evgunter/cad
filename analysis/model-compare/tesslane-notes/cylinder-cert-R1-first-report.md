# Review of PR 7044 at 5e933bfff1 (tesslane/cylinder-cert) — full review + style lane

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The new distance is right on the lanes' domain, no mesh moves, the rows catch the mutants that matter. The fixes are wording (three claims overreach, one of them in the PR body's central sentence) and one unpinned design choice.

## Claims

1. **True distance for every triangle — held on the lanes' domain, corrected off it.** Own oracle: exact rational arithmetic (min over interior foot / three sides), ~480k triangles through the real functions. Inscribed sphere and cylinder triangles (needles, caps, two/three coincident corners, ruling-sided, near-ruling to 1e-12, diameter, wrapping the axis, 10⁶ radii along the axis, tilted non-unit axes): worst too long **0.31** rounding allowances (sphere cell-half), worst too short **0.27**. Off the domain, two exceptions:
   - a cap sliver with the foot at or near its inside: too long by **5.4·10⁵ allowances** (1.6e-7 on a triangle of length ~100, true 6e-15), too short by 5.3·10⁵. Both inside the inradius, as the doc says.
   - coordinates below ~1e-77 from `p`: the products underflow, the inside test reads false, and it answers the nearest side: **2·10¹⁴ allowances** too long at 1e-125. Overflow is guarded; underflow is not.
2. **Bit-identical in six orders — held.** 0 mismatches in all oracle classes. Removing the sort reds only the distance fuzz, as the PR says; the table row passes without it. Bit-equality is more than the lanes need (agreement to rounding would do); it costs the sort.
3. **The two certificates — held.** Certificate error is the distance error with the sign turned: never under the truth by more than 0.31 allowance on inscribed triangles. "Unreachable from an inscribed triangle" holds by argument (a triangle holding its circumcentre has no angle past a right one) and by the sphere-cap and cylinder classes above (max 0.2).
4. **"No mesh over δ" — held on what I re-ran; its stated reason corrected.** Own instrument over the whole `mesh` suite (main's distance in six orders beside the new one, at every cylinder/sphere certificate call):
   - read NaN so 0: **6 291 curved / 94 386 trimmed** — the PR's `mesh` row exactly. All cylinder, each NaN in exactly 2 of 6 orders, 0 sphere, 0 finite-long, 0 verdict flips.
   - largest true deviation among them: curved 1.00000000036 δ (the coordinate-900 row, inside its allowance); **trimmed 0.652 δ**.
   - **Correction:** the trimmed cylinder arm has no worst-triangle theorem; it sizes at `UNPROVED_HEADROOM` 0.5 and names the certificate as its guarantee. The suite's trimmed triangles main read as 0 sit at 0.4–0.5 δ in bulk with a tail to 0.65 δ (a ruling side spanning 1.14 lattice steps, past the README's `1 + BOUNDARY_CLEARANCE`). So "within δ because the lanes size their steps so (`[mesh.sizing]`)" is true of the curved lane and is headroom, not a theorem, on the arm holding 94 % of the misreads.
   - The 561-point grid alone cannot bound a deviation to 0.02 % (a parabola sampled at 1/32 can miss its peak by ~0.1 %); the claim stands instead on the new certificate deciding every row, which is exact.
   - **I did not build an over-δ body.** It needs a ruling-sided trimmed triangle spanning ≥ √2 steps; by reading, the lattice plus the rim-chord rule keep a nearer vertex visible. Analysis only, not a constructed attempt. `sweep` and the tour were not re-run.
5. **No byte moves — held for meshes, corrected for refusals.** Digest of every `tessellate` / `tessellate_with` result over the `mesh` suite, head vs main's `cert.rs` in the same tree: 11 691 results, all 11 443 meshes identical, all 248 refusals the same kind. **Four `CertificateExceeded` refusals report a bound moved in the last one or two digits** (e.g. 0.11990393700810942 → …953). Nothing pinned moved: `d9_mesh_goldens` passes, `tools/tess-meter` 36/36.
6. **Poisoning — held.** A NaN certificate reads `Exceeded`, and the caller sees `CertificateExceeded { bound: NaN }`: "certifies to only NaN m … kernel-side defect or degenerate geometry", true enough and already what the cone gives. `cone_chord_sag`'s NaN needs slants near 1e154: no real face. One gap, finding m2.
7. **Rows and mutants — held, one survivor.** 17 mutants against `cert::` and `sizing::`:
   - old clamp, `r` unchecked, no range guard, cone overflow → the poison row.
   - no bracket, no upper end, no lower end, inradius ×4, no sort → the distance fuzz only.
   - fixed side (at zero area, and always), never the interior, segment end-clamp dropped, bracket floor always → table + both fuzz rows (+ a `sizing` row).
   - nearest side only at exactly zero area over main's algorithm → four rows.
   - **Survive:** the normal from one fixed pair of sides after the sort (finding m1); `>=` for `>` in the inside test (equivalent).
   - The certificate fuzz reads the cylinder against a closed form from the azimuths (independent). Its sampled `dev` goes through `geom::Line::offset`, the projection the certificate uses.
8. **Sweep and filings — held by reading only.** I spot-checked `cert_torus`, `position_scale`, `verdict`, `worse` and the two filed rows; nothing left for another row looked reachable with a valid body. The sweep's own stated blind spot (overflow reading a finite wrong value) is where m2 sits.
9. **Cost — measured.** Release, 2M calls: `dist_line_triangle` **73 ns vs 37 ns** on main, about +35 ns per certified triangle. Suite-level time not isolated (my two runs were under different load).
10. **Spec deviations — all four are corrections, all reported.** The wider cure owed no stop: exact-zero-only leaves the finite misread (old code up to 10¹³ allowances too long on a tilted ruling in my oracle), no verdict moves, no ratified text changes.
11. **Hosted CI — green on this head.** Run 43987660938, merge of 5e933bfff1 onto b0c685281a: `gate ok`, lint, test (kernel), test (app), mesh budget all success. `mesh` ran in the app shard (4554 passed); the four rows passed at default, 1e-6 and 1e-12; slowest 0.143 s against the 0.5 s bar. **`tools` and `demos` were skipped by the change filter**, so the meter's bytes and tour pins rest on local runs (I re-ran the meter; nobody but the implementer ran the tour).

## Findings

- **m1 MINOR, DEMONSTRATED — `cert.rs` `dist_point_triangle`, the normal's choice of sides is unpinned.** `let n = ab.cross(bc);` in place of the two-shorter-sides select passes all 32 rows. The PR's table lists "the normal from the two longer sides" as red; a fixed pair is not.
- **m2 MINOR, DEMONSTRATED — `dist_line_triangle`: "`d` need not be unit" fails silently at extreme magnitudes.** A tilted axis of norm 1e155 answers 2.449 for a true 0.5385 (`|d|²` overflows, nothing is projected): too long, so the certificate reads 0. At 1e-160 it is off in the 9th digit; NaN only below ~1e-162. Not reachable with a unit axis, but the doc and the evidence added to `work/chord/dist-line-triangle-takes-an-unchecked-unit-direction.md` ("holds for any nonzero `dir`") say more than is true, and the corners got a range guard where the axis did not.
- **m3 MINOR, DEMONSTRATED — PR body, "the lanes size their steps so".** As claim 4: false as a reason on the trimmed arm. `work/chord/S236.md`'s new evidence repeats "No returned mesh was over δ" without the qualifier that it is corpus-measured there.
- **n1 NOTE, DEMONSTRATED — four refusals' reported bounds move by rounding** (claim 5); the PR says nothing reported moves.
- **n2 NOTE, DEMONSTRATED — underflow** (claim 1): "for every triangle" and "a triangle of no area answers `s` exactly" do not cover coordinates under ~1e-77.
- **n3 NOTE, BY-INSPECTION — the spec file's Finding / Why-it-matters still tell the sibling story** the PR corrects; status reads `dispatched` where the PR says it closes.

## Spec deviations

Reported by the PR: all four (sibling no safeguard; finite misread; trimmed arm; budget job). SILENT: none found. Unreported facts rather than deviations: n1, and that hosted CI skipped `tools` and `demos`.

## Style

Questions exercised: Q1, Q2, Q3, Q5, Q6, Q7, Q8 (production half of `cert.rs` read whole, and the four new rows; the older cone/torus tests skimmed). Q4 only as the spec-file read.

- **Q5, `cert.rs:57` — sure.** The headline "the distance … for every triangle" is narrowed three paragraphs later to "within the inradius" for slivers. Measured gap 5·10⁵ allowances. The headline is what a caller reads.
- **Q8/Q5, `cert.rs:96-101` — likely.** I believe the 192·m⁴ comment and the guard (it tests 256·reach² ≥ 256·m⁴). The asymmetry with underflow is unstated.
- **Q1, `cert.rs:174`, `:254`, `:342` — likely.** Three spellings of "not finite poisons": `inside_bound` tests `is_finite` on two values, `cert_cone` guards at entry, `cert_torus` tests `is_nan` only and lets an infinity through as `+∞`. The poison row accordingly asserts `is_nan` for three certificates and `!is_finite` for two. One rule, no one home.
- **Stance (fresh instance), `cert.rs:121-123` — sure it is there, unsure it matters.** A unit whose subject is `f64::max` dropping a NaN ships a `.max(sides - inradius)` that relies on dropping one. It is commented and the floor is the safe end.
- **Q2, `cert.rs:57-86` — likely.** Thirty lines of argument over thirty-seven of code, and the part that carries the safety (inscribed corners never make the ill-conditioned case) is enforced by nothing but the fuzz.
- **Q3, the distance fuzz's `plane` term (`cert.rs:1755`) — likely.** The tolerance on slivers is the implementation's own error model, up to the inradius, so on that shape the row pins the implementation, not the distance. On fat triangles it is tight.
- **Q3, the table row — sure.** Its "bit-identical in all six orders" assertion cannot go red without the sort on any of its fixtures (the PR says so). It reads as a pin of the sort and is not one.
- **Q6 — likely.** "No mesh is over δ" is a measurement with no guard; `S236` (a standing sample of the mesher's own cylinder triangles) is named as still open, which is a schedule only if that row is scheduled.
- **Q7 — unsure.** For the cylinder the point lies in the shadow's plane, so the normal, the foot and the bracket do nothing there; the 3-D machinery serves the sphere alone. I would have accepted order-dependence to rounding rather than sort. Neither is wrong.
- **Q1 — sure, clean.** Prose sweep and a grep for another point–segment / point–triangle helper in `crates/*/src`: none.

## Rubric

- **Idiom / structure: 4.** Small, total, branch-light; `inside_bound` is the right single home for the clamp. The sort and the deliberate NaN-dropping `max` are the oddities.
- **Test quality: 4.** Independent closed form, built nearest points, 15 of 17 mutants red. Loses a point for m1 and the sliver tolerance.
- **Doc / comment honesty: 3.** Careful and mostly exact, but three statements say more than holds (every triangle; any nonzero `d`; sized so by the lanes).

## Executed / read only

Executed: exact-rational oracle over ~480k triangles through a probe binary; 17 mutants; the whole `mesh` suite twice with result digests (head, and head with main's `cert.rs`) and a per-triangle probe on head; release micro-benchmark; `tools/tess-meter` tests; extreme-axis probe. All probe files and the extra worktree are removed; the checkout is clean at the frozen head; nothing committed, pushed or posted.

Read only: hosted CI logs (through a log read, all of the meter job and the last ~5000 lines of the app shard); the sweep's dispositions; `trimmed.rs` and `curved.rs` around the verdict; the tracker files. Not run: `sweep`'s meshing rows, the tour, any constructed over-δ body.
