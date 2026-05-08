use crate::{
    object::{HitRecord, Hittable, aabb::Aabb},
    ray::Ray,
    scene::Float0,
};

#[derive(Debug)]
pub struct Bvh {
    root: Option<BvhNode>,
}

#[derive(Debug)]
enum BvhNode {
    Leaf {
        bbox: Aabb,
        indices: Vec<usize>,
    },
    Branch {
        bbox: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
}

impl Bvh {
    pub fn build(objects: &[Box<dyn Hittable>]) -> (Self, Vec<usize>) {
        let mut bounded = Vec::new();
        let mut unbounded = Vec::new();

        for (index, object) in objects.iter().enumerate() {
            if let Some(bbox) = object.bounding_box() {
                bounded.push((index, bbox));
            } else {
                unbounded.push(index);
            }
        }

        let root = BvhNode::build(&mut bounded);
        (Self { root }, unbounded)
    }

    pub fn hit<'a>(
        &'a self,
        objects: &'a [Box<dyn Hittable>],
        ray: &Ray,
        t_min: Float0,
        t_max: Float0,
    ) -> Option<HitRecord<'a>> {
        self.root
            .as_ref()
            .and_then(|root| root.hit(objects, ray, t_min, t_max))
    }
}

impl BvhNode {
    fn build(entries: &mut [(usize, Aabb)]) -> Option<Self> {
        match entries.len() {
            0 => None,
            1..=4 => {
                let bbox = entries
                    .iter()
                    .map(|(_, bbox)| *bbox)
                    .reduce(Aabb::surrounding)?;
                Some(Self::Leaf {
                    bbox,
                    indices: entries.iter().map(|(index, _)| *index).collect(),
                })
            }
            _ => {
                let bbox = entries
                    .iter()
                    .map(|(_, bbox)| *bbox)
                    .reduce(Aabb::surrounding)?;
                let axis = bbox.longest_axis();
                entries.sort_by(|(_, a), (_, b)| {
                    a.centroid_axis(axis)
                        .partial_cmp(&b.centroid_axis(axis))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
                let mid = entries.len() / 2;
                let (left_entries, right_entries) = entries.split_at_mut(mid);
                let left = Box::new(Self::build(left_entries)?);
                let right = Box::new(Self::build(right_entries)?);
                Some(Self::Branch { bbox, left, right })
            }
        }
    }

    fn bbox(&self) -> Aabb {
        match self {
            Self::Leaf { bbox, .. } | Self::Branch { bbox, .. } => *bbox,
        }
    }

    fn hit<'a>(
        &'a self,
        objects: &'a [Box<dyn Hittable>],
        ray: &Ray,
        t_min: Float0,
        t_max: Float0,
    ) -> Option<HitRecord<'a>> {
        if !self.bbox().hit(ray, t_min, t_max) {
            return None;
        }

        match self {
            Self::Leaf { indices, .. } => {
                let mut closest = t_max;
                let mut hit_record = None;
                for index in indices {
                    if let Some(record) = objects[*index].hit(ray, t_min, closest) {
                        closest = record.t;
                        hit_record = Some(record);
                    }
                }
                hit_record
            }
            Self::Branch { left, right, .. } => {
                let left_hit = left.hit(objects, ray, t_min, t_max);
                let closest = left_hit.as_ref().map_or(t_max, |hit| hit.t);
                let right_hit = right.hit(objects, ray, t_min, closest);
                right_hit.or(left_hit)
            }
        }
    }
}
