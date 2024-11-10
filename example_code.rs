use wgpu_ui::prelude::*;

fn main() {
    let mut fw = Framework::new();
    fw.set_on_update(|fw: &mut Framework| {
        // ...
    });
    fw.start_dom(|| -> Dom { Dom::new(Css::new()) });
    fw.run();
}
