---
id: fillet-leg-carrier-renders-raw-float-noise
kind: issue
title: FilletLegCarrier::Arc renders its radius and angular margin through f64's Display, so a fillet refusal carries the arithmetic's noise
status: open
opened: 2026-09-12
---


Found by the sweep obligation on
`num-relative-tolerance-collides-above-a-decimetre`, whose blind-spot
section named `crates/profile/src/validate.rs` as the first place to
look. It is the only hit, and it is the OPPOSITE defect from the one
that unit fixed: not a collision, but no shortening at all.

## The defect

`crates/profile/src/validate.rs:264-277`:

```rust
impl fmt::Display for FilletLegCarrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line => f.write_str("straight"),
            Self::Arc { radius, angular_margin } => write!(
                f,
                "circular (carrier radius {radius} m, angular margin {angular_margin} rad)"
            ),
        }
    }
}
```

Both fields are `f64` (`validate.rs:258`, `:260`). `{radius}` reaches
`f64`'s own `Display`, which is the shortest round-tripping spelling —
the arithmetic's noise, unshortened. An 8 mm carrier a subtraction
produced renders as:

```
circular (carrier radius 0.008000000000000002 m, angular margin 0.0034999999999999996 rad)
```

That is exactly the sentence `path::num` exists to prevent, and it is
the one unit-bearing scalar rendering in `crates/profile/src/` that
does not route through it (swept for `{ident} m` / `{ident} rad` over
the whole of `crates/profile/src/`; `path.rs`'s 38 sites all bind
`x = num(x)`, and `ProfileError`'s arms render only indices and
counts).

**The reach is not confined to `validate.rs`.** `FilletLegCarrier`'s
`Display` is interpolated into `CornerReason`'s sentences in
`path.rs` — the `{carrier}` of
`CornerReason::AnchorOutsideTrimmedExtent` (`path.rs:722`) is this
impl — so a `path` refusal whose every own scalar is shortened still
carries an unshortened one inside its carrier clause.

## The shape of the fix

`num` is private to the `path` module (zero call sites outside
`path.rs`, verified), so `validate.rs` cannot reach it as it stands.
Either promote `num` to a crate-private helper both modules import, or
give `FilletLegCarrier` its own. Promoting is the better answer —
one grid for the crate's refusals is the property worth having, and a
second copy is a second thing to get wrong — but it moves a helper
whose doc comment carries the ratified argument for the grid, so it is
a small unit rather than a one-liner.

Whatever carries it owes a pin on the rendered sentence: no existing
row asserts any `FilletLegCarrier::Arc` text at all.
