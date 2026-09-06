---
id: adjacent-same-typed-arguments-are-the-same-swap
kind: issue
title: the adjacent-swappable-argument class extends past bool and viewer/src has a dozen, transform_node's two [Expr; 3] worst
status: open
opened: 2026-09-06
---



PR #2055 names this class as its sweep's blind spot — *"same-typed
non-`bool` adjacencies (two `usize`, two `RecipeNodeId`), which are the
same class one type away"* — and stops there. A blind spot that is
named but never counted is not a negative result, and the count is not
small: parsing every `fn` header in `crates/viewer/src` for adjacent
same-typed parameters finds roughly a dozen, and the argument #2055
makes for `Outstanding` applies to each of them unchanged. Two of them
are worse than the `bool` pair the unit fixed, because their swap
survives further.

**The worst: two `[Expr; 3]` relayed positionally over two hops.**

    crates/viewer/src/combine.rs:453
    pub fn transform_node(input: RecipeNodeId, translation: [Expr; 3],
                          rotation_axis: [Expr; 3], rotation_angle: Expr)

    crates/viewer/src/session.rs:1626   fn add_transform(… same four …)
    crates/viewer/src/session.rs:1637   combine::transform_node(input, translation,
                                                               rotation_axis, rotation_angle)

A translation vector and a rotation axis have nothing in common but
their type. Swapped, the call compiles, the node is built, the
evaluator accepts it and the body lands somewhere plausible — a
placement is exactly the kind of output a reader checks by looking at
it, which is #2055's own "the chrome it produces is plausible"
argument. The relay means one transposition at either hop is enough.

**The others**, with their swap consequence, because the class has a
gradient and a fix pass should not treat it as flat:

- `session.rs:1586` `add_boolean(op, a: RecipeNodeId, b: RecipeNodeId)`
  — both seats are kind-gated to `Body`, so nothing catches a swap and
  `Subtract` is not commutative.
- `session.rs:1612` `add_split(target: RecipeNodeId, tool: RecipeNodeId)`
  — swappable, but caught: the gate wants `Body` then `Plane`. Worth
  recording as the shape that is already defended, not as a hit.
- `session.rs:1568` `add_revolve(profile, axis)`, `display.rs:325`
  `derives_from(doc, node, source)`, `session/refuse.rs:308`
  `self_instance(open: DocumentId, id: DocumentId)`.
- `frame.rs:1658` `prefs_path_in(config_home: Option<&OsStr>,
  home: Option<&OsStr>)` — swapped, preferences are written under
  `$HOME/pncad/` or `$XDG_CONFIG_HOME/.config/pncad/`, and the
  test row at `crates/viewer/tests/prefs.rs` names its two readings
  positionally the same way the chooser row does.
- `pick.rs:408`/`445` `address(node, body: u32, position: usize,
  flat: usize)`, `bounds.rs:357` `midpoint(valid: f64, invalid: f64,
  …)`, `camera.rs:908` `clamp_distance(distance: f64,
  scene_radius: f64)`, `sketch.rs:1000` `arc_points(radius: f64,
  theta: f64, chord: f64)`.

## The test used

`rg` cannot read a multi-line signature. Every `fn` header under
`crates/viewer/src` was parsed, its parameter list split at top-level
commas, and every adjacent pair with textually identical types
reported. **What it cannot match**: parameters separated by another
argument, positional tuple-struct and struct-variant construction
(`Node::Boolean { op, a, b }` is field-named, but a tuple struct over
two same-typed fields is not), same-typed pairs behind differing
generic spellings that resolve to one type, and pairs whose types
differ but are freely inter-convertible.
