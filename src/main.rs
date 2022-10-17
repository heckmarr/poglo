use egui;


fn main() {

        egui::__run_test_ctx(|ctx| {
                loop {
                    egui::CentralPanel::default().show(&ctx, |ui|  {
                    ui.add(egui::Label::new("hola"));
                    ui.label("Hall000000000000000000000000000000000000 out dere!");
                    });
            }


        });

}
