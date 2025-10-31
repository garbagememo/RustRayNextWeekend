use crate::raymod::*;

use std::cmp::Ordering;
use std::f64;

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Shape>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut Shape: Vec<Box<dyn Shape>>) -> Self {
        fn box_compare(axis: usize,) -> impl FnMut(&Box<dyn Shape>, &Box<dyn Shape>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box();
                let b_bbox = b.bounding_box();
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min[axis] + a.max[axis];
                    let bc = b.min[axis] + b.max[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
        }

        let axis_random = random();
        let if axis_random<0.33 {axis = 0 } else if axis_random<0.66 { axis = 1 } else { axis = 2 };

        Shape.sort_unstable_by(box_compare(axis));
        let len = Shape.len();
        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = Shape.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box() {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
            _ => {
                let right = BVH::new(Shape.drain(len / 2..).collect(), );
                let left = BVH::new(Shape, );
                let bbox = surrounding_box(&left.bbox, &right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Shape for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(&ray, t_min, t_max) {
            return None;
        }
        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(&ray, t_min, t_max),
            BVHNode::Branch { left, right } => {
                let left = left.hit(&ray, t_min, t_max);
                if let Some(l) = &left {
                    t_max = l.t
                };
                let right = right.hit(&ray, t_min, t_max);
                if right.is_some() {
                    right
                } else {
                    left
                }
            }
        }
    }

    fn bounding_box(&self,) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
