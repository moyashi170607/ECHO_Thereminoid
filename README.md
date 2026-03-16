# ECHO_Thereminoid

HC-SR04 超音波距離センサーで手との距離を計測し、その距離に応じた周波数をパッシブブザーから出力する theremin 風デバイスです。
CH32V003 マイコン上で動作する Rust 製の組み込みファームウェアです。

`lib.rs`は`embedded_hal`を用いて`ch32-hal`に依存せずコードを書いたため、`main.rs`を調整すれば、`ch32`系以外のチップでも用いることができるはずです。

## 動作原理

1. HC-SR04 で手とセンサーの距離を計測
2. 距離を周波数にマッピング
3. TIM1 の PWM でパッシブブザーを鳴らす

距離が有効範囲外のときはブザーをミュートします。

## ハードウェア

| 部品         | 型番                    |
| ------------ | ----------------------- |
| マイコン     | UIAPduino(CH32V003F4P6) |
| 距離センサー | HC-SR04                 |
| ブザー       | パッシブブザー          |

### ピン配線

| 機能         | ピン           |
| ------------ | -------------- |
| ブザー (PWM) | PC3 (TIM1 CH3) |
| HC-SR04 TRIG | PD0            |
| HC-SR04 ECHO | PC6            |

HC-SR04はGRDとVCCもそれぞれグランドと5V電源につなぐ必要があります。

## 環境構築

### 必要なツール

- [`minichlink`](https://github.com/cnlohr/ch32v003fun/tree/master/minichlink) — CH32V003 フラッシュツール
- `rust-objcopy` (`cargo install cargo-binutils` でインストール)

### セットアップ

```sh
# cargo-binutils のインストール（rust-objcopy を使うため）
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## ビルド & 書き込み

```sh
cargo run --release
```

`cargo run` を実行すると以下が自動的に行われます：

1. `riscv32ec-unknown-none-elf` ターゲット向けにビルド
2. `rust-objcopy` で `.bin` ファイルに変換
3. `minichlink` で CH32V003 に書き込み・リセット


## ライセンス

MIT OR Apache-2.0