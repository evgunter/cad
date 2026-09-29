---
id: the-toolbars-status-line-runs-past-the-panel-below-a-floor-wide-window
kind: issue
title: viewer: in a window narrower than the message floor, the toolbar's status line is drawn past the panel's right edge
status: open
opened: 2026-09-22
priority: P2
cost: E
refs: [wrapping-at-a-region-with-no-floor-produces-a-four-character-ribbon]
---


`crates/viewer/src/app.rs`'s `toolbar_ui` draws the status line through
`crate::widgets::message` in the toolbar's `horizontal_wrapped` row. The
toolbar is an `egui::Panel`, not a scroll area, so the *then scroll*
half of the message floor has nothing to scroll: a line laid out wider
than the panel is drawn past its edge and cut off by the window.

## Measured (review of PR 3089, headless, through `app`'s `toolbar_with`)

The status fixture `STATUS` (`app::tests`), in the real toolbar:

- **150-point window**: the panel spans x = 8 to 142, and the three
  status lines end at 171.0, 170.0 and 174.4, which is about 32 points
  past it.
- **180-point window**: the panel spans x = 8 to 172, and the two lines
  end at 255.0 and 252.4, which is about 83 points past it.
- 300 and 400 points: inside the panel.

The reviewer reports the same overrun on main before the floor existed,
so it is pre-existing and not caused by the floor. Nothing covers the
toolbar below 400 points: `app::tests`'s `NARROW` is the narrowest
window any toolbar row drives. `the_toolbars_canceled_line_begins_every_line_in_the_same_place`
sweeps from 200 points but asserts alignment, not containment.

## What a fix has to decide

Either the toolbar is held to a width it can be narrower than (a
narrower `NARROW`, with the status line moved somewhere that scrolls or
wraps below the floor), or the floor does not apply in a container that
cannot scroll. That second option is a question for
`crate::widgets::message`'s doc, which today assumes every message sits
in a scroll area. Per Ev's ruling on the ribbon row, a site that
reaches the floor is a finding about the layout. Here the layout is a
status sentence in a one-row toolbar.

## Evidence: the document's name in the same row (`chrome/create-messages`, 2026-09-24)

The status line is not the only unbounded text in that toolbar row.
`crate::app`'s toolbar draws `ui.label(document_name(self.session.path()))`
into the same `horizontal_wrapped` row. `document_name` answers the
open file's stem, which the user chose, so nothing this crate controls
bounds its width. A label in a wrapping row starts beside whatever came
before it and continues at the panel's left edge. Unlike the status
line, it is not a `message`, so it has no floor. It is a NAME by
provenance and not by width. A fix to this row has to decide it too:
wrap it as a message, bound it by characters with the whole name on
hover, or give it its own line. Recorded here rather than fixed,
because `app.rs` was in another CHROME lane's diff.
