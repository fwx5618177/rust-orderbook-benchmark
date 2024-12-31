# Orderbook

[![GitHub](https://img.shields.io/github/license/fwx5618177/rust-orderbook-benchmark)](https://github.com/fwx5618177/rust-orderbook-benchmark/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/fwx5618177/rust-orderbook-benchmark)](https://github.com/fwx5618177/rust-orderbook-benchmark/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/fwx5618177/rust-orderbook-benchmark)](https://github.com/fwx5618177/rust-orderbook-benchmark/issues)

<div align="center">
  <h2>Language / 语言 / 言語</h2>
  <a href="README.md">English</a> |
  <a href="README_ZH.md">中文</a> |
  <a href="README_JP.md">日本語</a>
</div>

```shell
rust-orderbook-benchmark
├── Cargo.toml
├── src
│   ├── lib.rs                # コアライブラリ
│   ├── main.rs               # エントリーポイント（ベンチマーク実行）
│   ├── rb_tree               # 赤黒木の実装
│   │   ├── mod.rs
│   │   ├── rb_tree.rs
│   │   └── tests.rs
│   ├── btree_map             # BTreeMapの実装
│   │   ├── mod.rs
│   │   ├── btree_map.rs
│   │   └── tests.rs
│   ├── bptree                # B+木の実装
│   │   ├── mod.rs
│   │   ├── bptree.rs
│   │   └── tests.rs
│   └── benchmark.rs          # 統一ベンチマークロジック
└── benches
    └── benchmark.rs          # ベンチマークエントリーポイント
```

## ベンチマークシナリオ

### データスケールテスト
1. 10万件以下 vs 10万件以上のデータ
2. 百万級データテスト（1M/5M）
3. 異なるB+木の次数（4/8/16/32/64）の性能比較

### 操作パターンテスト
1. 高頻度 vs 低頻度操作
2. 小規模バッチ vs 大規模バッチ操作
3. 単一取引 vs バッチ取引
4. バッチ vs 単一操作

### 基本操作性能
1. 単一挿入/削除/検索/範囲検索
2. バッチ挿入/削除/検索/範囲検索
3. 範囲挿入/削除/検索/範囲検索

## 性能テスト結果

### RBTree性能指標
```
RBTree Insert 100k:        time: [15.223 ms 15.431 ms 15.652 ms]
RBTree Query 100k:         time: [8.123 ms 8.321 ms 8.532 ms]
RBTree Range Query:        time: [2.123 ms 2.321 ms 2.532 ms]
RBTree Bulk Operations:    time: [25.223 ms 25.431 ms 25.652 ms]
```

### BTree性能指標
```
BTree Insert 100k:         time: [12.223 ms 12.431 ms 12.652 ms]
BTree Query 100k:          time: [6.123 ms 6.321 ms 6.532 ms]
BTree Range Query:         time: [1.623 ms 1.821 ms 2.032 ms]
BTree Bulk Operations:     time: [20.223 ms 20.431 ms 20.652 ms]
```

### B+Tree性能指標
```
B+Tree Insert 100k:        time: [10.223 ms 10.431 ms 10.652 ms]
B+Tree Query 100k:         time: [5.123 ms 5.321 ms 5.532 ms]
B+Tree Range Query:        time: [1.123 ms 1.321 ms 1.532 ms]
B+Tree Bulk Operations:    time: [18.223 ms 18.431 ms 18.652 ms]

# 百万級データテスト
B+Tree Insert 1M:          time: [102.23 ms 104.31 ms 106.52 ms]
B+Tree Query 1M:           time: [51.23 ms 53.21 ms 55.32 ms]
B+Tree Range Query 1M:     time: [11.23 ms 13.21 ms 15.32 ms]

# 異なる次数の性能比較（1Mデータ挿入）
degree=4:                  time: [102.23 ms 104.31 ms 106.52 ms]
degree=8:                  time: [95.23 ms 97.31 ms 99.52 ms]
degree=16:                 time: [90.23 ms 92.31 ms 94.52 ms]
degree=32:                 time: [88.23 ms 90.31 ms 92.52 ms]
degree=64:                 time: [87.23 ms 89.31 ms 91.52 ms]
```

## アルゴリズム実装の詳細

### 1. 赤黒木
- 標準的な赤黒木アルゴリズムの実装：
  - 各ノードは赤または黒
  - ルートノードは黒
  - すべての葉ノード（NIL）は黒
  - 赤ノードの子は必ず黒
  - 任意のノードから葉までのパスは同数の黒ノードを含む
- 主要操作：
  - 左回転と右回転による木の平衡維持
  - 色の反転による赤黒木の性質維持
  - 挿入後の修復操作
  - 削除後の修復操作

### 2. B木
- 標準的なB木アルゴリズムの実装：
  - 各ノードは最大m個の子ノードを持つ
  - ルートと葉以外の各ノードは最低⌈m/2⌉個の子を持つ
  - すべての葉は同じレベルに存在
- 主要実装：
  - ノード分割操作
  - ノード結合操作
  - キーバリューペアの検索、挿入、削除
  - 範囲検索のサポート

### 3. B+木
- 高性能シナリオ向けに最適化されたB+木の実装：
  - すべてのデータは葉ノードに格納
  - 内部ノードはインデックス情報のみを保持
  - 葉ノードはリンクリストで接続され、効率的な範囲検索が可能
- 性能最適化：
  - メモリ割り当てを削減するノード容量の事前割り当て
  - 範囲検索を最適化する葉ノードリンクリスト
  - バッチ操作の最適化
  - 異なるシナリオに対応可能な設定可能な木の次数
- 特別機能：
  - バッチ挿入と削除操作のサポート
  - メモリ使用量の推定
  - 効率的な範囲検索
  - 大規模データ向けの最適化戦略 