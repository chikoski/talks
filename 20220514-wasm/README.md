---
marp: true
title: WebAssembly 最近のトレンド
theme: gaia
paginate: true
headingDivider: 1
---

# WebAssembly 最近のトレンド

@chikoski

2022/05/14 TechFeed Conference 2022

<!--
_class: lead
_paginate: false
-->

# Wasm: 柔軟性と安全性

- 柔軟性
  - プログラミング言語やツールを選べる
  - 差し替えが（比較的）容易
- 安全性
  - 実行コードの生成
  - サンドボックス化した実行環境

![bg right:30%](./assets/jana-muller-02SDUKEZMzU-unsplash.jpg)
![bg](./assets/ostap-senyuk-7C8c-7fwk34-unsplash.jpg)

<!--
_footer: Photo by [jana müller]("https://unsplash.com/@janamxe?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) and [Ostap Senyuk](https://unsplash.com/@kintecus?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText) on [Unsplash](https://unsplash.com/s/photos/flexible?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText)
-->



# Amazon Prime Video  

- ドングルやSTB向けアプリ
  - UI は JS で実現
  - 下回りは Wasm で実現
- 柔軟性
  - プロダクトの更新
  - 開発環境での差し替え
  - 開発言語の選択

![bg right:35% fit](./assets/amazom-prime-video.svg)

<!--
_footer: cited from [How Prime Video updates its app for more than 8,000 device types](https://www.amazon.science/blog/how-prime-video-updates-its-app-for-more-than-8-000-device-types)
-->


# Features

- MVP
- Finished proposals
- In-progress proposals
  - Relaxed SIMD
  - ESM integration
  - Custome annotations

![bg right:50% contain](./assets/compatiblity-table.png)

# インタプリター

- 文脈
  - (Cold) start time
  - セキュリティ
  - 組み込み
- 事例
  - JSC
  - [DrumBrake](https://docs.google.com/document/d/1OIJ4Sv2XfTlI5NmTS1QI8v8wPL0LUT5s1W2D9OlJmMc/preview#)
  - [wasm3](https://github.com/wasm3/wasm3)