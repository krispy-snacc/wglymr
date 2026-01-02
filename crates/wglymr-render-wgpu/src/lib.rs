mod batch;
mod buffers;
mod device;
mod draw;
mod execute;
mod gpu;
mod pipeline;
mod renderer;
pub mod text;

pub use batch::PrimitiveBatch;
pub use device::{create_gpu_context, GpuContext};
pub use draw::draw_grid;
pub use execute::execute_wgsl_f32;
pub use gpu::ViewportResources;
pub use renderer::PrimitiveRenderer;
pub use text::{GlyphAtlas, GlyphEntry, GlyphKey, GpuGlyph, TextBatch, TextPipeline, TextRenderer};

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
