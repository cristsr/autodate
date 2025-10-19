use crate::tray::events::TrayEvent;
use crate::tray::menu::TrayMenu;
use crate::tray::tray::Tray;
use crate::tray::tray_ref::TrayRef;
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
}

impl TrayRunner {
    pub fn new(tray: Tray, menu: TrayMenu) -> Self {
        Self {
            tray: Arc::new(Mutex::new(tray)),
            menu: Arc::new(Mutex::new(menu)),
        }
    }

    pub fn run(&self, callback: impl Fn(TrayEvent, &mut TrayRef) + Send + 'static + Sync) {
        let event_loop = self.create_event_loop();
        let mut tray_ref = self.get_ref();

        event_loop
            .run(move |event, ael| {
                ael.set_control_flow(ControlFlow::Wait);

                if let Event::UserEvent(UserEvent::MenuEvent(e)) = event {
                    let tray_event = TrayEvent::from(e.id.as_ref());

                    match tray_event {
                        TrayEvent::Title => {}
                        TrayEvent::Running => tray_ref.update_menu(),
                        TrayEvent::Exit => ael.exit(),
                    }

                    callback(tray_event.clone(), &mut tray_ref);
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
        TrayRef::new(self.tray.clone(), self.menu.clone())
    }
}
