---
id: a-face-frames-tree-label-could-say-which-face
kind: issue
title: A face frame's tree label says whose face it is read off, not which face, though names now have words
status: open
opened: 2026-10-06
---


Found by the fourth review of PR 3886 (RECIPE, `names-render-a-faces-leaf-role-in-words`).

`viewer::tree::frame_pose` labels a `Datum::FaceFrame` "on <node>'s face" (`crates/viewer/src/tree.rs`, the `Node::Datum(Datum::FaceFrame { at, .. })` arm). Its doc said it could not say which face because a role path had no words; PR 3886 removed that reason (a name now has words: `editor_core::leaf_role`, `Doc::spoken_name`), and the doc now says only what the label does. Whether the label should name the face — a long phrase on a tree row, against the two frames a centimetre apart the label exists to tell apart — is this program's call.
