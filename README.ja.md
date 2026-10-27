<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**ライセンス** このプロジェクトは、[MIT License](LICENSE)および[Creative Commons Attribution 4.0 International (CC-BY-4.0)](LICENSE-DOCS)のデュアルライセンスで提供されています。

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**商標** このプロジェクトには、プロジェクト、製品、サービスの商標やロゴが含まれる場合があります。Microsoftの商標やロゴを使用する場合は、[Microsoftの商標およびブランドに関するガイドライン](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general)に従ってください。このプロジェクトを変更したバージョンでMicrosoftの商標やロゴを使用する場合、混乱を招いたり、Microsoftが後援していると誤解させたりしてはなりません。第三者の商標やロゴの使用には、それぞれの権利者のポリシーが適用されます。

</div>

# Rustトレーニングブック

[English](README.md)

さまざまなプログラミング経験を持つ方に向けた7つのRustトレーニングコースに加え、非同期処理、高度なパターン、エンジニアリングの実践を深く掘り下げます。

本教材は、独自の内容に加え、Rustエコシステムの優れたリソースから着想を得たアイデアや例を組み合わせています。書籍、ブログ、カンファレンストーク、動画シリーズに散在する知識を、学習しやすい一貫したカリキュラムとしてまとめ、技術的に正確で詳しい教材を提供することを目指しています。

> **免責事項：** 本書は学習用教材であり、規範的なリファレンスではありません。正確性の確保に努めていますが、重要な内容は必ず[公式Rustドキュメント](https://doc.rust-lang.org/)および[Rust Reference](https://doc.rust-lang.org/reference/)で確認してください。

### 参考資料と謝辞

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) — 本教材の基礎となる書籍
- [**Jon Gjengset**](https://www.youtube.com/c/JonGjengset) — Rust内部の高度な解説配信、`Crust of Rust`シリーズ
- [**withoutboats**](https://without.boats/blog/) — asyncの設計、`Pin`、futuresモデル
- [**fasterthanlime (Amos)**](https://fasterthanli.me/) — 原理から学ぶシステムプログラミングと、読み応えのある解説
- [**Mara Bos**](https://marabos.nl/) — *Rust Atomics and Locks*、並行処理プリミティブ
- [**Aleksey Kladov (matklad)**](https://matklad.github.io/) — Rust Analyzer、API設計、エラー処理のパターン
- [**Niko Matsakis**](https://smallcultfollowing.com/babysteps/) — 言語設計、借用チェッカーの内部、Polonius
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/)と[**Rustonomicon**](https://doc.rust-lang.org/nomicon/) — 実践的なパターンとunsafeの詳解
- [**This Week in Rust**](https://this-week-in-rust.org/) — 多くの例に反映されたコミュニティの発見
- [**Binary Musings - Tag(Rust)**](https://binarymusings.org/posts/category/rust/) — Rust内部の詳しい解説
- …そして、ブログ記事、カンファレンストーク、RFC、フォーラムでの議論を通じて本教材に知見を寄せてくださったRustコミュニティの皆さまに感謝します。個別に挙げきれないほど多くの方々に、心よりお礼申し上げます。

## 📖 読み始める

ご自身の経験に合った本を選んでください。学習の道筋を立てやすいよう、難易度別にまとめています。

| レベル | 説明 |
|-------|------|
| 🟢 **Bridge（入門）** | 他の言語からRustを学ぶ方はこちら |
| 🔵 **Deep Dive（詳解）** | Rustの主要なサブシステムを深く学ぶ |
| 🟡 **Advanced（応用）** | Rustに慣れた方向けのパターンとテクニック |
| 🟣 **Expert（上級）** | 最先端の型レベル技法と正しさの保証 |
| 🟤 **Practices（実践）** | エンジニアリング、ツール、本番環境への対応 |

| 書籍 | レベル | 対象・内容 |
|------|-------|-------------|
| [**C/C++プログラマーのためのRust**](https://microsoft.github.io/RustTraining/c-cpp-book/) | 🟢 Bridge | ムーブセマンティクス、RAII、FFI、組み込み、no_std |
| [**C#プログラマーのためのRust**](https://microsoft.github.io/RustTraining/csharp-book/) | 🟢 Bridge | Swift / C# / Javaから、所有権と型システムへ |
| [**PythonプログラマーのためのRust**](https://microsoft.github.io/RustTraining/python-book/) | 🟢 Bridge | 動的型付けから静的型付けへ、GILに依存しない並行処理 |
| [**Async Rust**](https://microsoft.github.io/RustTraining/async-book/) | 🔵 Deep Dive | Tokio、ストリーム、キャンセル安全性 |
| [**Rust Patterns**](https://microsoft.github.io/RustTraining/rust-patterns-book/) | 🟡 Advanced | Pin、アロケーター、ロックフリー構造、unsafe |
| [**Type-Driven Correctness**](https://microsoft.github.io/RustTraining/type-driven-correctness-book/) | 🟣 Expert | 型状態、ファントム型、ケイパビリティトークン |
| [**Rust Engineering Practices**](https://microsoft.github.io/RustTraining/engineering-book/) | 🟤 Practices | ビルドスクリプト、クロスコンパイル、CI/CD、Miri |

各書籍には15〜16章があり、Mermaid図、編集可能なRust Playground、演習、全文検索を利用できます。

> **ヒント：** サイドバーのナビゲーションや検索を使って、[GitHub Pagesサイト](https://microsoft.github.io/RustTraining/)で書籍を閲覧できます。
>
> **ローカルでプレビュー：** オフラインで読んだり、コントリビュートしたりする場合は、先に[Rustをインストール](https://rustup.rs/)してください。
> ```
> git clone https://github.com/microsoft/RustTraining.git
> cd RustTraining
> cargo install mdbook mdbook-mermaid
> cargo xtask serve    # http://localhost:3000
> ```

---

## 🔧 メンテナー向け

<details>
<summary>書籍のローカルでのビルド、配信、編集</summary>

### 前提条件

まだの場合は、[**rustup**でRustをインストール](https://rustup.rs/)してから、次を実行します。

```bash
cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
```

### リポジトリをクローン

```bash
git clone https://github.com/microsoft/RustTraining.git
cd RustTraining
```

### ビルドと配信

```bash
cargo xtask build               # すべての書籍をsite/にビルド（ローカルプレビュー用）
cargo xtask serve               # ビルドしてhttp://localhost:3000で配信
cargo xtask deploy              # GitHub Pages用にすべての書籍をdocs/にビルド
cargo xtask clean               # site/とdocs/を削除
```

個別の書籍をビルドまたは配信するには、次のようにします。

```bash
cd c-cpp-book && mdbook serve --open    # http://localhost:3000
```

### デプロイ

`main`へのpush時に、`.github/workflows/pages.yml`を通じてGitHub Pagesへ自動デプロイされます。手動での作業は不要です。

</details>
