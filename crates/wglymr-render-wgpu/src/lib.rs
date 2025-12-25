mod buffers;
mod device;
mod execute;
mod pipeline;
mod primitives;

pub use device::{create_gpu_context, GpuContext};
pub use execute::execute_wgsl_f32;
pub use primitives::PrimitiveRenderer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_wgsl_returns_42() {
        let wgsl = r#"
            @group(0) @binding(0) var<storage, read_write> output: array<f32>;

            @compute @workgroup_size(1)
            fn main() {
                output[0] = 42.0;
            }
        "#;

        let result = execute_wgsl_f32(wgsl);
        assert_eq!(result, 42.0);
    }
}
