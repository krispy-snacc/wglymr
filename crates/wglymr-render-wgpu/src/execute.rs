use crate::buffers::{create_output_buffers, read_buffer_f32};
use crate::device::create_gpu_context;
use crate::pipeline::create_compute_pipeline;

pub fn execute_wgsl_f32(wgsl: &str) -> f32 {
    let ctx = create_gpu_context();

    let buffers = create_output_buffers(&ctx.device);

    let compute = create_compute_pipeline(&ctx.device, wgsl, &buffers.storage);

    let mut encoder = ctx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&compute.pipeline);
        compute_pass.set_bind_group(0, &compute.bind_group, &[]);
        compute_pass.dispatch_workgroups(1, 1, 1);
    }

    encoder.copy_buffer_to_buffer(
        &buffers.storage,
        0,
        &buffers.staging,
        0,
        std::mem::size_of::<f32>() as u64,
    );

    ctx.queue.submit(Some(encoder.finish()));

    read_buffer_f32(&ctx.device, &ctx.queue, &buffers.staging)
}
