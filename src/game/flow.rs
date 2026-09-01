use super::scene::SceneId;

pub enum FlowEvent {
    None,
    DemoFinished,
    StartGame,
}

pub fn handle(event: FlowEvent) -> Option<SceneId> {
    match event {
        FlowEvent::None => None,
        FlowEvent::DemoFinished => Some(SceneId::Menu),
        FlowEvent::StartGame => Some(SceneId::Demo),
    }
}