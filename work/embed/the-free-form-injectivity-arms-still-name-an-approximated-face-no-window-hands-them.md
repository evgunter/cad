---
id: the-free-form-injectivity-arms-still-name-an-approximated-face-no-window-hands-them
kind: issue
title: injective::by_kind's Nurbs | Approx arm and present's Spindle | Approx arm name an approximated face, which no window hands them since the window reads it as its fit
status: open
opened: 2026-11-25
priority: P4
cost: E
---


## Finding

`topo::clearance::window_of` reads an approximated face as its fit
(`carrier_lifted`, through `Surface::spline_chart`, OFFSET-DESIGN O2),
so every window's surface is `Surface::Nurbs` for such a face and no
reader downstream of a window is handed `Surface::Approx` (PR 7767,
lane `embedwalk-approx`). Two arms in `crates/topo/src/injective.rs`
still spell a policy for it:

- `by_kind`: `Surface::Nurbs(_) | Surface::Approx(_) => Err(Unsupported { carrier: "a free-form face, which no carrier kind makes injective", .. })`;
  the `Approx` half is dead for windows.
- `present`: `Surface::Spindle { .. } | Surface::Approx(_) => Err(Unpresented::Structural)`;
  the `Approx` half is dead for windows, and it says the opposite of
  what the face now gets (`present_chart`, as `Nurbs`).

Two spellings of one rule, one of them dead: the dead one reads as live
policy to the next reader. Also `crates/editor-core/tests/m10_5_r2_probes_interval.rs`
(`the_unsupported_arm_names_its_carrier_class_when_it_fires`) builds an
`EngineRefusal::Unsupported { carrier: "an approximated face", .. }`
literal; the engine no longer raises that text, though the row asserts
only `name()`.

## What is needed

Say in each arm that an approximated face reaches the engine as its fit
(fold `Approx` into the arm that reads it as `Nurbs`, or mark it
unreachable with that reason), and give the editor-core literal a
carrier text the engine still raises.

Filed by EMBEDWALK (lane `embedwalk-approx`), 2026-11-25.
