use wglymr_node_editor::editor::renderer::draw_canvas;
use wglymr_node_editor::editor::view_state::{Camera, RenderEdge, RenderNode, Rect};
use wglymr_node_editor::editor::wgpu_renderer::WgpuNodeEditorRenderer;
use wglymr_render_wgpu::{create_gpu_context, PrimitiveRenderer};

#[test]
fn test_wgpu_renderer_instantiation() {
    let ctx = create_gpu_context();
    let mut primitive_renderer = PrimitiveRenderer::new(&ctx.device, wgpu::TextureFormat::Rgba8UnormSrgb);
    
    let camera = Camera {
        pan: [0.0, 0.0],
        zoom: 1.0,
    };
    
    let _renderer = WgpuNodeEditorRenderer::new(
        &ctx.device,
        &ctx.queue,
        &mut primitive_renderer,
        camera,
    );
}

#[test]
fn test_wgpu_renderer_draw_calls() {
    let ctx = create_gpu_context();
    let mut primitive_renderer = PrimitiveRenderer::new(&ctx.device, wgpu::TextureFormat::Rgba8UnormSrgb);
    
    let camera = Camera {
        pan: [0.0, 0.0],
        zoom: 1.0,
    };
    
    let mut renderer = WgpuNodeEditorRenderer::new(
        &ctx.device,
        &ctx.queue,
        &mut primitive_renderer,
        camera,
    );
    
    let nodes = vec![
        RenderNode {
            node_id: wglymr_node_editor::document::commands::NodeId(1),
            rect: Rect {
                min: [0.0, 0.0],
                max: [100.0, 100.0],
            },
            input_sockets: vec![],
            output_sockets: vec![],
        },
    ];
    
    let edges = vec![
        RenderEdge {
            edge_id: wglymr_node_editor::document::commands::EdgeId(1),
            from: [50.0, 50.0],
            to: [200.0, 200.0],
        },
    ];
    
    draw_canvas(&mut renderer, &nodes, &edges);
}
