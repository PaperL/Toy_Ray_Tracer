# PaperL's Toy Ray Tracer

## 简介

- 作者：上海交通大学，2020级ACM班，PaperL
- 版本：`v0.5.1`
- 说明：2020-2021学年暑期小学期，PPCA课程项目。[本项目题面](https://github.com/aik2mlj/raytracer-tutorial)，[原题面](https://github.com/skyzh/raytracer-tutorial)

- 特别感谢：
  - 感谢助教 [AIK2](https://github.com/aik2mlj), 助教 [XHRlyb](https://github.com/XHRlyb) 关于项目的耐心指导
  - 感谢 [Marcythm](https://github.com/Marcythm) 关于 Rust 语言的详尽教导
  - 感谢与 [yichuan~](https://github.com/yichuan520030910320) 关于光追原理的交流和讨论



## 当前版本渲染结果效果图

![Output](output/preview.jpg)

> 图为封闭的康奈尔盒子
>
> 编译运行指令 `make run-release`



## 主要工作

- [x] 实现 [Ray Tracing in One Weekend 系列教程](https://raytracing.github.io/) Book1~3

- 提高代码质量

  - [x] 整理源文件结构
  - [x] 规范变量命名
  - [x] 规范浮点数计算，处理精度问题

- 改进算法

  - [x] 合并三个方向的 `Rectangle` 类
  - [x] 实现单向透光的 `OneWayRectangle` 类
  - [x] 实现 `Transform` 类的 PDF 功能
  - [x] 修正教程中 PDF 相关公式错误，实现更合理的混合多种 PDF 的方式
  - [x] 使用泛型避免不定长的 `dyn` 类型相对于定长类型的额外开销
  - [ ] 实现 `Triangle` 类
  - [ ] 使用过程宏生成静态 BVH 数据，提高渲染效率

- 扩展功能

  - [x] 使用 GitHub Action，实现自动根据 tag 将稳定版本代码编译运行，并将生成结果上传至 Release
  - [x] 提供友好的控制台 UI 界面
  - [x] 实现多线程并行计算
  - [x] 以自定义质量因子的 `JPEG` 格式输出渲染结果，平衡图像大小与质量
  - [ ] 支持从 `obj` 格式文件输入场景
  - [ ] 支持从 `yaml` 或 `JSON` 文件读取场景并生成对应的静态 BVH 数据
  - [ ] 使用 `criterion crate` 实现基准测试 (Benchmark)，用于比较不同版本代码的差异



## Module Tree

- **create**
  - `scene`
  - **basic**
    - `vec3`, `ray`, `camera`, `onb`
  - **hittable**
    - **instance**
      - `translate`, `rotate_y`, `flip`
    - **object**
      - `sphere`, `moving_sphere`, `rectangle`, `cube`, `constant_medium`
  - **bvh**
    - `aabb`, `bvh_node`
  - **material**
    - `lambertian`, `metal`, `dielectric`, `diffuse_light`, `isotropic`
  - **pdf**
    - `cos_pdf`, `hittable_pdf`
  - **texture**
    - `solid_color`, `checker_texture`, `image_texture`

> 自动代码格式化指令 `cargo fmt`
>
> 代码检查指令 `cargo clippy --all-targets --all-features`



## 学习感想

1. 原教程存在较多可以改进的地方以及代码中的公式错误
   - 变量命名规范及源文件结构
   - 浮点运算精度问题
   - `Rectangle` 的 `Hit` 函数会产生 `Nan`,  `pdf_value` 函数在无碰撞时产生错误的返回值 `0`
2. 很好地学习了 Rust 语言基础
3. 对于基础的光追系统有了较为深入的认识
4. 提高了关于 GitHub 的熟悉程度
   - GitAction 及 tag 的使用
   - 及时 Commit 稳定代码来帮助快速定位 Bug 或回滚





## Commit Checklist

- **New Release?** 
  - [x] Yes
  - [ ] No

- **源代码**
  - [x] `Cargo fmt`
  - [x] `Cargo Clippy`
  - [x] 渲染参数*（线程数，分辨率，采样率，图像质量）*
- **文件**
  - [x] `output` 文件夹
- **其他**
  - [x] `README.md` 版本号
  - [x] Rust 项目版本号
  - [x] GitHub Action - Workflow - Release Body


