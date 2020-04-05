#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate gl;
extern crate sdl2;
extern crate nalgebra_glm as glm;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

enum TARGET {
    MODEL,
    CAMERA
}

enum PARAM {
    X,
    Y
}

struct Msg {
    target: TARGET,
    param: PARAM,
    value: f32
}

type SendSyncSender = std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Sender<Msg>>>;


fn target_to_enum(target_name: &rocket::http::RawStr)->Result<TARGET, ()> {
    match target_name.as_str() {
        "camera" => Ok(TARGET::CAMERA),
        "model" => Ok(TARGET::MODEL),
        _ => Err(())
    }
}

fn param_to_enum(param_name: &rocket::http::RawStr)->Result<PARAM, ()> {
    match param_name.as_str() {
        "x" => Ok(PARAM::X),
        "y" => Ok(PARAM::Y),
        _ => Err(())
    }
}

#[get("/<some_target>/<some_param>/<some_value>")]
fn index(some_target: &rocket::http::RawStr, some_param: &rocket::http::RawStr, some_value: f32, sender: rocket::State<SendSyncSender>) -> &'static str {

    match (target_to_enum(some_target) , param_to_enum(some_param)) {
        (Ok(some_target), Ok(some_param)) => {
            if !sender.lock().unwrap().send(Msg {
                target : some_target,
                param : some_param,
                value : some_value
            }).is_ok() {
                println!("sending the message failed" )
            }
        }
        _ => println!("some cast fail" )
    }
    "Hello, world!"
}



pub mod shader;
pub mod buffer;
pub mod camera;
//pub mod resources;





fn main() {
    let (sender, receiver) = channel::<Msg>();

    // First thread owns sender
    // thread::spawn(move || {
    //     sender.send(1).unwrap();
    // });
    let thread_safe_sender = Arc::new(Mutex::new(sender));

    thread::spawn(|| {
        rocket::ignite().manage(thread_safe_sender).mount("/", routes![index]).launch();
    });
    //let res = receiver.recv().unwrap();
    //receiver.try_iter()
    //println!("{}", res);

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    let mut camera = camera::Camera::from_position_and_look_at(&glm::vec3(-6.0,0.0, 5.0), &glm::vec3(0., 0., 0.));

    unsafe {
 
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    use std::ffi::CString;
    let vert_shader =
        shader::Shader::from_vert_source(&gl ,&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        shader::Shader::from_frag_source(&gl, &CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let mut shader_program = shader::Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

    shader_program.set_used();
    shader_program.u_vp_value = camera.get_view_projection();


    //positions
    let positions: Vec<f32> = vec![
        -0.5, -0.5, 0.0, 
        0.5, -0.5, 0.0, 
        0.0, 0.5, 0.0
    ];
    let vbo_pos = buffer::Vbo::from_vector(&gl, &positions);
    
    //colors
    let colors: Vec<f32> = vec![
        0.0, 1.0, 0.0, 
        1.0, 0.0, 0.0, 
        0.0, 0.0, 1.0
    ];
    let vbo_col = buffer::Vbo::from_vector(&gl, &colors);
        
    let vao = buffer::Vao::new(&gl);
    vao.attach_vbo(&vbo_pos, 0);
    vao.attach_vbo(&vbo_col, 1);




    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in receiver.try_iter() {
            match event.target {
                TARGET::CAMERA => {
                    match event.param {
                        PARAM::X => camera.set_position_x(event.value),
                        PARAM::Y => camera.set_position_y(event.value)
                    };
                    shader_program.u_vp_value = camera.get_view_projection();
                }
                TARGET::MODEL => {
                        shader_program.set_offset(event.value);
                    
                }
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseMotion {x, .. } => {
                    let v = x as f32;
                    shader_program.set_offset(v);

                       
                },
                _ => {}
            }
        }

        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
        }
        shader_program.set_used();
        unsafe {
            vao.bind();
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}