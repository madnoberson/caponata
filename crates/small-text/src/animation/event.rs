#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationEvent {
    FrameGenerated,
    Ended,
}
