pub trait WindowApp {
    fn update(&mut self);
    fn draw(&self);
    fn on_window_event(&mut self, event: &sdl2::event::Event);
}

pub struct WindowAppRunner{
    pub gl: gl::Gl,
    pub sdl: sdl2::Sdl,
    pub window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    pub v: u32,
    //on_draw: Option<dyn Fn(&Self) + 'static>
    pub window_app: Box<(dyn WindowApp)>
}

impl WindowAppRunner {
    pub fn new<F>(init: F) -> WindowAppRunner
    where F : FnOnce(&gl::Gl) -> Box<dyn WindowApp>  {
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
    
        let window_app = init(&gl);
        
        Self {
            gl,
            sdl,
            window,
            _gl_context,
            v: 4,
            window_app
        }
    }
    pub fn draw(&self) {
        self.window_app.draw();
            
    }



    pub fn run_loop(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        'main: loop {


            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    _ => self.window_app.on_window_event(&event)
            
                }
            }

            self.window_app.update();
            self.window_app.draw();



            self.window.gl_swap_window();

        }
    }
    
}
