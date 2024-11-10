use wgpu_ui::run;

fn main() {
    init_logger();
    log::info!("Running!");
    pollster::block_on(run());
}

fn init_logger() {
    let mut build = env_logger::builder();
    build.filter(Some("wgpu_ui"), log::LevelFilter::Info);
    build.filter(Some("wgpu"), log::LevelFilter::Info);
    build.init();
}
