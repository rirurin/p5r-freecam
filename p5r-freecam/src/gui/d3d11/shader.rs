use glam::Mat4;
use windows::{
    core::PCSTR,
    Win32::Graphics::{
        Dxgi::Common::{
            DXGI_FORMAT_R32G32_FLOAT,
            DXGI_FORMAT_R8G8B8A8_UNORM
        },
        Direct3D11::{
            D3D11_BIND_CONSTANT_BUFFER,
            D3D11_BUFFER_DESC,
            D3D11_CPU_ACCESS_WRITE,
            D3D11_INPUT_PER_VERTEX_DATA,
            D3D11_INPUT_ELEMENT_DESC,
            D3D11_USAGE_DYNAMIC,
            ID3D11Buffer,
            ID3D11Device,
            ID3D11InputLayout,
            ID3D11PixelShader,
            ID3D11VertexShader,
        }
    }
};

#[derive(Debug)]
pub struct VertexShader {
    shader: Option<ID3D11VertexShader>,
    input_layout: Option<ID3D11InputLayout>,
    cbuffer: Option<ID3D11Buffer>
}
impl VertexShader {
    fn uninit() -> Self {
        Self {
            shader: None,
            input_layout: None,
            cbuffer: None,
        }
    }
    pub unsafe fn new(device: &ID3D11Device) -> windows::core::Result<VertexShader> {
        let mut out = VertexShader::uninit();
        const VERTEX_SHADER: &[u8] = include_bytes!("vs.dxbc");
        device.CreateVertexShader(
            VERTEX_SHADER, 
            None, 
            Some(&raw mut out.shader)
        )?;
        let local_layout = [
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR(b"POSITION\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR(b"TEXCOORD\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 8,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PCSTR("COLOR\0".as_ptr()),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                InputSlot: 0,
                AlignedByteOffset: 16,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];
        device.CreateInputLayout(
            &local_layout, 
            VERTEX_SHADER, 
            Some(&raw mut out.input_layout)
        )?;
        let desc = D3D11_BUFFER_DESC {
            ByteWidth: size_of::<Mat4>() as _,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
            StructureByteStride: 0,
        };
        device.CreateBuffer(&desc, None, Some(&raw mut out.cbuffer))?;
        Ok(out)
    }
    pub fn get_shader(&self) -> Option<&ID3D11VertexShader> {
        self.shader.as_ref()
    }

    pub fn get_input_layout(&self) -> Option<&ID3D11InputLayout> {
        self.input_layout.as_ref()
    }

    pub fn get_constant_buffer(&self) -> Option<&ID3D11Buffer> {
        self.cbuffer.as_ref()
    }

    pub fn get_constant_buffer_owned(&self) -> Option<ID3D11Buffer> {
        self.cbuffer.clone()
    }
}

#[derive(Debug)]
pub struct PixelShader(Option<ID3D11PixelShader>);
impl PixelShader {
    pub unsafe fn new(device: &ID3D11Device) -> windows::core::Result<Self> {
        let mut out = None;
        let pixel_shader: &[u8] = include_bytes!("ps.dxbc");
        device.CreatePixelShader(
            pixel_shader,
            None, 
            Some(&raw mut out)
        )?;
        Ok(Self(out))
    }
    pub fn get_shader(&self) -> Option<&ID3D11PixelShader> {
        self.0.as_ref()
    }
}