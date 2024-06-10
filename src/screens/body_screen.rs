
#[derive(Default)]
pub struct BodyScreen {
    id: usize,
    name: String,
    kind: String,
    surface_area: f64,
    orbital_data: usize,
    attached: usize,
    landed: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum BodyScreenMsgs {
    /// Rename the Body in question.
    Rename,
    PageUp,
    PageDown,
}

use iced::widget::{button, column, text, Column};

impl BodyScreen {
    pub fn view(&self) -> Column<BodyScreenMsgs> {
        column![
            button("+").on_press(BodyScreenMsgs::PageUp),
            text(self.id).size(50),
            text(&self.name).size(100),
            button("-").on_press(BodyScreenMsgs::PageDown)
        ]
    }

    pub fn update(&mut self, message: BodyScreenMsgs) {
        match message {
            BodyScreenMsgs::Rename => todo!(),
            BodyScreenMsgs::PageUp => self.id += 1,
            BodyScreenMsgs::PageDown => self.id -= 1,
        }
    }
}