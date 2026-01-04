use wgpu::{Buffer, BufferUsages, Device, Queue};

pub struct BufferPair {
    pub storage: Buffer,
    pub staging: Buffer,
}

pub fn create_output_buffers(device: &Device) -> BufferPair {
    let storage = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Storage Buffer"),
        size: std::mem::size_of::<f32>() as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: std::mem::size_of::<f32>() as u64,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    BufferPair { storage, staging }
}

pub fn read_buffer_f32(device: &Device, queue: &Queue, staging: &Buffer) -> f32 {
    queue.submit(None);

    let buffer_slice = staging.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();

    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).unwrap();
    });

    let _ = device.poll(wgpu::MaintainBase::Wait);
    receiver.recv().unwrap().expect("Failed to map buffer");

    let data = buffer_slice.get_mapped_range();
    let result = bytemuck::cast_slice::<u8, f32>(&data)[0];

    drop(data);
    staging.unmap();

    result
}
