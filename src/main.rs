use std::ffi::CString;
use std::num::NonZeroU32;

// Winit imports
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

// Glutin imports
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext,
    Version,
};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};

// Raw window handle import
use raw_window_handle::HasWindowHandle;

struct App {
    window: Option<Window>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_surface: Option<Surface<WindowSurface>>,
}

impl App {
    fn new() -> Self {
        Self {
            window: None,
            gl_context: None,
            gl_surface: None,
        }
    }
}

impl ApplicationHandler for App {
    // 1. "resumed" is the new entry point where you create windows.
    // It runs when the OS says your app is ready to display content.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = WindowAttributes::default()
            .with_title("Glutin + Winit 0.30 Hello World");

        let template = ConfigTemplateBuilder::new();
        
        // DisplayBuilder helps create the window and GL config simultaneously
        let display_builder = DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                // Find a config that supports the window
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        // Unwrap the window (it might be None if the builder failed to create it immediately)
        let window = window.expect("failed to create window");
        
        // Create Raw Window Handle wrapper for Glutin
        let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

        // Create the GL Context
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(raw_window_handle);

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")
        };

        // Create the Surface
        let attrs = window
            .build_surface_attributes(SurfaceAttributesBuilder::<WindowSurface>::new());
        
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs.unwrap())
                .expect("failed to create surface")
        };

        // Make context current
        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("failed to make context current");

        // Load OpenGL function pointers (using the 'gl' crate)
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            gl_config.display().get_proc_address(symbol.as_c_str()) as *const _
        });

        // Store everything in our App struct
        self.window = Some(window);
        self.gl_context = Some(gl_context);
        self.gl_surface = Some(gl_surface);
    }

    // 2. Handle window events (Resize, Redraw, Close)
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                if let (Some(gl_context), Some(gl_surface), Some(window)) =
                    (&self.gl_context, &self.gl_surface, &self.window)
                {
                    if size.width != 0 && size.height != 0 {
                        // Some platforms require resizing the surface explicitly
                        gl_surface.resize(
                            gl_context,
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let (Some(gl_context), Some(gl_surface), Some(window)) =
                    (&self.gl_context, &self.gl_surface, &self.window)
                {
                    // --- RENDER CODE GOES HERE ---
                    unsafe {
                        gl::ClearColor(0.1, 0.2, 0.3, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }
                    // -----------------------------

                    gl_surface.swap_buffers(gl_context).unwrap();
                    window.request_redraw(); // Request next frame for continuous rendering
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    
    // ControlFlow::Poll is often used for games/animations (continuous rendering)
    // ControlFlow::Wait is for desktop apps (wait for input)
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    
    // The new entry point
    event_loop.run_app(&mut app).unwrap();
}