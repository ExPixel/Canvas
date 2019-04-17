use super::math::*;
use super::opengl::{
    VertexArray,
    Buffer,
    BufferType,
    BufferUsage,
    Shader,
    ShaderType,
    Program,
    Texture,
    BufferDataType,
    check_gl_errors,
};

const MAX_VERTS: usize = 40;
const MAX_ELEMS: usize = 64;

pub struct Context {
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    elems_buffer: Buffer,

    uniform_projmtx: gl::types::GLint,
    uniform_transform: gl::types::GLint,

    program: Program,
    shaders: (/* vertex shader */ Shader, /* fragment shader */ Shader),

    vertices: Vec<Vert>,
    elements: Vec<u32>,

    /// Final transform applied to each vertex to convert from the screen's coordinates to
    /// device coordinates.
    ortho_matrix: Mat4f,
    transform: Decomposed2f,
}

impl Context {
    pub fn new() -> Context {
        let vertex_shader = Shader::compile(ShaderType::Vertex, VERTEX_SHADER).expect("failed to compile vertex shader");
        let fragment_shader = Shader::compile(ShaderType::Fragment, FRAGMENT_SHADER).expect("failed to compile fragment shader");
        let program = Program::link(&[&vertex_shader, &fragment_shader]).expect("failed to link GL program");

        let attrib_pos = program.attrib_location("Position\0");
        let attrib_col = program.attrib_location("Color\0");
        let uniform_projmtx = program.uniform_location("ProjMtx\0");
        let uniform_transform = program.uniform_location("Transform\0");

        let vertex_array = VertexArray::new();
        let vertex_buffer = Buffer::new(BufferType::ArrayBuffer);

        vertex_array.bind();
        vertex_buffer.bind();

        unsafe {
            let szfloat = std::mem::size_of::<f32>() as i32;
            gl::EnableVertexAttribArray(attrib_pos as _);
            gl::VertexAttribPointer(attrib_pos as _, 2, gl::FLOAT, gl::FALSE, 6 * szfloat, std::mem::transmute(0usize));
            gl::EnableVertexAttribArray(attrib_col as _);
            gl::VertexAttribPointer(attrib_col as _, 4, gl::FLOAT, gl::FALSE, 6 * szfloat, std::mem::transmute(2 * szfloat as usize));
        }

        check_gl_errors(|e| println!("GL Error: {}", e));

        Context {
            vertex_array:   vertex_array,
            vertex_buffer:  vertex_buffer,
            elems_buffer:   Buffer::new(BufferType::ElementArrayBuffer),
            uniform_projmtx: uniform_projmtx,
            uniform_transform: uniform_transform,

            program:        program,
            shaders:        (vertex_shader, fragment_shader),

            vertices:       Vec::with_capacity(MAX_VERTS),
            elements:       Vec::with_capacity(MAX_ELEMS),

            ortho_matrix:   cgmath::ortho(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0),
            transform:      decomposed2f(1.0, 0.0, vec2f(0.0, 0.0)),
        }
    }

    pub fn push_verts(&mut self, verts: &[Vert], elems: &[u32]) {
        if self.vertices.len() + verts.len() > MAX_VERTS || self.elements.len() + elems.len() > MAX_ELEMS{
            self.flush_verts();
        }

        debug_assert!(elems.len() % 3 == 0, "number of elements must be a multiple of 3");
        let elem_delta = self.vertices.len() as u32;

        for v in verts.iter() {
            // self.vertices.push(v.transform(&self.display_transform));
            self.vertices.push(v.clone());
        }

        for e in elems.iter() {
            debug_assert!(*e < verts.len() as u32, "element is larger than the number of vertices provided");
            self.elements.push((*e) + elem_delta);
        }
    }

    pub fn flush_verts(&mut self) {
        if self.vertices.len() == 0 { return }

        self.program.bind();

        unsafe {
            gl::UniformMatrix4fv(self.uniform_projmtx, 1, gl::FALSE, self.ortho_matrix.as_ptr());
            let transform_mtx: Mat3f = self.transform.into();
            gl::UniformMatrix3fv(self.uniform_transform, 1, gl::FALSE, transform_mtx.as_ptr());
        }

        self.vertex_array.bind();
        self.vertex_buffer.bind();
        self.elems_buffer.bind();
        self.vertex_buffer.set_data(&self.vertices, BufferUsage::StreamDraw);
        self.elems_buffer.set_data(&self.elements, BufferUsage::StreamDraw);

        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.elements.len() as _, gl::UNSIGNED_INT, std::mem::transmute(0usize));
        }

        self.vertices.clear();
        self.elements.clear();
    }

    pub fn set_clear_color(&self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn rect<P: Into<f32>, S: Into<f32>>(&mut self, color: Color, x: P, y: P, width: S, height: S) {
        let (x, y, w, h) = (x.into(), y.into(), width.into(), height.into());

        let vcol = VertCol::new(color.r, color.g, color.b, color.a);
        let tl = Vert::with_pc(VertPos::new(x, y), vcol);
        let tr = Vert::with_pc(VertPos::new(x + w, y), vcol);
        let bl = Vert::with_pc(VertPos::new(x, y - h), vcol);
        let br = Vert::with_pc(VertPos::new(x + w, y - h), vcol);

        // let tl_t = tl.transform(&self.display_transform);
        // let tr_t = tr.transform(&self.display_transform);
        // let bl_t = bl.transform(&self.display_transform);
        // let br_t = br.transform(&self.display_transform);
        // unsafe {
        //     println!("   TOP  LEFT: {}, {} -> {}, {}", tl.pos.x, tl.pos.y, tl_t.pos.x, tl_t.pos.y);
        //     println!("   TOP RIGHT: {}, {} -> {}, {}", tr.pos.x, tr.pos.y, tr_t.pos.x, tr_t.pos.y);
        //     println!("BOTTOM  LEFT: {}, {} -> {}, {}", bl.pos.x, bl.pos.y, bl_t.pos.x, bl_t.pos.y);
        //     println!("BOTTOM RIGHT: {}, {} -> {}, {}", br.pos.x, br.pos.y, br_t.pos.x, br_t.pos.y);
        // }
        // println!();

        self.push_verts(
            &[tl, tr, bl, br],
            &[0, 1, 2, 2, 3, 1],
        );
    }

    pub fn set_display_size(&mut self, width: f32, height: f32) {
        self.ortho_matrix = cgmath::ortho(0.0, width, height, 0.0, -1.0, 1.0);
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VertPos {
    pub x: f32,
    pub y: f32,
}

impl VertPos {
    #[inline]
    pub fn new(x: f32, y: f32) -> VertPos {
        VertPos { x, y }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct VertCol {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl VertCol {
    #[inline]
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> VertCol {
        VertCol { r, g, b, a }
    }
}

#[repr(C, packed)]
pub struct Vert {
    pub pos: VertPos,
    pub col: VertCol,
}

impl Vert {
    #[inline]
    pub fn new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> Vert {
        Vert {
            pos: VertPos::new(x, y),
            col: VertCol::new(r, g, b, a),
        }
    }

    #[inline]
    pub fn with_pc(pos: VertPos, col: VertCol) -> Vert {
        Vert {
            pos: pos,
            col: col,
        }
    }
}

impl Clone for Vert {
    fn clone(&self) -> Self {
        Vert::new(
            self.pos.x, self.pos.y,
            self.col.r, self.col.g, self.col.b, self.col.a
        )
    }
}

impl super::opengl::BufferDataType for Vert {}

pub const VERTEX_SHADER: &str   = "\
#version 130

uniform mat3 Transform;
uniform mat4 ProjMtx;
in  vec2 Position;
in  vec4 Color;
out vec4 FragColor;

void main() {
    FragColor = Color;
    vec3 t = Transform * vec3(Position.xy, 1.0);
    vec4 pos = vec4(t.xy, 1.0, 1.0);
    gl_Position = ProjMtx * pos;
}\0";

pub const FRAGMENT_SHADER: &str = "\
#version 130

in  vec4 FragColor;
out vec4 OutColor;

void main() {
    OutColor = FragColor;
}\0";

