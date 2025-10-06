use crate::gui::d3d11::{
    backup::StateBackup,
    buffer::{ IndexBuffer, VertexBuffer },
    devices::DeviceObjects,
    font::{ FONT_TEX_ID, FontObjects },
    shader::{ PixelShader, VertexShader },
};
use glam::{ Vec4, Mat4 };
use imgui::{
    internal::RawWrapper,
    BackendFlags,
    Context as ImContext,
    DrawCmd,
    DrawCmdParams,
    DrawData,
    DrawIdx,
    DrawVert,
    Textures,
    TextureId
};
use std::{
    error::Error,
    mem::MaybeUninit
};
use windows::{
    core::{ Error as WinError, HRESULT },
    Win32::{
        Foundation::{ DXGI_STATUS_OCCLUDED, E_FAIL, HMODULE, HWND, RECT },
        Graphics::{
            Dxgi::{
                Common::{
                    DXGI_FORMAT_R8G8B8A8_UNORM,
                    DXGI_MODE_DESC,
                    DXGI_MODE_SCALING_UNSPECIFIED,
                    DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    DXGI_RATIONAL,
                    DXGI_SAMPLE_DESC,
                },
                DXGI_ERROR_INVALID_CALL,
                DXGI_PRESENT,
                DXGI_PRESENT_TEST,
                DXGI_SWAP_EFFECT_DISCARD,
                DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
                DXGI_SWAP_CHAIN_DESC,
                DXGI_SWAP_CHAIN_FLAG,
                IDXGISwapChain
            },
            Direct3D::{
                D3D_DRIVER_TYPE_HARDWARE,
                D3D_FEATURE_LEVEL,
                D3D_FEATURE_LEVEL_11_0,

            },
            Direct3D11::{
                D3D11_CREATE_DEVICE_FLAG,
                D3D11_MAPPED_SUBRESOURCE,
                D3D11_MAP_WRITE_DISCARD,
                D3D11_SDK_VERSION,
                D3D11_VIEWPORT,
                ID3D11Device,
                ID3D11DeviceContext,
                ID3D11RenderTargetView,
                ID3D11ShaderResourceView,
                ID3D11Texture2D
            }
        }
    }
};

pub const BUFFER_COUNT: usize = 2;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct State {
    device: ID3D11Device,
    swapchain: IDXGISwapChain,
    render_target_view: Option<ID3D11RenderTargetView>,
    // targets: RenderTargets,
    context: ID3D11DeviceContext,
    vertex_shader: VertexShader,
    pixel_shader: PixelShader,
    device_objects: DeviceObjects,
    font_data: FontObjects,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    textures: Textures<ID3D11ShaderResourceView>,
}

impl State {
    pub unsafe fn new(im_ctx: &mut ImContext, hwnd: HWND) -> Result<Self, Box<dyn Error>> {
        // From CreateDeviceD3D
        let desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width: 0,
                Height: 0,
                RefreshRate: DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED
            },
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: BUFFER_COUNT as u32,
            OutputWindow: hwnd,
            Windowed: true.into(),
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32
        };

        
        let mut swapchain: Option<IDXGISwapChain> = None;
        let mut device: Option<ID3D11Device> = None;
        let mut feature_level: MaybeUninit<D3D_FEATURE_LEVEL> = MaybeUninit::uninit();

        windows::Win32::Graphics::Direct3D11::D3D11CreateDeviceAndSwapChain(
            None, 
            D3D_DRIVER_TYPE_HARDWARE, 
            HMODULE::default(),
            D3D11_CREATE_DEVICE_FLAG(0),
            // D3D11_CREATE_DEVICE_DEBUG,
            Some(&[D3D_FEATURE_LEVEL_11_0]),
            D3D11_SDK_VERSION,
            Some(&raw const desc),
            Some(&raw mut swapchain),
            Some(&raw mut device), 
            Some(feature_level.as_mut_ptr()), 
           None 
        )?;
        let device = device.ok_or(WinError::from_hresult(E_FAIL))?;
        let swapchain = swapchain.ok_or(WinError::from_hresult(E_FAIL))?;
        // CreateRenderTarget
        let mut render_target_view: Option<ID3D11RenderTargetView> = None;
        let back_buffer = swapchain.GetBuffer::<ID3D11Texture2D>(0)?;
        device.CreateRenderTargetView(&back_buffer, None, Some(&raw mut render_target_view))?;
        // ImGui_ImplDX11_Init
        im_ctx.io_mut().backend_flags |= BackendFlags::RENDERER_HAS_VTX_OFFSET 
            | BackendFlags::RENDERER_HAS_VIEWPORTS;
        // ImGui_ImplDX11_InitMultiViewportSupport
        // ImGui_ImplDX11_CreateDeviceObjects
        let vertex_shader = VertexShader::new(&device)?;
        let pixel_shader = PixelShader::new(&device)?;
        let device_objects = DeviceObjects::new(&device)?;
        let font_data = FontObjects::new(im_ctx.fonts(), &device)?;
        let vertex_buffer = VertexBuffer::new(&device, 0)?;
        let index_buffer = IndexBuffer::new(&device, 0)?;
        let context = device.GetImmediateContext()?;
        Ok(Self {
            device, swapchain, context, render_target_view,
            vertex_shader, pixel_shader, device_objects,
            font_data, vertex_buffer, index_buffer,
            textures: Textures::new(),
        })
    } 
    pub fn render(&mut self, draw_data: &DrawData, color: Vec4) -> windows::core::Result<()> {
        
        unsafe { 
            self.context.OMSetRenderTargets(Some(&[self.render_target_view.clone()]), None);
            self.context.ClearRenderTargetView(self.render_target_view.as_ref(), &[color.x, color.y, color.z, color.w]);
        }

        if draw_data.display_size[0] <= 0.0 
        || draw_data.display_size[1] <= 0.0 {
            return Ok(());
        }
        unsafe {
            if self.vertex_buffer.len() < draw_data.total_vtx_count as usize {
                self.vertex_buffer = VertexBuffer::new(&self.device, draw_data.total_vtx_count as usize)?;
            }
            if self.index_buffer.len() < draw_data.total_idx_count as usize {
                self.index_buffer = IndexBuffer::new(&self.device, draw_data.total_idx_count as usize)?;
            }
            let _state_guard = StateBackup::backup(Some(self.context.clone()));
            self.write_buffers(draw_data)?;
            self.setup_render_state(draw_data);
            self.render_impl(draw_data)?;
            _state_guard.restore(); 
        }
        Ok(())
    }
    pub fn present(&mut self) -> windows::core::Result<()> {
        let present_result = unsafe { self.swapchain.Present(1, DXGI_PRESENT(0)) };
        if present_result.is_err() {
            return Err(windows::core::Error::from_hresult(HRESULT(present_result.0)))
        }
        Ok(())
    }

    pub fn occulsion_test(&mut self) -> bool {
        let present_result = unsafe { self.swapchain.Present(0, DXGI_PRESENT_TEST) };
        present_result == DXGI_STATUS_OCCLUDED
    }

    unsafe fn write_buffers(&self, draw_data: &DrawData) -> windows::core::Result<()> {
        let mut vtx_resource: MaybeUninit<D3D11_MAPPED_SUBRESOURCE> = MaybeUninit::uninit();
        let mut idx_resource: MaybeUninit<D3D11_MAPPED_SUBRESOURCE> = MaybeUninit::uninit();
        self.context.Map(self.vertex_buffer.get_buffer().map(|v| v.into()), 0, D3D11_MAP_WRITE_DISCARD, 0, Some(vtx_resource.as_mut_ptr()))?;
        self.context.Map(self.index_buffer.get_buffer().map(|v| v.into()), 0, D3D11_MAP_WRITE_DISCARD, 0, Some(idx_resource.as_mut_ptr()))?;
        let mut vtx_dst = std::slice::from_raw_parts_mut(
            vtx_resource.assume_init_ref().pData.cast::<DrawVert>(),
            draw_data.total_vtx_count as usize,
        );
        let mut idx_dst = std::slice::from_raw_parts_mut(
            idx_resource.assume_init_ref().pData.cast::<DrawIdx>(),
            draw_data.total_idx_count as usize,
        );

        for (vbuf, ibuf) in
            draw_data.draw_lists().map(|draw_list| (draw_list.vtx_buffer(), draw_list.idx_buffer()))
        {
            vtx_dst[..vbuf.len()].copy_from_slice(vbuf);
            idx_dst[..ibuf.len()].copy_from_slice(ibuf);
            vtx_dst = &mut vtx_dst[vbuf.len()..];
            idx_dst = &mut idx_dst[ibuf.len()..];
        }

        self.context.Unmap(self.vertex_buffer.get_buffer().map(|v| v.into()), 0);
        self.context.Unmap(self.index_buffer.get_buffer().map(|v| v.into()), 0);
        
        let mut mapped_resource: MaybeUninit<D3D11_MAPPED_SUBRESOURCE> = MaybeUninit::uninit();
        self.context.Map(
            self.vertex_shader.get_constant_buffer().map(|v| v.into()),
            0, D3D11_MAP_WRITE_DISCARD, 0, 
            Some(mapped_resource.as_mut_ptr())
        )?;
        let l = draw_data.display_pos[0];
        let r = draw_data.display_pos[0] + draw_data.display_size[0];
        let t = draw_data.display_pos[1];
        let b = draw_data.display_pos[1] + draw_data.display_size[1];
        let mvp = Mat4::from_cols(
            Vec4::new(2.0 / (r - l), 0., 0., 0.,),
            Vec4::new(0.0, 2.0 / (t - b), 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.5, 0.0),
            Vec4::new((r + l) / (l - r), (t + b) / (b - t), 0.5, 1.0),
        );
        *mapped_resource.assume_init_ref().pData.cast::<Mat4>() = mvp;
        self.context.Unmap(self.vertex_shader.get_constant_buffer().map(|v| v.into()), 0);

        Ok(())
    }

    unsafe fn setup_render_state(&self, draw_data: &DrawData) {
        let ctx = &self.context;
        let vp = D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: draw_data.display_size[0],
            Height: draw_data.display_size[1],
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        let draw_fmt = if size_of::<DrawIdx>() == 2 {
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_UINT
        } else {
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_UINT
        };
        let stride = size_of::<DrawVert>() as u32;
        let blend_factor = 0.0;

        ctx.RSSetViewports(Some(&[vp]));
        ctx.IASetInputLayout(self.vertex_shader.get_input_layout());
        ctx.IASetVertexBuffers(0, 1, Some(self.vertex_buffer.get_buffers()), Some(&stride), Some(&0));
        ctx.IASetIndexBuffer(self.index_buffer.get_buffer(), draw_fmt, 0);
        ctx.IASetPrimitiveTopology(windows::Win32::Graphics::Direct3D::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        ctx.VSSetShader(self.vertex_shader.get_shader(), Some(&[]));
        ctx.VSSetConstantBuffers(0, Some(&[self.vertex_shader.get_constant_buffer_owned()]));
        ctx.PSSetShader(self.pixel_shader.get_shader(), Some(&[]));
        ctx.PSSetSamplers(0, Some(&[self.font_data.get_font_sampler_owned()]));
        ctx.GSSetShader(None,Some(&[]));
        ctx.HSSetShader(None,Some(&[]));
        ctx.DSSetShader(None,Some(&[]));
        ctx.CSSetShader(None,Some(&[]));
        ctx.OMSetBlendState(self.device_objects.get_blend_state(), Some(&[blend_factor; 4]), 0xFFFFFFFF);
        ctx.OMSetDepthStencilState(self.device_objects.get_depth_stencil_state(), 0);
        ctx.RSSetState(self.device_objects.get_rasterizer_state());
    }

    unsafe fn render_impl(&self, draw_data: &DrawData) -> windows::core::Result<()> {
        let clip_off = draw_data.display_pos;
        let clip_scale = draw_data.framebuffer_scale;
        let mut vertex_offset = 0;
        let mut index_offset = 0;
        let mut last_tex = TextureId::from(FONT_TEX_ID);
        let context = &self.context;
        context.PSSetShaderResources(0, Some(&[self.font_data.get_font_resource_view()]));
        for draw_list in draw_data.draw_lists() {
            for cmd in draw_list.commands() {
                match cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params: DrawCmdParams { clip_rect, texture_id, .. },
                    } => {
                        if texture_id != last_tex {
                            let texture = if texture_id.id() == FONT_TEX_ID {
                                self.font_data.get_font_resource_view()
                            } else {
                                Some(self.textures
                                    .get(texture_id)
                                    .ok_or(DXGI_ERROR_INVALID_CALL)?
                                    .clone())
                            };
                            context.PSSetShaderResources(0, Some(&[texture]));
                            last_tex = texture_id;
                        }

                        let r = RECT {
                            left: ((clip_rect[0] - clip_off[0]) * clip_scale[0]) as i32,
                            top: ((clip_rect[1] - clip_off[1]) * clip_scale[1]) as i32,
                            right: ((clip_rect[2] - clip_off[0]) * clip_scale[0]) as i32,
                            bottom: ((clip_rect[3] - clip_off[1]) * clip_scale[1]) as i32,
                        };
                        context.RSSetScissorRects(Some(&[r]));
                        context.DrawIndexed(
                            count as u32,
                            index_offset as u32,
                            vertex_offset as i32,
                        );
                        index_offset += count;
                    },
                    DrawCmd::ResetRenderState => self.setup_render_state(draw_data),
                    DrawCmd::RawCallback { callback, raw_cmd } => {
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }
            vertex_offset += draw_list.vtx_buffer().len();
        }
        Ok(())
    }
    pub fn resize(&mut self, width: u32, height: u32) -> windows::core::Result<()> {
        unsafe {
            self.render_target_view = None;
            self.swapchain.ResizeBuffers(0, width, height, DXGI_FORMAT_R8G8B8A8_UNORM,DXGI_SWAP_CHAIN_FLAG(0))?;
            let mut render_target_view: Option<ID3D11RenderTargetView> = None;
            let back_buffer = self.swapchain.GetBuffer::<ID3D11Texture2D>(0)?;
            self.device.CreateRenderTargetView(&back_buffer, None, Some(&raw mut render_target_view))?;
            self.render_target_view = render_target_view;
        }
        Ok(())
    }
}