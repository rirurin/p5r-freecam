use windows::Win32::Graphics::Direct3D11::{
    D3D11_BLEND_DESC,
    D3D11_DEPTH_STENCIL_DESC,
    D3D11_DEPTH_STENCILOP_DESC,
    D3D11_RASTERIZER_DESC,
    D3D11_RENDER_TARGET_BLEND_DESC,
    ID3D11BlendState,
    ID3D11Device,
    ID3D11DepthStencilState,
    ID3D11RasterizerState,
};

#[derive(Debug)]
pub struct DeviceObjects {
    blend_state: Option<ID3D11BlendState>,
    raster_state: Option<ID3D11RasterizerState>,
    depth_stencil_state: Option<ID3D11DepthStencilState>
}
impl DeviceObjects {
    fn uninit() -> Self {
        Self {
            blend_state: None,
            raster_state: None,
            depth_stencil_state: None,
        }
    }
    pub unsafe fn new(device: &ID3D11Device) -> windows::core::Result<DeviceObjects> {
        let mut out = DeviceObjects::uninit();
        // Create the blending setup
        let desc = D3D11_BLEND_DESC {
            AlphaToCoverageEnable: false.into(),
            IndependentBlendEnable: true.into(),
            RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: true.into(),
                SrcBlend: windows::Win32::Graphics::Direct3D11::D3D11_BLEND_SRC_ALPHA,
                DestBlend: windows::Win32::Graphics::Direct3D11::D3D11_BLEND_INV_SRC_ALPHA,
                BlendOp: windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_ADD,
                SrcBlendAlpha: windows::Win32::Graphics::Direct3D11::D3D11_BLEND_ONE,
                DestBlendAlpha: windows::Win32::Graphics::Direct3D11::D3D11_BLEND_INV_SRC_ALPHA,
                BlendOpAlpha: windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_ADD,
                RenderTargetWriteMask: windows::Win32::Graphics::Direct3D11::D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
            }; 8],
        };
        device.CreateBlendState(&desc, Some(&raw mut out.blend_state))?;
        // Create the rasterizer state
        let desc = D3D11_RASTERIZER_DESC {
            FillMode: windows::Win32::Graphics::Direct3D11::D3D11_FILL_SOLID,
            CullMode: windows::Win32::Graphics::Direct3D11::D3D11_CULL_NONE,
            DepthClipEnable: true.into(),
            ScissorEnable: true.into(),
            ..Default::default()
        };
        device.CreateRasterizerState(&desc, Some(&raw mut out.raster_state))?;
        // Create depth-stencil state
        let stencil_op_desc = D3D11_DEPTH_STENCILOP_DESC {
            StencilFailOp: windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP,
            StencilDepthFailOp: windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP,
            StencilPassOp: windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP,
            StencilFunc: windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_ALWAYS,
        };
        let desc = D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: false.into(),
            DepthWriteMask: windows::Win32::Graphics::Direct3D11::D3D11_DEPTH_WRITE_MASK_ALL,
            DepthFunc: windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_ALWAYS,
            StencilEnable: false.into(),
            StencilReadMask: 0,
            StencilWriteMask: 0,
            FrontFace: stencil_op_desc,
            BackFace: stencil_op_desc,
        };
        device.CreateDepthStencilState(&desc, Some(&raw mut out.depth_stencil_state))?;
        Ok(out)
    }
    pub fn get_blend_state(&self) -> Option<&ID3D11BlendState> {
        self.blend_state.as_ref().map(|v| v.into())
    }
    pub fn get_rasterizer_state(&self) -> Option<&ID3D11RasterizerState> {
        self.raster_state.as_ref().map(|v| v.into())
    }
    pub fn get_depth_stencil_state(&self) -> Option<&ID3D11DepthStencilState> {
        self.depth_stencil_state.as_ref().map(|v| v.into())
    }
}