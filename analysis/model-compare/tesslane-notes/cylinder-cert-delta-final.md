# PR 7044 delta re-review at e43ce957e9 — FINAL (supersedes the interim)

**Verdict: APPROVE-WITH-FIXES.** No MAJOR. The rewritten distance is as accurate as the first head's on the lanes' domain, fixes m2 and n2, and moves no mesh. The fixes are test gaps around the new scaling (four mutants of it pass every row), one leftover `+∞` in the poison rule, and a 40 % higher per-call cost that the PR does not state.

## Claims

1. **Still the distance on the lanes' domain — held.** Same exact-rational oracle and classes, ~480k triangles. Worst too long **0.31** allowance (sphere cell-half), worst too short **0.27**: identical to the first head. Six orders bit-identical everywhere. The cap-sliver gap is unchanged (5.4·10⁵ allowances, within the inradius, off the domain, and now documented as such).
2. **Fixed pair as good as the select — held.** 135k needles built so the far corner sorts between the close two: sphere-inscribed isosceles needles with the foot inside (width 1e-14 to 1e-1 of length), generic and axis-aligned poses, two and three corners sharing `x`; cylinder shadows about z, y and a tilted axis. Worst 0.17 too long, 0.20 too short. I found no inscribed triangle where the fixed pair is worse; the comment's argument (the cancelling component tilts the plane about the needle's length) checks out by hand as well.
3. **Scale-free — held, with one wording note.** Corners at 1e300–1e307, subnormal, 1e-150–1e-80, `p` equal to a corner, all four equal (answers 0), a small triangle 1e-20 to 1e-120 of its distance from `p`, coordinates spread over 30 decades: all within 0.27 allowance. `1.0 / unit` at `2^1023` is subnormal but exact. The "under 1e-77 of the scale" sentence held on every draw. First-review cases now correct: axis norm 1e155, 1e300, 1.7e308, 1e-160, 1e-300 all answer 0.53851648071345; corners 1e-125 from `p` within 0.2 allowance.
4. **No silent wrong finite answer — held.** None found across the above plus subnormal directions, directions spread over 600 decades, and offsets near `f64::MAX`.
5. **No mesh byte, no verdict — held.** Whole `mesh` suite, head vs main's `cert.rs` in one tree: 11 691 results, all 11 443 meshes identical, all 248 refusals the same kind. The same four `CertificateExceeded` bounds differ from main, with the same digits as on the first head.
6. **Poisoning has one home — corrected** (finding n-a). No production code reads `is_infinite` or `INFINITY` on a certificate, so removing `cert_torus`'s `+∞` changes nothing a caller branches on.
7. **Rows and mutants — held for the PR's ten, corrected for the scaling.** 21 of my own on the new code, against `cert::` and `sizing::` (33 rows):
   - **Red (15):** multiply-back dropped (6 rows); sort by `x` only, stable or unstable (distance fuzz); no sort (needle row + fuzz); normal from the two longer sides (needle row + fuzz); never the interior (3 rows); bracket floor dropped, ceiling dropped (fuzz); direction unscaled (fuzz); `all_finite` removed from sphere, cylinder, the distance, spindle (poison row); `.max(0.0)` for `worse` (poison row).
   - **Survive (6):** findings m-a and n-b.
   - My first survivors: the fixed pair is now the code; `>=` for `>` is still equivalent.
8. **Docs — mostly held.** The function doc now states the three cases with their widths; `all_finite` has one statement; the PR body's sibling paragraph is per lane and says corpus-measured; "What moves" names the four bounds; both `work/chord` evidence lines are accurate (the `S236` qualifier is in, the unit-direction row quotes the 1e155 figure). Not addressed: n3.
9. **Cost — measured, quiet machine.** `dist_line_triangle` **~104 ns** per call against main's ~39 (first head: 73). About +65 ns per certified triangle. Suite wall time is within noise (199.6 s head, 191.1 s with main's `cert.rs`, runs under different load).
10. **Hosted CI — green.** Run 44004761901 on e43ce957e9: `gate ok`, lint, test (kernel), test (app), mesh budget meter + certificate falsifier, change filter: success. Skipped by the filter: tools, demos, step import, corrupt input, viewer app feature, python suite, interval, prime the build cache. I did not read this run's logs or re-run the meter on this head.

## Findings

- **m-a MINOR, DEMONSTRATED — `cert.rs:106-113`, which coordinate sets the scale is unpinned.** Taking `unit` from the smallest nonzero coordinate, or from the first corner alone, passes all 33 rows. The scale-invariance assertions hold for any consistent power of two, and no fixture has coordinates far enough apart for a wrong choice to overflow. By inspection the mutant is a real defect: a coordinate ~1e-200 beside unit-sized ones overflows the products, and a `+∞` distance would read as certificate 0 through `worse(r − dist, 0.0)`.
- **n-a NOTE, DEMONSTRATED — `cert.rs` `cert_torus` / `cert_spindle` answer `+∞`, not NaN, where finite inputs overflow on the way.** `cert_torus(1.0, 0.2, [[1e308,0],[-1e308,0],[0,0.1]])` is `+∞` (NaN only when the `v` extent is 0); the same through `cert_spindle` is `+∞`; `cert_torus(1e308, 1e308, flat)` is `+∞`. Cone, cylinder and sphere answer NaN on the like input. `all_finite`'s doc reserves `+∞` for finite corners that are no triangle of the surface, and the poison row tests the overflow case for cylinder and sphere only.
- **n-b NOTE, DEMONSTRATED — four more scaling mutants survive, each benign or a refusal.** `unit` as the largest coordinate itself, or three times its binade (so "a power of two" is unpinned; the effect is rounding only); `binade` without its `MIN_POSITIVE` floor (all four points equal would answer NaN); the `area2 == 0.0` test removed (a feature under 1e-77 of the scale would answer NaN). No row has four equal points or a sub-1e-77 feature.
- **n-c NOTE, DEMONSTRATED — cost.** Claim 9: the fix pass added ~30 ns per call over the first head and the PR body gives no figure.
- **n-d NOTE, DEMONSTRATED — `dist_line_triangle` answers NaN for some finite offsets within a small factor of `f64::MAX`** (8 of 6000 draws at 1e290–8e307): the dot product with the direction overflows. The doc names only an offset that is itself not finite. A refusal, never a wrong number.
- **n-e NOTE, BY-INSPECTION — `binade`'s doc, "Dividing by it is exact",** holds only while the quotient stays normal; the function doc says as much one level up.

## First-review findings

- **m1** moot: the select is gone, the needle row and the two-longer-sides mutant pin the order.
- **m2** fixed, demonstrated.
- **m3** fixed in the PR body and `S236`.
- **n1** fixed in the PR body.
- **n2** fixed, demonstrated.
- **n3 not addressed:** the spec file's "Why it matters" still tells the sibling story and its status reads `dispatched`.
- Style: Q5 headline fixed; Q1 three spellings are one, save n-a; the NaN-dropping `.max` is replaced by explicit branches; the table row no longer claims a pin it could not red.

## Executed / read only

Executed: the exact oracle on the first review's classes plus new needle, scale and direction classes (~720k triangles in all); 21 mutants plus a scratch probe test; the whole `mesh` suite twice with result digests; the release micro-benchmark. Probe files and the extra worktree are removed; the checkout is clean at e43ce957e9; nothing committed, pushed or posted.

Read only: the rewritten PR body, the check runs, the `work/chord` and spec-file changes. Not run on this head: `tools/tess-meter`, `sweep`'s meshing rows, the tour.
