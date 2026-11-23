PR 7044 delta re-review at e43ce957e9 — INTERIM (mutants still running, about 16 left at ~3 min each; final report follows).

No MAJOR so far. Provisional verdict: APPROVE-WITH-FIXES (two small items, both below).

**Held so far**
- **Claim 1 (lanes' domain):** same exact-rational oracle, same classes, ~480k triangles on the new head. Worst too long 0.31 allowance, worst too short 0.27 — unchanged from the first head. No class regressed.
- **Claim 2 (fixed pair after the sort):** 135k needles built to put the far corner in the middle of the sort (sphere-inscribed isosceles needles with the foot inside, width 1e-14 to 1e-1 of length; generic and axis-aligned poses; two and three corners sharing `x`; cylinder shadows about z, y and a tilted axis). Worst 0.20 allowance too short, 0.17 too long. I found no inscribed triangle where the fixed pair is worse than the select.
- **Claim 3 (scale):** corners at 1e300–1e307, subnormal, 1e-150–1e-80, `p` equal to a corner, all four equal, a small triangle 1e-20 to 1e-120 of its distance from `p`, coordinates mixed over 30 decades: all within 0.27 allowance, six orders bit-identical. The three first-review cases are correct now (axis norm 1e155, 1e300, 1.7e308, 1e-160, 1e-300 all answer 0.53851648071345; corners 1e-125 from `p` within 0.2 allowance). The "under 1e-77 of the scale" sentence holds on what I ran.
- **Claim 4 (no silent wrong finite answer):** none found, including subnormal directions and offsets near `f64::MAX` (those answer NaN in 8 of 6000 draws, never a wrong number).
- **Claim 5 (digest):** whole `mesh` suite, head vs main's `cert.rs` in one tree: 11 691 results, all 11 443 meshes identical, all 248 refusals the same kind. The same four `CertificateExceeded` bounds differ from main, with the same digits as on the first head.
- **Claim 10 (CI):** run 44004761901 on e43ce957e9: `gate ok`, lint, test (kernel), test (app), mesh budget meter all success. Skipped by the filter: tools, demos, step import, corrupt input, viewer app feature, python, interval, prime the build cache.

**Corrected so far**
- **Claim 6, DEMONSTRATED — `cert_torus` / `cert_spindle` still answer `+∞`, not NaN, where finite inputs overflow on the way.** `cert_torus(1.0, 0.2, [[1e308,0],[-1e308,0],[0,0.1]])` is `+∞` (NaN only when the `v` extent is 0); the same corners through `cert_spindle` are `+∞`; `cert_torus(1e308, 1e308, flat)` is `+∞`. The cone, cylinder and sphere answer NaN on the like input. `all_finite`'s doc reserves `+∞` for "finite corners that are no triangle of the surface". No production reader distinguishes the two (I grepped for `is_infinite` / `INFINITY` on a certificate: none), so nothing a caller sees but the refusal's `bound` payload. NOTE-sized.
- **Claim 7, DEMONSTRATED — "scale by the nearest coordinate" survives all 33 `cert::` / `sizing::` rows.** With `unit` taken from the smallest nonzero coordinate instead of the largest, nothing reds: the scale-invariance assertions hold for any power of two, and no fixture has coordinates far enough apart for the products to overflow. A row with one coordinate ~1e-200 beside unit-sized ones would catch it. MINOR (test gap; the code is right).

**Cost (claim 9), provisional:** ~115–135 ns per `dist_line_triangle` call against main's ~38, measured while the mutants were compiling; I will re-measure on a quiet machine. First head was 73.

**First findings:** m1 (unpinned select) is moot, the select is gone and the needle row exists; m2 fixed (demonstrated); m3 and n1 fixed in the PR body and `S236`; n2 fixed (demonstrated). n3 not addressed: the spec file's "Why it matters" still tells the sibling story and its status reads `dispatched`.

Still to come: the rest of the mutants, a quiet benchmark, the doc read (claim 8).
