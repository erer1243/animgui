/*
This file contains the CameraControls struct that keeps track of mouse movement and relevant
keyboard state while the right mouse button is being held.
This is used in the main loop to adjust the angle and position of the camera.

TODO:
*/

#[derive(Default)]
pub struct CameraControls {
    // Whether RMB is being held at the moment
    right_held: bool,

    // The current position of the mouse. Will be None before getting mouse input, and will be
    // reset to None after losing mouse input. That means the first mouse move event is always
    // skipped
    pos: Option<(f64, f64)>,

    // Whether keys are being held at the moment
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    space: bool,
    shift: bool,
}

impl CameraControls {
    pub fn grab(&mut self) {
        self.right_held = true;
    }

    pub fn release(&mut self) {
        self.pos = None;
        self.right_held = false;
        self.w = false;
        self.a = false;
        self.s = false;
        self.d = false;
        self.space = false;
        self.shift = false;
    }

    pub fn rmb_held(&self) -> bool {
        self.right_held
    }

    pub fn mouse_moved(&mut self, xpos: f64, ypos: f64) -> (f32, f32) {
        match self.pos {
            Some((prev_x, prev_y)) => {
                // Calculate relative mouse move
                let dx = xpos - prev_x;
                let dy = ypos - prev_y;

                // Store new mouse pos
                self.pos = Some((xpos, ypos));

                // Return relative mouse move
                (dx as f32, dy as f32)
            }

            None => {
                // Skip first mouse move and store pos
                self.pos = Some((xpos, ypos));
                (0., 0.)
            }
        }
    }

    // Normalized right movement for this frame
    pub fn right_movement(&self) -> f32 {
        if self.d {
            1.
        } else if self.a {
            -1.
        } else {
            0.
        }
    }

    // Normalized left movement for this frame
    pub fn forward_movement(&self) -> f32 {
        if self.w {
            1.
        } else if self.s {
            -1.
        } else {
            0.
        }
    }

    // Normalized up movement for this frame
    pub fn up_movement(&self) -> f32 {
        if self.space {
            1.
        } else if self.shift {
            -1.
        } else {
            0.
        }
    }

    // Functions to keep track of press and release of buttons
    pub fn w_input(&mut self, val: bool) {
        self.w = val;
    }

    pub fn a_input(&mut self, val: bool) {
        self.a = val;
    }

    pub fn s_input(&mut self, val: bool) {
        self.s = val;
    }

    pub fn d_input(&mut self, val: bool) {
        self.d = val;
    }

    pub fn space_input(&mut self, val: bool) {
        self.space = val;
    }

    pub fn shift_input(&mut self, val: bool) {
        self.shift = val;
    }
}
