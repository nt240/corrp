# corrp

**corrp** は Rust で作られた、**相関係数 r から両側検定の p 値を計算する CLI ツール**です。
Linux x86_64 (musl) 静的リンクバイナリを提供しており、Rust 環境がなくてもすぐに使えます。

---

## ⚡ 特徴

* 両側検定のみ対応
* `--r` と `--n` で指定
* Linux 静的リンクバイナリ
* Rust 環境不要

---

## 📥 インストール

### 1. GitHub Releases からダウンロード

```bash
wget https://github.com/nt240/corrp/releases/latest/download/corrp
```

### 2. 実行権限を付与

```bash
chmod +x corrp
```

### 3. 実行

```bash
./corrp --r -0.5 --n 30
```

出力例:

```
t=-3.0550504633038935, p=0.0048999336670687565
```

---

## 🚀 使い方

```bash
./corrp --r <相関係数 r> --n <サンプルサイズ n>
```

例:

```bash
./corrp --r 0.5 --n 30
# 出力例
# t=-3.0550504633038935, p=0.0048999336670687565
```

* `r` は -1.0 から 1.0 の値
* `n` はサンプルサイズ（整数 ≥ 3）

---

## 🔧 ビルド方法（Rust 環境がある場合）

```bash
git clone https://github.com/nt240/corrp.git
cd corrp
cargo build --release --target x86_64-unknown-linux-musl
```

生成されるバイナリ:

```
target/x86_64-unknown-linux-musl/release/corrp
```

---

## 📌 ライセンス

MIT License
