use std::ffi::CStr;

use glam::{Mat4, Quat, Vec3, Vec4};

use crate::{lovr_sys, lovr, lovr::lovr_assert};

// enums

#[derive(Clone, Copy)]
pub enum DrawStyle { Fill, Line }

impl From<DrawStyle> for lovr_sys::DrawStyle {
    fn from(val: DrawStyle) -> Self {
        match val {
            DrawStyle::Fill => lovr_sys::DrawStyle_STYLE_FILL,
            DrawStyle::Line => lovr_sys::DrawStyle_STYLE_LINE,
        }
    }
}

#[derive(Clone, Copy)]
pub enum StackType { Transform, State }

impl From<StackType> for lovr_sys::StackType {
    fn from(val: StackType) -> Self {
        match val {
            StackType::Transform => lovr_sys::StackType_STACK_TRANSFORM,
            StackType::State => lovr_sys::StackType_STACK_STATE,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BlendMode { Alpha, Add, Subtract, Multiply, Lighten, Darken, Screen, None }

impl From<BlendMode> for lovr_sys::BlendMode {
    fn from(val: BlendMode) -> Self {
        match val {
            BlendMode::Alpha => lovr_sys::BlendMode_BLEND_ALPHA,
            BlendMode::Add => lovr_sys::BlendMode_BLEND_ADD,
            BlendMode::Subtract => lovr_sys::BlendMode_BLEND_SUBTRACT,
            BlendMode::Multiply => lovr_sys::BlendMode_BLEND_MULTIPLY,
            BlendMode::Lighten => lovr_sys::BlendMode_BLEND_LIGHTEN,
            BlendMode::Darken => lovr_sys::BlendMode_BLEND_DARKEN,
            BlendMode::Screen => lovr_sys::BlendMode_BLEND_SCREEN,
            BlendMode::None => lovr_sys::BlendMode_BLEND_NONE,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BlendAlphaMode { AlphaMultiply, Premultiplied }

impl From<BlendAlphaMode> for lovr_sys::BlendAlphaMode {
    fn from(val: BlendAlphaMode) -> Self {
        match val {
            BlendAlphaMode::AlphaMultiply => lovr_sys::BlendAlphaMode_BLEND_ALPHA_MULTIPLY,
            BlendAlphaMode::Premultiplied => lovr_sys::BlendAlphaMode_BLEND_PREMULTIPLIED,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CullMode { None, Front, Back }

impl From<CullMode> for lovr_sys::CullMode {
    fn from(val: CullMode) -> Self {
        match val {
            CullMode::None => lovr_sys::CullMode_CULL_NONE,
            CullMode::Front => lovr_sys::CullMode_CULL_FRONT,
            CullMode::Back => lovr_sys::CullMode_CULL_BACK,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CompareMode { None, Equal, NEqual, Less, LEqual, Greater, GEqual }

impl From<CompareMode> for lovr_sys::CompareMode {
    fn from(val: CompareMode) -> Self {
        match val {
            CompareMode::None => lovr_sys::CompareMode_COMPARE_NONE,
            CompareMode::Equal => lovr_sys::CompareMode_COMPARE_EQUAL,
            CompareMode::NEqual => lovr_sys::CompareMode_COMPARE_NEQUAL,
            CompareMode::Less => lovr_sys::CompareMode_COMPARE_LESS,
            CompareMode::LEqual => lovr_sys::CompareMode_COMPARE_LEQUAL,
            CompareMode::Greater => lovr_sys::CompareMode_COMPARE_GREATER,
            CompareMode::GEqual => lovr_sys::CompareMode_COMPARE_GEQUAL,
        }
    }
}

#[derive(Clone, Copy)]
pub enum DrawMode { Points, Lines, Triangles }

impl From<DrawMode> for lovr_sys::DrawMode {
    fn from(val: DrawMode) -> Self {
        match val {
            DrawMode::Points => lovr_sys::DrawMode_DRAW_POINTS,
            DrawMode::Lines => lovr_sys::DrawMode_DRAW_LINES,
            DrawMode::Triangles => lovr_sys::DrawMode_DRAW_TRIANGLES,
        }
    }
}

#[derive(Clone, Copy)]
pub enum StencilAction { Keep, Zero, Replace, Increment, Decrement, IncrementWrap, DecrementWrap, Invert }

impl From<StencilAction> for lovr_sys::StencilAction {
    fn from(val: StencilAction) -> Self {
        match val {
            StencilAction::Keep => lovr_sys::StencilAction_STENCIL_KEEP,
            StencilAction::Zero => lovr_sys::StencilAction_STENCIL_ZERO,
            StencilAction::Replace => lovr_sys::StencilAction_STENCIL_REPLACE,
            StencilAction::Increment => lovr_sys::StencilAction_STENCIL_INCREMENT,
            StencilAction::Decrement => lovr_sys::StencilAction_STENCIL_DECREMENT,
            StencilAction::IncrementWrap => lovr_sys::StencilAction_STENCIL_INCREMENT_WRAP,
            StencilAction::DecrementWrap => lovr_sys::StencilAction_STENCIL_DECREMENT_WRAP,
            StencilAction::Invert => lovr_sys::StencilAction_STENCIL_INVERT,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Winding { CounterClockwise, Clockwise }

impl From<Winding> for lovr_sys::Winding {
    fn from(val: Winding) -> Self {
        match val {
            Winding::CounterClockwise => lovr_sys::Winding_WINDING_COUNTERCLOCKWISE,
            Winding::Clockwise => lovr_sys::Winding_WINDING_CLOCKWISE,
        }
    }
}

// functions

pub fn get_window_pass() -> lovr::Result<Option<Pass>> {
    unsafe {
        let mut pass_ptr: *mut lovr_sys::Pass = std::ptr::null_mut();
        lovr_assert(lovr_sys::lovrGraphicsGetWindowPass(&mut pass_ptr))?;
        Ok(Pass::from_raw(pass_ptr))
    }
}

pub fn submit(passes: &mut [Pass]) -> lovr::Result<()> {
    unsafe {
        let mut pass_ptrs: Vec<*mut lovr_sys::Pass> = passes.iter_mut().map(|p| p.ptr).collect();
        lovr_assert(lovr_sys::lovrGraphicsSubmit(
            pass_ptrs.as_mut_ptr(),
            passes.len().try_into().unwrap() // TODO: don't use unwrap, return error
        ))
    }
}

pub fn present() -> lovr::Result<()> {
    unsafe {
        lovr_assert(lovr_sys::lovrGraphicsPresent())
    }
}

pub struct Pass {
    ptr: *mut lovr_sys::Pass
}

impl Pass {
    pub unsafe fn from_raw(ptr: *mut lovr_sys::Pass) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            unsafe {
                lovr_sys::lovrRetain(ptr as *mut std::ffi::c_void);
            }
            Some(Self { ptr })
        }
    }
}

impl Drop for Pass {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                lovr_sys::lovrRelease(
                    self.ptr as *mut std::ffi::c_void,
                    Some(lovr_sys::lovrPassDestroy));
            }
        }
    }
}

impl Pass {
    pub fn capsule(&mut self, transform: Mat4, segments: u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassCapsule(self.ptr, transform.to_cols_array().as_mut_ptr(), segments))
        }
    }

    pub fn circle(&mut self, transform: Mat4, style: DrawStyle, angle1: f32, angle2: f32, segments: u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassCircle(self.ptr, transform.to_cols_array().as_mut_ptr(), style.into(), angle1, angle2, segments))
        }
    }

    pub fn cone(&mut self, transform: Mat4, segments: u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassCone(self.ptr, transform.to_cols_array().as_mut_ptr(), segments))
        }
    }

    pub fn cube(&mut self, transform: Mat4, style: DrawStyle) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassBox(self.ptr, transform.to_cols_array().as_mut_ptr(), style.into()))
        }
    }

    pub fn cylinder(&mut self, transform: Mat4, capped: bool, angle1: f32, angle2: f32, segments: u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassCylinder(self.ptr, transform.to_cols_array().as_mut_ptr(), capped, angle1, angle2, segments))
        }
    }

    // TODO: draw_model
    // TODO: draw_part
    // TODO: draw_mesh
    // TODO: draw_texture

    // TODO: fill

    fn convert_points(vertices: &[Vec3]) -> (Vec<f32>, Vec<*const f32>){
        // TODO: should return an object with the data, the len as u32, and the mutable pointer
        let mut flat: Vec<f32> = Vec::with_capacity(vertices.len() * 3);
        for v in vertices {
            flat.extend_from_slice(&v.to_array());
        }
        let ptrs: Vec<*const f32> = flat
            .chunks_exact(3)
            .map(|chunk| chunk.as_ptr())
            .collect();
        (flat, ptrs)
    }

    pub fn line(&mut self, vertices: &[Vec3]) -> lovr::Result<()> {
        let (_flat, ptrs) = Self::convert_points(vertices);
        unsafe {
            lovr_assert(lovr_sys::lovrPassLine(
                self.ptr,
                ptrs.len().try_into().unwrap(), // TODO: don't use unwrap, return error
                ptrs.as_ptr() as *mut *mut f32))
        }
    }

    // TODO: mesh

    pub fn plane(&mut self, transform: Mat4, style: DrawStyle, cols: u32, rows: u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassPlane(self.ptr, transform.to_cols_array().as_mut_ptr(), style.into(), cols, rows))
        }
    }

    pub fn points(&mut self, vertices: &[Vec3]) -> lovr::Result<()> {
        let (_flat, ptrs) = Self::convert_points(vertices);
        unsafe {
            lovr_assert(lovr_sys::lovrPassPoints(
                self.ptr,
                ptrs.len().try_into().unwrap(), // TODO: don't use unwrap, return error
                ptrs.as_ptr() as *mut *mut f32))
        }
    }

    pub fn polygon(&mut self, vertices: &[Vec3]) -> lovr::Result<()> {
        let (_flat, ptrs) = Self::convert_points(vertices);
        unsafe {
            lovr_assert(lovr_sys::lovrPassPolygon(
                self.ptr,
                ptrs.len().try_into().unwrap(), // TODO: don't use unwrap, return error
                ptrs.as_ptr() as *mut *mut f32))
        }
    }

    pub fn round_rect(&mut self, transform: Mat4, radius: f32, segments: u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassRoundrect(self.ptr, transform.to_cols_array().as_mut_ptr(), radius, segments))
        }
    }

    // TODO: skybox

    pub fn sphere(&mut self, transform: Mat4, longitudes: u32, latitudes:u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassSphere(self.ptr, transform.to_cols_array().as_mut_ptr(), longitudes, latitudes))
        }
    }

    // TODO: text

    pub fn torus(&mut self, transform: Mat4, segments_t: u32, segments_p:u32) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassTorus(self.ptr, transform.to_cols_array().as_mut_ptr(), segments_t, segments_p))
        }
    }
 
    pub fn origin(&mut self) {
        unsafe {
            lovr_sys::lovrPassOrigin(self.ptr)
        }
    }

    pub fn pop(&mut self, stack: StackType) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassPop(self.ptr, stack.into()))
        }
    }

    pub fn push(&mut self, stack: StackType) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassPush(self.ptr, stack.into()))
        }
    }

    pub fn rotate(&mut self, rotation: Quat) {
        let mut quat_array = rotation.to_array();
        unsafe {
            lovr_sys::lovrPassRotate(self.ptr, quat_array.as_mut_ptr())
        }
    }

    pub fn scale(&mut self, scale: Vec3) {
        let mut vec3_array = scale.to_array();
        unsafe {
            lovr_sys::lovrPassScale(self.ptr, vec3_array.as_mut_ptr())
        }
    }

    pub fn transform(&mut self, transform: Mat4) {
        unsafe {
            lovr_sys::lovrPassTransform(self.ptr, transform.to_cols_array().as_mut_ptr())
        }
    }

    pub fn translate(&mut self, translation: Vec3) {
        let mut vec3_array = translation.to_array();
        unsafe {
            lovr_sys::lovrPassTranslate(self.ptr, vec3_array.as_mut_ptr())
        }
    }
    
    pub fn set_alpha_to_coverage(&mut self, enabled: bool) {
        unsafe {
            lovr_sys::lovrPassSetAlphaToCoverage(self.ptr, enabled)
        }
    }

    pub fn set_blend_mode(&mut self, index: u32, mode: BlendMode, alpha_mode: BlendAlphaMode) {
        unsafe {
            lovr_sys::lovrPassSetBlendMode(self.ptr, index, mode.into(), alpha_mode.into())
        }
    }

    pub fn set_color(&mut self, color: Vec4) {
        let mut vec4_array = color.to_array();
        unsafe {
            lovr_sys::lovrPassSetColor(self.ptr, vec4_array.as_mut_ptr())
        }
    }

    pub fn set_color_write(&mut self, index: u32, r: bool, g: bool, b: bool, a: bool) {
        unsafe {
            lovr_sys::lovrPassSetColorWrite(self.ptr, index, r, g, b, a)
        }
    }

    pub fn set_depth_clamp(&mut self, enabled: bool) {
        unsafe {
            lovr_sys::lovrPassSetDepthClamp(self.ptr, enabled)
        }
    }

    pub fn set_depth_offset(&mut self, offset: f32, sloped: f32) {
        unsafe {
            lovr_sys::lovrPassSetDepthOffset(self.ptr, offset, sloped)
        }
    }

    pub fn set_depth_test(&mut self, test: CompareMode) {
        unsafe {
            lovr_sys::lovrPassSetDepthTest(self.ptr, test.into())
        }
    }

    pub fn set_depth_write(&mut self, enabled: bool) {
        unsafe {
            lovr_sys::lovrPassSetDepthWrite(self.ptr, enabled)
        }
    }

    pub fn set_face_cull(&mut self, mode: CullMode) {
        unsafe {
            lovr_sys::lovrPassSetFaceCull(self.ptr, mode.into())
        }
    }

    // TODO: set_font
    // TODO: set_material

    pub fn set_mesh_mode(&mut self, mode: DrawMode) {
        unsafe {
            lovr_sys::lovrPassSetMeshMode(self.ptr, mode.into())
        }
    }

    // TODO: set_sampler
   
    pub fn set_stencil_test(&mut self, mode: CompareMode, value: u8, mask: u8) -> lovr::Result<()>{
        unsafe {
            lovr_assert(lovr_sys::lovrPassSetStencilTest(self.ptr, mode.into(), value, mask))
        }
    }

    pub fn set_stencil_write(&mut self, action: &[StencilAction; 3], value: u8, mask: u8) -> lovr::Result<()>{
        let mut actions: [lovr_sys::StencilAction; 3] = action.map(|x| x.into());
        unsafe {
            lovr_assert(lovr_sys::lovrPassSetStencilWrite(self.ptr, actions.as_mut_ptr(), value, mask))
        }
    }

    pub fn set_view_cull(&mut self, enabled: bool) {
        unsafe {
            lovr_sys::lovrPassSetViewCull(self.ptr, enabled)
        }
    }

    pub fn set_winding(&mut self, winding: Winding) {
        unsafe {
            lovr_sys::lovrPassSetWinding(self.ptr, winding.into())
        }
    }

    pub fn set_wireframe(&mut self, enabled: bool) {
        unsafe {
            lovr_sys::lovrPassSetWireframe(self.ptr, enabled)
        }
    }

    // TODO: send_buffer
    // TODO: send_texture
    // TODO: send_sampler
    // TODO: send_data
    // TODO: set_shader

    pub fn barrier(&mut self) {
        unsafe {
            lovr_sys::lovrPassBarrier(self.ptr)
        }
    }

    // TODO: compute

    // TODO: begin_tally
    // TODO: finish_tally
    // TODO: get_tally_buffer
    // TODO: set_tally_buffer

    pub fn get_projection(&mut self, index: u32) -> lovr::Result<Mat4> {
        let mut cols_array = [0.; 16];
        unsafe {
            lovr_assert(lovr_sys::lovrPassGetProjection(self.ptr, index, cols_array.as_mut_ptr()))?
        }
        Ok(Mat4::from_cols_array(&cols_array))
    }

    pub fn get_view_count(&mut self) -> u32 {
        unsafe {
            lovr_sys::lovrPassGetViewCount(self.ptr)
        }
    }

    pub fn get_view_matrix(&mut self, index: u32) -> lovr::Result<Mat4> {
        let mut cols_array = [0.; 16];
        unsafe {
            lovr_assert(lovr_sys::lovrPassGetViewMatrix(self.ptr, index, cols_array.as_mut_ptr()))?
        }
        Ok(Mat4::from_cols_array(&cols_array))
    }

    pub fn set_projection(&mut self, index: u32, transform: Mat4) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassSetProjection(self.ptr, index, transform.to_cols_array().as_mut_ptr()))
        }
    }

    pub fn set_scissor(&mut self, x: u32, y: u32, w: u32, h: u32) {
        unsafe {
            lovr_sys::lovrPassSetScissor(self.ptr, [x, y, w, h].as_mut_ptr())
        }
    }

    pub fn set_view_matrix(&mut self, index: u32, transform: Mat4) -> lovr::Result<()> {
        unsafe {
            lovr_assert(lovr_sys::lovrPassSetViewMatrix(self.ptr, index, transform.to_cols_array().as_mut_ptr()))
        }
    }

    pub fn set_viewport(&mut self, x: f32, y: f32, w: f32, h: f32, depth_min: f32, depth_max: f32) {
        unsafe {
            lovr_sys::lovrPassSetViewport(self.ptr, [x, y, w, h, depth_min, depth_max].as_mut_ptr())
        }
    }

    // TODO: get_canvas
    // TODO: get_clear
    
    pub fn get_dimensions(&mut self) -> [u32; 2] {
        unsafe {
            [ lovr_sys::lovrPassGetWidth(self.ptr), lovr_sys::lovrPassGetHeight(self.ptr) ]
        }
    }

    pub fn get_height(&mut self) -> u32 {
        unsafe {
            lovr_sys::lovrPassGetHeight(self.ptr)
        }
    }

    pub fn get_width(&mut self) -> u32 {
        unsafe {
            lovr_sys::lovrPassGetWidth(self.ptr)
        }
    }

    // TODO: set_canvas
    // TODO: set_clear

    pub fn get_label(&mut self) -> Option<String> {
        unsafe {
            let ptr = lovr_sys::lovrPassGetLabel(self.ptr);
            if ptr.is_null() {
                return None
            }
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    }

    // TODO: get_stats

    pub fn reset(&mut self) {
        unsafe {
            lovr_sys::lovrPassReset(self.ptr)
        }
    }
}
