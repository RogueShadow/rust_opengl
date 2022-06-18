use std::mem::{size_of, size_of_val};

const VERT_SHADER: &str = r##"
    #version 330 core
    layout (location = 0) in vec3 pos;
    layout (location = 1) in vec2 aTexCoord;
    layout (location = 2) in vec4 aColor;

    out vec2 TexCoord;
    out vec4 Color;

    void main() {
        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        TexCoord = aTexCoord;
        Color = aColor;
    }
    "##;
const FRAG_SHADER: &str = r##"
    #version 330 core
    out vec4 final_color;

    in vec2 TexCoord;
    in vec4 Color;

    uniform sampler2D tex;

    void main() {
        final_color = texture(tex, TexCoord) * Color;
    }
    "##;

pub struct GsnRenderer {
    vao: u32,
    vbo: u32,
    ebo: u32,
    shader_program: u32,
    screen_buffer_texture: u32,
    clear_color: Pixel,
    width: u32,
    height: u32,
    pub(crate) buffer: GsnSprite,
}


pub fn new_gsn_renderer() -> GsnRenderer {
    let gsn_renderer = GsnRenderer {
        vao: 0,
        vbo: 0,
        ebo: 0,
        shader_program: 0,
        screen_buffer_texture: 0,
        clear_color: pixel_rgb(0,0,0),
        width: 0,
        height: 0,
        buffer: GsnSprite {
            width: 0,
            height: 0,
            data: vec![]
        },
    };

    gsn_renderer
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

pub fn pixel_rgb(r: u8, g: u8, b: u8) -> Pixel {
    Pixel {
        r: (r as f32 / u8::MAX as f32),
        g: (g as f32 / u8::MAX as f32),
        b: (b as f32 / u8::MAX as f32),
        a: 1.0,
    }
}


pub const WHITE: Pixel = Pixel {r: 1.0, g: 1.0, b: 1.0, a: 1.0};
pub const BLACK: Pixel = Pixel {r: 0.0, g: 0.0, b: 0.0, a: 1.0};
pub const RED: Pixel = Pixel {r: 1.0, g: 0.0, b: 0.0, a: 1.0};
pub const GREEN: Pixel = Pixel {r: 0.0, g: 1.0, b: 0.0, a: 1.0};
pub const BLUE: Pixel = Pixel {r: 0.0, g: 0.0, b: 1.0, a: 1.0};

pub struct GsnSprite {
    pub(crate) width: u32,
    pub(crate) height: u32,
    data: Vec<Pixel>,
}

impl GsnSprite {
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Pixel) -> bool {
        if x < self.width.try_into().unwrap() && y < self.height.try_into().unwrap() {
            let index: usize = y as usize * self.width as usize + x as usize;
            self.data[index] = pixel;
            true
        }else{
            false
        }
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let x = x.clamp(0,self.width);
        let y = y.clamp(0, self.height);
        let index: usize = y as usize * self.width as usize + x as usize;
        self.data[index]

    }
    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, p: Pixel) {
        let x = x.clamp(0,self.width);
        let y = y.clamp(0, self.height);
        let x2 = (x + w).clamp( 0,self.width);
        let y2 = (y + h).clamp(0,self.height);

        for dx in x..x2 {
            for dy in y..y2 {
                self.set_pixel(dx,dy,p);
            }
        }
    }
    pub fn clear(&mut self, p: Pixel) {
        self.fill_rect(0, 0, self.width, self.height, p);
    }
}

impl GsnRenderer {
    pub fn initialize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;


        self.buffer = GsnSprite {
            width,
            height,
            data: Vec::with_capacity((width * height).try_into().unwrap()),
        };

        unsafe { self.buffer.data.set_len(self.buffer.data.capacity()); }
        for p in self.buffer.data.iter_mut() {
            *p = pixel_rgb(0, 0, 0);
        }


        unsafe {
            gl::ClearColor(
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            );

            gl::GenVertexArrays(1, &mut self.vao);
            assert_ne!(self.vao, 0);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            assert_ne!(0, self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            gl::GenBuffers(1, &mut self.ebo);
            assert_ne!(0, self.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                size_of_val(&INDICES) as isize,
                INDICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            type Vertex = [f32; 9];
            const ZOOM: f32 = 1.0;
            const VERTICES: [Vertex; 4] =
                [[-ZOOM, -ZOOM, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], // bottom left
                    [ZOOM, -ZOOM, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0],  // bottom right
                    [ZOOM, ZOOM, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],   // top right
                    [-ZOOM, ZOOM, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0]   // top left
                ];

            const INDICES: [i32; 6] = [
                0, 1, 3,
                1, 2, 3
            ];

            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(&VERTICES) as isize,
                VERTICES.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                (size_of::<f32>() * 3) as *const _,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                (size_of::<f32>() * 5) as *const _,
            );

            gl::GenTextures(1, &mut self.screen_buffer_texture);
            gl::BindTexture(gl::TEXTURE_2D, self.screen_buffer_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA.try_into().unwrap(),
                self.buffer.width.try_into().unwrap(),
                self.buffer.height.try_into().unwrap(),
                0,
                gl::RGBA,
                gl::FLOAT,
                self.buffer.data.as_ptr().cast(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            let vertex_shader = get_shader(
                VERT_SHADER,
                gl::VERTEX_SHADER,
            );

            let fragment_shader = get_shader(
                FRAG_SHADER,
                gl::FRAGMENT_SHADER,
            );

            self.shader_program = gl::CreateProgram();
            gl::AttachShader(self.shader_program, vertex_shader);
            gl::AttachShader(self.shader_program, fragment_shader);
            gl::LinkProgram(self.shader_program);
            let mut success = 0;
            gl::GetProgramiv(self.shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(
                    self.shader_program,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
    }
    fn update_texture(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.screen_buffer_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA.try_into().unwrap(),
                self.buffer.width.try_into().unwrap(),
                self.buffer.height.try_into().unwrap(),
                0,
                gl::RGBA,
                gl::FLOAT,
                self.buffer.data.as_ptr().cast(),
            );
        }
    }

    pub fn render(&mut self) {
        self.update_texture();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BindTexture(gl::TEXTURE_2D, self.screen_buffer_texture);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }
    }
}

pub fn get_shader(src: &str, shader_type: gl::types::GLenum) -> u32 {
    let shader_id = unsafe {
        let shader_id = gl::CreateShader(shader_type);
        assert_ne!(shader_id, 0);

        gl::ShaderSource(
            shader_id,
            1,
            &(src.as_bytes().as_ptr().cast()),
            &(src.len().try_into().unwrap()),
        );

        gl::CompileShader(shader_id);

        let mut success = 0;
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                shader_id,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
        }
        shader_id
    };
    return shader_id;
}