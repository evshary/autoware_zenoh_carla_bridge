use super::actor_bridge::ActorBridge;
use crate::error::Result;
use carla::client::Actor;
use std::sync::Arc;
use zenoh::Session;

pub struct OtherActorBridge {
    _actor: Actor,
}

impl OtherActorBridge {
    pub fn new(_z_session: Arc<Session>, _actor: Actor) -> Result<OtherActorBridge> {
        Ok(OtherActorBridge { _actor })
    }
}

impl ActorBridge for OtherActorBridge {
    fn step(&mut self, _timestamp: f64) -> Result<()> {
        Ok(())
    }
}
