---
id: trait-generic-sole-bracket
kind: issue
title: bounds-allowlist.sh's trait-declaration alternative fires on a trait generic over a SOLE bracket bound
status: closed
opened: 2026-09-03
track: K
refs: [D102, D68]
branch: gates/bounds-small
pr: 2029
closed: 2026-09-06
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

## Repair (branch `gates/bounds-small`)

The third alternative is now `gate_trait_declarations`, an awk reader
that skips a balanced `<…>` after the trait name and then applies the
same test the regex applied (`^[^;{]*:[^;{]*…(Bounds|Enclosure)` on what
follows). It is one function in two modes: `records` feeds the scan
through the new `gate_matcher`, `names` feeds the alias census, so the
skip cannot be carried by one reader and not the other again — which is
how the false positive arose. An ERE cannot express the skip, which is
why the alternative left the regex.

It is **not** the same matcher either side of the skip, and the style
review measured the two differences: `match()` takes the FIRST `trait`
token on a line, so a second declaration written after a first on ONE
line is no longer read; and a `;`/`{` inside the skipped generic list no
longer stops the search, so `trait Arr<T: Array<[u8; 4]>>: Bounds {}` is
read where the regex's `[^;{]*` could not cross it. Both have zero live
population and both move toward the answer the rule wants; they are
stated at the function rather than claimed away as identity.

`plant_sole_bracket_bounds` gains four `trait` forms: bare, depth-2
nested (`trait Carrier<T: CertifiedBounds, P: ControlPoint<T>>`), and an
`Fn(u8) -> u8` parameter in both orders — the arrow's `>` closed the list
early for the first depth counter, so the tail handed back carried the
real parameter colons and the false positive survived. Every fixture was
shown to red against the matcher without its fix.

Hit-set diff on the live tree, matcher records surviving the definition
skip and before the per-file filters: 163 records over 26 files, byte
identical before and after; the one live alt-3 hit
(`profile/src/path/arc_fillet.rs:924`) has no generic list. The census's
declaration set is unchanged at one entry.

## Claimed by GATES (2026-09-06)

Moved from `work/code-quality/` to `work/gates/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track K; `bounds-allowlist.sh`'s third matcher alternative.
