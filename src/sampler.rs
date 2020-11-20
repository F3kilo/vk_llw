use ash::vk;

#[derive(Clone)]
pub struct Sampler {}

impl Sampler {
    /// # Safety
    ///
    pub unsafe fn handle(&self) -> &vk::Sampler {
        todo!()
    }
}
