# Toy Ray Tracer

## 简介

- 上海交通大学，2020级ACM班，PaperL
- 2020-2021学年暑期小学期，PPCA课程项目
- **初步掌握Rust语言**
- **学习光线追踪入门知识**



## Module Tree

- create
  - basic
    - vec3
    - ray
    - camera
  - hittable
    - sphere
    - moving_sphere
    - rectangle
    - cube
    - constant_medium
  - bvh
    - aabb
    - bvh_node
  - material
    - lambertian
    - metal
    - dielectric
    - diffuse_light
    - isotropic
  - texture
    - solid_color
    - checker_texture
    - image_texture



## 当前Commit运行结果

![Output](output/preview.jpg)

> 8线程并行计算，渲染总耗时19分钟



## 待完成工作

- 完成 Book3 内容
- 实现 BvnNode 数据在编译期计算完成