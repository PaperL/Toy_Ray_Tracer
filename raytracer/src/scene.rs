use crate::{
    basic::vec3::{Point3, RGBColor, Vec3},
    bvh::bvh_node::BvhNode,
    hittable::{
        instance::{
            constant_medium::ConstantMedium, motion_rotate::MotionRotate,
            motion_translate::MotionTranslate, rotate::Rotate, translate::Translate, zoom::Zoom,
        },
        obj_model::OBJModel,
        object::{
            cube::Cube,
            rectangle::{OneWayRectangle, Rectangle},
            sphere::Sphere,
            triangle::Triangle,
        },
        HittableList,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::solid_color::SolidColor,
};

pub fn _cornell_box_bvh(world: &mut HittableList, lights: &mut HittableList) {
    let mut objects = HittableList::default();

    // Material
    let red = Lambertian::new(SolidColor::new_from_value(0.65, 0.05, 0.05));
    let green = Lambertian::new(SolidColor::new_from_value(0.12, 0.45, 0.15));
    let white = Lambertian::new(SolidColor::new_from_value(0.73, 0.73, 0.73));
    let light_white = DiffuseLight::new_from_color(RGBColor::new(255., 223., 184.) / 255. * 30.); // Color of 4700K
    let light_gloden = DiffuseLight::new_from_color(RGBColor::new(248., 231., 28.) / 255. * 120.);
    let aluminum = Metal::new(RGBColor::new(0.8, 0.85, 0.88), 0.);
    let glass = Dielectric::new(1.5);

    // Wall
    objects.add(Rectangle::new(1, 0., 555., 0., 555., 0., red));
    objects.add(Rectangle::new(1, 0., 555., 0., 555., 555., green));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 0., white.clone()));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 555., white.clone()));
    objects.add(OneWayRectangle::new(
        0,
        0.,
        555.,
        0.,
        555.,
        0.,
        white.clone(),
        true,
    ));
    objects.add(Rectangle::new(0, 0., 555., 0., 555., 555., white));

    // Light
    let light_obj = OneWayRectangle::new(2, 213., 343., 227., 332., 554., light_white, false);
    objects.add(light_obj.clone());

    // Cube
    let cube = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        aluminum.clone(),
    );
    let moved_cube = Translate::new(Rotate::new(cube, 1, 15.), Vec3::new(295., 0., 255.));
    let cm = ConstantMedium::new_from_color(moved_cube, 0.01, RGBColor::new(0.0, 0.0, 0.3));
    objects.add(cm);

    // Glass Ball
    let glass_ball = Sphere::new(Point3::new(200., 90., 150.), 90., glass);
    objects.add(glass_ball.clone());

    // Triangle
    let triangle = Triangle::new(
        [
            Vec3::new(250., 0., 500.),
            Vec3::new(50., 0., 200.),
            Vec3::new(0., 260., 350.),
        ],
        aluminum,
    );
    objects.add(triangle.clone());

    // Golden Snitch
    //(277., 277., 50.)
    let snitch = Sphere::new(Point3::new(140., 0., 0.), 10., light_gloden);
    let flying_snitch = MotionTranslate::new(
        MotionRotate::new(snitch, 1, 360. * 2.5, 0., 1.),
        Vec3::new(0., 320., 0.),
        0.,
        1.,
    );
    let moved_flaying_snitch = Translate::new(flying_snitch, Vec3::new(405., 0., 410.));
    objects.add(moved_flaying_snitch);

    // Air
    // let air = ConstantMedium::new_from_color(
    //     Cube::new(Vec3::new(0., 0., 0.), Vec3::new(555., 555., 555.), white),
    //     0.000001,
    //     RGBColor::new(1.0, 1.0, 1.0),
    // );
    // objects.add(air);

    // *world = objects;
    // BVH
    world.add(BvhNode::new_from_list(objects, 0., 1.));

    // Hittable PDF
    lights.add(light_obj);
    lights.add(glass_ball);
    lights.add(triangle);
}

pub fn paper_world(world: &mut HittableList, lights: &mut HittableList) {
    let mut objects = HittableList::default();

    // Material
    let gray = Lambertian::new(SolidColor::new_from_value(0.73, 0.73, 0.73));
    let red = Lambertian::new(SolidColor::new_from_value(0.65, 0.05, 0.05));
    let green = Lambertian::new(SolidColor::new_from_value(0.12, 0.45, 0.15));
    let light_white = DiffuseLight::new_from_color(RGBColor::new(255., 223., 184.) / 255. * 15.); // Color of 4700K
    let glass = Dielectric::new(1.5);
    // let aluminum = Metal::new(RGBColor::new(0.8, 0.85, 0.88), 0.);
    // let black_metal = Metal::new(RGBColor::new(0.2, 0.2, 0.2), 0.01);

    // Wall
    objects.add(Rectangle::new(1, 0., 555., 0., 555., 0., red));
    objects.add(Rectangle::new(1, 0., 555., 0., 555., 555., green));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 0., gray.clone()));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 555., gray.clone()));
    objects.add(OneWayRectangle::new(
        0,
        0.,
        555.,
        0.,
        555.,
        0.,
        gray.clone(),
        true,
    ));
    objects.add(Rectangle::new(0, 0., 555., 0., 555., 555., gray));

    // Light
    let light_obj = OneWayRectangle::new(2, 213., 343., 227., 332., 554., light_white, false);
    objects.add(light_obj.clone());

    let chess = OBJModel::load_from_file("raytracer/model/Chess set.obj", glass, 0., 1.);
    let big_chess = Zoom::new(chess, 4.0);
    let moved_chess = Rotate::new(
        Translate::new(Rotate::new(big_chess, 0, 270.), Vec3::new(205., 0., 340.)),
        1,
        15.,
    );
    world.add(moved_chess);

    // *world = objects;
    // BVH
    world.add(BvhNode::new_from_list(objects, 0., 1.));

    // Hittable PDF
    lights.add(light_obj);
}
