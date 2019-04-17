use glutin::{
    EventsLoop,
    WindowedContext,
    ContextTrait,
};

pub struct Window {
    events_loop: Option<EventsLoop>,
    win_context: WindowedContext,
    is_running: bool,

    size: (f32, f32),
}

impl Window {
    pub fn new(title: &str, width: f64, height: f64) -> Window {
        let el = EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));
        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(wb, &el)
            .expect("Failed to build windowed context.");

        unsafe {
            windowed_context.make_current().unwrap();
            gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
        }

        Window {
            events_loop: Some(el),
            win_context: windowed_context,

            is_running: true,
            size: (width as _, height as _),
        }
    }

    pub fn running(&self) -> bool {
        self.is_running
    }

    pub fn handle_events(&mut self) {
        if let Some(mut events_loop) = self.events_loop.take() {
            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent { event, .. } => {
                        self.handle_window_event(event);
                    },

                    _ => { /* NOP */ }
                }
            });
            self.events_loop = Some(events_loop);
        } else {
            panic!("Attempted to run tick with no active events loop.");
        }
    }

    fn handle_window_event(&mut self, event: glutin::WindowEvent) {
        match event {
            glutin::WindowEvent::CloseRequested => self.is_running = false,
            glutin::WindowEvent::Resized(logical_size) => {
                let dpi_factor = self.win_context.get_hidpi_factor();
                let physical_size = logical_size.to_physical(dpi_factor);
                self.win_context.resize(physical_size);
                self.size = (physical_size.width as _, physical_size.height as _);
                unsafe {
                    gl::Viewport(0, 0, physical_size.width as _, physical_size.height as _);
                }
            },
            _ => { /* NOP */ }
        }
    }

    pub fn flip(&self) {
        self.win_context.swap_buffers().unwrap();
    }

    pub fn width(&self) -> f32 {
        self.size.0
    }

    pub fn height(&self) -> f32 {
        self.size.1
    }
}
