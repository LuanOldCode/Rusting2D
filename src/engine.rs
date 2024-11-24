use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::renderer::Renderer;
use crate::entities::EntityManager;

pub struct Engine {
    renderer: Renderer,
    entity_manager: EntityManager,
    event_loop: EventLoop<()>,
    window: Window,
}

impl Engine {
    pub fn is_ok(&self) -> bool {
        true
    }
}

impl Engine {
    pub async fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        let renderer = Renderer::new(&window).await;
        let entity_manager = EntityManager::new();

        Self {
            renderer,
            entity_manager,
            event_loop,
            window,
        }
    }

    pub fn add_entity(&mut self, entity: crate::entities::Entity) {
        self.entity_manager.add_entity(entity);
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    self.renderer.render();
                }
                _ => {}
            }
        });
    }
}
