
use std::{sync::{Arc, Mutex}, net::TcpStream};
use fyrox::{
    core::{visitor::prelude::*, reflect::prelude::*, type_traits::prelude::*, algebra::{Vector3, Vector4, Vector2}},
    event::Event, script::{ScriptContext, ScriptDeinitContext, ScriptTrait},
};

use crate::network::{player::NetworkPlayer, messages::{Transform, PlayerOutput}};

#[derive(Visit, Reflect, Default, Debug, Clone, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "8c64bd35-9d5e-44ce-babb-8b664f4d062a")]
#[visit(optional)]
pub struct Player {
    position: Vector3<f32>,
    rotation: Vector4<f32>,
    scale: Vector2<f32>,
    network: NetworkPlayer
}

impl ScriptTrait for Player {
    fn on_init(&mut self, context: &mut ScriptContext) {
        // Put initialization logic here.
    }

    fn on_start(&mut self, context: &mut ScriptContext) {
        // There should be a logic that depends on other scripts in scene.
        // It is called right after **all** scripts were initialized.
    }

    fn on_deinit(&mut self, context: &mut ScriptDeinitContext) {
        // Put de-initialization logic here.
    }

    fn on_os_event(&mut self, event: &Event<()>, context: &mut ScriptContext) {
        // Respond to OS events here.
    }

    fn on_update(&mut self, context: &mut ScriptContext) {
        if let Some(input) = self.network.try_read_stream_input(){
            let transform = self.apply_transform(input.transform, context.dt);
            let output = PlayerOutput::new(transform, 100.0);
            self.network.try_write_stream_output(output);
        }
    }

    fn on_message(
        &mut self,
        #[allow(unused_variables)] message: &mut dyn fyrox::script::ScriptMessagePayload,
        #[allow(unused_variables)] ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        if let Some(stream) = message.downcast_mut::<Arc<Mutex<TcpStream>>>(){
            self.network = NetworkPlayer::new(stream.clone());
        }
    }
}

impl Player {
    fn apply_transform(&mut self, transform: Transform, dt: f32) -> Transform {
        let translation = Vector3::from_vec(transform.translation.into());
        let rotation = Vector4::from_vec(transform.rotation.into());
        let scale = Vector2::from_vec(transform.scale.into());
        self.position += translation * dt;
        self.rotation += rotation * dt;
        self.scale += scale * dt;
        Transform::from_vectors(self.position, self.rotation, self.scale)
    }
}
