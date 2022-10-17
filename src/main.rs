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

        let ctx = egui::Context::default();

        loop {
            let raw_input = egui::RawInput::default();
            let full_output = ctx.run(raw_input, |ctx| {
                egui::CentralPanel::default().show(&ctx, |ui|  {
                    ui.label("Hallo world!");
                    if ui.button("Click me").clicked() {
                        println!("Boop");
                    }
                });
            });
 //           handle_platform_output(full_output.platform_output);
   //         let clipped_primitives = ctx.tessellate(full_output.shapes);
     //       paint(full_output.textures_delta, clipped_primitives);
        }

}
