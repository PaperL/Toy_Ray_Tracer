use rand::{prelude::ThreadRng, random, Rng};

use crate::{
    basic::{
        rand_1, tp,
        vec3::{Point3, RGBColor, Vec3},
    },
    bvh::bvh_node::BvhNode,
    hittable::{
        instance::{rotate_y::RotateY, translate::Translate},
        object::{
            constant_medium::ConstantMedium, cube::Cube, moving_sphere::MovingSphere,
            rectangle::Rectangle, sphere::Sphere,
        },
        HittableList,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, solid_color::SolidColor,
    },
};

pub fn random_scene(
    world: &mut HittableList,
    background: &mut RGBColor,
    look_from: &mut Point3,
    look_at: &mut Point3,
    vfov: &mut f64,
) {
    *world = HittableList::default();
    *background = RGBColor::new(0.9, 0.9, 0.95);
    *look_from = Point3::new(13., 2., -3.);
    *look_at = Point3::new(0., 0., 0.);
    *vfov = 25.;

    let checker = tp(Lambertian {
        albedo: tp(CheckerTexture {
            odd: tp(SolidColor::new_from_value(0.2, 0.3, 0.1)),
            even: tp(SolidColor::new_from_value(0.9, 0.9, 0.9)),
        }),
    });
    let metal = tp(Metal {
        albedo: RGBColor::new(1., 1., 1.),
        fuzz: 0.,
    });
    let glass = tp(Dielectric { ir: 1.5 });

    world.add(tp(Sphere {
        cen: Point3::new(0., -1000., 0.),
        r: 1000.,
        mat: metal,
    }));

    world.add(tp(Sphere {
        cen: Point3::new(-2., 1., 0.),
        r: 1.,
        mat: glass,
    }));

    world.add(tp(Translate::new(
        tp(RotateY::new(
            tp(Sphere {
                cen: Point3::new(0., 0., 0.),
                r: 1.,
                mat: checker,
            }),
            180.,
        )),
        Vec3::new(4., 1., 0.),
    )));

    let mut rnd: ThreadRng = rand::thread_rng();
    for i in -7..7 {
        for j in -7..7 {
            let cen = Point3::new(
                i as f64 + 1.1 * rnd.gen::<f64>(),
                rnd.gen_range(0.17..0.23),
                j as f64 + 1.1 * rnd.gen::<f64>(),
            );

            if (cen - Point3::new(4., cen.y, 0.)).length() > 1.2 {
                let mat_dice = random::<f64>();
                if mat_dice < 0.8 {
                    let sphere_material = tp(Lambertian {
                        albedo: tp(SolidColor {
                            color_value: RGBColor::rand_1(),
                        }),
                    });
                    world.add(tp(MovingSphere {
                        r: cen.y,
                        mat: sphere_material,
                        cen,
                        mov: Vec3::new(0., rnd.gen_range(0.0..0.5), 0.),
                        tm: 0.,
                        dur: 1.,
                    }));
                } else if mat_dice < 0.95 {
                    let sphere_material = tp(Metal {
                        albedo: RGBColor::rand(0.5, 1.),
                        fuzz: rnd.gen_range(0.0..0.5),
                    });
                    world.add(tp(Sphere {
                        cen,
                        r: cen.y,
                        mat: sphere_material,
                    }));
                } else {
                    let sphere_material = tp(Dielectric { ir: 1.5 });
                    world.add(tp(Sphere {
                        cen,
                        r: cen.y,
                        mat: sphere_material,
                    }));
                }
            }
        }
    }
}

pub fn simple_dark_scene(
    world: &mut HittableList,
    background: &mut RGBColor,
    look_from: &mut Point3,
    look_at: &mut Point3,
    vfov: &mut f64,
) {
    *world = HittableList::default();
    *background = RGBColor::new(0., 0., 0.);
    *look_from = Point3::new(26., 3., 6.);
    *look_at = Point3::new(0., 2., 0.);
    *vfov = 40.;

    let earth_texture = tp(ImageTexture::new_from_file(
        &"texture/earth.jpg".to_string(),
    ));

    let solid_texture = tp(SolidColor::new_from_value(1.0, 1.0, 0.9));

    world.add(tp(Sphere {
        cen: Point3::new(0., -1000., 0.),
        r: 1000.,
        mat: tp(Lambertian {
            albedo: solid_texture,
        }),
    }));
    world.add(tp(Sphere {
        cen: Point3::new(0., 2., 0.),
        r: 1.7,
        mat: tp(Lambertian {
            albedo: earth_texture,
        }),
    }));

    let light_texture = tp(DiffuseLight::new_from_color(RGBColor::new(4., 4., 4.3)));

    world.add(tp(Rectangle::new(
        0,
        -1.5,
        1.5,
        1.,
        3.,
        -4.,
        light_texture.clone(),
    )));
    world.add(tp(Rectangle::new(
        1,
        1.,
        3.,
        -1.5,
        1.5,
        -4.,
        light_texture.clone(),
    )));
    world.add(tp(Rectangle::new(
        2,
        -1.5,
        1.5,
        -1.5,
        1.5,
        4.,
        light_texture.clone(),
    )));
    world.add(tp(Rectangle::new(
        2,
        -1.5,
        1.5,
        -1.5,
        1.5,
        0.1,
        light_texture,
    )));
}

pub fn cornell_box_bvh(
    world: &mut HittableList,
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

    let red = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.65, 0.05, 0.05)),
    });
    let green = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.12, 0.45, 0.15)),
    });
    let white = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.73, 0.73, 0.73)),
    });
    let light = tp(DiffuseLight::new_from_color(RGBColor::new(30., 30., 30.)));

    tmp_world.add(tp(Rectangle::new(1, 0., 555., 0., 555., 0., red.clone())));
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
    tmp_world.add(tp(Rectangle::new(0, 0., 555., 0., 555., 555., white)));

    // tmp_world.add(tp(Rectangle::new(
    //     2,
    //     213.,
    //     343.,
    //     227.,
    //     332.,
    //     554.,
    //     light.clone(),
    // )));

    let cube1 = Cube::new(Point3::new(0., 0., 0.), Point3::new(165., 330., 165.), red);

    let cube2 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        tp(Metal::new(RGBColor::new(0.8, 0.8, 0.9), 0.1)),
    );

    let moved_cube1 = Translate {
        item: tp(RotateY::new(tp(cube1), 15.)),
        offset: Vec3::new(265., 0., 295.),
    };

    let moved_cube2 = Translate {
        item: tp(RotateY::new(tp(cube2), -18.)),
        offset: Vec3::new(130., 0., 65.),
    };

    tmp_world.add(tp(ConstantMedium::new_from_color(
        tp(moved_cube1),
        0.01,
        RGBColor::new(0., 0., 0.),
    )));
    // objects.add(tp(ConstantMedium::new_from_color(
    //     tp(moved_cube2),
    //     0.01,
    //     RGBColor::new(1., 1., 1.),
    // )));

    tmp_world.add(tp(moved_cube2));

    // lamp
    for _i in 0..1000 {
        let bulb = Sphere::new(Point3::rand_unit() * 70., rand_1(), light.clone());
        let t_bulb = Translate {
            item: tp(bulb),
            offset: Vec3::new(277., 570., 277.),
        };
        tmp_world.add(tp(t_bulb));
    }

    // air
    let boundary = Sphere::new(
        Point3::new(277., 277., 277.),
        500.,
        tp(Lambertian::new_from_color(RGBColor::default())),
    );
    tmp_world.add(tp(ConstantMedium::new(
        tp(boundary),
        0.001,
        tp(SolidColor::new_from_value(1., 1., 1.)),
    )));

    world.add(tp(BvhNode::new_from_list(&tmp_world, 0., 1.)));
}

pub fn book2_final_scene(
    world: &mut HittableList,
    background: &mut RGBColor,
    look_from: &mut Point3,
    look_at: &mut Point3,
    vfov: &mut f64,
) {
    let mut ground = HittableList::default();
    *world = HittableList::default();
    *background = RGBColor::new(0., 0., 0.);
    *look_from = Point3::new(478., 278., -600.);
    *look_at = Point3::new(278., 278., 0.);
    *vfov = 40.;

    // ground cubes
    let boxes_per_side = 20;
    let mut rnd: ThreadRng = rand::thread_rng();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = rnd.gen_range(1.0..100.0);
            let z1 = z0 + w;
            let ground_material = tp(Lambertian::new_from_color(
                RGBColor::new(0.48, 0.83, 0.53) * rnd.gen_range(0.8..1.1),
            ));
            ground.add(tp(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground_material,
            )));
        }
    }
    world.add(tp(BvhNode::new_from_list(&ground, 0., 1.)));

    // light
    let light = tp(DiffuseLight::new_from_color(RGBColor::new(7., 7., 7.)));
    world.add(tp(Rectangle::new(2, 123., 423., 147., 412., 554., light)));

    // moving yellow ball
    let cen0 = Point3::new(330., 400., 220.);
    let cen1 = cen0 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = tp(Lambertian::new_from_color(RGBColor::new(0.7, 0.3, 0.1)));
    world.add(tp(MovingSphere::new(
        cen0,
        cen1,
        0.,
        1.,
        50.,
        moving_sphere_material,
    )));

    // glass ball
    world.add(tp(Sphere::new(
        Point3::new(240., 170., 20.),
        60.,
        tp(Dielectric::new(1.5)),
    )));

    // iron ball
    world.add(tp(Sphere::new(
        Point3::new(80., 150., 10.),
        50.,
        tp(Metal::new(RGBColor::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // smooth blue ball
    let boundary1 = Sphere::new(Point3::new(350., 150., 155.), 50., tp(Dielectric::new(1.5)));
    world.add(tp(boundary1.clone()));
    world.add(tp(ConstantMedium::new(
        tp(boundary1),
        0.2,
        tp(SolidColor::new_from_value(0.2, 0.4, 0.9)),
    )));

    // air
    let boundary2 = Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        tp(Lambertian::new_from_color(RGBColor::default())),
    );
    world.add(tp(ConstantMedium::new(
        tp(boundary2),
        0.0001,
        tp(SolidColor::new_from_value(1., 1., 1.)),
    )));

    // globe
    let earth_texture = tp(ImageTexture::new_from_file(
        &"texture/earth.jpg".to_string(),
    ));
    world.add(tp(Sphere::new(
        Point3::new(380., 220., 400.),
        120.,
        tp(Lambertian::new(earth_texture)),
    )));

    // plastic foam
    let mut plastic_foam_list = HittableList::default();
    let white = tp(Lambertian::new_from_color(RGBColor::new(0.73, 0.73, 0.73)));
    for _i in 0..1000 {
        plastic_foam_list.add(tp(Sphere::new(Vec3::rand_1() * 165., 10., white.clone())));
    }
    world.add(tp(Translate::new(
        tp(RotateY::new(
            tp(BvhNode::new_from_list(&plastic_foam_list, 0., 1.)),
            15.,
        )),
        Vec3::new(-100., 270., 395.),
    )));
}

pub fn cornell_box(
    world: &mut HittableList,
    background: &mut RGBColor,
    look_from: &mut Point3,
    look_at: &mut Point3,
    vfov: &mut f64,
) {
    *world = HittableList::default();
    *background = RGBColor::new(1., 1., 1.);
    *look_from = Point3::new(278., 278., -800.);
    *look_at = Point3::new(278., 278., 0.);
    *vfov = 40.;

    let red = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.65, 0.05, 0.05)),
    });
    let green = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.12, 0.45, 0.15)),
    });
    let white = tp(Lambertian {
        albedo: tp(SolidColor::new_from_value(0.73, 0.73, 0.73)),
    });
    let light = tp(DiffuseLight::new_from_color(RGBColor::new(20., 20., 20.)));

    world.add(tp(Rectangle::new(1, 0., 555., 0., 555., 0., red.clone())));
    world.add(tp(Rectangle::new(1, 0., 555., 0., 555., 555., green)));
    world.add(tp(Rectangle::new(2, 0., 555., 0., 555., 0., white.clone())));
    world.add(tp(Rectangle::new(
        2,
        0.,
        555.,
        0.,
        555.,
        555.,
        white.clone(),
    )));
    world.add(tp(Rectangle::new(0, 0., 555., 0., 555., 555., white)));

    // world.add(tp(Rectangle::new(
    //     2,
    //     213.,
    //     343.,
    //     227.,
    //     332.,
    //     554.,
    //     light.clone(),
    // )));

    let cube1 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        red.clone(),
    );

    let cube2 = Cube::new(Point3::new(0., 0., 0.), Point3::new(165., 165., 165.), red);

    let moved_cube1 = Translate {
        item: tp(RotateY::new(tp(cube1), 15.)),
        offset: Vec3::new(265., 0., 295.),
    };

    let moved_cube2 = Translate {
        item: tp(RotateY::new(tp(cube2), -18.)),
        offset: Vec3::new(130., 0., 65.),
    };

    world.add(tp(moved_cube1));
    world.add(tp(moved_cube2));

    // lamp
    for _i in 0..500 {
        let bulb = Sphere::new(Point3::rand_unit() * 70., rand_1(), light.clone());
        let t_bulb = Translate {
            item: tp(bulb),
            offset: Vec3::new(277., 570., 277.),
        };
        world.add(tp(t_bulb));
    }
}
