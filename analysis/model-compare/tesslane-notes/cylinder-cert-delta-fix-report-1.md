PR 7044, delta fix pass. Head 639edff0dbbe3a705b950d80a1d33ef880a6eca1 (`git ls-remote` agrees), one commit on e43ce957e9. `crates/mesh/src/cert.rs` only.

**1. Scale by the nearest coordinate: now red.** New row `cert::tests::corners_far_apart_in_size_are_read_at_the_largest`: a triangle about the origin's foot with one coordinate 1e-200 beside unit-sized ones, and its mirror, one unit-sized coordinate beside 1e200 ones, in all six orders, held to one allowance of its plane's distance (0.5 and 5e199). Your mutant (`unit` from the smallest nonzero coordinate) reds it on the first ("1e-200 beside 1e0: inf, not 5e-1"); "no scaling" reds it on the mirror, and the distance fuzz as before.

**2. Torus and spindle overflow: NaN, the first option.** `cert_torus` ends in a comparison, `if bound.is_finite() { bound } else { f64::NAN }`: it never answers `+∞` on purpose (that is `cert_spindle`'s two early returns, which come before it), so a non-finite bound there is always arithmetic past the range. `cert_spindle` inherits it. Your three inputs are in the poison row, each asserting NaN and `Verdict::Exceeded`; the mutant that lets the bound through reds it ("torus, a u extent past the range: inf"). `all_finite`'s doc now says finite inputs whose arithmetic passes the range on the way answer NaN too, each certificate where it happens.

Run on this head: `cargo nextest run -p mesh` 483/483; `cargo clippy -p mesh --all-targets --features budget -- -D warnings` clean; `cargo test -p mesh --features budget` 217 + 272 passed; `cargo fmt --all --check` clean; `bash scripts/run-gates.sh` rc 0; `work.py lint` ok.

## PR body: changed lines only

(a) Under "The change", in the paragraph **One rule for an input that is not finite, in one place.**, replace its sentence "`cert_torus` tested for NaN alone and let an infinity through as `+∞`, and `cert_spindle` did not read an apex corner's azimuth at all; both now answer NaN." with:

`cert_torus` tested for NaN alone and let an infinity through as `+∞`, and `cert_spindle` did not read an apex corner's azimuth at all; both now answer NaN. Finite inputs whose arithmetic passes `f64`'s range on the way answer NaN too in all five: the torus's and spindle's extents and radii (a `u` extent of 2·10³⁰⁸ answered `+∞`), the cone's slant product, the cylinder's and sphere's differences.

(b) Under "The change", add one sentence at the end of the "How it is computed" list, as its own paragraph:

**Cost.** The one order, the scaling and the three side distances are paid on every certified cylinder triangle: review measures `dist_line_triangle` at 115 to 135 ns a call against main's 38 (provisional, to be re-measured on a quiet machine). No mesh is slower by a measurable amount in the suite's timings.

(c) Under "Rows", add after the needle row's bullet:

- `corners_far_apart_in_size_are_read_at_the_largest`: a triangle with one coordinate 10⁻²⁰⁰ beside unit-sized ones, and its mirror with one unit-sized coordinate beside 10²⁰⁰ ones, to one allowance in all six orders.

(d) Under "Rows", in the poison row's bullet, replace "two points whose difference overflows, and the cone's overflowed slant product." with:

two points whose difference overflows, the cone's overflowed slant product, and the torus's and spindle's extents and radii past the range.

(e) In the mutant table, replace the row "the corners not read at their own scale | the distance fuzz" with these three rows:

| the corners not read at their own scale | the far-apart row, the distance fuzz |
| the corners read at the scale of the nearest coordinate | the far-apart row |
| `cert_torus` passing an overflowed bound through | the poison row |

(f) Under "The class", in "Fixed here", replace the `cert_torus`, `cert_spindle` line with:

- `cert::cert_torus`, `cert::cert_spindle`: an infinity read as `+∞`, an overflow on the way read as `+∞`, an apex azimuth not read.

One caution on (b): the last sentence, "No mesh is slower by a measurable amount in the suite's timings", is my reading of whole-suite wall time only (`-p mesh` took 194 s before the fix pass and 190 s now on this box, with other load varying); I did not benchmark a mesh. Drop that sentence if you want the cost line to carry only the reviewer's number.
