extern crate gtk;
use gtk::{
  main_quit, Button, ButtonExt, ContainerExt, Entry, EntryExt, Grid, GridExt, GtkWindowExt,
  Inhibit, InputPurpose, Label, WidgetExt, Window, WindowPosition, WindowType,
};
use std::env;
use std::process;
use std::sync::{Arc, Mutex};

fn main() {
  let (exit_code, password) = prompt();
  println!("{}", password);
  process::exit(exit_code);
}

fn prompt() -> (i32, String) {
  if gtk::init().is_err() {
    eprintln!("failed to initialize GTK Application");
    process::exit(1);
  }

  let args: Vec<String> = env::args().collect();

  let message: &str = if args.len() > 1 {
    &args[1]
  } else {
    "No message given... You should probably not type your password."
  };

  let app = App::new(message);
  app.window.show_all();
  app.content.setup_calbacks(&app.state);
  // Start the GTK main event loop
  gtk::main();

  let state_clone = app.state.clone();
  let state = state_clone.lock().unwrap();
  return (state.exit_code, state.password.clone());
}

pub struct Content {
  pub grid: Grid,
  ok_button: Button,
  cancel_button: Button,
  entry: Entry,
}

pub struct App {
  pub window: Window,
  pub content: Content,
  state: Arc<Mutex<AppState>>,
}

#[derive(Clone)]
pub struct AppState {
  pub exit_code: i32,
  pub password: String,
}

impl AppState {
  fn new() -> AppState {
    AppState {
      exit_code: 1,
      password: "".to_owned(),
    }
  }
}

impl App {
  fn new(message: &str) -> App {
    let state = Arc::new(Mutex::new(AppState::new()));
    let window = Window::new(WindowType::Toplevel);

    window.set_title("Unlock ssh key");
    window.set_wmclass("app-name", "gsshaskpass");
    window.set_position(WindowPosition::Center);
    Window::set_default_icon_name("dialog-password");
    window.connect_delete_event(move |_, _| {
      main_quit();
      Inhibit(false)
    });

    let content = Content::new(message);

    window.add(&content.grid);

    App {
      window,
      content,
      state,
    }
  }
}

impl Content {
  fn new(message: &str) -> Content {
    let label: Label = Label::new(message);
    let cancel_button: Button = Button::new_with_label("Cancel");
    let ok_button = Button::new_with_label("Ok");
    let entry = Entry::new();
    entry.set_visibility(false);
    entry.set_input_purpose(InputPurpose::Password);

    let grid = Grid::new();
    let sub_grid = Grid::new();

    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);
    sub_grid.set_column_homogeneous(true);
    sub_grid.set_row_homogeneous(true);
    grid.set_row_spacing(16);
    sub_grid.set_row_spacing(16);
    sub_grid.set_border_width(16);

    sub_grid.attach(&label, 0, 1, 4, 1);
    sub_grid.attach(&entry, 0, 2, 4, 1);

    grid.attach(&sub_grid, 0, 0, 4, 3);

    grid.attach(&cancel_button, 0, 3, 2, 1);

    grid.attach(&ok_button, 2, 3, 2, 1);

    Content {
      grid,
      ok_button,
      cancel_button,
      entry,
    }
  }

  fn setup_calbacks(&self, app_state: &Arc<Mutex<AppState>>) -> &Content {
    let e = self.entry.clone();
    let b = self.ok_button.clone();
    let state_arc = app_state.clone();
    self.ok_button.connect_clicked(move |_| {
      let mut state = state_arc.lock().unwrap();
      state.exit_code = 0;
      state.password = e.get_text().unwrap();
      main_quit();
    });

    self.entry.connect_activate(move |_| {
      b.emit_clicked();
    });

    self.cancel_button.connect_clicked(|_| {
      main_quit();
    });

    self
  }
}
