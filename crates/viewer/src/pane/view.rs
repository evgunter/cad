//! The View pane: display tolerance, datums and the camera's state.
//!
//! Module kind: **driver** (`crates/viewer/README.md`, The drivers).

use eframe::egui;

use crate::app::ViewerBehavior;

impl ViewerBehavior<'_> {
    /// The view pane: the numbers the camera and the tessellation are
    /// actually running at.
    pub(crate) fn view_ui(&mut self, ui: &mut egui::Ui) {
        let stats = self.scene.stats();
        ui.heading("View");
        // One δ, because there is only ever one: the budget chose it
        // when the document opened or the user did, and the note says
        // which. The triangle count below is the picture's own, so
        // nothing here is a prediction.
        self.delta_ui(ui);
        if self.budget_delta.is_some() {
            ui.weak("chosen for the triangle budget; δ is yours from here");
        }
        ui.label(format!("faces: {}", stats.faces));
        ui.label(format!("triangles: {}", stats.triangles));
        ui.separator();
        // **Datum visibility, and why it is a switch at all.**
        // Construction geometry is drawn over the part, which is
        // where it has to be for a plane to say what it cuts — and it
        // is also in the way once a document has several. A view
        // setting rather than a document one: which datums exist is
        // the recipe's business, and whether this window draws them is
        // this window's.
        ui.checkbox(self.show_datums, "show datums");
        ui.separator();
        ui.label(format!(
            "camera yaw {:.1}°, pitch {:.1}°",
            self.camera.yaw().to_degrees(),
            self.camera.pitch().to_degrees()
        ));
        ui.label(format!(
            "distance {:.1} mm (band {:.1}–{:.1})",
            self.camera.distance() * 1000.0,
            self.camera.min_distance() * 1000.0,
            self.camera.max_distance() * 1000.0
        ));
        ui.separator();
        ui.label(format!("history: {} states", self.session.history().len()));
        match self.session.path() {
            Some(path) => ui.label(format!("file: {}", path.display())),
            None => ui.weak("unsaved document"),
        };
        if let Some(status) = self.status.as_ref() {
            ui.separator();
            ui.label(status.as_str());
        }
    }

    /// The δ control: the display tolerance as a number the user types,
    /// in millimetres.
    ///
    /// **A text field rather than a pair of step buttons.** δ is a
    /// LENGTH, and the question a user has is "how fine, in mm" — a
    /// halve/double pair answers it only by repeated clicking and
    /// cannot reach a number in between. It is also not a `DragValue`:
    /// a drag would commit a tessellation per frame, which is the one
    /// mistake [`crate::widgets::drag_ops`] exists to keep out of this file.
    ///
    /// Committing on lost focus covers Enter too — egui's singleline
    /// field surrenders focus on Enter — so there is one commit path,
    /// not two. What is typed is a DRAFT until then: nothing
    /// re-tessellates while a number is half-entered, and `0.` on the
    /// way to `0.05` never reaches the tessellator.
    pub(crate) fn delta_ui(&mut self, ui: &mut egui::Ui) {
        let in_force = self.delta.get() * 1.0e3;
        let field = ui
            .horizontal(|ui| {
                let text = self
                    .drafts
                    .delta_mm
                    .get_or_insert_with(|| format!("{in_force:.3}"));
                let field = ui.add(egui::TextEdit::singleline(text).desired_width(56.0));
                ui.label("mm display δ");
                field
            })
            .inner;
        if field.lost_focus()
            && let Some(typed) = self.drafts.delta_mm.take()
        {
            match typed.trim().parse::<f64>() {
                // Judged by `DisplayTolerance`, not here: a δ that is
                // not a finite positive length is refused at that one
                // door, wherever it came from.
                Ok(mm) => *self.delta_request = Some(mm * 1.0e-3),
                Err(error) => {
                    *self.status = Some(format!(
                        "display δ: {:?} is not a number ({error})",
                        typed.trim()
                    ));
                }
            }
        }
        // The draft lives exactly as long as the focus does, so a δ
        // that moved under the field (the budget's choice on open)
        // shows up in it.
        if !field.has_focus() {
            self.drafts.delta_mm = None;
        }
    }
}
