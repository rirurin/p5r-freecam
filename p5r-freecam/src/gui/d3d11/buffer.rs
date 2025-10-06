use windows::Win32::Graphics::Direct3D11::{
    D3D11_BUFFER_DESC,
    D3D11_USAGE_DYNAMIC,
    ID3D11Buffer,
    ID3D11Device,
};
use imgui::{ DrawIdx, DrawVert };

pub(crate) const VERTEX_BUF_ADD_CAPACITY: usize = 5000;
pub(crate) const INDEX_BUF_ADD_CAPACITY: usize = 10000;

#[derive(Debug)]
pub struct VertexBuffer(Option<ID3D11Buffer>, usize);
impl VertexBuffer {
    pub unsafe fn new(device: &ID3D11Device, vtx_count: usize) -> windows::core::Result<Self> {
        let len = vtx_count + VERTEX_BUF_ADD_CAPACITY;
        let mut out = Self(None, len);
        let desc = D3D11_BUFFER_DESC {
            ByteWidth: (len * size_of::<DrawVert>()) as u32,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: windows::Win32::Graphics::Direct3D11::D3D11_BIND_VERTEX_BUFFER.0 as u32,
            CPUAccessFlags: windows::Win32::Graphics::Direct3D11::D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        device.CreateBuffer(&desc, None, Some(&raw mut out.0))?;
        Ok(out)
    }

    pub fn len(&self) -> usize { self.1 }

    pub fn get_buffer(&self) -> Option<&ID3D11Buffer> {
        self.0.as_ref()
    }
    pub fn get_buffers(&self) -> *const Option<ID3D11Buffer> {
        &raw const self.0
    }
}
#[derive(Debug)]
pub struct IndexBuffer(Option<ID3D11Buffer>, usize);
impl IndexBuffer {
    pub unsafe fn new(device: &ID3D11Device, vtx_count: usize) -> windows::core::Result<Self> {
        let len = vtx_count + INDEX_BUF_ADD_CAPACITY;
        let mut out = Self(None, len);
        let desc = D3D11_BUFFER_DESC {
            ByteWidth: (len * size_of::<DrawIdx>()) as u32,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: windows::Win32::Graphics::Direct3D11::D3D11_BIND_INDEX_BUFFER.0 as u32,
            CPUAccessFlags: windows::Win32::Graphics::Direct3D11::D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        device.CreateBuffer(&desc, None, Some(&raw mut out.0))?;
        Ok(out)
    }

    pub fn len(&self) -> usize { self.1 }

    pub fn get_buffer(&self) -> Option<&ID3D11Buffer> {
        self.0.as_ref()
    }
}