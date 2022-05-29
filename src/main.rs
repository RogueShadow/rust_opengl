extern crate glfw;
extern crate gl;

use std::mem::size_of_val;
use std::mem::size_of;
use glfw::*;
use gl::*;

fn main() {
    // Setting up the window.
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(
        800,
        800,
        "Hello this is a window.",
        WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    // Setting up the opengl.
    let mut vao = 0;
    let mut vbo = 0;
    let mut shader_program = 0;
    unsafe {
        load_with(|s| window.get_proc_address(s));

        ClearColor(0.2, 0.3, 0.3, 1.0);

       //let mut vao = 0;
        GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        BindVertexArray(vao);

        //let mut vbo = 0;
        GenBuffers(1, &mut vbo);
        assert_ne!(0, vbo);
        BindBuffer(ARRAY_BUFFER, vbo);

        type Vertex = [f32; 3];
        const VERTICES: [Vertex; 3] =
        [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0],[0.0, 0.5, 0.0]];

        BufferData(
            ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            STATIC_DRAW,
        );

        VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        EnableVertexAttribArray(0);

        let vertex_shader = CreateShader(VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        const  VERT_SHADER: &str = r#"#version 330 core
                                      layout (location = 0) in vec3 pos;
                                      void main() {
                                        gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
                                      }
                                      "#;
        ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        CompileShader(vertex_shader);
        let mut success = 0;
        GetShaderiv(vertex_shader, COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            GetShaderInfoLog(
                vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }

        let fragment_shader = CreateShader(FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        const FRAG_SHADER: &str = r#"#version 330 core
                                     out vec4 final_color;

                                     void main() {
                                        final_color = vec4(1.0, 0.5, 0.2, 1.0);
                                     }
                                     "#;
        ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        CompileShader(fragment_shader);
        let mut success = 0;
        GetShaderiv(fragment_shader, COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            GetShaderInfoLog(
                fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        shader_program = CreateProgram();
        AttachShader(shader_program, vertex_shader);
        AttachShader(shader_program, fragment_shader);
        LinkProgram(shader_program);
        let mut success = 0;
        GetProgramiv(shader_program, LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        DeleteShader(vertex_shader);
        DeleteShader(fragment_shader);


    }

    // The looping.
    while !window.should_close() {

        // Handle window events.
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
        // Handle, the things.



        // Handle the drawing.
        unsafe {
            Clear(COLOR_BUFFER_BIT);
            UseProgram(shader_program);
            BindVertexArray(vao);
            DrawArrays(TRIANGLES, 0, 3);
        }
        window.swap_buffers();
    }

    // The ending.
}
