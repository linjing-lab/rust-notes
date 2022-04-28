# 简介

## 为什么要用Rust？

&emsp;&emsp;`Rust`是一种令人兴奋的编程语言，它可以让每个人编写可靠且高效的软件。它可以用来替换C/C++，Rust和它们具有同样的性能，但是很多常见的`bug`在编译时就可以被“消灭”。`Rust`是一种通用的编程语言，但是它更善于以下场景：
* 需要运行时的速度
* 需要内存安全
* 更好的利用多处理器

## 与其它语言的比较

&emsp;&emsp;C/C++性能非常好，但类型系统和内存都不太安全。Java/C#，拥有GC，能保证内存安全，也有很多优秀特性，但是性能不行。 `Rust`擅长的领域：
* 安全
* 无需GC
* 易于维护、调试，代码安全高效

`Rust`特别擅长的领域：
* 高性能`Web Service`
* WebAssembly
* 命令行工具
* 网络编程
* 嵌入式设备
* 系统编程

## Rust与Firefox

&emsp;&emsp;`Rust`最初是Mozilla公司的一个研究型项目。Firefox是`Rust`产品应用的一个重要例子。 Mozilla一直以来都在用`Rust`创建一个名为Servo的实验性浏览器引擎，其中的所有内容都是并行执行的（目前Servo的部分功能已经被集成到Firefox里面了）。Firefox原来的量子版包含了Servo的渲染引擎（`Rust`使得Firefox在这方面得到了巨大的性能改进）。

## Rust的用户与案例

* Google: 新操作系统Fuschia，其中`Rust`代码量大约占30%
* Amazon: 基于Linux开发的可以直接在裸机、虚机上运行容器的操作系统
* System76: 纯`Rust`开发了下一代安全操作系统Redox
* 蚂蚁金服: 库操作系统Occlum
* 斯坦福和密歇根大学: 嵌入式实时操作系统，应用于Google的加密产品
* 微软: 正在使用`Rust`重写Windows操作系统中的一些低级组件，WinRT（Rust项目）
* Dropbox、Yelp、Coursera、LINE、Cloudflare、Atlassian、npm、Ceph、百度、华为、Sentry、Deno...

## Rust的优点与缺点

`优点`:
* 性能
* 安全性
* 无所畏惧的并发

`缺点`: 难学
> `Rust`有很多独有的概念，它们和现在大多主流语言都不同。