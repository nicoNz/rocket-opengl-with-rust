pub trait WindowApp {
    fn draw(&self);
}

pub struct WindowAppRunner<'a>{
    pub gl: gl::Gl,
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    pub v: u32,
    //on_draw: Option<dyn Fn(&Self) + 'static>
    pub window_app: Option<&'a (dyn WindowApp)>
}

impl<'a> WindowAppRunner<'a> {
    pub fn new(window_app: &'a dyn WindowApp) -> WindowAppRunner<'a> {
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
    
        unsafe {
            gl.Viewport(0, 0, 900, 700);
            gl.ClearColor(0.3, 0.3, 0.5, 1.0);
        }
    
        
        Self {
            gl,
            sdl,
            window,
            _gl_context,
            v: 4,
            window_app : Some(window_app)
        }
    }
    pub fn draw(&self) {
        match self.window_app {
            Some( window_app ) => {
                window_app.draw();
            },
            None => {
                println!("err");
            }
        }
    }
}
