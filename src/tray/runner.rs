use crate::tray::menu::TrayMenu;
use crate::tray::tray::Tray;
use crate::tray::tray_ref::TrayRef;
use crate::tray::types::TrayEvent;
use std::sync::{Arc, Mutex};
use tray_icon::menu::MenuEvent;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy};

#[derive(Debug, Clone)]
pub enum UserEvent {
    MenuEvent(MenuEvent),
}

pub struct TrayRunner {
    tray: Arc<Mutex<Tray>>,
    menu: Arc<Mutex<TrayMenu>>,
    running: Arc<Mutex<bool>>,
}

impl TrayRunner {
    pub fn new(tray: Tray, menu: TrayMenu) -> Self {
        Self {
            tray: Arc::new(Mutex::new(tray)),
            menu: Arc::new(Mutex::new(menu)),
            running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn run(&self, callback: impl Fn(TrayEvent, &mut TrayRef) + Send + 'static + Sync) {
        let event_loop = self.create_event_loop();
        let mut tray_ref = self.get_ref();

        event_loop
            .run(move |event, ael| {
                ael.set_control_flow(ControlFlow::Wait);

                match event {
                    Event::UserEvent(UserEvent::MenuEvent(e)) => {
                        log::debug!("Menu event: {:?}", e);
                        let tray_event = TrayEvent::from(e.id.as_ref());
                        Self::event_listener(tray_event.clone(), &mut tray_ref);
                        callback(tray_event.clone(), &mut tray_ref);
                    }
                    _ => {}
                }
            })
            .unwrap();
    }

    fn set_event_handler(&self, proxy: EventLoopProxy<UserEvent>) {
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            proxy.send_event(UserEvent::MenuEvent(event)).unwrap();
        }));
    }

    fn create_event_loop(&self) -> EventLoop<UserEvent> {
        let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
        self.set_event_handler(event_loop.create_proxy());
        event_loop
    }

    fn get_ref(&self) -> TrayRef {
        TrayRef::new(self.running.clone(), self.tray.clone(), self.menu.clone())
    }

    fn event_listener(event: TrayEvent, tray_ref: &mut TrayRef) {
        match event {
            TrayEvent::Running => tray_ref.update_menu(),
            _ => {}
        }
    }
}
