/*
This file contains the KeyframeV3 struct and its related functionality. This struct represents
a nalgebra_glm::Vec3 value that has specific values at specific frames and that can calculate the
interpolated value it would have at a given frame via tweening.

TODO:
    * More than 1 kind of tweening
    * More tests
*/
use nalgebra_glm::Vec3;
use std::{
    cell::RefCell,
    collections::{btree_map::Keys, BTreeMap},
};

// Type alias for clarity
pub type Frame = usize;

// Enum that represents one Vec3 that can be keyframed and tweened.
// The two cases for KeyframeV3. Either it has one value for its whole existence, or it has
// keyframed values. The reason it is split, rather than just have a tree with 1 element, is because
// there is no sensible default 'Frame' value (key to the map) for the initial value.
pub enum KeyframeV3 {
    Single(Vec3),
    Multiple {
        tree: BTreeMap<Frame, Vec3>,
        cache: RefCell<Option<(Frame, Vec3)>>,
    },
}

impl KeyframeV3 {
    pub fn new(init: Vec3) -> KeyframeV3 {
        KeyframeV3::Single(init)
    }

    // Returns the value that this would have at a given frame. Depending on data held, it may or
    // may not require any tweening calculation
    pub fn at(&self, frame: Frame) -> Vec3 {
        match &self {
            // There is only ever one value for the float, return it
            KeyframeV3::Single(v) => *v,

            // There are keyframed values, and the value at this frame may need to be calculated
            KeyframeV3::Multiple { tree, cache } => {
                let cache_ref = cache.borrow();

                match *cache_ref {
                    // There is a cached value for this frame. Return it
                    Some((cframe, cvalue)) if frame == cframe => cvalue,

                    // There is not a cached value for this frame. Calculate a new one
                    _ => {
                        // Calculate new value from tree
                        let value = if let Some(v) = tree.get(&frame) {
                            // If there is an exact value for this frame in the tree, return it
                            *v
                        } else {
                            // There is no exact value for this frame, so it must be calculated
                            let prev_val = tree.range(..frame).next_back();
                            let next_val = tree.range(frame..).next();

                            match (prev_val, next_val) {
                                // If between two keyframes, calculate the value with tweening
                                (Some((pf, pv)), Some((nf, nv))) => {
                                    let frame = frame as f32;
                                    let pf = *pf as f32;
                                    let nf = *nf as f32;
                                    let x = linear_tween((pf, pv.x), (nf, nv.x), frame);
                                    let y = linear_tween((pf, pv.y), (nf, nv.y), frame);
                                    let z = linear_tween((pf, pv.z), (nf, nv.z), frame);

                                    Vec3::new(x, y, z)
                                }

                                // If behind or in front of just one keyframe, return its value
                                (Some((_pf, pv)), None) => *pv,
                                (None, Some((_nf, nv))) => *nv,

                                // Empty tree. Currently this is unreachable
                                (None, None) => unreachable!(),
                            }
                        };

                        // Store value in cache
                        drop(cache_ref);
                        *cache.borrow_mut() = Some((frame, value));

                        // Return value
                        value
                    }
                }
            }
        }
    }

    pub fn frames(&self) -> Option<Keys<Frame, Vec3>> {
        match self {
            KeyframeV3::Single(_) => None,
            KeyframeV3::Multiple { tree, .. } => Some(tree.keys()),
        }
    }

    pub fn set_at(&mut self, frame: Frame, val: Vec3) {
        match self {
            KeyframeV3::Single(_) => {
                // Turn self.0 into a btreemap with the given value. This throws out the old
                // initial value
                let mut tree = BTreeMap::new();
                tree.insert(frame, val.clone());
                *self = KeyframeV3::Multiple {
                    tree,
                    cache: RefCell::new(None),
                };
            }

            KeyframeV3::Multiple { tree, cache } => {
                // Insert given value
                tree.insert(frame, val);

                // Reset cache
                *cache.borrow_mut() = None
            }
        }
    }
}

// Tweening functions
// =================================================================================================
fn linear_tween((x1, y1): (f32, f32), (x2, y2): (f32, f32), x: f32) -> f32 {
    // Uses point slope formula:
    // y-y1 = m(x-x1)
    // y = m(x-x1)+y1
    // where m = (y2-y1)/(x2-x1)
    let m = (y2 - y1) / (x2 - x1);
    m * (x - x1) + y1
}

// Tweening tests
// =================================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Does a 'close enough' approximation of two f32 values
    fn feq(n1: f32, n2: f32) -> bool {
        n1.max(n2) - n1.min(n2) < 0.00005
    }

    // Does a 'close enoguh' approximation of two Vec3 values
    fn veq(v1: Vec3, v2: Vec3) -> bool {
        feq(v1.x, v2.x) && feq(v1.y, v2.y) && feq(v1.z, v2.z)
    }

    #[test]
    fn linear_tween_test() {
        assert!(feq(linear_tween((0.0, 0.0), (5.0, 5.0), 2.0), 2.0));
        assert!(feq(linear_tween((1.0, 8.0), (2.0, 13.0), -2.0), -7.0));
    }

    #[test]
    fn keyframev3_frames_test() {
        let mut kf = KeyframeV3::new(Vec3::zeros());
        let mut frames: Vec<Frame> = Vec::new();

        // Should have no frames
        assert!(kf.frames().is_none());

        // Insert 1 value
        kf.set_at(10, Vec3::zeros());

        // Should have 1 frame
        frames.extend(kf.frames().unwrap());
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0], 10);

        // Insert 1 more value
        kf.set_at(20, Vec3::zeros());

        // Should have 2 frames
        frames.clear();
        frames.extend(kf.frames().unwrap());
        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0], 10);
        assert_eq!(frames[1], 20);
    }

    #[test]
    fn keyframev3_at_test() {
        let mut kf = KeyframeV3::new(Vec3::zeros());

        assert!(veq(kf.at(00), Vec3::zeros()));
        assert!(veq(kf.at(10), Vec3::zeros()));
        assert!(veq(kf.at(20), Vec3::zeros()));

        kf.set_at(10, Vec3::new(1.0, 2.0, 3.0));

        assert!(veq(kf.at(00), Vec3::new(1.0, 2.0, 3.0)));
        assert!(veq(kf.at(10), Vec3::new(1.0, 2.0, 3.0)));
        assert!(veq(kf.at(20), Vec3::new(1.0, 2.0, 3.0)));

        kf.set_at(10, Vec3::new(4.0, 5.0, 6.0));

        assert!(veq(kf.at(00), Vec3::new(4.0, 5.0, 6.0)));
        assert!(veq(kf.at(10), Vec3::new(4.0, 5.0, 6.0)));
        assert!(veq(kf.at(20), Vec3::new(4.0, 5.0, 6.0)));

        kf.set_at(20, Vec3::new(5.0, 6.0, 7.0));

        assert!(veq(kf.at(05), Vec3::new(4.0, 5.0, 6.0)));
        assert!(veq(kf.at(10), Vec3::new(4.0, 5.0, 6.0)));
        assert!(veq(kf.at(15), Vec3::new(4.5, 5.5, 6.5)));
        assert!(veq(kf.at(20), Vec3::new(5.0, 6.0, 7.0)));
        assert!(veq(kf.at(25), Vec3::new(5.0, 6.0, 7.0)));

        kf.set_at(30, Vec3::new(8.0, 3.0, 0.0));

        assert!(veq(kf.at(05), Vec3::new(4.0, 5.0, 6.0)));
        assert!(veq(kf.at(10), Vec3::new(4.0, 5.0, 6.0)));
        assert!(veq(kf.at(15), Vec3::new(4.5, 5.5, 6.5)));
        assert!(veq(kf.at(20), Vec3::new(5.0, 6.0, 7.0)));
        assert!(veq(kf.at(25), Vec3::new(6.5, 4.5, 3.5)));
        assert!(veq(kf.at(30), Vec3::new(8.0, 3.0, 0.0)));
        assert!(veq(kf.at(35), Vec3::new(8.0, 3.0, 0.0)));

        kf.set_at(20, Vec3::new(2.0, 3.0, 4.0));

        assert!(veq(kf.at(20), Vec3::new(2.0, 3.0, 4.0)));
    }
}
