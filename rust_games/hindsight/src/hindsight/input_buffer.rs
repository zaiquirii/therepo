use crate::hindsight::FrameId;
use crate::hindsight::ring_buffer::FrameCache;

#[derive(Default)]
struct FrameInput<TInput> {
    frame: FrameId,
    real: Option<TInput>,
    predicted: Option<TInput>,
    confirmed: bool,
}

pub struct InputBuffer<TInput: Default> {
    inputs: FrameCache<FrameInput<TInput>>,
}

impl<TInput: Default> InputBuffer<TInput> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inputs: FrameCache::new(capacity),
        }
    }

    pub fn get_frame_input(&mut self, frame: FrameId) -> Option<&TInput> {
        let frame_input = self.inputs.get_mut(frame);
        debug_assert!(frame_input.frame == frame);
        frame_input.real.as_ref()
    }

    pub fn set_frame_input(&mut self, frame: FrameId, input: TInput) {
        let frame_input = FrameInput {
            frame,
            real: Some(input),
            predicted: None,
            confirmed: true,
        };
        self.inputs.set(frame, frame_input);
    }
}