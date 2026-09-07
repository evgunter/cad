---
id: rule-d-multiple-reader-wraps-on-a-huge-dyadic-coefficient
kind: issue
title: rule D's argument reader forms its multiple with a wrapping shift, so a coefficient past i128 folds as if it were in range
status: open
opened: 2026-09-07
---


**Found by M10-10's R1 review** (PR #2100 at `e904d9691`), reproduced
at the scalar.

## The defect

`crates/geom-core/src/sym/trig.rs:181-188`, `read_argument`:

```rust
let (k, m) = if q.exp2 >= 0 {
    (k.checked_shl(u32::try_from(q.exp2).ok()?)?, 0)
} else {
    (k, q.exp2.unsigned_abs())
};
if k.unsigned_abs() > MAX_MULTIPLE.unsigned_abs() || m > MAX_HALVINGS {
    return None;
}
```

`i128::checked_shl` refuses only a shift of 128 or more; for any
smaller shift it WRAPS on overflow. So the guard on the next line is
applied to the wrapped value, not to the multiple. A dyadic
coefficient `q = k · 2^e` with `k` odd, `bits(k) ≤ 127` and
`bits(k) + e > 128` is read as some unrelated small `k'`, and the node
folds to the closed form of `cos(k'·atan X)` / `sin(k'·atan X)` —
an argument form the module's own contract says never folds.

`fold_at_half_pi` (`trig.rs:222-226`) shifts the same way but reads
only `k mod 4`, which a wrapping shift preserves exactly, and every
wrapped case there is an even integer multiple of `π` whose cosine
really is `1`. That fold is unaffected.

## Reproduced

`geom-core`'s
`m10_10_r1_sym_probes::r1_the_multiple_reader_wraps_on_a_huge_dyadic_coefficient`
(branch `m10/m10-10-r1-probes`), at `Sym<Interval>` inside a session,
with `x` a parameter over `[0.4, 0.6]`:

```
cos((2^123+1)·32·atan x) − cos(32·atan x)  =>  theorem
```

`(2^123 + 1) · 32` normalises to `Rat { num: 2^123 + 1 (odd), exp2: 5 }`;
`(2^123+1) << 5 = 2^128 + 32` wraps to `32`, so both sides fold to the
closed form of `cos(32·atan X)`, the difference is the zero polynomial
and `sign_within` answers `Sign::Zero` counted `symbolic_zero` — a
FALSE theorem, since the two cosines differ for generic `x`. The row
is red on the probe branch on purpose.

## How reachable, and why it still matters

Not from any measured document: the coefficients an arc's forms carry
come from `f64` literals whose mantissas are 53 bits, and a product of
three of those is already past `i128` and is kept as `Int::Big`, which
`read_argument` rejects outright (`let super::Int::Small(k) = q.num
else { return None }`). What is needed is a coefficient that FITS in
`i128` while its dyadic exponent pushes the product past it — e.g. two
literals of order `1e30` multiplied onto an `atan` atom
(`num` ≈ 106 bits, `exp2` ≈ 94). No CAD dimension looks like that.

It matters anyway because it is the one place in rule D where the
stated contract ("only an argument whose form is EXACTLY `q · A` …
with `|k| ≤ MAX_MULTIPLE`") is not what the code enforces, and because
the tier's whole claim is that a symbolic `Zero` is a theorem with no
caveat about the size of a coefficient. The fix is one line —
`k.checked_mul(2_i128.checked_pow(e)?)?`, or a `bits(k) + e ≤ 127`
guard before the shift — and it costs nothing.
