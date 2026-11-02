---
id: the-feature-trees-hide-toggles-an-undrawn-instance-silently
kind: issue
title: the feature tree's instance checkbox toggles an undrawn instance's hide with nothing to say why the picture does not change
status: open
opened: 2026-11-01
---


Found in the dual review of PR 6265.

The feature tree draws a "shown" checkbox on instance rows only (`feature_row_ui`, `crates/viewer/src/pane/features.rs`). Since PR 6265 it pushes `SessionOp::SetBodyHidden`, which `display::body_check` admits for any body, drawn or not. An instance that an operation reads as a body (a pattern's source, a union's operand) is not drawn. So its checkbox flips and the picture does not change, with nothing on screen saying why. Before PR 6265 the fused case refused `FusedGeometry`, and that notice was disabled under the checkbox; that notice is gone with the propagation it described.

**What it should do.** The per-body and per-space toggles belong to the View pane (`the-view-pane-toggles-spaces-and-bodies`). There a body the default leaves out is listed as such, and its toggle shows it. The tree's instance-only checkbox either goes, with the View pane's toggles replacing it, or says when its body is not drawn (`world::is_drawn`).

Done when: no hide toggle flips a body the picture does not draw without the control saying so. A row drives the tree or the View pane over an instance a pattern reads.
