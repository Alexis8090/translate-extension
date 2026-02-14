axum

只用get和post接口 网关post 数据更改日志

https://mp.weixin.qq.com/s?__biz=Mzk0MDI3MTE0MQ==&mid=2247500421&idx=2&sn=fee7956373b015a0512b1ec2d1b928c8&chksm=c3d0961cfb94b5612c7059f6f4e0edb3c64d341ad1bd09854133953d2b7182d2ccb508b9a4ea&scene=27

1. 必须上异步
1. 利用cookie？登陆状态
2. 有日志吗？
3. 调用第三方 3接口 超时处理
4. 自己错误返回500
5. 内置业务缓存
6. nodejs 和 rust ipc交互
7. sqlite ssd  diesel（ 不是async哈） > rusqlte https://github.com/diesel-rs/metrics/
8. postgreSQL 连接池/ORM wtx  > diesel-async
8. flash_log, 保留日志


# 架构
3. 数据库
    1. pg
    2. pgcat
    3. ReadySet sql性能优化 看能不能（放在pgcat和pg从库 前面使用）
    3. 迁移
    3. pg分区大全 pgcat pg从库 https://github.com/pgpartman/pg_partman
    4. graphql-pg
    6. pgpg迁移工具 pg备份工具https://github.com/pgmoneta/pgmoneta
    7. pg实时计算risingware && ParadeDB
    8. https://github.com/le0pard/pgtune设置参数 pg调优
    11. https://github.com/Vonng/pg
5. 脚本 异步 bun，兼容性不行上node



1.done 写复杂一点的增删改查接口，带有分页 带有请求头
2. 各种的服务里的中间件处理
    1. done 超时处理 超时就放弃 具体按照nodejs,
    2. done 统一请求错误 打印格式,可以用ntex 框架的data模式但是是字符串格式
    4. done 1. 跨域（nginx上，pingora）2.前端调试自己设置跨域
    5. 网关上做日志，done 日志系统  日志自动删除 压缩 保留2月 fast_log 只能在服务做，不能在js，切压缩~（等rust出了好的日志库再换）,然后每7天 本地机子 取前3周～前2周 的zip 7天，上面至少保留2周
    5. done 我们服务是不能被外网调的所以上线的服务只能用127.0.0.1，而不是0.0.0.0
    7. 就是优雅，无停机启动程序，只能根据网关负载均衡upstream 来处理 准备4000 4001来启动， nodejs，ssh2分两步一步是先上传成功，第二步是启动
    3. 终端命令行生成模版,低代码做准备 增删改查
    5. 邮件 stalwartlabs  用mail-send， 但是要启动gmail 和 飞书，或者tg
    6. 先了解 captchap 他们的专业术语 拼图滑动位置在后台验证

# pg
3. pg分区大全 pgcat pg从库 https://github.com/pgpartman/pg_partman
3. ReadySet sql性能优化 看能不能（放在pgcat和pg从库 前面使用）
4. graphql-pg
5. pgcat
6. pgpg迁移工具 pg备份工具https://github.com/pgmoneta/pgmoneta
7. pg实时计算risingware && ParadeDB
8. https://github.com/le0pard/pgtune设置参数 pg调优
11. https://github.com/Vonng/pg

# graphql
seaography // axum可以


# pingora
自我写配置


# 权限管理




4.  [option] rnacos
10. [option] timescaledb


1. pg的坑，没有无符号，全部有符号，分布式项目id必须是uuid或者其他的，其他的用bigserial 但是很坑i64 相当于u32

wrk -t12 -c200 -d30s http://localhost:80989/dmail/user/





axum 进来数据验证流程
 数据从tcp 二进制 报文进来
 进来axum
 先native serde序列化 query 失败直接放给前端了（目前我们不能控制，axum库决定）
 再到axum parse
 返回给自定义httpError错误


服务器内在错误
500并给出msg但是在前端渲染

200说明请求成功
逻辑成功的格式：直接json化的数据
逻辑错误的格式：直接text错误信息

判断状态码200


JSON 序列化 (JSON Serialization): 测试框架的基础功能，包括 keep-alive 支持、请求路由、请求头解析、对象实例化、JSON 序列化、响应头生成以及请求吞吐量。
* json返回

单数据库查询 (Single Database Query): 测试框架的对象关系映射器 (ORM)、随机数生成器、数据库驱动以及数据库连接池。
* 单一请求+json返回

多数据库查询 (Multiple Database Queries): 这是测试 #2 的一个变种，并且也使用 World 表。它通过获取多行数据来更显著地对数据库驱动和连接池施加压力。在测试的最高每请求查询次数（20次）时，该测试表明，随着数据库活动的增加，所有框架的每秒请求数都趋向于零。
* 请求并发promise请求+json返回


Fortunes (运势签): 测试 ORM、数据库连接性、动态大小的集合、排序、服务器端模板、XSS (跨站脚本攻击) 防护措施以及字符编码。
* 模版整个ssr服务

数据库更新 (Database Updates): 这是测试 #3 的一个变种，测试 ORM 的对象持久化能力以及数据库驱动在执行 UPDATE 语句或类似操作时的性能。该测试的核心目的是测试可变数量的“先读后写”式数据库操作。
* 请求并发promise批量更新（不是sql并发）

纯文本 (Plaintext): 仅测试请求路由的基础功能，旨在特别展示高性能平台的能力。请求将使用 HTTP 流水线发送。响应负载仍然很小，这意味着仍然需要良好的性能才能使测试环境的千兆以太。
* 返回纯文本


缓存 (Caching): 测试平台或框架对来自数据库的信息进行内存缓存的能力。为了简化实现，其要求与多数据库查询测试（测试 #3）非常相似，但相当宽松/容错，允许应用各个平台或框架的最佳实践。
* 本地缓存map

# 上ntex


# 开发日志
## log
选用tracing tokio生态

## tokio-console
tokio的 task 监听 类似 htop


## 本地缓存
Lcu_cache

## restful
后端
200业务请求成功，返回如果有值必须是满足json
400业务逻辑错误+请求参数错误，有返回text
500服务内部错误
前端
200（完全成功json） 400（text） 500（text） 去判断

## 请求别人的sdk


## 超时和重试


## 发邮件


## ipc

##

还傻乎乎 计算机啊叫

SQLITE_LIMIT_VARIABLE_NUMBER

一般的vps
sqlite oha
query 43K/s
插入 2.6K 插入并返回2.3k
批量插入 1.2k 量越多越好

root@localhost:~/code/rust-axum-server/target/release# time curl  'http://l
ocalhost:8089/dmail/user/q?name=2---&ps=10&id=354757&age=4'
[{"id":354758,"created_at":"2025-05-14T18:01:52Z","name":"2---","age":4},{"id":354760,"created_at":"2025-05-14T18:01:52Z","name":"2---","age":4}]
real    0m0.009s
user    0m0.003s
sys     0m0.006s



总结： http的一个服务，不是单单看框架的



