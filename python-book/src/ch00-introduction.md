# PythonプログラマーのためのRust：総合トレーニングガイド

Pythonの経験がある開発者向けに、Rustの基礎から高度なパターンまでを網羅する総合ガイドです。
動的型付けとガベージコレクションを備えた言語から、コンパイル時のメモリ安全性を持つ
静的型付けのシステム言語へ移る際に必要となる、考え方の変化を中心に解説します。

## 本書の使い方

**独学向けの構成：** まず第I部（第1〜6章）を学びましょう。ここでは、すでに知っているPythonの概念と対応づけながら説明します。第II部（第7〜12章）では、所有権やトレイトなどRust固有の考え方を紹介します。第III部（第13〜16章）では、発展的なトピックと移行について扱います。

**学習ペースの目安：**

| 章 | トピック | 所要時間の目安 | 到達目標 |
|----------|-------|---------------|------------|
| 1–4 | 環境構築、型、制御フロー | 1日 | RustでCLIの温度変換ツールを書ける |
| 5–6 | データ構造、列挙型、パターンマッチング | 1〜2日 | データを持つ列挙型を定義し、`match`ですべてのケースを網羅できる |
| 7 | 所有権と借用 | 1〜2日 | `let s2 = s1`でなぜ`s1`が使えなくなるのか説明できる |
| 8–9 | モジュール、エラー処理 | 1日 | 複数ファイルのプロジェクトを作り、`?`でエラーを伝播できる |
| 10–12 | トレイト、ジェネリクス、クロージャ、イテレータ | 1〜2日 | リスト内包表記をイテレータチェーンに書き換えられる |
| 13 | 並行処理 | 1日 | `Arc<Mutex<T>>`を使ってスレッドセーフなカウンターを書ける |
| 14 | unsafe、PyO3、テスト | 1日 | PyO3を介してPythonからRust関数を呼び出せる |
| 15–16 | 移行、ベストプラクティス | 自分のペースで | 実際のコードを書く際に参照資料として活用できる |
| 17 | 総合プロジェクト | 2〜3日 | 学んだ内容を統合したCLIアプリを作れる |

**演習の進め方：**
- 各章には、解答を折りたたみ表示できる`<details>`ブロック形式の実践演習があります。
- **解答を開く前に、必ず自分で演習に取り組んでください。** 借用チェッカーに苦戦することも学習の一部です。コンパイラーのエラーメッセージが助けになります。
- 15分以上行き詰まったら、解答を開いて学び、いったん閉じてから最初からもう一度挑戦しましょう。
- [Rust Playground](https://play.rust-lang.org/)を使えば、ローカルにインストールしなくてもコードを実行できます。

**難易度の目安：**
- 🟢 **初級** — Pythonの概念から直接対応づけて学べます
- 🟡 **中級** — 所有権やトレイトの理解が必要です
- 🔴 **上級** — ライフタイム、asyncの内部、unsafeコードを扱います

**行き詰まったときは：**
- コンパイラーのエラーメッセージを注意深く読みましょう。Rustのエラー表示はとても役立ちます。
- 該当する節を読み直してみましょう。所有権（第7章）のような概念は、2回目に理解できることもよくあります。
- [Rust標準ライブラリのドキュメント](https://doc.rust-lang.org/std/)は充実しています。型やメソッドを検索してみてください。
- asyncのパターンをさらに深く学ぶには、関連教材の[Async Rust Training](../async-book/)を参照してください。

---

## 目次

### 第I部 — 基礎

#### 1. はじめに：Rustを学ぶ理由 🟢
- [Python開発者にとってのRust](ch01-introduction-and-motivation.md#the-case-for-rust-for-python-developers)
- [Rustが解決するPythonのよくある課題](ch01-introduction-and-motivation.md#common-python-pain-points-that-rust-addresses)
- [PythonよりRustを選ぶとき](ch01-introduction-and-motivation.md#when-to-choose-rust-over-python)

#### 2. 開発環境の準備 🟢
- [インストールと環境設定](ch02-getting-started.md#installation-and-setup)
- [最初のRustプログラム](ch02-getting-started.md#your-first-rust-program)
- [Cargoとpip/Poetryの比較](ch02-getting-started.md#cargo-vs-pippoetry)

#### 3. 組み込み型と変数 🟢
- [変数と可変性](ch03-built-in-types-and-variables.md#variables-and-mutability)
- [プリミティブ型の比較](ch03-built-in-types-and-variables.md#primitive-types-comparison)
- [文字列型：`String`と`&str`](ch03-built-in-types-and-variables.md#string-types-string-vs-str)

#### 4. 制御フロー 🟢
- [条件分岐](ch04-control-flow.md#conditional-statements)
- [ループと反復処理](ch04-control-flow.md#loops-and-iteration)
- [式ブロック](ch04-control-flow.md#expression-blocks)
- [関数と型シグネチャ](ch04-control-flow.md#functions-and-type-signatures)

#### 5. データ構造とコレクション 🟢
- [タプル、配列、スライス](ch05-data-structures-and-collections.md#tuples-and-destructuring)
- [構造体とクラス](ch05-data-structures-and-collections.md#structs-vs-classes)
- [`Vec`と`list`、`HashMap`と`dict`](ch05-data-structures-and-collections.md#vec-vs-list)

#### 6. 列挙型とパターンマッチング 🟡
- [代数的データ型とユニオン型](ch06-enums-and-pattern-matching.md#algebraic-data-types-vs-union-types)
- [網羅的なパターンマッチング](ch06-enums-and-pattern-matching.md#exhaustive-pattern-matching)
- [`Option`による`None`の安全な扱い](ch06-enums-and-pattern-matching.md#option-for-none-safety)

### 第II部 — 中核となる概念

#### 7. 所有権と借用 🟡
- [所有権を理解する](ch07-ownership-and-borrowing.md#understanding-ownership)
- [ムーブセマンティクスと参照カウント](ch07-ownership-and-borrowing.md#move-semantics-vs-reference-counting)
- [借用とライフタイム](ch07-ownership-and-borrowing.md#borrowing-and-lifetimes)
- [スマートポインター](ch07-ownership-and-borrowing.md#smart-pointers)

#### 8. クレートとモジュール 🟢
- [RustのモジュールとPythonのパッケージ](ch08-crates-and-modules.md#rust-modules-vs-python-packages)
- [クレートとPyPIパッケージ](ch08-crates-and-modules.md#crates-vs-pypi-packages)

#### 9. エラー処理 🟡
- [例外と`Result`](ch09-error-handling.md#exceptions-vs-result)
- [`?`演算子](ch09-error-handling.md#the--operator)
- [thiserrorによる独自エラー型](ch09-error-handling.md#custom-error-types-with-thiserror)

#### 10. トレイトとジェネリクス 🟡
- [トレイトとダックタイピング](ch10-traits-and-generics.md#traits-vs-duck-typing)
- [プロトコル（PEP 544）とトレイト](ch10-traits-and-generics.md#protocols-pep-544-vs-traits)
- [ジェネリクスの制約](ch10-traits-and-generics.md#generic-constraints)

#### 11. FromトレイトとIntoトレイト 🟡
- [Rustの型変換](ch11-from-and-into-traits.md#type-conversions-in-rust)
- [From、Into、TryFrom](ch11-from-and-into-traits.md#rust-frominto)
- [文字列変換のパターン](ch11-from-and-into-traits.md#string-conversions)

#### 12. クロージャとイテレータ 🟡
- [クロージャとラムダ式](ch12-closures-and-iterators.md#rust-closures-vs-python-lambdas)
- [イテレータとジェネレーター](ch12-closures-and-iterators.md#iterators-vs-generators)
- [マクロ：コードを生成するコード](ch12-closures-and-iterators.md#why-macros-exist-in-rust)

### 第III部 — 発展的なトピックと移行

#### 13. 並行処理 🔴
- [GILなしで実現する真の並列処理](ch13-concurrency.md#no-gil-true-parallelism)
- [スレッド安全性を保証する型システム](ch13-concurrency.md#thread-safety-type-system-guarantees)
- [async/awaitの比較](ch13-concurrency.md#asyncawait-comparison)

#### 14. unsafe Rust、FFI、テスト 🔴
- [unsafeを使う場面と理由](ch14-unsafe-rust-and-ffi.md#when-and-why-to-use-unsafe)
- [PyO3：Python向けRust拡張](ch14-unsafe-rust-and-ffi.md#pyo3-rust-extensions-for-python)
- [ユニットテストとpytest](ch14-unsafe-rust-and-ffi.md#unit-tests-vs-pytest)

#### 15. 移行パターン 🟡
- [RustでのPythonの一般的なパターン](ch15-migration-patterns.md#common-python-patterns-in-rust)
- [Python開発者に役立つクレート](ch08-crates-and-modules.md#essential-crates-for-python-developers)
- [段階的な導入戦略](ch15-migration-patterns.md#incremental-adoption-strategy)

#### 16. ベストプラクティス 🟡
- [Python開発者向けの慣用的なRust](ch16-best-practices.md#idiomatic-rust-for-python-developers)
- [よくある落とし穴と解決策](ch16-best-practices.md#common-pitfalls-and-solutions)
- [Python→Rust対応表](ch16-best-practices.md#rosetta-stone-python-to-rust)
- [学習の道筋と参考資料](ch16-best-practices.md#learning-path-and-resources)

---

### 第IV部 — 総合プロジェクト

#### 17. 総合プロジェクト：CLIタスク管理ツール 🔴
- [プロジェクト：`rustdo`](ch17-capstone-project.md#the-project-rustdo)
- [データモデル、ストレージ、コマンド、ビジネスロジック](ch17-capstone-project.md#step-1-define-the-data-model-ch-3-6-10-11)
- [テストと発展課題](ch17-capstone-project.md#step-7-tests-ch-14)

***
