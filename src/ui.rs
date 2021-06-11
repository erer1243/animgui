/*
Each function (other than draw()) represents one imgui window, except for main_menu which draws the
menu bar at the top of the screen. Each widget is denoted by starting with a description of what it
is, a line of ~ and ending with a line of -

TODO:
    * Animation sequencer
    * Animation controls for 1 object
*/
use crate::animation::Frame;
use crate::project::Project;
use crate::Object;
use glium::Display;
use imgui::*;

pub fn draw(
    ui: &mut Ui,
    project: &mut Project,
    state: &mut UIState,
    display: &Display,
    _current_frame: &mut Frame,
) {
    // Special case for main menu. If resetting UI, skip this frame and reset
    if state.reset {
        state.reset_all();
        return;
    }

    main_menu(ui, state);
    meshes_list(ui, state, project);
    objects_list(ui, state, project);
    new_mesh(ui, state, project, display);
    new_object(ui, state, project);
    object_attributes(ui, state, project);
}

fn main_menu(ui: &mut Ui, state: &mut UIState) {
    // Draw menu bar
    ui.main_menu_bar(|| {
        if MenuItem::new(im_str!("Reset UI")).build(ui) {
            state.reset = true;
            return;
        }
    });
}

fn meshes_list(ui: &mut Ui, state: &mut UIState, project: &Project) {
    Window::new(im_str!("Meshes"))
        .position([0., 19.], Condition::Appearing)
        .size([300., 300.], Condition::Appearing)
        .build(ui, || {
            // Load mesh button
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            if ui.button(im_str!("Load mesh"), [80., 20.]) {
                state.loading_new_mesh = true;
            }
            // -------------------------------------------------------------------------------------

            // "Make into object" button
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            if let Some(i) = state.selected_mesh {
                // If the user has selected a mesh, render a button
                if ui.button(im_str!("Make into object"), [130., 20.]) {
                    // Mark mesh as being created into object
                    // This logic is handled in the [New Object] window
                    state.mesh_for_obj = Some(i);
                }
            } else {
                // If the user hasn't yet selected a mesh, render disabled button
                disabled_button(ui, "Make into object (select a mesh first)", [130., 20.]);
            }
            // -------------------------------------------------------------------------------------

            // Mesh selecion list
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            for (i, mesh) in project.meshes.iter().enumerate() {
                ui.radio_button(mesh.name_imstr(), &mut state.selected_mesh, Some(i));
            }
            // -------------------------------------------------------------------------------------
        });
}

fn objects_list(ui: &mut Ui, state: &mut UIState, project: &mut Project) {
    Window::new(im_str!("Objects"))
        .position([0., 319.], Condition::Appearing)
        .size([300., 300.], Condition::Appearing)
        .build(ui, || {
            // Render object selection list
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            for (i, object) in project.objs.iter().enumerate() {
                ui.radio_button(object.name_imstr(), &mut state.selected_object, Some(i));
            }
            // -------------------------------------------------------------------------------------
        });
}

fn new_mesh(ui: &mut Ui, state: &mut UIState, project: &mut Project, display: &Display) {
    if state.loading_new_mesh {
        modal(ui, im_str!("Load Mesh"), || {
            // If there is an error waiting to be rendered, show it. Otherwise, show the path
            // input and buttons
            if let Some(err_msg) = &state.new_mesh_error {
                // Show error message
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                ui.text_wrapped(unsafe { ImStr::from_utf8_with_nul_unchecked(err_msg.as_bytes()) });
                // ---------------------------------------------------------------------------------

                // Ok button to go back to work
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                if ui.button(im_str!("Ok"), [80., 20.]) {
                    state.reset_new_mesh();
                }
            // ---------------------------------------------------------------------------------
            } else {
                // Path input
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                ui.input_text(im_str!("Path"), &mut state.new_mesh_path)
                    .resize_buffer(true)
                    .allow_tab_input(false)
                    .build();
                // ---------------------------------------------------------------------------------

                // Cancel button
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                if ui.button(im_str!("Cancel"), [80., 20.]) {
                    state.reset_new_mesh();
                }
                // ---------------------------------------------------------------------------------

                // Make Ok and Cancel buttons on same line
                ui.same_line(0.);

                // Ok button, clickable if state.new_mesh_path isnt empty
                // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
                if button_if(
                    ui,
                    !state.new_mesh_path.is_empty(),
                    im_str!("Ok"),
                    "Ok (enter a path)",
                    [80., 20.],
                ) {
                    // Try loading mesh from path provided
                    let res = project.load_mesh_from_file(display, state.new_mesh_path.to_str());

                    // If a new mesh was successfully loaded, reset mesh list ui and this modal
                    if res.is_ok() {
                        state.selected_mesh = res.ok();
                        state.reset_new_mesh();
                    } else {
                        // Assign error to new_mesh_error, if it exists
                        state.new_mesh_error = res.err();
                    }
                }
                // ---------------------------------------------------------------------------------
            }
        });
    }
}

fn new_object(ui: &mut Ui, state: &mut UIState, project: &mut Project) {
    // state.mesh_for_obj is set in the [Meshes] window
    if let Some(i) = state.mesh_for_obj {
        modal(ui, im_str!("New Object"), || {
            // Object name input
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            ui.input_text(im_str!("Name"), &mut state.new_obj_name)
                .resize_buffer(true)
                .allow_tab_input(false)
                .build();
            // -------------------------------------------------------------------------------------

            // Cancel button
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            if ui.button(im_str!("Cancel"), [80., 20.]) {
                state.reset_new_object();
            }
            // -------------------------------------------------------------------------------------

            // Make Cancel and Ok buttons on same line
            ui.same_line(0.);

            // Ok button, disabled if input text is empty
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            if button_if(
                ui,
                !state.new_obj_name.is_empty(),
                im_str!("Ok"),
                "Ok (enter a name)",
                [80., 20.],
            ) {
                // Add new object
                let mesh = project.meshes[i].clone();
                let name = state.new_obj_name.clone();
                let object = Object::new(name, mesh);
                project.objs.push(object);

                // Reset state related to a making new object
                state.reset_new_object();
            }
            // -------------------------------------------------------------------------------------
        });
    }
}

fn object_attributes(ui: &mut Ui, state: &mut UIState, project: &mut Project) {
    Window::new(im_str!("Object Attributes"))
        .position([0., 619.], Condition::Appearing)
        .size([300., 300.], Condition::Appearing)
        .build(ui, || {
            // Slider::new(im_str!("Numbers"))
            //     .display_format(im_str!("%.3f"))
            //     .range(-100.0..=100.0)
            //     .build_array(ui, &mut state.ns);
        });
}

#[derive(Default)]
pub struct UIState {
    // UI Reset (set in [main_menu])
    // =============================================================================================
    // Whether to reset right now. If this is true, the ui will skip rendering for a frame to reset
    // the position and size of all windows
    reset: bool,

    // Loading a [new_mesh]
    // =============================================================================================
    // Whether or not currently loading a new mesh
    loading_new_mesh: bool,

    // Path of obj file to load from
    new_mesh_path: ImString,

    // Error message to display if there was a problem loading a mesh in [New Mesh]
    new_mesh_error: Option<String>,

    // [meshes_list] menu
    // =============================================================================================
    // Which mesh is selected in the Meshes dropdown
    selected_mesh: Option<usize>,

    // [objects_list] menu
    // =============================================================================================
    // Which object is selected in the Objects dropdown. Also used in [object_attributes]
    selected_object: Option<usize>,

    // Creating a [new_object]
    // =============================================================================================
    // Which mesh is currently being made into an object
    mesh_for_obj: Option<usize>,

    // Name for new object
    new_obj_name: ImString,
}

impl UIState {
    pub fn new() -> UIState {
        // For any future initialization
        UIState::default()
    }

    fn reset_all(&mut self) {
        *self = UIState::new();
    }

    fn reset_new_mesh(&mut self) {
        self.new_mesh_path.clear();
        self.loading_new_mesh = false;
        self.new_mesh_error = None;
    }

    fn reset_new_object(&mut self) {
        self.mesh_for_obj = None;
        self.new_obj_name.clear();
    }
}

// Draws and shows a modal
fn modal<F: FnOnce()>(ui: &Ui, title: &ImStr, func: F) {
    ui.popup_modal(title).build(func);
    ui.open_popup(title);
}

// Draws a button that is only clickable if a given condition is met
fn button_if(ui: &Ui, cond: bool, button_text: &ImStr, tooltip_text: &str, size: [f32; 2]) -> bool {
    if cond {
        ui.button(button_text, size)
    } else {
        disabled_button(ui, tooltip_text, size);
        false
    }
}

// Draws a hollow box which shows a tooltip when hovered
fn disabled_button(ui: &Ui, tooltip_text: &str, [w, h]: [f32; 2]) {
    // Get required drawing attributes
    let draw_list = ui.get_window_draw_list();
    let [px, py] = ui.clone_style().frame_padding;
    let [cx, cy] = ui.cursor_screen_pos();

    // Draw rectangle
    draw_list
        .add_rect([cx, cy], [w + cx, h + cy], (0.137, 0.286, 0.443))
        .build();

    // Move draw cursor to appropriate position
    // ui.set_cursor_screen_pos([cx, h + cy + py]);
    ui.dummy([w + px, h + py]);

    // Show tooltip on hover
    let [mx, my] = ui.io().mouse_pos;
    if mx < cx + w + px && mx > cx && my < cy + h + py && my > cy {
        ui.tooltip_text(tooltip_text);
    }
}
