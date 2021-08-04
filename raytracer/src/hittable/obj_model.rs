use crate::{
    basic::{ray::Ray, vec3::Point3},
    bvh::{aabb::AABB, bvh_node::BvhNode},
    hittable::{object::triangle::Triangle, HittableList},
    material::Material,
};

use super::{HitRecord, Hittable};

pub struct OBJModel {
    pub triangles: BvhNode,
}

impl OBJModel {
    pub fn load_from_file<TM>(file_name: &str, mat: TM, tm: f64, dur: f64) -> Self
    where
        TM: Material + 'static + Clone,
    {
        let tmp_tri = tobj::load_obj(
            file_name,
            &tobj::LoadOptions {
                single_index: false,
                triangulate: true,
                ..Default::default()
            },
        );
        assert!(tmp_tri.is_ok());

        let (tri, _mtl_mat) = tmp_tri.expect("Failed to load OBJ file.");

        // for item in tri {
        //     println!("\n\n\nHere's a ITEM\n\n\n");
        //     let mut cnt = 0;
        //     for p in item.mesh.positions {
        //         print!("{} ", p);
        //         cnt += 1;
        //         if cnt % 9 == 0 {
        //             println!("\n");
        //         }
        //     }
        // }

        let mut objects = HittableList::default();
        for obj in tri {
            let mut cnt = 0;
            let mut pos = [0; 3];
            let mesh = &obj.mesh;
            for p in &mesh.indices {
                pos[cnt] = (*p as usize) * 3;
                cnt += 1;
                if cnt == 3 {
                    objects.add(Triangle::new(
                        [
                            Point3::new(
                                mesh.positions[pos[0]] as f64,
                                mesh.positions[pos[0] + 1] as f64,
                                mesh.positions[pos[0] + 2] as f64,
                            ),
                            Point3::new(
                                mesh.positions[pos[1]] as f64,
                                mesh.positions[pos[1] + 1] as f64,
                                mesh.positions[pos[1] + 2] as f64,
                            ),
                            Point3::new(
                                mesh.positions[pos[2]] as f64,
                                mesh.positions[pos[2] + 1] as f64,
                                mesh.positions[pos[2] + 2] as f64,
                            ),
                        ],
                        mat.clone(),
                    ));
                    cnt = 0;
                }
            }
        }

        Self {
            triangles: BvhNode::new_from_list(objects, tm, dur),
        }
    }
}

impl Hittable for OBJModel {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.triangles.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB> {
        self.triangles.bounding_box(tm, dur)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{basic::vec3::RGBColor, material::lambertian::Lambertian};

    use super::*;

    #[test]
    fn test() {
        let path = env::current_dir().unwrap();
        println!("{}", path.display());
        let white = Lambertian::new_from_color(RGBColor::new(1., 1., 1.));
        OBJModel::load_from_file("model/Chess set.obj", white, 0., 1.);
    }
}
