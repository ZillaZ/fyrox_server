//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use fyrox_server::GameConstructor;

fn main() {
    let mut executor = Executor::new();
    executor.set_headless(true);
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}
