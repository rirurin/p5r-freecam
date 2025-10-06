use windows::Win32::{
    Foundation::RECT,
    Graphics::{
        Dxgi::Common::DXGI_FORMAT,
        Direct3D::{
            D3D_PRIMITIVE_TOPOLOGY,
            D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        },
        Direct3D11::{
            D3D11_VIEWPORT,
            // D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE, (16)
            ID3D11BlendState,
            ID3D11Buffer,
            ID3D11ClassInstance,
            ID3D11DepthStencilState,
            ID3D11DeviceContext,
            ID3D11GeometryShader,
            ID3D11InputLayout,
            ID3D11PixelShader,
            ID3D11RasterizerState,
            ID3D11SamplerState,
            ID3D11ShaderResourceView,
            ID3D11VertexShader
        }
    }
};

// From imgui source code: https://github.com/ocornut/imgui/blob/master/backends/imgui_impl_dx11.cpp#L201
// "Backup DX state that will be modified to restore it afterwards (unfortunately this is very ugly looking and verbose. Close your eyes!)"
// So true bestie


const MAX_VIEWPORTS: usize = 16;
const BLEND_FACTOR: usize = 4;
const MAX_CLASS_INSTANCES: usize = 256;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct StateBackup {
    context: Option<ID3D11DeviceContext>,
    scissor_rects: [RECT; MAX_VIEWPORTS],
    viewports: [D3D11_VIEWPORT; MAX_VIEWPORTS],
    rasterizer_state: Option<ID3D11RasterizerState>,
    blend_state: Option<ID3D11BlendState>,
    blend_factor: [f32; BLEND_FACTOR],
    sample_mask: u32,
    depth_stencil_state: Option<ID3D11DepthStencilState>,
    stencil_ref: u32,
    shader_resource: Vec<Option<ID3D11ShaderResourceView>>,
    sampler: Vec<Option<ID3D11SamplerState>>,
    ps_shader: Option<ID3D11PixelShader>,
    ps_instances: [Option<ID3D11ClassInstance>; MAX_CLASS_INSTANCES],
    vs_shader: Option<ID3D11VertexShader>,
    vs_instances: [Option<ID3D11ClassInstance>; MAX_CLASS_INSTANCES],
    constant_buffer: Vec<Option<ID3D11Buffer>>,
    gs_shader: Option<ID3D11GeometryShader>,
    gs_instances: [Option<ID3D11ClassInstance>; MAX_CLASS_INSTANCES],
    index_buffer: Option<ID3D11Buffer>,
    index_buffer_offset: u32,
    index_buffer_format: DXGI_FORMAT,
    vertex_buffer: Option<ID3D11Buffer>,
    vertex_buffer_offset: u32,
    vertex_buffer_stride: u32,
    topology: D3D_PRIMITIVE_TOPOLOGY,
    input_layout: Option<ID3D11InputLayout>,
}

impl Default for StateBackup {
    fn default() -> Self {
        Self {
            context: Option::<ID3D11DeviceContext>::default(),
            scissor_rects: [RECT::default(); MAX_VIEWPORTS],
            viewports: [D3D11_VIEWPORT::default(); MAX_VIEWPORTS],
            rasterizer_state: Option::<ID3D11RasterizerState>::default(),
            blend_state: Option::<ID3D11BlendState>::default(),
            blend_factor: std::array::from_fn::<_, BLEND_FACTOR, _>(|_| f32::default()),
            sample_mask: u32::default(),
            depth_stencil_state: Option::<ID3D11DepthStencilState>::default(),
            stencil_ref: u32::default(),
            shader_resource: Vec::<Option::<ID3D11ShaderResourceView>>::default(),
            sampler: Vec::<Option::<ID3D11SamplerState>>::default(),
            ps_shader: Option::<ID3D11PixelShader>::default(),
            ps_instances: std::array::from_fn::<_, MAX_CLASS_INSTANCES, _>(|_| Option::<ID3D11ClassInstance>::default()),
            vs_shader: Option::<ID3D11VertexShader>::default(),
            vs_instances: std::array::from_fn::<_, MAX_CLASS_INSTANCES, _>(|_| Option::<ID3D11ClassInstance>::default()),
            constant_buffer: Vec::<Option::<ID3D11Buffer>>::default(),
            gs_shader: Option::<ID3D11GeometryShader>::default(),
            gs_instances: std::array::from_fn::<_, MAX_CLASS_INSTANCES, _>(|_| Option::<ID3D11ClassInstance>::default()),
            index_buffer: Option::<ID3D11Buffer>::default(),
            index_buffer_offset: u32::default(),
            index_buffer_format: DXGI_FORMAT::default(),
            vertex_buffer: Option::<ID3D11Buffer>::default(),
            vertex_buffer_offset: u32::default(),
            vertex_buffer_stride: u32::default(),
            topology: D3D_PRIMITIVE_TOPOLOGY::default(),
            input_layout: Option::<ID3D11InputLayout>::default(),
        }
    }
}

#[allow(dead_code)]
impl StateBackup {
    pub(crate) unsafe fn backup(context: Option<ID3D11DeviceContext>) -> Self {
        let mut result = Self::default();
        let ctx = context.as_ref().unwrap();
        ctx.RSGetScissorRects(&mut 16, Some(result.scissor_rects.as_mut_ptr()));
        ctx.RSGetViewports(&mut 16, Some(result.viewports.as_mut_ptr()));
        result.rasterizer_state = match ctx.RSGetState() {
            Ok(v) => Some(v),
            Err(_) => None
        };
        ctx.OMGetBlendState(
            Some(&mut result.blend_state),
            Some(&mut result.blend_factor),
            Some(&mut result.sample_mask),
        );
        ctx.OMGetDepthStencilState(
            Some(&mut result.depth_stencil_state), 
            Some(&mut result.stencil_ref)
        );
        ctx.PSGetShaderResources(0, Some(&mut result.shader_resource));
        ctx.PSGetSamplers(0, Some(&mut result.sampler));
        ctx.PSGetShader(&mut result.ps_shader, Some(result.ps_instances.as_mut_ptr()), Some(&mut 256));
        ctx.VSGetShader(&mut result.vs_shader, Some(result.vs_instances.as_mut_ptr()), Some(&mut 256));
        ctx.VSGetConstantBuffers(0, Some(result.constant_buffer.as_mut_slice()));
        ctx.GSGetShader(&mut result.gs_shader, Some(result.gs_instances.as_mut_ptr()), Some(&mut 256));
        result.topology = ctx.IAGetPrimitiveTopology();
        ctx.IAGetIndexBuffer(
            Some(&mut result.index_buffer),
            Some(&mut result.index_buffer_format),
            Some(&mut result.index_buffer_offset),
        );
        ctx.IAGetVertexBuffers(
            0,
            1,
            Some(&mut result.vertex_buffer),
            Some(&mut result.vertex_buffer_stride),
            Some(&mut result.vertex_buffer_offset),
        );
        result.input_layout = match ctx.IAGetInputLayout() {
            Ok(v) => Some(v),
            Err(_) => None
        };
        result.context = context;
        result
    }
    pub(crate) fn restore(self) {
        unsafe {
            let ctx = self.context.as_ref().unwrap();

            ctx.RSSetScissorRects(Some(self.scissor_rects.as_ref()));
            ctx.RSSetViewports(Some(self.viewports.as_ref()));
            ctx.RSSetState(self.rasterizer_state.as_ref());
            ctx.OMSetBlendState(self.blend_state.as_ref(), Some(&self.blend_factor), 0xFFFFFFFF);
            ctx.OMSetDepthStencilState(self.depth_stencil_state.as_ref(), self.stencil_ref);
            ctx.PSSetShaderResources(0, Some(self.shader_resource.as_slice()));
            ctx.PSSetSamplers(0, Some(self.sampler.as_slice()));
            ctx.PSSetShader(self.ps_shader.as_ref(), Some(self.ps_instances.as_slice()));
            ctx.VSSetShader(self.vs_shader.as_ref(), Some(self.vs_instances.as_slice()));
            ctx.VSSetConstantBuffers(0, Some(self.constant_buffer.as_slice()));
            ctx.GSSetShader(self.gs_shader.as_ref(), Some(&[]));
            ctx.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            ctx.IASetIndexBuffer(
                self.index_buffer.as_ref(),
                self.index_buffer_format,
                self.index_buffer_offset,
            );
            ctx.IASetVertexBuffers(
                0,
                1,
                Some(&raw const self.vertex_buffer),
                Some(&raw const self.vertex_buffer_stride),
                Some(&raw const self.vertex_buffer_offset),
            );
            ctx.IASetInputLayout(self.input_layout.as_ref());
        }
    }
}