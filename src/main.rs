use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use notify::{Event, RecursiveMode, Watcher};
use notify::event::CreateKind;
use notify::EventKind::Create;
use tray_item::{IconSource, TrayItem};
use winapi::um::wincon::{ATTACH_PARENT_PROCESS, AttachConsole, FreeConsole};

enum Message {
    Quit,
    Green,
    Red,
}

fn daemon() {
    // Instantiate a tray item with a title and an icon
    let mut tray = TrayItem::new(
        "Invoices",
        IconSource::Resource("name-of-icon-in-rc-file"),
    )
        .unwrap();

    // Add a label to the tray item
    tray.add_label("Invoices").unwrap();

    // Add a separator to the tray label
    tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);


    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
        .unwrap();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        match res {
            Ok(res) => {
                match res.kind {
                    Create(CreateKind::Any) => {
                        let path = res.paths.first().unwrap();


                        let file_attrs = std::fs::metadata(path).unwrap();

                        if file_attrs.is_dir() {
                            return;
                        }

                        // rename file to format YYYY-MM using current date

                        thread::sleep(Duration::from_secs(1));

                        println!("file created: {:?}", file_attrs);

                        let date = chrono::Local::now().format("%Y-%m");
                        println!("current date is {}", date);

                        let ext = path.extension().unwrap().to_str().unwrap();

                        let new_name = date.to_string() + "." + ext;

                        let new_path = path.parent().unwrap().join(new_name);

                        println!("new name is {:?}", new_path);

                        std::fs::rename(path, new_path).unwrap();
                    }
                    _ => {}
                }
            }
            Err(e) => {},
        }
    }).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("C:\\Users\\styve\\My Drive\\1 - Finanzas"), RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            _ => {}
        }
    }
}

fn main() {
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }

    // It hides the console window
    unsafe {
        FreeConsole();
    }

    daemon();
}
