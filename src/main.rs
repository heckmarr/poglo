use egui;

pub struct PokeTheBunny {
    furname: String,
    age: i32,
}

impl PokeTheBunny {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Bunny coding");

    }
}

fn main() {

        let mut ctx = egui::Context::default();

        let input = egui::RawInput::default();
        ctx.begin_frame(input);

        egui::CentralPanel::default().show(&ctx, |ui|  {
            ui.label("Hallo furrkin!");
        });

        let full_output = ctx.end_frame();
}
