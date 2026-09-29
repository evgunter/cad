---
id: startup-notices-door-takes-text
kind: issue
title: frame::startup_notices takes &[String], so the last message door a type does not pin accepts anyone's text
status: open
opened: 2026-09-28
priority: P1
cost: M
refs: [document-news-has-no-home, startup-notices-need-holding-to-badge]
---


## Finding

Found by `vnews/tool-news-is-a-value`'s sweep of `frame.rs`'s message
doors (every `pub fn` whose return is a `Message`, a `Badge` or a
`StatusUpdate`, read by parameter type). With `frame::tool_news` now
taking `tools::PanelRefusal`, the one production door left whose
input is text is `frame::startup_notices(notices: &[String])`
(`crates/viewer/src/frame.rs`, ~:1898). Its doc says so ("**Not
type-pinned**") and argues that what it buys is one place the subject
is decided.

That argument is the one `tool_news` made before it took a value, and
the same answer is available: every source is already typed where
`app::ViewerApp::new` collects it (`crates/viewer/src/app.rs`, the
`store.load()` match ~:777 onward) — `prefs::Notice` (the file's own
complaints, and the theme and keys resolutions), `prefs::PrefsError`,
`prefs::StoreError` — and each is rendered with `to_string()` before
it reaches the door. The one sentence written at the site is the
unreadable launch directory (`format!("launch directory unreadable
({error}); …")`, ~:802). A `StartupNotice` enum over those four, with
a `Display`, would pin the door the way `PanelRefusal` pins
`tool_news`, and the launch-directory sentence would get a home in a
type rather than at a `format!`.

## Sibling

`work/vseam/startup-notices-need-holding-to-badge` would change this
door anyway (holding the notices means holding values, which is the
typed shape this row asks for). Whoever takes either should read both.
