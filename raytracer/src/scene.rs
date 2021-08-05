use std::f64::consts::PI;

use rand::{prelude::StdRng, Rng, SeedableRng};

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
            ring::BrokenRing,
            sphere::Sphere,
            triangle::Triangle,
        },
        HittableList,
    },
    material::{
        dielectric::Dielectric,
        diffuse_light::DiffuseLight,
        lambertian::Lambertian,
        metal::{ColoredMetal, Metal},
    },
    texture::{checker::Checker, gradient::Gradient, solid_color::SolidColor},
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

pub fn paper_world(world: &mut HittableList, _lights: &mut HittableList) {
    let mut objects = HittableList::default();

    // Material
    let glass = Dielectric::new(1.5);
    // let white = Lambertian::new(SolidColor::new_from_value(0.73, 0.73, 0.73));
    // let aluminum = Metal::new(RGBColor::new(0.8, 0.85, 0.88), 0.);
    // let black_metal = Metal::new(RGBColor::new(0.2, 0.2, 0.2), 0.01);
    // let white_metal = Metal::new(RGBColor::new(0.9, 0.9, 0.9), 0.01);

    // Stars
    let light_star = [
        DiffuseLight::new(Gradient::new(
            vec![
                RGBColor::new(217., 250., 255.) / 255. * 1., //.pow2() / 65025.
                RGBColor::new(120., 238., 255.) / 255. * 1.,
            ],
            vec![0., 1.],
        )),
        DiffuseLight::new(Gradient::new(
            vec![
                RGBColor::new(199., 232., 255.) / 255. * 1.,
                RGBColor::new(84., 225., 255.) / 255. * 1.,
                RGBColor::new(199., 232., 255.) / 255. * 1.,
            ],
            vec![0., 0.3, 1.],
        )),
        DiffuseLight::new(Gradient::new(
            vec![
                RGBColor::new(255., 251., 182.) / 255. * 1.,
                RGBColor::new(182., 232., 255.) / 255. * 1.,
                RGBColor::new(255., 251., 182.) / 255. * 1.,
                RGBColor::new(182., 232., 255.) / 255. * 1.,
            ],
            vec![0., 0.4, 0.8, 1.],
        )),
        DiffuseLight::new(Gradient::new(
            vec![
                RGBColor::new(238., 236., 211.) / 255. * 1.,
                RGBColor::new(238., 237., 225.) / 255. * 1.,
                RGBColor::new(255., 249., 193.) / 255. * 1.,
                RGBColor::new(238., 236., 211.) / 255. * 1.,
            ],
            vec![0., 0.2, 0.7, 1.],
        )),
    ];

    let mut rnd = StdRng::seed_from_u64(19260817);
    let ring_num = 120;
    for i in 0..ring_num {
        let mut point_list = Vec::<f64>::new();
        let k = (ring_num as f64 - i as f64) / ring_num as f64;
        let step_len = rnd.gen_range(0.03 + k * 0.1..0.5 - k * 0.2);
        let mut p = 0.;
        while p < 1. {
            p += rnd.gen_range(0.01..step_len);
            point_list.push(p * 2. * PI);
        }

        let mat = light_star[rnd.gen_range(0..4)].clone();

        let ring = BrokenRing::new(
            400. + i as f64 * (110. + rnd.gen::<f64>() * 10.),
            2. + rnd.gen::<f64>() * 6.,
            point_list,
            mat,
        );
        let moved_ring = Rotate::new(
            Translate::new(Rotate::new(ring, 0, 55.), Vec3::new(0., 3000., 2000.)),
            1,
            10.,
        );
        objects.add(moved_ring);
    }

    let mut chess_set = HittableList::default();
    let chess_id = [1, 2, 12, 9, 5, 6, 14];
    for id in chess_id {
        chess_set.add(OBJModel::load_from_file(
            "raytracer/model/Chess set.obj",
            id,
            glass.clone(),
            0.,
            1.,
        ));
    }
    let big_chess_set = Zoom::new(chess_set, Vec3::new(40., 40., 40.));
    let moved_chess_set = Translate::new(
        Rotate::new(big_chess_set, 0, 270.),
        Vec3::new(100., 0., 1800.),
    );
    objects.add(moved_chess_set);

    // Polyhedron
    let light_polyhedron = DiffuseLight::new_from_color(RGBColor::new(0., 240., 207.) / 255. * 15.);
    let polyhedron_light = OBJModel::load_from_file(
        "raytracer/model/Polyhedron.obj",
        0,
        light_polyhedron,
        0.,
        1.,
    );
    let moved_polyhedron_light = Translate::new(
        Zoom::new(polyhedron_light, Vec3::new(15., 15., 15.)),
        Vec3::new(800., 2900., 2000.),
    );
    let polyhedron = OBJModel::load_from_file("raytracer/model/Polyhedron.obj", 0, glass, 0., 1.);
    let moved_polyhedron = Translate::new(
        Zoom::new(polyhedron, Vec3::new(40., 40., 40.)),
        Vec3::new(800., 2900., 2000.),
    );
    objects.add(moved_polyhedron_light);
    objects.add(moved_polyhedron);

    // Ground
    let ground = Sphere::new(
        Point3::new(0., -5000000., 0.),
        5000000.,
        ColoredMetal::new(
            RGBColor::new(1., 1., 1.),
            0.05,
            Checker::new(
                SolidColor::new_from_value(0., 0., 0.),
                SolidColor::new_from_value(1., 1., 1.),
                0.01,
            ),
        ),
    );
    objects.add(ground);

    // Background
    let mut color_set = vec![
        RGBColor::new(216., 129., 110.),
        RGBColor::new(216., 129., 110.),
        RGBColor::new(198., 182., 168.),
        RGBColor::new(60., 103., 159.),
        RGBColor::new(33., 71., 121.),
        RGBColor::new(10., 26., 57.),
    ];
    for col in &mut color_set {
        col.x = col.x.powi(2) / 65025.;
        col.y = col.y.powi(2) / 65025.;
        col.z = col.z.powi(2) / 65025.;
    }
    let pos_set = vec![0., 0.50, 0.53, 0.57, 0.62, 1.];
    let background_sphere = Sphere::new(
        Vec3::new(0., -12000., 0.),
        50000.,
        DiffuseLight::new(Gradient::new(color_set, pos_set)),
        // Gradient::new(color_set, pos_set)
    );
    let rotated_background_sphere = Rotate::new(background_sphere, 2, 18.);
    objects.add(rotated_background_sphere);

    // *world = objects;
    // BVH
    world.add(BvhNode::new_from_list(objects, 0., 1.));

    // Hittable PDF
    // lights.add(BvhNode::new_from_l);
}
