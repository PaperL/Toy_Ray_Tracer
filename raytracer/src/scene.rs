use crate::{
    basic::{
        tp,
        vec3::{Point3, RGBColor, Vec3},
    },
    hittable::{
        instance::{flip::Flip, rotate_y::RotateY, translate::Translate},
        object::{
            // constant_medium::ConstantMedium,
            cube::Cube,
            //   moving_sphere::MovingSphere,
            rectangle::Rectangle,
            // sphere::Sphere,
        },
        HittableList,
    },
    material::{
        // dielectric::Dielectric
        diffuse_light::DiffuseLight,
        lambertian::Lambertian,
        metal::Metal,
    },
    texture::solid_color::SolidColor,
};

pub fn cornell_box_bvh(
    world: &mut HittableList,
    lights: &mut HittableList,
    background: &mut RGBColor,
    look_from: &mut Point3,
    look_at: &mut Point3,
    vfov: &mut f64,
) {
    *world = HittableList::default();
    *background = RGBColor::new(0., 0., 0.);
    *look_from = Point3::new(278., 278., -800.);
    *look_at = Point3::new(278., 278., 0.);
    *vfov = 40.;

    let mut tmp_world = HittableList::default();

    // Material
    let red = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.65, 0.05, 0.05)),
    });
    let green = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.12, 0.45, 0.15)),
    });
    let white = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.73, 0.73, 0.73)),
    });
    let light = tp(DiffuseLight::new_from_color(RGBColor::new(15., 15., 15.)));
    let aluminum = tp(Metal::new(RGBColor::new(0.8, 0.85, 0.88), 0.));

    // Wall
    tmp_world.add(tp(Rectangle::new(1, 0., 555., 0., 555., 0., red)));
    tmp_world.add(tp(Rectangle::new(1, 0., 555., 0., 555., 555., green)));
    tmp_world.add(tp(Rectangle::new(2, 0., 555., 0., 555., 0., white.clone())));
    tmp_world.add(tp(Rectangle::new(
        2,
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    tmp_world.add(tp(Rectangle::new(
        0,
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));

    // Light
    tmp_world.add(tp(Flip::new(tp(Rectangle::new(
        2,
        213.,
        343.,
        227.,
        332.,
        554.,
        light.clone(),
    )))));

    // Cube
    let cube1 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        aluminum,
    );

    let cube2 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    );

    let moved_cube1 = Translate {
        item: tp(RotateY::new(tp(cube1), 15.)),
        offset: Vec3::new(265., 0., 295.),
    };

    let moved_cube2 = Translate {
        item: tp(RotateY::new(tp(cube2), -18.)),
        offset: Vec3::new(130., 0., 65.),
    };

    tmp_world.add(tp(moved_cube1));
    tmp_world.add(tp(moved_cube2));

    world.add(tp(tmp_world));
    // world.add(tp(BvhNode::new_from_list(&tmp_world, 0., 1.)));

    lights.add(tp(Rectangle::new(2, 213., 343., 227., 332., 554., light)));
}
