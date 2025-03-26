# network_sniffer

## 项目结构

- app/capture.rs : 使用pnet捕获网络数据包
- app/parser.rs : 解析数据包, 提取源IP、目的IP、协议、源端口、目的端口、数据长度等信息
- app/analysis.rs : 分析数据包, 提供流量统计或异常检测
- app/models.rs : 定义其数据结构

- utils/logger.rs : 日志模块

- ui/ : 样式
