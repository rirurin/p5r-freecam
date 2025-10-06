use windows::Win32::Graphics::{
    Dxgi::Common::{
        DXGI_FORMAT_R8G8B8A8_UNORM,
        DXGI_SAMPLE_DESC
    },
    Direct3D11::{
        D3D11_SAMPLER_DESC,
        D3D11_SHADER_RESOURCE_VIEW_DESC,
        D3D11_SUBRESOURCE_DATA,
        D3D11_TEXTURE2D_DESC,
        D3D11_USAGE_DEFAULT,
        ID3D11Device,
        ID3D11SamplerState,
        ID3D11ShaderResourceView,
        ID3D11Texture2D
    }
};
pub const FONT_TEX_ID: usize = usize::MAX;
use imgui::TextureId;

#[derive(Debug)]
pub struct FontObjects {
    font_sampler: Option<ID3D11SamplerState>,
    font_resource_view: Option<ID3D11ShaderResourceView>,
}
impl FontObjects {
    fn uninit() -> Self {
        Self {
            font_sampler: None,
            font_resource_view: None,
        }
    }

    pub unsafe fn new(
        fonts: &mut imgui::FontAtlas,
        device: &ID3D11Device,
    ) -> windows::core::Result<Self> {
        let mut out = Self::uninit();
        // Build texture atlas and upload to graphics system
        let fa_tex = fonts.build_rgba32_texture();
        let desc = D3D11_TEXTURE2D_DESC {
            Width: fa_tex.width,
            Height: fa_tex.height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: windows::Win32::Graphics::Direct3D11::D3D11_BIND_SHADER_RESOURCE.0 as u32,
            ..Default::default()
        };
        let sub_resource = D3D11_SUBRESOURCE_DATA {
            pSysMem: fa_tex.data.as_ptr().cast(),
            SysMemPitch: desc.Width * 4,
            SysMemSlicePitch: 0,
        };
        let mut texture: Option<ID3D11Texture2D> = None;
        device.CreateTexture2D(
            &desc, 
            Some(&sub_resource),
            Some(&raw mut texture)
        )?;
        let mut srv_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ViewDimension: windows::Win32::Graphics::Direct3D::D3D11_SRV_DIMENSION_TEXTURE2D,
            ..Default::default()
        };
        srv_desc.Anonymous.Texture2D.MipLevels = desc.MipLevels;
        srv_desc.Anonymous.Texture2D.MostDetailedMip = 0;
        device.CreateShaderResourceView(texture.as_ref().map(|v| v.into()), Some(&srv_desc), Some(&raw mut out.font_resource_view))?;

        fonts.tex_id = TextureId::from(FONT_TEX_ID);

        // Create texture sampler
        let desc = D3D11_SAMPLER_DESC {
            Filter: windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            AddressU: windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_WRAP,
            AddressV: windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_WRAP,
            AddressW: windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_WRAP,
            MipLODBias: 0.0,
            ComparisonFunc: windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_ALWAYS,
            MinLOD: 0.0,
            MaxLOD: 0.0,
            ..Default::default()
        };
        device.CreateSamplerState(&desc, Some(&raw mut out.font_sampler))?;
        Ok(out)
    }
    pub fn get_font_sampler_owned(&self) -> Option<ID3D11SamplerState> {
        self.font_sampler.clone()
    }
    pub fn get_font_resource_view(&self) -> Option<ID3D11ShaderResourceView> {
        self.font_resource_view.clone()
    }
}