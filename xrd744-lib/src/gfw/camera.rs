use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use opengfd::{
    kernel::allocator::GfdAllocator,
    object::{
        camera::Camera as GfdCamera,
        node::Node as GfdNode
    }
};

#[repr(C)]
pub struct Camera<A = GfdAllocator>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    resrc: *mut u8,
    node: GfdNode<A>,
    // field138: *mut Resrc,
    entity: Option<NonNull<GfdCamera<A>>>,
    field148: *mut u8,
    _allocator: A
}

impl<A> Camera<A>
where A: Allocator + Clone
{
    pub fn get_entity(&self) -> Option<&GfdCamera<A>> {
        unsafe { self.entity.map(|v| v.as_ref()) }
    }
    pub fn get_entity_mut(&mut self) -> Option<&mut GfdCamera<A>> {
        unsafe { self.entity.map(|mut v| v.as_mut()) }
    }
    pub fn get_node(&self) -> &GfdNode<A> { &self.node }
    pub fn get_node_mut(&mut self) -> &mut GfdNode<A> { &mut self.node }
}