#![windows_subsystem = "windows"]

use notify::event::CreateKind;
use notify::EventKind::Create;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{process, thread};
use tray_item::{IconSource, TrayItem};

type Channel<T> = (mpsc::Sender<T>, Arc<Mutex<mpsc::Receiver<T>>>);

type Tray = Option<Arc<Mutex<TrayItem>>>;

struct App {
    path: Box<Path>,
    tray: Tray,
    watcher: Option<RecommendedWatcher>,
    tray_channel: Channel<&'static str>,
    watcher_channel: Channel<notify::Result<Event>>,
}

impl App {
    pub fn new(path: Box<Path>) -> Self {
        let tray_channel = mpsc::channel();
        let watcher_channel = mpsc::channel();

        Self {
            path,
            tray: None,
            watcher: None,
            tray_channel: (tray_channel.0, Arc::new(Mutex::new(tray_channel.1))),
            watcher_channel: (watcher_channel.0, Arc::new(Mutex::new(watcher_channel.1))),
        }
    }

    pub fn run(&mut self) -> notify::Result<()> {
        self.initialize_tray()?;
        self.initialize_watcher()?;
        self.listen_events();
        Ok(())
    }

    fn initialize_tray(&mut self) -> notify::Result<()> {
        let mut tray = TrayItem::new("Invoices", IconSource::Resource("icon-green")).unwrap();

        let (tx, _) = &self.tray_channel;

        // Add a label to the tray item
        tray.add_label("Invoices").unwrap();

        // Add a separator to the tray label
        tray.inner_mut().add_separator().unwrap();

        let tx_tap = tx.clone();
        let cb = move || tx_tap.send("tap").unwrap();
        tray.add_menu_item("Tap", cb).unwrap();

        let tx_quit = tx.clone();
        let cb = move || tx_quit.send("quit").unwrap();
        tray.add_menu_item("Quit", cb).unwrap();

        self.tray = Some(Arc::new(Mutex::new(tray)));

        Ok(())
    }

    fn initialize_watcher(&mut self) -> notify::Result<()> {
        let sender = &self.watcher_channel.0;

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(sender.clone(), Config::default())?;

        let path = self.path.as_ref().to_owned();
        watcher.watch(&path, RecursiveMode::Recursive).unwrap();

        self.watcher = Some(watcher);

        Ok(())
    }

    fn listen_events(&mut self) {
        println!("Listening events");
        let tray_channel = self.tray_channel.1.clone();
        let watcher_channel = self.watcher_channel.1.clone();
        let tray = self.tray.clone().unwrap();

        // Tray thread
        let tray_handle = thread::spawn(move || {
            let tray_channel = tray_channel.lock().unwrap();
            // Obtener el canal del Arc y bloquearlo

            loop {
                let event = tray_channel.recv().unwrap();
                println!("tray event: {:?}", event);

                if event == "quit" {
                    process::exit(0);
                }
            }
        });

        // Watcher thread
        let watcher_handle = thread::spawn(move || {
            let mut tray = tray.lock().unwrap();
            let watcher_channel = watcher_channel.lock().unwrap();

            loop {
                let event = watcher_channel.recv().unwrap();

                let res = event.unwrap();

                match res.kind {
                    Create(CreateKind::Any) => {
                        tray.set_icon(IconSource::Resource("icon-red"))
                            .unwrap_or_else(|err| {
                                eprintln!("Error al cambiar el icono: {:?}", err);
                            });

                        let path = match res.paths.first() {
                            Some(path) => path,
                            None => {
                                eprintln!("No se encontró ningún camino en el evento");
                                continue; // Continuar con la próxima iteración del bucle
                            }
                        };

                        let file_attrs = match std::fs::metadata(path) {
                            Ok(attrs) => attrs,
                            Err(err) => {
                                eprintln!("Error al obtener los metadatos del archivo: {:?}", err);
                                continue; // Continuar con la próxima iteración del bucle
                            }
                        };

                        if file_attrs.is_dir() {
                            continue;
                        }

                        // rename file to format YYYY-MM using current date
                        thread::sleep(Duration::from_secs(5));

                        println!("File created: {:?}", file_attrs);

                        let date = chrono::Local::now().format("%Y-%m");
                        println!("current date is {}", date);

                        let ext = path.extension().unwrap().to_str().unwrap();

                        let new_name = date.to_string() + "." + ext;

                        let new_path = path.parent().unwrap().join(new_name);

                        println!("El nuevo nombre es {:?}", new_path);

                        match std::fs::rename(path, new_path) {
                            Ok(_) => {}
                            Err(err) => eprintln!("Error al renombrar el archivo: {:?}", err),
                        }

                        tray.set_icon(IconSource::Resource("icon-green"))
                            .expect("Error al cambiar el icono");
                    }
                    _ => {
                        // println!("No event match {:#?}", res.kind);
                    }
                }
            }
        });

        tray_handle.join().unwrap_or_else(|err| {
            println!("Error tray_handle: {:?}", err);
        });

        watcher_handle.join().unwrap_or_else(|err| {
            println!("Error watcher_handle: {:?}", err);
        });

        println!("Listening events ended")
    }
}

fn main() -> notify::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let path = Path::new("C:\\Users\\styve\\My Drive\\1 - Finanzas");

    log::info!("Watching {path}", path = path.display());

    let mut app = App::new(Box::from(path));

    app.run().unwrap();

    Ok(())
}
