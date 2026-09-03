---
id: trait-generic-sole-bracket
kind: issue
title: bounds-allowlist.sh's trait-declaration alternative fires on a trait generic over a SOLE bracket bound
status: open
opened: 2026-09-03
track: K
refs: [D102, D68]
---


## Was

unrowed. Found by the `D68` lane while building the alias roster's
declaration reader, which had to decide what a supertrait colon is.

## Finding

### The third alternative anchors on a `:` the trait's own generic list can supply, so a sole bracket bound on a type parameter reds

`scripts/gates/bounds-allowlist.sh`'s third matcher alternative is

```
(\btrait\s+\w+\b[^;{]*:[^;{]*\w*(Bounds|Enclosure)\b)
```

and `[^;{]*` between the trait name and the `:` spans the generic list.
So

```rust
pub trait ArrivalSpec<T: CertifiedBounds> {}
```

matches: `<T` is eaten by the first `[^;{]*`, the parameter's own colon
is read as the supertrait colon, and `CertifiedBounds` follows it.
Measured against the shipped gate under a scratch `--root` (one planted
crate plus the clean fixture): exit 1, `crates/planted/src/lib.rs`,
`compound Bounds/Enclosure bound outside the ratified seams`.

**That is a SOLE bracket bound**, which the gate's own
`plant_sole_bracket_bounds` pins as must-NOT-fire — *"a matcher that
fired on it would red geom-brep/src/ssi/enclose.rs, geom/src/net.rs and
both geom nurbs files"*. The bundle plants the shape as a `fn`, a
path-qualified `fn`, a bare `Bounds` `fn` and a `struct`; it does not
plant it as a `trait`, which is the one form that fires.

**`D102` records this construct as the hypothetical price of a
widening** — *"widening alternative three to drop its `:` requirement
false-positives on a trait generic over a SOLE bracket bound
(`trait ArrivalSpec<T: CertifiedBounds>`), which is outside this gate's
class"*. It does not need the widening. The `:` the alternative was
narrowed to require is already there, inside the angle brackets, and the
false positive is live in the gate as it stands.

**Population today: zero.** Swept `crates/*/src` through the gate's own
code-only view for `\btrait\s+\w+\s*<[^>]*:[^>]*\w*(Bounds|Enclosure)`
— no hits, and the tree is green. Swept `crates/` and `demos/` raw for
the same shape — no hits. **What the sweep could not match**: a
declaration whose generic list is broken across lines (the matcher is
line-based, so it would not fire either), and one produced by a macro.

**The shape it is waiting for is common**: twelve `trait X<T: SomeAlias>`
declarations exist under `crates/*/src` — `geom/src/net.rs:20`,
`profile/src/path.rs:{3373,3405,3443}`, `profile/src/lib.rs:263`,
`profile/src/path/family.rs:{378,775,812,962,1469}`,
`sweep/src/swept.rs:137`, `topo/src/ray_parity.rs:81`. Any one of them
acquiring a `…Bounds`-named parameter bound reds a construct the rule
allows, and the cheap green is a file entry — the cry-wolf-then-allowlist
outcome `S63` records at `linalg/mat.rs`.

**Not repaired in the `D68` lane, deliberately.** Narrowing the
alternative changes what this gate matches over a population nobody has
counted, which is `D102`'s grandfathering caveat, and `D102` is open. The
`D68` lane's alias census reads declarations through its own narrower
reader (`gate_alias_declaration_names`, which skips a balanced `<…>`
after the trait name) so the census does not inherit the false positive;
the main matcher is untouched. `plant_trait_generic_sole_bracket_ratified`
plants that narrowing on the census side only.

**Fix shape**, not prescribed: skip a balanced `<…>` after the trait name
in the alternative itself, the way the census reader does, and plant the
`trait` form into `plant_sole_bracket_bounds`. Whoever takes it owes the
before/after hit-set diff `D102` asks for.
