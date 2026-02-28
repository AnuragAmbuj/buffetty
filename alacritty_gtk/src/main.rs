use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, GLArea, Label, Notebook};
use std::cell::RefCell;
use std::rc::Rc;

use alacritty::config::debug::RendererPreference;
use alacritty::display::color::Rgb;
use alacritty::renderer::Renderer;

mod context;
use context::GtkContext;

struct TabState {
    renderer: Option<Renderer>,
}

fn main() {
    env_logger::init();
    let app = Application::builder().application_id("org.alacritty.gtk").build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Alacritty GTK")
        .default_width(800)
        .default_height(600)
        .build();

    // Set transparency if possible (GTK4 handles this with CSS mostly, but visual matters)
    // Note: GTK4 doesn't have set_visual like GTK3. CSS is key.

    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(
        "
        window { background-color: rgba(0, 0, 0, 0.8); }
        notebook header { background-color: rgba(0, 0, 0, 0.5); }
    ",
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &css_provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let notebook =
        Notebook::builder().tab_pos(gtk4::PositionType::Top).hexpand(true).vexpand(true).build();

    // Create a tab with GLArea
    let gl_area = GLArea::new();
    gl_area.set_has_depth_buffer(false);
    gl_area.set_has_stencil_buffer(false);
    gl_area.set_hexpand(true);
    gl_area.set_vexpand(true);

    // State to hold renderer
    let state = Rc::new(RefCell::new(TabState { renderer: None }));

    gl_area.connect_realize(|area| {
        area.make_current();
        if let Some(err) = area.error() {
            log::error!("GLArea error: {}", err);
        }
    });

    gl_area.connect_render(move |area, _context| {
        area.make_current();

        // Wrap GDK context
        let gdk_context = area.context().expect("No GL context");
        let gtk_context = GtkContext::new(gdk_context);

        let mut state = state.borrow_mut();

        if state.renderer.is_none() {
            // Initialize renderer
            log::info!("Initializing Renderer");
            match Renderer::new(&gtk_context, None) {
                Ok(renderer) => {
                    state.renderer = Some(renderer);
                },
                Err(e) => {
                    log::error!("Failed to create renderer: {}", e);
                    return gtk4::glib::Propagation::Stop;
                },
            }
        }

        if let Some(renderer) = &state.renderer {
            // Just clear screen for now to prove it works
            renderer.clear(Rgb::new(0, 255, 0), 1.0); // Green screen test
        }

        gtk4::glib::Propagation::Proceed
    });

    let tab_label = Label::new(Some("Term 1"));
    notebook.append_page(&gl_area, Some(&tab_label));

    // Add a second placeholder tab
    let label2 = Label::new(Some("Terminal 2 Placeholder"));
    let tab_label2 = Label::new(Some("Tab 2"));
    notebook.append_page(&label2, Some(&tab_label2));

    window.set_child(Some(&notebook));
    window.present();
}
