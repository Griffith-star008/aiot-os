# Architecture Decision Record: 0001-workspace

## Tiêu đề
Phân rã Monolith thành hệ thống Multi-crate Cargo Workspace.

## Trạng thái
**Accepted** (Chấp thuận)

## Bối cảnh
Trước đây, toàn bộ nền tảng Runtime AIOT được gom chung vào một file khổng lồ (`safe_aiot.rs`). Tuy nhiên, khi hệ thống phình to với các module nâng cao như Thermal, Safety, Scheduler, Cognitive, việc duy trì một monolithic file gây ra hiện tượng:
- Khó kiểm soát quyền truy cập bộ nhớ.
- Dễ bị Circular Dependency.
- Khó khăn trong việc build độc lập từng module.

## Quyết định Kiến trúc
- Chuyển dự án sang mô hình Cargo Workspace.
- Chia nhỏ hệ thống thành gần 40 crate độc lập (`core`, `runtime`, `thermal`, `safety`, `scheduler`, v.v.).
- Ràng buộc luồng Dependency 1 chiều (Directed Acyclic Graph - DAG). Không cho phép import ngược.
- Toàn bộ crate phải tuân thủ chuẩn API Module:
  - Chỉ expose `api.rs`.
  - Logic nội bộ giấu kín trong `internal.rs`.
- Ép linter gắt gao: `#![no_std]` và `#![deny(unsafe_code)]`.

## Hậu quả
- **Tích cực**: Thời gian compile nhanh chóng, code được cách ly tốt, dễ test độc lập, dễ dàng thay thế hot-swap các module như Plugin hoặc Scheduler.
- **Tiêu cực**: Việc quản lý Cargo.toml ở root có phần phức tạp hơn, số lượng file tăng lên đáng kể.
