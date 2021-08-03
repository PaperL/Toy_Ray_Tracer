use crate::{
    basic::vec3::{Point3, RGBColor, Vec3},
    bvh::bvh_node::BvhNode,
    hittable::{
        instance::{rotate_y::RotateY, translate::Translate},
        object::{
            constant_medium::ConstantMedium,
            cube::Cube,
            rectangle::{OneWayRectangle, Rectangle},
            sphere::Sphere,
        },
        HittableList,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::solid_color::SolidColor,
};

pub fn cornell_box_bvh(world: &mut HittableList, lights: &mut HittableList) {
    let mut objects = HittableList::default();

    // Material
    let red = Lambertian::new(SolidColor::new_from_value(0.65, 0.05, 0.05));
    let green = Lambertian::new(SolidColor::new_from_value(0.12, 0.45, 0.15));
    let white = Lambertian::new(SolidColor::new_from_value(0.73, 0.73, 0.73));
    // 4700K
    let light = DiffuseLight::new_from_color(RGBColor::new(255., 223., 184.) / 255. * 20.);
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
    let light_obj = OneWayRectangle::new(2, 213., 343., 227., 332., 554., light, false);
    objects.add(light_obj.clone());

    // Cube
    let cube = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        aluminum,
    );
    let moved_cube = Translate::new(RotateY::new(cube, 15.), Vec3::new(265., 0., 295.));
    let cm = ConstantMedium::new_from_color(moved_cube, 0.01, RGBColor::new(0.0, 0.0, 0.4));
    objects.add(cm);

    let glass_ball = Sphere::new(Point3::new(190., 90., 190.), 90., glass);
    objects.add(glass_ball.clone());

    // *world = objects;
    world.add(BvhNode::new_from_list(objects, 0., 1.));

    lights.add(light_obj);
    lights.add(glass_ball);
    // lights.add(moved_cube);
}
