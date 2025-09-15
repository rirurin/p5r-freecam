use bitflags::bitflags;
use crate::fld::main::MainParam;
use glam::{ Vec2, Vec3A };
use std::ptr::NonNull;
use opengfd::{
    pak::file::PakFile,
    utility::misc::Rect
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RoadmapState : u32 {
        const TEX_READY=1;
        const TEX_READ=2;
        const WHOLE=4;
        const WHOLE_FRAME=8;
        const VISIBLE=16;
        const INDEPENDENT=32;
        const MASKDRAW=64;
        const FRAMEIN=128;
        const FRAMEIN_ANIM=256;
        const FRAMEOUT_ANIM=512;
        const TEX_READ_PAUSE=1024;
        const TEX_CHANGE=2048;
        const ALPHA_INC=4096;
        const ALPHA_DEC=8192;
        const DEBUG=2147483648;
    }
}

#[allow(non_camel_case_types)]
type fldPanelRoadmap_t = u8;
#[allow(non_camel_case_types)]
type fldPanelRoadmapTex_t = u8;
#[allow(non_camel_case_types)]
type fldPanelRoadmapParts_t = u8;

#[repr(C)]
pub struct Roadmap {
    field0: [u8; 0x10],
    step: u32,
    state: RoadmapState,
    field18: usize,
    h_roadmap_tbl: *mut PakFile,
    p_roadmap_tbl: *mut fldPanelRoadmap_t,
    h_tex_pack_tbl: *mut PakFile,
    p_tex_pack_tbl: *mut fldPanelRoadmapTex_t,
    h_parts_tbl: *mut PakFile,
    p_parts_tbl: *mut fldPanelRoadmapParts_t,
    h_disp_tbl: *mut PakFile,
    p_disp_tbl: *mut fldPanelRoadmapParts_t,
    p_roadmap: *mut fldPanelRoadmap_t,
    p_tex_pack: *mut fldPanelRoadmapTex_t,
    p_parts: *mut fldPanelRoadmapParts_t,
    p_disp: *mut fldPanelRoadmapParts_t,
    p_current_parts: *mut fldPanelRoadmapParts_t,
    p_current_disp: *mut fldPanelRoadmapParts_t,
    texture: Option<NonNull<u8>>, // GfdTexture
    alpha_ratio: f32,
    alpha_2_ratio: f32,
    p_layer_symbol_geom: [Vec3A; 6],
    pxl_length: f32,
    current_layer: i32,
    base_rc: Rect,
    disp_rc: Rect,
    clip_rc: Rect,
    layer_order: i32,
    mask_circle_center: Vec2,
    mask_circle_radius: f32,
    anim_rot_center: Vec2,
    anim_rot: f32,
    anim_time: f32,
    p_parent_work: NonNull<u8>,
    p_field_param: MainParam,
}
