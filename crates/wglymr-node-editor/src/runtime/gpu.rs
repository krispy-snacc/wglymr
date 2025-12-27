use wgpu::{Adapter, Device, Instance, Queue, Surface};

pub enum SurfaceHandle {
    Web(Surface<'static>),
}

pub struct GpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl GpuContext {
    pub fn new(instance: Instance, adapter: Adapter, device: Device, queue: Queue) -> Self {
        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}
