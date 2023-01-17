# 上应小风筝（服务端）

## 概要

上应小风筝，旨在为 [上海应用技术大学](www.sit.edu.cn) 的学生提供校园信息整合与管理服务，本仓库管理并维护服务端代码。

> 对于一位大学（本科）生而言，在校时间不过四年左右。而 “小风筝” 项目从 2020 年初疫情爆发时开始编写，到如今的 2023 已历经了
> 3个年头了。项目
> 团队的好友们所经历的 “从 ‘不太会使用 git 和 Github’、‘仅仅听说过 Rust 语言’，到如今虽然代码仍然写得不怎么样，但写出来的东西勉强能看、能用”
> 的时光，令人倍感珍惜与怀念。

让我来回顾一下项目发展的历史吧。

由于云计算厂商的服务器一般较为昂贵（他们一般希望 “帮助” 企业节约人工成本，因而设备租用价格并不低），加之经费报销的限制、个人习惯等因素，我们在
项目开工之初便决定尽可能地节约资源占用。开发之初缺乏有关开发和运营的经验，对 “并发数”、“用户数” 等概念和它背后的含义没有直观了解，我偏执地追
求着性能优化。我们希望它能在单核 1G 内存的机器上与数据库 PostgreSQL 流畅运行，并承载和选课阶段差不多的访问量。受
[TechEmpower Framework Benchmarks](https://www.techempower.com/benchmarks/) 的影响，我们在最初的版本中开发中选择了 Rust
语言
及 [actix-web](https://github.com/actix/actix-web) 框架，彼时它正霸占排行榜的第一位，而且后几位全是它的小弟～

2020 年中旬，我们开发出了 kite-server 第一版。9 月，提供 “新生入学查询”（即 “迎新”）服务，前端为微信小程序 “
[上应小风筝](https://github.com/SIT-kite/kite-microapp)”。当时的版本基于 actix-web 1 或 2，上线初发现：

- reqwest 库对微信 API 服务器的连接无法正常释放，`nestat` 报告了大量处于 `CLOSE_WAIT` 状态的连接
- 程序内存占用过大，猜测存在内存泄漏问题。内存占用 300 - 500M 左右时崩溃

在 Rust 中使用数据库和 SQL 这件事不好说难或简单。现成的库不少，社区尤为偏爱 PostgreSQL （现在我也是它的粉丝），但当时主流的
ORM 只有 diesel
（现在还有 SeaORM 等），我曾为了 diesel 的一个泛型错误找了一天资料，也曾失手将周末写的代码删除，后来因为其对异步的支持不好，以及复杂查询导致
“嵌套地狱” 的关系，项目便改用 sqlx 库了——裸写 SQL 语句也不错，至少有它帮忙把数据库返回的记录行转换成结构体。在 Web
后端开发过程中，文章匮乏
使人迷茫，连现成的项目也少得可怜，一般的例子都会带你写一组 TODO 接口，增删改查之后便草草结束——没有大一点的后端项目供我照葫芦画瓢，只能根据经验
不断地 “重构”，好让代码看起来更舒服一些。有一个 rust-admin 项目，是国人开发的，kite-server 照着它划了几瓢，可惜由于作者身体原因这个项目没
有进一步完善。

为了解决微信小程序访问校园网的问题，我们开发了 [kite-agent](https://github.com/SIT-kite/kite-agent) 程序，以期望它在校内提供中转服务。
具体是这样的：校园网内的机器（agent）先连接到 server, 后续由 server 将用户请求转发到 agent。绝大多数的 RPC 库都是 client 向
server 建立连
接并发送请求，而这个（反向的）需求很冷门，一时竟无法解决，于是我基于 bincode （序列化和反序列化库）和 socket 完成了一个简易的
RPC 工具。2021
年，在一番探索中，我兴奋地发现了 [tower](https://github.com/tower-rs/tower) 框架——它仿佛是为了我们这个需求而生的。暑假时我们就有了基于
tower 的 kite-agent。tower 把服务与底层的连接进行了分离，支持各种中间件，让后端项目的结构上了个档次。从这一点，可以看出 Rust
的优点：底层。
据我所知，一些其他语言的框架不太能完成这样的操作，他们默认了 server 就是打开 socket 去监听的那一方。一定程度上来讲，这个需求也可以用
frp、ssh
转发等方法实现，但当校园网内有多个 agent 实例时，这样的方式不太好做负载均衡。在后期我在 server 中支持了负载均衡的操作，
server 可以将用户请求
随机地发给一个 agent，但也遇到了另一个问题：我需要一个结构去管理和 agent 的连接，当 agent 断开时，我希望自动从连接池中删除它。因为缺乏开发经验，
在所有权上和编译器斗智斗勇，这个功能始终没能实现。迫不得已，改成：发现连接中断（此时 send 会报错 broken pipe）时删除该连接。这便是
Rust 基础不
牢带来的影响。

在 Rust 中做爬虫和网页解析更是麻烦。虽然有 Mozilla 开源的神器 html5ever，可 python 的 BeautifulSoup 谁也不想总是做那么底层的操作。直到
遇到了 scraper，这个库一定程度上简化了对 DOM 的操作，开发体验马马虎虎。

在 kite-server 1 和 kite-agent 开发期间，@zhangzqs 和 @B635 加入了开发，主要集中于 kite-agent 的编写。2021 年下半年的时候，我们的小
程序被校方要求下架，便改行做 App 开发，后端也要随之变动。后端趁机转向 v2 版本 ——基于油条哥的 poem-web 开发。总的来说，开发体验比现有的各个
Rust Web 框架好一点。这期间，因为计划在 App 上直接连接校园网，后端便省去了与 agent 通信的环节，逐渐成为纯粹的 CRUD 机器。

在 Web 开发的过程中，慢慢体会到，接口是前后端交流的根据。于是，本分支下的 kite-server 3, 开始采用 tonic 框架，使用 gRPC
通信。在选型过程中，字节跳动团队开发的 volo 框架不断出现在视野中，但最终没有选择它原因是：

1. 生态较弱，最近没太多时间踩坑
2. 我更倾向于将 protobuf 生成的代码作为项目文件使用，而不是在 service 层使用类似 `incldue_proto!("xxx.proto")` 的方式编译
   proto 文件
   ——至少在 Clion 上，它将使得 IDE 提示完全失效，开发体验较差。而 tonic-build 给出了指定输出文件的方法。

本质上，本分支所管理的 v3 版本也是单纯的 CRUD 版本。可看的是，在这一版本中，将引入一个利用用户端网络和 gRPC 双向流去访问校园网的方法。同时，
该版本的项目组织与代码结构也更正式，可供编写类似项目的同仁参考。

## 功能

- [x] 电费查询
- [x] 空教室查询
- [ ] 二手闲置交易
- [x] 入学信息查询
- [ ] 失物招领

本版本废弃了一部分更新频率较少的接口，这部分数据将直接以静态方式（[仓库](https://github.com/SIT-kite/kite-static)）存储，以便于
CDN 分发，同时节约成本、提高访问速度。

## 环境配置

待完善

## 有关项目

| 项目                                                         | 说明            |
|------------------------------------------------------------|---------------|
| [kite-app](https://github.com/SIT-kite/kite-app)           | App 前端        |
| [zf-tools](https://github.com/sunnysab/zf-tools)           | 正方教务系统爬虫      |
| [kite-string](https://github.com/SIT-Kite/kite-string)     | 校园网爬虫工具       |
| [kite-agent](https://github.com/sunnysab/kite-agent)       | 后端数据抓取工具（已废弃） |
| [kite-protocol](https://github.com/SIT-Kite/kite-protocol) | 通信协议库（已废弃）    |

## 如何贡献

算了，我都写麻了

## 开源协议

[GPLv3](https://github.com/SIT-Kite/kite-server/blob/master/LICENSE) © 上海应用技术大学易班 小风筝团队

除此之外，您不能将本程序用于各类竞赛、毕业设计、论文等。

