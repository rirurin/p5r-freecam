use std::num::NonZeroUsize;
use glam::{EulerRot, Quat, Vec3A};
use imgui::Ui;
use riri_inspector_components::table::TableDraw;
use rkyv::{Archive, Deserialize, Place, Portable, Serialize};
use rkyv::bytecheck::CheckBytes;
use rkyv::rancor::{Error, Fallible};
#[cfg(target_endian = "big")]
use rkyv::rend::{f32_be, u32_be};
#[cfg(target_endian = "little")]
use rkyv::rend::{f32_le, u32_le};
use rkyv::traits::NoUndef;
use rkyv::rancor::Source as RkyvErrorSource;
use crate::state::camera::Freecam;

// quaternion
#[derive(Debug, Clone)]
pub struct FreecamNode {
    pub(crate) trans: Vec3A,
    pub(crate) rot: Quat,
}

impl FreecamNode {
    pub fn new(trans: Vec3A, rot: Quat) -> Self {
        Self { trans, rot }
    }
    pub fn new_euler(trans: Vec3A, pan: f32, pitch: f32, roll: f32) -> Self {
        let rot = Quat::from_euler(EulerRot::YXZEx, pan, pitch, roll);
        Self { trans, rot }
    }
}

impl Default for FreecamNode {
    fn default() -> Self {
        Self {
            trans: Vec3A::default(),
            rot: Quat::default()
        }
    }
}

#[cfg(target_endian = "little")]
pub(crate) type f32_ne = f32_le;
#[cfg(target_endian = "big")]
pub(crate) type f32_ne = f32_be;
#[cfg(target_endian = "little")]
pub(crate) type u32_ne = u32_le;
#[cfg(target_endian = "big")]
pub(crate) type u32_ne = u32_be;

#[repr(C)]
#[derive(Portable)]
pub struct ArchivedFreecamNode {
    trans: [f32_ne; 3],
    rot: [f32_ne; 4]
}

unsafe impl NoUndef for ArchivedFreecamNode {}

impl Into<ArchivedFreecamNode> for &FreecamNode {
    fn into(self) -> ArchivedFreecamNode {
        ArchivedFreecamNode {
            trans: [ f32_ne::from_native(self.trans.x), f32_ne::from_native(self.trans.y), f32_ne::from_native(self.trans.z) ],
            rot: [ f32_ne::from_native(self.rot.x), f32_ne::from_native(self.rot.y), f32_ne::from_native(self.rot.z), f32_ne::from_native(self.rot.w) ],
        }
    }
}

impl Into<FreecamNode> for &ArchivedFreecamNode {
    fn into(self) -> FreecamNode {
        FreecamNode {
            trans: Vec3A::new(f32_ne::to_native(self.trans[0]), f32_ne::to_native(self.trans[1]), f32_ne::to_native(self.trans[2])),
            rot: Quat::from_array([ f32_ne::to_native(self.rot[0]), f32_ne::to_native(self.rot[1]), f32_ne::to_native(self.rot[2]), f32_ne::to_native(self.rot[3]) ])
        }
    }
}

impl Archive for FreecamNode {
    type Archived = ArchivedFreecamNode;
    type Resolver = ();
    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        out.write(self.into());
    }
}

impl<D> Deserialize<FreecamNode, D> for ArchivedFreecamNode
where D: Fallible + ?Sized,
      D::Error: RkyvErrorSource
{
    fn deserialize(&self, _: &mut D) -> Result<FreecamNode, D::Error> {
        Ok(self.into())
    }
}

impl<S> Serialize<S> for FreecamNode
where S: Fallible + ?Sized,
      S::Error: RkyvErrorSource
{
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

unsafe impl<C> CheckBytes<C> for ArchivedFreecamNode
where C: Fallible, C::Error: RkyvErrorSource
{
    unsafe fn check_bytes(
        value: *const Self,
        _: &mut C,
    ) -> Result<(), <C as Fallible>::Error> {
        Ok(())
    }
}

pub struct FreecamNodeEntry<'a> {
    node: &'a FreecamNode,
    index: usize,
}

impl<'a> TableDraw<Freecam> for FreecamNodeEntry<'a> {
    fn draw_contents(&self, ui: &Ui, ctx: &mut Freecam, index: usize) {
        match index {
            0 => ui.text(format!("{}", self.index)),
            1 => {
                let mut trans: [f32; 3] = self.node.trans.into();
                ui.set_next_item_width(ui.content_region_avail()[0]);
                if ui.input_float3(format!("##TranslationForFreecamNodeEntry{}", self.index), &mut trans).build() {
                    ctx.nodes[self.index].trans = trans.into();
                }
            },
            2 => {
                let mut rot: [f32; 3] = self.node.rot.to_euler(EulerRot::YXZEx).into();
                ui.set_next_item_width(ui.content_region_avail()[0]);
                if ui.input_float3(format!("##RotationForFreecamNodeEntry{}", self.index), &mut rot).build() {
                    ctx.nodes[self.index].rot = Quat::from_euler(EulerRot::YXZEx, rot[0], rot[1], rot[2]);
                }
            },
            3 => {
                let is_preview_button = ctx.preview_node.map_or(false, |f| f.get() - 1 == self.index);
                if is_preview_button {
                    if ui.button(format!("Exit Preview##ForFreecamNodeEntry{}", self.index)) {
                        ctx.preview_node = None;
                    }
                } else {
                    if ui.button(format!("Preview##ForFreecamNodeEntry{}", self.index)) {
                        ctx.preview_node = Some(unsafe { NonZeroUsize::new_unchecked(self.index + 1) });
                    }
                }
                ui.same_line_with_spacing(0., 10.);
                if ui.button(format!("Remove##ForFreecamNodeEntry{}", self.index)) {
                    ctx.nodes.remove(self.index);
                }
            },
            _ => ()
        }
    }
}

impl<'a> FreecamNodeEntry<'a> {
    pub fn new(node: &'a FreecamNode, index: usize) -> Self {
        Self { node, index }
    }
}