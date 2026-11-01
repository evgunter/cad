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
        assert!(
            !driver.shows("Add profile"),
            "the open section offers the form's door, not the form"
        );
        driver.click("Add profile…", 0).expect("the form opens");
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
        driver.click("Create document", 0).expect("created");
        driver.settle().expect("settles");
        driver.click("Add feature", 0).expect("the section opens");
        driver.click("Add profile…", 0).expect("the form opens");
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
            matches!(row("Revolve 2").status, viewer::tree::RowStatus::Ok),
            "the second is the sphere: {said:#?}"
        );
        let (doc, _) = driver.session().landed_pair().expect("landed");
        let copies = crate::common::drawn_copies(doc);
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

    /// **After a duplicate, a click on either body reaches the tool** —
    /// the replay of
    /// `work/chrome/after-a-duplicate-the-original-cannot-be-picked-into-a-tool.md`.
    /// The duplicate's copy lands beside the plate, so the plate's body
    /// is read by two placements. With the boolean tool open, a click on
    /// the original's material seats the plate's extrude, as it does with
    /// no duplicate, and a click on the copy seats the copy's placement.
    /// The replay's own point, framed with both bodies in view, falls in
    /// the original's hole: a miss, which seats nothing.
    #[test]
    fn after_a_duplicate_a_click_on_either_body_seats_it() {
        let mut driver = started("drive-duplicate-pick");
        driver.resize(egui::vec2(1280.0, 1600.0));
        driver.settle().expect("settles");
        let vclick = |driver: &mut Driver, fx: f32, fy: f32| {
            let at = driver.in_viewport(fx, fy).expect("the viewport");
            driver
                .click_at(at, egui::PointerButton::Primary, egui::Modifiers::NONE, 1)
                .expect("clicked");
            driver.settle().expect("settles");
        };
        driver
            .click("Combine bodies", 0)
            .expect("the section opens");
        driver.settle().expect("settles");
        driver.click("Duplicate tool…", 0).expect("the tool opens");
        vclick(&mut driver, 0.3, 0.4);
        driver.click("Commit duplicate", 0).expect("committed");
        driver.settle().expect("settles");
        driver.click("Zoom to fit", 0).expect("framed");
        driver.settle().expect("settles");
        driver.click("Boolean tool…", 0).expect("the tool opens");
        driver.settle().expect("settles");
        let members = |driver: &Driver| {
            driver
                .screen()
                .iter()
                .find(|landed| landed.text.starts_with("member 1") || landed.text == "no picks yet")
                .map(|landed| landed.text.clone())
                .expect("the boolean tool's picks line")
        };
        // The replay's point: the hole in the original.
        vclick(&mut driver, 0.3, 0.47);
        assert_eq!(
            members(&driver),
            "no picks yet",
            "the replay's click is a miss"
        );
        // The original's material, beside the hole.
        vclick(&mut driver, 0.2, 0.47);
        let picked = members(&driver);
        assert!(
            picked.starts_with("member 1: Extrude "),
            "a click on the original seats its extrude: {picked}"
        );
        // The copy.
        vclick(&mut driver, 0.7, 0.55);
        let picked = members(&driver);
        assert!(
            picked.contains("member 2: Place \"Part 1\""),
            "a click on the copy seats the copy's placement: {picked}"
        );
    }

    /// **A click that meets nothing while a tool is open says so**: the
    /// boolean tool takes no pick from the ground around the plate, and
    /// the line says that rather than leaving `no picks yet` as the only
    /// answer to the click.
    #[test]
    fn a_click_on_nothing_with_a_tool_open_says_no_pick_was_taken() {
        let mut driver = started("drive-missed-pick");
        driver
            .click("Combine bodies", 0)
            .expect("the section opens");
        driver.settle().expect("settles");
        driver.click("Boolean tool…", 0).expect("the tool opens");
        driver.settle().expect("settles");
        let corner = driver.in_viewport(0.02, 0.02).expect("the viewport");
        driver
            .click_at(
                corner,
                egui::PointerButton::Primary,
                egui::Modifiers::NONE,
                1,
            )
            .expect("clicked");
        driver.settle().expect("settles");
        assert!(
            driver.shows("no picks yet"),
            "the premise: the click took nothing"
        );
        assert!(
            driver.shows("boolean tool: nothing under the cursor — no pick taken"),
            "the miss is said on the line: {}",
            driver.dump()
        );
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

    /// **The split panel seeds its plane through the body it picked,
    /// says the staged split's verdict, and the viewport draws the plane
    /// on that body** (row `the-split-tools-plane-is-typed-blind`): the
    /// row's own replay, through the chrome. The startup plate picked
    /// in the viewport seeds the plane's point at the plate's centre,
    /// the panel reads the door's verdict that it cuts, and the plane
    /// is drawn at the plate's mid-height, where its world copy stands.
    ///
    /// *Breaks if* the panel stops seeding (the old default lies on the
    /// plate's bottom face, and the panel would say so), stops asking
    /// the staged split, or the preview is not drawn on the picked copy.
    #[test]
    fn the_split_tool_seeds_its_plane_through_the_picked_plate_and_draws_it_there() {
        let mut driver = started("drive-split-plane");
        driver.click("Zoom to fit", 0).expect("the fit");
        driver
            .click("Combine bodies", 0)
            .expect("the section opens");
        driver.click("Split tool…", 0).expect("the tool opens");
        let plate = driver.in_viewport(0.3, 0.45).expect("a viewport");
        driver
            .click_at(
                plate,
                egui::PointerButton::Primary,
                egui::Modifiers::NONE,
                1,
            )
            .expect("the pick settles");
        let bounds = viewer::scene::plate_bounds();
        let mid = 0.5 * (bounds.min_z + bounds.max_z);
        let origin = driver.split_origin();
        assert!(
            (origin[2] - mid).abs() < 1e-12,
            "the panel seeds the plane at the plate's centre: {origin:?}"
        );
        assert!(
            driver.shows("the plane cuts the body in two"),
            "the panel reads the staged split's verdict: {}",
            driver.dump()
        );
        let plane = driver
            .split_plane()
            .expect("the plane is drawn on the picked plate");
        assert!(
            (plane.centre.z - mid).abs() < 1e-9,
            "the plane is drawn at the plate's mid-height: {:?}",
            plane.centre
        );
    }
}

/// Review rows for PR 6239: the miss notice's negative and lifetime
/// halves, which the PR's own row does not pin.
#[cfg(feature = "app")]
mod missed_pick_bounds {
    use crate::common;
    use viewer::app::drive::Driver;

    fn started(label: &str) -> Driver {
        Driver::start(
            common::tempdir(label).join("prefs.toml"),
            pncad::tolerance::witness(),
        )
        .expect("the app starts")
    }

    fn vclick(driver: &mut Driver, fx: f32, fy: f32) {
        let at = driver.in_viewport(fx, fy).expect("the viewport");
        driver
            .click_at(at, egui::PointerButton::Primary, egui::Modifiers::NONE, 1)
            .expect("clicked");
        driver.settle().expect("settles");
    }

    /// **With no tool open a miss stays quiet**: it is a deselect.
    #[test]
    fn a_miss_with_no_tool_open_says_nothing() {
        let mut driver = started("missed-pick-no-tool");
        vclick(&mut driver, 0.02, 0.02);
        assert!(
            !driver.dump().contains("no pick taken"),
            "{}",
            driver.dump()
        );
    }

    /// **The miss names what the tool's pick kind did not find**: under
    /// the blend tool, which picks edges only, a click on the plate's
    /// material away from every edge met a face, not nothing.
    #[test]
    fn an_edges_only_tool_says_no_edge_was_under_the_cursor() {
        let mut driver = started("missed-pick-blend");
        driver.click("Blend edges", 0).expect("the section opens");
        driver.settle().expect("settles");
        driver.click("Blend tool…", 0).expect("the tool opens");
        driver.settle().expect("settles");
        vclick(&mut driver, 0.35, 0.5);
        let dump = driver.dump();
        assert!(
            dump.contains("blend tool: no edge under the cursor — no pick taken"),
            "{dump}"
        );
        assert!(!dump.contains("nothing under the cursor"), "{dump}");
    }

    /// **Closing the tool retires its miss**: once the boolean tool is
    /// cancelled, nothing on screen says it took no pick.
    #[test]
    fn closing_the_tool_retires_its_miss() {
        let mut driver = started("missed-pick-close");
        driver
            .click("Combine bodies", 0)
            .expect("the section opens");
        driver.settle().expect("settles");
        driver.click("Boolean tool…", 0).expect("the tool opens");
        driver.settle().expect("settles");
        vclick(&mut driver, 0.02, 0.02);
        assert!(driver.dump().contains("no pick taken"), "the premise");
        driver.click("Cancel", 0).expect("the tool closes");
        driver.settle().expect("settles");
        let dump = driver.dump();
        assert!(!dump.contains("no pick taken"), "{dump}");
    }

    /// **Repeated misses say it once, and a pick retires it**: the line
    /// carries one telling however many misses, and the click that
    /// seats something clears it.
    #[test]
    fn repeated_misses_say_it_once_and_a_pick_retires_it() {
        let mut driver = started("missed-pick-repeat");
        driver
            .click("Combine bodies", 0)
            .expect("the section opens");
        driver.settle().expect("settles");
        driver.click("Boolean tool…", 0).expect("the tool opens");
        driver.settle().expect("settles");
        vclick(&mut driver, 0.02, 0.02);
        vclick(&mut driver, 0.03, 0.03);
        vclick(&mut driver, 0.04, 0.02);
        let dump = driver.dump();
        assert_eq!(dump.matches("no pick taken").count(), 1, "{dump}");
        // Beside the plate's central hole, on its material.
        vclick(&mut driver, 0.35, 0.5);
        let dump = driver.dump();
        assert!(dump.contains("member 1: Extrude "), "the premise: {dump}");
        assert!(!dump.contains("no pick taken"), "{dump}");
    }
}
