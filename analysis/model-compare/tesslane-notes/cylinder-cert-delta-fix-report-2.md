PR 7044, final-review items 3 to 7. Head ee1b801a412c6c580d46a70fe091e9be6cba9f99 (`git ls-remote` agrees), one commit on 639edff0db (which carried items 1 and 2; my note on those stands). `crates/mesh/src/cert.rs` only.

**3. The scale's choice. One mutant reds, the other cannot be made to, and I measured why.**
- `unit` from the smallest nonzero coordinate: red on `corners_far_apart_in_size_are_read_at_the_largest` ("1e-200 beside 1e0: inf, not 5e-1").
- `unit` from the first corner alone: **not red, and I could not build a row that reds it.** It is benign, not a defect. A corner's own distance bounds the answer, so when the other corners overflow against the first one's scale, the nearest side read through that corner is still finite and the error is at most that corner's offset, which is by then under 1e-77 of the triangle's scale. Measured: 400 000 triangles with one corner 1e-250 to 1e100 in size and the other two up to 1e300 times larger, in-plane and generic: no non-finite answer, worst difference from the true code 6.5e-140 of one allowance. I first wrote two fixtures claiming it overflows; they passed under the mutant, so I removed them and the claim rather than leave a row that asserts nothing. If you want it pinned regardless, the only handle is bit-identity against the largest-coordinate reading, which pins an implementation choice, not an answer; I have not added that.

**4. The four benign scaling mutants.**
- `binade` without its `MIN_POSITIVE` floor: now red. Fixture "four points at one place" answers 0 (the mutant answers NaN).
- `area2 == 0.0` removed: **not red.** I added the sub-1e-77 fixture (a triangle 1e-170 wide, answers its nearest side's distance, 0.5), but it does not reach the branch, and I could not find an input that does: 2.6 million draws over three families of triangles with a normal under the smallest number gave bit-identical answers with and without the test. Reason, now a comment at the site: the foot's weights are each at most |n|² and underflow with it, so only their rounding can read such a foot as inside. The test stays as the guard against `0/0`; the fixture pins the answer on that domain, not the branch.
- `unit` as the largest coordinate itself, or three times its binade: one sentence at the site, as you offered: a power of two so the scaling itself rounds nothing; any scale within a small factor keeps the products in range and differs by rounding only.

**5.** `dist_line_triangle`'s doc now says NaN for a corner whose offset from `o` is not finite *or is within a small factor of `f64`'s largest number, where its product with the direction overflows: a refusal, never a wrong number*.

**6.** `binade`'s doc: dividing by it is exact while the quotient is a normal number, and rounds to the nearest subnormal below that.

**7.** Cost: in the body lines below.

Run on this head: `cargo nextest run -p mesh` 483/483; `cargo clippy -p mesh --all-targets --features budget -- -D warnings` clean; `cargo test -p mesh --features budget` 217 + 272; `cargo fmt --all --check` clean; `bash scripts/run-gates.sh` rc 0; `work.py lint` ok.

## PR body: changed lines, replacing the ones I sent with the last note where they overlap

(b, replaces my earlier cost paragraph, and drops the sentence I flagged) Under "The change", after the "How it is computed" list:

**Cost.** The one order, the scaling and the three side distances are paid on every certified cylinder triangle: review measures `dist_line_triangle` at about 104 ns a call on a quiet machine against main's 39, about 65 ns more per certified triangle. The `mesh` suite's wall time is within noise (199.6 s against 191.1 s under different load).

(c, replaces my earlier bullet) Under "Rows", after the needle row's bullet:

- `corners_far_apart_in_size_are_read_at_the_largest`: in all six orders to one allowance, a triangle with one coordinate 10⁻²⁰⁰ beside unit-sized ones and its mirror with one unit-sized coordinate beside 10²⁰⁰ ones (their plane's distance); a triangle 10⁻¹⁷⁰ wide (its nearest side's distance); four points at one place (0).

(e, replaces my earlier three table rows) In the mutant table, in place of "the corners not read at their own scale | the distance fuzz":

| the corners not read at their own scale | the far-apart row, the distance fuzz |
| the corners read at the scale of the nearest coordinate | the far-apart row |
| the scale without its floor at the smallest normal number | the far-apart row |
| `cert_torus` passing an overflowed bound through | the poison row |

(g, new) After the mutant table, before the paragraph on the normal from the two shorter sides:

Mutants that no row reds, each read and measured as harmless: the scale taken from the first corner alone (a corner's own distance bounds the answer, so the error is at most that corner's offset, under 10⁻⁷⁷ of the triangle's scale by the time anything overflows: 400 000 mixed-size triangles, no non-finite answer, worst 6.5·10⁻¹⁴⁰ of an allowance); the scale as the largest coordinate itself or three times its power of two (rounding only); the test for a normal whose length underflowed (2.6 million triangles with such a normal answer the same to the bit without it: the foot's weights underflow with the normal, so the branch is reached only by their rounding).

(h, new) Under "The change", at the end of the `dist_line_triangle` bullet in "How it is computed", add:

A corner whose offset from the line's point is within a small factor of `f64`'s largest number answers NaN (its product with the direction overflows): a refusal, never a wrong number.

Items (a), (d) and (f) from my last note are unchanged.
