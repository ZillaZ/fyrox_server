//! Game project.
use fyrox::{
    core::{pool::Handle, futures::lock::Mutex},
    event::Event,
    gui::message::UiMessage,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::Scene, resource::model::{Model, ModelResourceExtension}, script::ScriptMessage,
};
use network::manager::NetworkManager;
use std::{path::Path, sync::{mpsc::Receiver, Arc}, net::TcpStream, rc::Rc, cell::RefCell};
use player::Player;

pub mod network;
pub mod player;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        _context.serialization_context.script_constructors.add::<Player>("Player");
    }

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    receiver: Receiver<TcpStream>
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(|| {
            let manager = NetworkManager::new(sender);
            manager.update();
        });

        Self {
            scene: Handle::NONE,
            receiver
        }
    }
    fn try_spawn_player(&mut self, context: &mut PluginContext<'_, '_>) {
        if let Ok(stream) = self.receiver.try_recv() {
            if let Some(scene) = context.scenes.try_get_mut(self.scene) {
                let model = context.resource_manager.request::<Model>("data/prefabs/player.rgs");
                let instance = model.instantiate(scene);
                let mutex = Arc::new(Mutex::new(stream));
                context.script_processor.scripted_scenes[0].message_sender.send(ScriptMessage { payload: Box::new(mutex), kind: fyrox::script::ScriptMessageKind::Targeted(instance) })
            }
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, context: &mut PluginContext) {
        self.try_spawn_player(context);
    }

    fn on_os_event(
        &mut self,
        _event: &Event<()>,
        _context: PluginContext,
    ) {
        // Do something on OS event here.
    }

    fn on_ui_message(
        &mut self,
        _context: &mut PluginContext,
        _message: &UiMessage,
    ) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
