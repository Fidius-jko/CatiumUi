use wgpu_ui::run;

fn main() {
    init_logger();
    log::info!("Running!");
    match pollster::block_on(run()) {
        Ok(()) => {}
        Err(e) => {
            log::error!("Running error: {e}")
        }
    }
}

fn init_logger() {
    let mut build = env_logger::builder();
    build.filter(Some("wgpu_ui"), log::LevelFilter::Info);
    build.init();
}
