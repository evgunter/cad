---
id: fillet-leg-carrier-renders-raw-float-noise
kind: issue
title: FilletLegCarrier::Arc renders its radius and angular margin through f64's Display, so a fillet refusal carries the arithmetic's noise
status: closed
opened: 2026-09-12
branch: fix/fillet-leg-carrier-num
pr: 2946
priority: P4
cost: E
closed: 2026-09-21
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

## Closed (2026-09-21) — PR 2946

`FilletLegCarrier::Arc`'s two `f64` fields render through
`path::num`, which is `pub(crate)` now rather than private to
`path.rs`. The sentence reads *"circular (carrier radius 0.008 m,
angular margin 0.0035 rad)"* where it read eighteen significant figures
of noise.

**The reach was wider than the row said, and the lane found it.**
`FilletLegCarrier`'s `Display` is interpolated as
`CornerReason::AnchorOutsideTrimmedExtent`'s `{carrier}`, whose own
`{setback}` and `{available}` were already shortened — so a `path`
refusal free of noise in every scalar of its own still carried noise
inside its carrier clause. The pin reads that composite sentence, not
just the carrier's.

**The pin states the defect rather than illustrating it.** Its two
scalars are SUBTRACTED, not typed: `0.008` and `0.0035` are each
exactly representable, so a literal would have rendered correctly with
no helper at all and proved nothing. `0.017 - 0.009` and
`0.0135 - 0.01` are what arithmetic lands on, and the row asserts the
premise (`assert_ne!(radius, 0.008)`) before asserting the rendering,
so it cannot pass by the subtraction quietly becoming exact. It also
asserts `to_bits()` equality: the shortening is a DISPLAY choice and
the payload keeps what the subtraction gave it.

**Nothing was re-baselined, and that is the finding.** No golden, count
or stored expectation moved, because nothing had ever pinned this
sentence — every consumer in the tree matches on the enum's FIELDS.
The old sentence could have shipped indefinitely. Instruction 3's "no"
again, and again the missing pin was part of the defect.

**The sweep, and the instrument that covered its blind spot.** 26 lines
and 36 interpolations of `{ident} m` / `{ident} rad` across
`crates/profile/src/`, all but this one already routed. Because that
pattern keys on a unit word, the lane ALSO enumerated all 22 `Display`
impls in the crate and read each body — which caught
`CornerRefusal` rendering two ordinates with no unit word at all, a
case the pattern could never have seen (already routed). Residual
stated honestly: a scalar reaching prose through a containing type's
`Debug` evades both instruments.

**Fence:** the whole unit is PATHS's ground (`crates/profile/src/`),
crossed by announcement; posted on `work/paths/log.md`.

**The fence question the lane flagged rather than decided, and my
ruling on it.** `num` is `pub(crate)`, the narrowest visibility serving
a sibling module. `work/props/patherror-display-renders-float-noise`
says *"a second consumer of `num` outside `path.rs` is exactly what
forces the home question"* — and the lane asked whether a second MODULE
trips that, or only a second CRATE. **A second crate.** The sentence
sits in a paragraph about where the helper LIVES across crates and
names `geom-core` as the candidate home; nothing crossed a crate
boundary here, nothing is re-exported, and PROPS's question is exactly
as open as it was. Recorded because the lane was right to ask rather
than assume.

**Filed:** no new row. Evidence appended to that PROPS row, including a
correction it needs: the row motivates the helper from `Real` carrying
`Debug` and no `Display`, but these fields are plain `f64`, whose own
`Display` is the same shortest round-tripping spelling — so the
`f64`-typed door reaches the identical defect and that row's sweep of
other crates' error types must look for `f64` fields too.
