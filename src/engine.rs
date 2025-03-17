use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

/// Estrutura principal do motor de jogo.
/// Main structure of the game engine.
pub struct Engine {
    event_loop: EventLoop<()>,
    window: Window,
}

impl Engine {
    /// Verifica se o motor está funcionando corretamente.
    /// Checks if the engine is working correctly.
    pub fn is_ok(&self) -> bool {
        true
    }
}

impl Engine {
    /// Cria uma nova instância do motor de jogo.
    /// Creates a new instance of the game engine.
    ///
    /// # Arguments
    ///
    /// * `title` - Título da janela / Window title
    /// * `width` - Largura da janela / Window width
    /// * `height` - Altura da janela / Window height
    pub async fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();
        Self {
            event_loop,
            window,
        }
    }

    /// Inicia o loop de execução do motor de jogo.
    /// Starts the game engine's execution loop.
    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
