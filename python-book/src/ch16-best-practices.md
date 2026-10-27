<a id="idiomatic-rust-for-python-developers"></a>
## Python開発者のための慣用的なRust

> **学習内容：** 身につけたい10の習慣、よくある落とし穴と解決策、体系的な3か月の学習計画、
> PythonからRustへの対応をまとめた「ロゼッタストーン」一覧、おすすめの学習リソース。
>
> **難易度：** 🟡 中級

```mermaid
flowchart LR
    A["🟢 1～2週目<br/>基礎<br/>「なぜコンパイルできない？」"] --> B["🟡 3～4週目<br/>主要な概念<br/>「なるほど、守ってくれているんだ」"]
    B --> C["🟡 2か月目<br/>中級<br/>「大切な理由が分かってきた」"]
    C --> D["🔴 3か月目以降<br/>上級<br/>「コンパイル時にバグを発見！」"]
    D --> E["🏆 6か月目<br/>習熟<br/>「どの言語でもより良いプログラマーに」"]
    style A fill:#d4edda
    style B fill:#fff3cd
    style C fill:#fff3cd
    style D fill:#f8d7da
    style E fill:#c3e6cb,stroke:#28a745
```

### 身につけたい10の習慣

1. **`if isinstance()`ではなくenumに対する`match`を使う**
   ```python
   # Python                              # Rust
   if isinstance(shape, Circle): ...     match shape { Shape::Circle(r) => ... }
   ```

2. **コンパイラを頼りにする** — エラーメッセージを注意深く読みましょう。Rustの
   コンパイラは、どこが間違っているかだけでなく、修正方法も教えてくれます。

3. **関数パラメーターでは`String`より`&str`を優先する** — より汎用的な型を受け取ります。
   `&str`は`String`にも文字列リテラルにも使えます。

4. **インデックスループではなくイテレーターを使う** — イテレーターチェーンはより慣用的で、
   多くの場合`for i in 0..vec.len()`より高速です。

5. **`Option`と`Result`を活用する** — 何でも`.unwrap()`しないでください。`?`、
   `map`、`and_then`、`unwrap_or_else`を使いましょう。

6. **積極的にトレイトをderiveする** — 多くの構造体に`#[derive(Debug, Clone, PartialEq)]`を
   付けましょう。追加コストはなく、テストも容易になります。

7. **必ず`cargo clippy`を使う** — スタイルや正しさに関する多くの問題を検出します。
   Rust版の`ruff`のように活用しましょう。

8. **借用チェッカーに逆らわない** — 手こずっているなら、データ構造の設計に問題があるかもしれません。
   所有権が明確になるようにリファクタリングしましょう。

9. **状態機械にはenumを使う** — 文字列フラグや真偽値ではなくenumを使いましょう。
   コンパイラがすべての状態を処理しているか確認してくれます。

10. **まずcloneし、最適化は後で行う** — 学習中は所有権の複雑さを避けるために`.clone()`を
    気軽に使いましょう。プロファイリングで必要性が確認された場合にのみ最適化します。

### Python開発者によくある間違い

| 間違い | 理由 | 修正方法 |
|---------|-----|-----|
| どこでも`.unwrap()`を使う | 実行時にパニックする | `?`または`match`を使う |
| &strではなくStringを使う | 不要なメモリ確保 | パラメーターには`&str`を使う |
| `for i in 0..vec.len()` | 慣用的でない | `for item in &vec` |
| clippyの警告を無視する | 簡単な改善を見逃す | `cargo clippy`を使う |
| `.clone()`を多用する | 性能上のオーバーヘッド | 所有権の設計を見直す |
| 巨大なmain()関数 | テストしにくい | lib.rsに切り出す |
| `#[derive()]`を使わない | 既存機能を再実装してしまう | よく使うトレイトをderiveする |
| エラーでパニックする | 復旧できない | `Result<T, E>`を返す |

***

## 性能比較

### ベンチマーク：一般的な処理
```text
処理                   Python 3.12    Rust（release）    高速化率
─────────────────────  ────────────   ──────────────    ─────────
フィボナッチ(40)       ~25s           ~0.3s             ~80倍
整数1000万個のソート   ~5.2s          ~0.6s             ~9倍
JSON解析（100MB）      ~8.5s          ~0.4s             ~21倍
正規表現（100万件）    ~3.1s          ~0.3s             ~10倍
HTTPサーバー（req/s）  ~5,000         ~150,000          ~30倍
SHA-256（1GBファイル） ~12s           ~1.2s             ~10倍
CSV解析（100万行）     ~4.5s          ~0.2s             ~22倍
文字列連結             ~2.1s          ~0.05s            ~42倍
```

> **注：** C拡張機能（NumPyなど）を使ったPythonでは、数値処理における差は大幅に縮まります。
> ここでのベンチマークは、純粋なPythonと純粋なRustを比較しています。

### メモリ使用量
```text
Python：                                Rust：
─────────                               ─────
- オブジェクトヘッダー：1オブジェクトあたり28バイト - オブジェクトヘッダーなし
- int：0でも28バイト                    - i32：4バイト、i64：8バイト
- str "hello"：54バイト                - &str "hello"：16バイト（ポインター＋長さ）
- intが1000個のlist：約36 KB            - Vec<i32>：約4 KB
  （ポインター8 KB + intオブジェクト28 KB）
- 要素100個のdict：約5.5 KB             - 要素100個のHashMap：約2.4 KB

一般的なアプリケーションの合計：
- Python：ベースラインで50～200 MB       - Rust：ベースラインで1～5 MB
```

***

<a id="common-pitfalls-and-solutions"></a>
## よくある落とし穴と解決策

### 落とし穴1：「借用チェッカーが許してくれない」
```rust
// 問題：反復処理と変更を同時に行おうとしている
let mut items = vec![1, 2, 3, 4, 5];
// for item in &items {
//     if *item > 3 { items.push(*item * 2); }  // ❌ 借用中に可変借用できない
// }

// 解決策1：変更内容を集めてから適用する
let additions: Vec<i32> = items.iter()
    .filter(|&&x| x > 3)
    .map(|&x| x * 2)
    .collect();
items.extend(additions);

// 解決策2：retain/extendを使う
items.retain(|&x| x <= 3);
```

### 落とし穴2：「文字列型が多すぎる」
```rust
// 迷ったとき：
// - 関数パラメーターには&str
// - 構造体のフィールドと戻り値にはString
// - &strが必要な場所では、&strリテラル（"hello"）も使える

fn process(input: &str) -> String {    // &strを受け取り、Stringを返す
    format!("Processed: {}", input)
}
```

### 落とし穴3：「Pythonの簡潔さが恋しい」
```rust
// Pythonでの1行記述：
// result = [x**2 for x in data if x > 0]

// Rustでの同等の記述：
let result: Vec<i32> = data.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * x)
    .collect();

// より冗長だが、次の利点がある：
// - コンパイル時に型安全性を確保
// - 10～100倍高速
// - 実行時の型エラーが起きない
// - メモリ確保（.collect()）が明示的
```

### 落とし穴4：「REPLはどこ？」
```rust
// RustにはREPLがない。代わりに次の方法を使う：
// 1. `cargo test`をREPLのように使い、小さなテストを書いて試す
// 2. 手早い実験にはRust Playground（play.rust-lang.org）を使う
// 3. 手軽なデバッグ出力には`dbg!()`マクロを使う
// 4. 保存時にテストを自動実行するには`cargo watch -x test`を使う

#[test]
fn playground() {
    // これを「REPL」として使う — `cargo test playground`で実行
    let result = "hello world"
        .split_whitespace()
        .map(|w| w.to_uppercase())
        .collect::<Vec<_>>();
    dbg!(&result);  // 出力例：[src/main.rs:5] &result = ["HELLO", "WORLD"]
}
```

***

<a id="learning-path-and-resources"></a>
## 学習計画とリソース

### 1～2週目：基礎
- [ ] Rustをインストールし、VS Codeにrust-analyzerを設定する
- [ ] 本ガイドの第1～4章（型、制御フロー）を完了する
- [ ] PythonスクリプトをRustに書き換えた小さなプログラムを5つ作る
- [ ] `cargo build`、`cargo test`、`cargo clippy`に慣れる

### 3～4週目：主要な概念
- [ ] 第5～8章（構造体、enum、所有権、モジュール）を完了する
- [ ] Pythonのデータ処理スクリプトをRustに書き換える
- [ ] `Option<T>`と`Result<T, E>`を自然に使えるまで練習する
- [ ] コンパイラのエラーメッセージを注意深く読む — 学びの助けになります

### 2か月目：中級
- [ ] 第9～12章（エラー処理、トレイト、イテレーター）を完了する
- [ ] `clap`と`serde`を使ってCLIツールを作る
- [ ] Pythonプロジェクトのボトルネックを置き換えるPyO3拡張機能を作る
- [ ] イテレーターチェーンを内包表記のように自然に感じるまで練習する

### 3か月目：上級
- [ ] 第13～16章（並行処理、unsafe、テスト）を完了する
- [ ] `axum`と`tokio`でWebサービスを作る
- [ ] オープンソースのRustプロジェクトに貢献する
- [ ] 理解を深めるために『Programming Rust』（O'Reilly）を読む

### おすすめのリソース
- **The Rust Book**: https://doc.rust-lang.org/book/ （公式の優れたガイド）
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/ （実践しながら学ぶ）
- **Rustlings**: https://github.com/rust-lang/rustlings （演習）
- **Rust Playground**: https://play.rust-lang.org/ （オンラインコンパイラ）
- **This Week in Rust**: https://this-week-in-rust.org/ （ニュースレター）
- **PyO3 Guide**: https://pyo3.rs/ （PythonとRustの橋渡し）
- **Comprehensive Rust**（Google）: https://google.github.io/comprehensive-rust/

<a id="rosetta-stone-python-to-rust"></a>
### PythonからRustへの対応表（ロゼッタストーン）

| Python | Rust | 章 |
|--------|------|---------|
| `list` | `Vec<T>` | 5 |
| `dict` | `HashMap<K,V>` | 5 |
| `set` | `HashSet<T>` | 5 |
| `tuple` | `(T1, T2, ...)` | 5 |
| `class` | `struct` + `impl` | 5 |
| `@dataclass` | `#[derive(...)]` | 5, 12a |
| `Enum` | `enum` | 6 |
| `None` | `Option<T>` | 6 |
| `raise`/`try`/`except` | `Result<T,E>` + `?` | 9 |
| `Protocol` (PEP 544) | `trait` | 10 |
| `TypeVar` | ジェネリクス`<T>` | 10 |
| `__dunder__`メソッド | トレイト（Display、Addなど） | 10 |
| `lambda` | `\|args\| body` | 12 |
| ジェネレーターの`yield` | `impl Iterator` | 12 |
| リスト内包表記 | `.map().filter().collect()` | 12 |
| `@decorator` | 高階関数またはマクロ | 12a, 15 |
| `asyncio` | `tokio` | 13 |
| `threading` | `std::thread` | 13 |
| `multiprocessing` | `rayon` | 13 |
| `unittest.mock` | `mockall` | 14a |
| `pytest` | `cargo test` + `rstest` | 14a |
| `pip install` | `cargo add` | 8 |
| `requirements.txt` | `Cargo.lock` | 8 |
| `pyproject.toml` | `Cargo.toml` | 8 |
| `with`（コンテキストマネージャー） | スコープベースの`Drop` | 15 |
| `json.dumps/loads` | `serde_json` | 15 |

***

## Python開発者への最後のメッセージ

```rust
Pythonを離れて惜しく感じるもの：
- REPLと対話的な試行錯誤
- 高速なプロトタイピング
- 豊富なML/AIエコシステム（PyTorchなど）
- 「そのまま動く」動的型付け
- pip installですぐに使える手軽さ

Rustで得られるもの：
- 「コンパイルできれば動く」という安心感
- 10～100倍の性能向上
- 実行時の型エラーがなくなる
- None/nullによるクラッシュがなくなる
- 真の並列処理（GILなし！）
- 単一バイナリでのデプロイ
- 予測可能なメモリ使用量
- どの言語よりも優れたコンパイラのエラーメッセージ

学習の道のり：
1週目：「なぜコンパイラは私を嫌うのだろう？」
2週目：「なるほど、バグから守ってくれているんだ」
1か月目：「なぜこれが大切なのか分かってきた」
2か月目：「本番障害になっていたバグをコンパイル時に見つけた」
3か月目：「型のないコードには戻りたくない」
6か月目：「Rustのおかげで、どの言語でもより良いプログラマーになれた」
```

---

## 演習

<details>
<summary><strong>🏋️ 演習：コードレビューのチェックリスト</strong>（クリックして展開）</summary>

**課題：** Python開発者が書いた次のRustコードをレビューし、慣用的な書き方に改善できる点を5つ挙げてください。

```rust
fn get_name(names: Vec<String>, index: i32) -> String {
    if index >= 0 && (index as usize) < names.len() {
        return names[index as usize].clone();
    } else {
        return String::from("");
    }
}

fn main() {
    let mut result = String::from("");
    let names = vec!["Alice".to_string(), "Bob".to_string()];
    result = get_name(names.clone(), 0);
    println!("{}", result);
}
```

<details>
<summary>🔑 解答</summary>

改善点は次の5つです。

```rust
// 1. Vec<String>ではなく&[String]を受け取る（Vec全体の所有権を取らない）
// 2. インデックスにはi32ではなくusizeを使う（インデックスは常に非負）
// 3. 空文字列ではなくOption<&str>を返す（型システムを活用する！）
// 4. 範囲を手動で確認せず、.get()を使う
// 5. mainでclone()せず、参照を渡す

fn get_name(names: &[String], index: usize) -> Option<&str> {
    names.get(index).map(|s| s.as_str())
}

fn main() {
    let names = vec!["Alice".to_string(), "Bob".to_string()];
    match get_name(&names, 0) {
        Some(name) => println!("{name}"),
        None => println!("Not found"),
    }
}
```

**要点：** Rustでは、何でもcloneする（借用を使う）、`""`のような番兵値を使う（`Option`を使う）、借用で十分な場面で所有権を取る、インデックスに符号付き整数を使う、といったPythonの習慣が問題になります。

</details>
</details>

***

*『PythonプログラマーのためのRustトレーニングガイド』終わり*
