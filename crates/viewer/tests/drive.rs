//! **The scripted driver reaches the session through the chrome**:
//! a control found by its label and clicked where it was painted
//! performs what clicking it at the window performs, and a label that
//! is not on screen is refused rather than clicked at a guess.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

test_utils::loud_skip_marker!(
    feature = "app",
    row = app_lane_skipped_no_driver_coverage_here,
    absent = "coverage of the scripted driver",
);

#[cfg(feature = "app")]
mod driven {
    use crate::common;
    use viewer::app::drive::{DriveError, Driver};

    /// The startup document, driven, over a preferences file of its own.
    fn started(label: &str) -> Driver {
        Driver::start(
            common::tempdir(label).join("prefs.toml"),
            pncad::tolerance::witness(),
        )
        .expect("the app starts")
    }

    #[test]
    fn a_profile_added_through_the_chrome_lands_in_the_tree() {
        let mut driver = started("drive-profile");
        let before = driver.session().tree_rows().len();
        assert!(
            !driver.shows("Add profile"),
            "the creation forms start folded"
        );
        driver.click("Add feature", 0).expect("the section opens");
        driver.click("rectangle", 0).expect("the shape is chosen");
        driver
            .click("Add profile", 0)
            .expect("the profile is added");
        let rows = driver.session().tree_rows();
        assert_eq!(rows.len(), before + 1, "one node per Add");
        let added = rows.last().expect("a row").spoken.to_string();
        assert!(
            added.contains("Profile 2"),
            "the new row is the profile the form named: {added}"
        );
        assert!(
            driver.dump().contains("Profile 2"),
            "and the feature tree on screen shows it"
        );
    }

    /// **A failed feature takes away its own picture and no other's**
    /// — the replay of `work/chrome/a-failed-feature-hides-every-later-edit.md`.
    /// A revolve that refuses (the default axis, along the semicircle's
    /// own diameter) leaves its copy undrawn; a second revolve of the
    /// same profile, at heading 0, is the sphere, and it is drawn and
    /// picked. Before, the first failure refused the whole index and the
    /// sphere never reached the picture.
    #[test]
    fn a_failed_revolve_does_not_hide_the_sphere_revolved_after_it() {
        let mut driver = started("drive-partial-picture");
        driver.resize(egui::vec2(2400.0, 1600.0));
        driver.click("New…", 0).expect("the new-document form");
        driver.click("document name", 0).expect("the name field");
        driver.type_text("ball").expect("typed");
        driver.click("Create", 0).expect("created");
        driver.settle().expect("settles");
        driver.click("Add feature", 0).expect("the section opens");
        driver.click("path", 0).expect("the path shape");
        driver.click("line_to", 0).expect("a line row");
        driver.click("arc_to", 0).expect("an arc row");
        driver.click("0.50", 0).expect("the arc's field");
        driver.type_text("1").expect("typed");
        driver
            .key(egui::Key::Enter, egui::Modifiers::NONE)
            .expect("entered");
        driver.click("×", 2).expect("a row removed");
        driver.click("×", 2).expect("another row removed");
        driver.settle().expect("settles");
        driver
            .click("Add profile", 0)
            .expect("the profile is added");
        driver.settle().expect("settles");
        for heading in [None, Some("0")] {
            driver.click("Revolve tool…", 0).expect("the tool opens");
            driver.settle().expect("settles");
            // The tree row, by the prefix its fresh tag follows.
            let row = driver
                .screen()
                .iter()
                .filter(|landed| landed.text.starts_with("Profile 1 "))
                .find_map(|landed| landed.visible().map(|rect| rect.center()))
                .expect("the profile's tree row is on screen");
            driver
                .click_at(row, egui::PointerButton::Primary, egui::Modifiers::NONE, 1)
                .expect("the profile is picked");
            driver.settle().expect("settles");
            if let Some(heading) = heading {
                driver.click("0.500", 0).expect("the heading field");
                driver.type_text(heading).expect("typed");
                driver
                    .key(egui::Key::Enter, egui::Modifiers::NONE)
                    .expect("entered");
            }
            driver.click("Commit revolve", 0).expect("committed");
            driver.settle().expect("settles");
        }

        let rows = driver.session().tree_rows();
        let said: Vec<String> = rows
            .iter()
            .map(|row| format!("{} {:?}", row.spoken, row.status))
            .collect();
        let row = |label: &str| {
            rows.iter()
                .find(|row| row.spoken.to_string().contains(&format!("\"{label}\"")))
                .unwrap_or_else(|| panic!("a {label} row: {said:#?}"))
        };
        assert!(
            matches!(
                row("Revolve 1").status,
                viewer::tree::RowStatus::Failed { .. }
            ),
            "the first revolve fails: {said:#?}"
        );
        assert!(
            matches!(row("Revolve 2").status, viewer::tree::RowStatus::Ok { .. }),
            "the second is the sphere: {said:#?}"
        );
        let (doc, _) = driver.session().landed_pair().expect("landed");
        let copies = viewer::world::drawn_copies(doc);
        assert_eq!(copies.len(), 2, "one copy per revolve: {said:#?}");
        let index = driver
            .index()
            .expect("the index builds over the failed copy");
        assert_eq!(
            index
                .undrawn()
                .iter()
                .map(|copy| copy.node())
                .collect::<Vec<_>>(),
            vec![copies[0]],
            "the failed revolve's copy, and only it, is undrawn"
        );
        assert!(
            !index.ids_of_node(copies[1]).is_empty(),
            "the sphere's copy is indexed"
        );
        assert!(
            driver.scene().stats().triangles > 0,
            "and the sphere is on screen"
        );
        assert_eq!(
            driver.index_badge(),
            None,
            "the tree's FAILED row is the notice; the toolbar adds none"
        );
        let centre = driver.in_viewport(0.5, 0.5).expect("the viewport");
        driver
            .click_at(
                centre,
                egui::PointerButton::Primary,
                egui::Modifiers::NONE,
                1,
            )
            .expect("clicked");
        driver.settle().expect("settles");
        match driver.session().selection() {
            viewer::session::Selection::Face(face) => {
                assert_eq!(face.node, copies[1], "the pick lands on the sphere");
            }
            other => panic!("a click on the sphere picks a face of it: {other:?}"),
        }
    }

    /// **The picture is the window's**: as many pixels as the window
    /// has, and inside the viewport the startup part is drawn — the
    /// pane's pixels are not all the ground it is painted on.
    #[test]
    fn a_picture_shows_the_window_with_the_part_drawn() {
        let driver = started("drive-picture");
        let (device, queue) = pollster::block_on(viewer::app::drive::picture_device())
            .expect("a device to render on");
        let picture = driver.picture(&device, &queue).expect("the window renders");
        let window = driver.window();
        assert_eq!(
            picture.size,
            [window.x as u32, window.y as u32],
            "one pixel per point at the driver's scale"
        );
        let viewport = driver.viewport().expect("the viewport was drawn");
        let pixel = |x: f32, y: f32| {
            let at = (y as usize * picture.size[0] as usize + x as usize) * 4;
            &picture.rgba[at..at + 4]
        };
        // The corner is ground: the startup view frames the part with
        // a margin around it.
        let ground = pixel(viewport.left() + 2.0, viewport.top() + 2.0);
        let (mut drawn, mut all) = (0_usize, 0_usize);
        for y in (viewport.top() as u32..viewport.bottom() as u32).step_by(4) {
            for x in (viewport.left() as u32..viewport.right() as u32).step_by(4) {
                all += 1;
                if pixel(x as f32, y as f32) != ground {
                    drawn += 1;
                }
            }
        }
        assert!(
            drawn * 10 > all,
            "the startup part covers more than a tenth of the viewport: {drawn} of {all} \
             sampled pixels are not the ground"
        );
    }

    #[test]
    fn a_label_not_on_screen_is_refused_with_what_was_meant() {
        let mut driver = started("drive-refusal");
        match driver.click("add feature", 0) {
            Err(DriveError::NotOnScreen { shown, near, .. }) => {
                assert_eq!(shown, 0, "labels match exactly, case included");
                assert!(
                    near.iter().any(|label| label == "Add feature"),
                    "the refusal names the label it was near: {near:?}"
                );
            }
            other => panic!("a mis-cased label was not refused: {other:?}"),
        }
        assert!(
            matches!(
                driver.find("Add feature", 1),
                Err(DriveError::NotOnScreen { shown: 1, .. })
            ),
            "a second painting of a label painted once is not on screen"
        );
    }
}
