<a id="rust-closures-vs-python-lambdas"></a>

## RustのクロージャとPythonのラムダ式

> **学習内容:** 1つの式に限らない複数行クロージャ、`Fn`/`FnMut`/`FnOnce`のキャプチャ意味論、
> イテレーターチェーンとリスト内包表記の比較、`map`/`filter`/`fold`、`macro_rules!`の基礎。
>
> **難易度:** 🟡 中級

### Pythonのクロージャとラムダ式
```python
# Python — ラムダ式は1つの式からなる無名関数
double = lambda x: x * 2
result = double(5)  # 10

# クロージャは外側のスコープにある変数をキャプチャする：
def make_adder(n):
    def adder(x):
        return x + n    # 外側のスコープから`n`をキャプチャ
    return adder

add_5 = make_adder(5)
print(add_5(10))  # 15

# 高階関数：
numbers = [1, 2, 3, 4, 5]
doubled = list(map(lambda x: x * 2, numbers))
evens = list(filter(lambda x: x % 2 == 0, numbers))
```

### Rustのクロージャ
```rust
// Rust — クロージャは|引数| 本体という構文
let double = |x: i32| x * 2;
let result = double(5);  // 10

// クロージャは外側のスコープにある変数をキャプチャする：
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // `move`で`n`の所有権をクロージャに移す
}

let add_5 = make_adder(5);
println!("{}", add_5(10));  // 15

// イテレーターを使った高階関数：
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
```

### クロージャ構文の比較
```text
Python:                              Rust:
─────────                            ─────
lambda x: x * 2                      |x| x * 2
lambda x, y: x + y                   |x, y| x + y
lambda: 42                           || 42

# 複数行
def f(x):                            |x| {
    y = x * 2                            let y = x * 2;
    return y + 1                         y + 1
                                      }
```

### クロージャのキャプチャ — Rustとの違い
```python
# Python — クロージャは参照でキャプチャする（遅延束縛！）
funcs = [lambda: i for i in range(3)]
print([f() for f in funcs])  # [2, 2, 2] — なんと、すべて同じ`i`をキャプチャしている

# デフォルト引数を使った回避策：
funcs = [lambda i=i: i for i in range(3)]
print([f() for f in funcs])  # [0, 1, 2]
```

```rust
// Rust — クロージャは正しくキャプチャする（遅延束縛の落とし穴はない）
let funcs: Vec<Box<dyn Fn() -> i32>> = (0..3)
    .map(|i| Box::new(move || i) as Box<dyn Fn() -> i32>)
    .collect();

let results: Vec<i32> = funcs.iter().map(|f| f()).collect();
println!("{:?}", results);  // [0, 1, 2] — correct!

// `move`は各クロージャに`i`のコピーをキャプチャする — 遅延束縛の意外な挙動はない。
```

### 3つのクロージャトレイト
```rust
// Rustのクロージャは、次のトレイトを1つ以上実装する：

// Fn — 複数回呼び出せ、キャプチャした値を変更しない（最も一般的）
fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 { f(x) }

// FnMut — 複数回呼び出せ、キャプチャした値を変更する場合がある
fn apply_mut(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 { f(x) }

// FnOnce — 1回だけ呼び出せる（キャプチャした値を消費する）
fn apply_once(f: impl FnOnce() -> String) -> String { f() }

// Pythonに相当する仕組みはなく、クロージャは常にFnのように振る舞う。
// Rustではコンパイラーがどのトレイトに該当するかを自動判定する。
```

***

## イテレーターとジェネレーター

### Pythonのジェネレーター
```python
# Python — yieldを使うジェネレーター
def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

# 遅延評価 — 必要になったときに値を計算
fib = fibonacci()
first_10 = [next(fib) for _ in range(10)]

# ジェネレーター式 — 遅延評価されるリスト内包表記に似ている
squares = (x ** 2 for x in range(1000000))  # No memory allocation
first_5 = [next(squares) for _ in range(5)]
```

### Rustのイテレーター
```rust
// Rust — Iteratorトレイト（概念は似ているが構文は異なる）
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}

// 遅延評価 — 必要になったときに値を計算（Pythonのジェネレーターと同様）
let first_10: Vec<u64> = Fibonacci::new().take(10).collect();

// イテレーターチェーン — ジェネレーター式に相当
let squares: Vec<u64> = (0..1_000_000u64).map(|x| x * x).take(5).collect();
```

***

## 内包表記とイテレーターチェーン

この節では、Pythonの内包表記をRustのイテレーターチェーンに対応付けます。

### リスト内包表記 → map/filter/collect
```python
# Pythonの内包表記：
squares = [x ** 2 for x in range(10)]
evens = [x for x in range(20) if x % 2 == 0]
names = [user.name for user in users if user.active]
pairs = [(x, y) for x in range(3) for y in range(3)]
flat = [item for sublist in nested for item in sublist]
```

```mermaid
flowchart LR
    A["入力<br/>[1,2,3,4,5]"] -->|".iter()"| B["イテレーター"]
    B -->|".filter(x: x%2==0)"| C["[2, 4]"]
    C -->|".map(x: x*x)"| D["[4, 16]"]
    D -->|".collect()"| E["Vec&lt;i32&gt;<br/>[4, 16]"]
    style A fill:#ffeeba
    style E fill:#d4edda
```

> **重要なポイント**：Rustのイテレーターは遅延評価され、`.collect()`するまで何も実行されません。Pythonのジェネレーターも同様ですが、リスト内包表記は即時評価されます。

```rust
// Rustのイテレーターチェーン：
let squares: Vec<i32> = (0..10).map(|x| x * x).collect();
let evens: Vec<i32> = (0..20).filter(|x| x % 2 == 0).collect();
let names: Vec<&str> = users.iter()
    .filter(|u| u.active)
    .map(|u| u.name.as_str())
    .collect();
let pairs: Vec<(i32, i32)> = (0..3)
    .flat_map(|x| (0..3).map(move |y| (x, y)))
    .collect();
let flat: Vec<i32> = nested.iter()
    .flat_map(|sublist| sublist.iter().copied())
    .collect();
```

### 辞書内包表記 → HashMapへのcollect
```python
# Python
word_lengths = {word: len(word) for word in words}
inverted = {v: k for k, v in mapping.items()}
```

```rust
// Rust
let word_lengths: HashMap<&str, usize> = words.iter()
    .map(|w| (*w, w.len()))
    .collect();
let inverted: HashMap<&V, &K> = mapping.iter()
    .map(|(k, v)| (v, k))
    .collect();
```

### 集合内包表記 → HashSetへのcollect
```python
# Python
unique_lengths = {len(word) for word in words}
```

```rust
// Rust
let unique_lengths: HashSet<usize> = words.iter()
    .map(|w| w.len())
    .collect();
```

### よく使うイテレーターメソッド

| Python | Rust | 備考 |
|--------|------|-------|
| `map(f, iter)` | `.map(f)` | 各要素を変換 |
| `filter(f, iter)` | `.filter(f)` | 条件に合う要素を残す |
| `sum(iter)` | `.sum()` | すべての要素を合計 |
| `min(iter)` / `max(iter)` | `.min()` / `.max()` | `Option`を返す |
| `any(f(x) for x in iter)` | `.any(f)` | 1つでも一致すればtrue |
| `all(f(x) for x in iter)` | `.all(f)` | すべて一致すればtrue |
| `enumerate(iter)` | `.enumerate()` | インデックスと値 |
| `zip(a, b)` | `a.zip(b)` | 要素をペアにする |
| `len(list)` | `.count()`（消費する！）または`.len()` | 要素数を数える |
| `list(reversed(x))` | `.rev()` | 逆順に反復 |
| `itertools.chain(a, b)` | `a.chain(b)` | イテレーターを連結 |
| `next(iter)` | `.next()` | 次の要素を取得 |
| `next(iter, default)` | `.next().unwrap_or(default)` | デフォルト値を指定 |
| `list(iter)` | `.collect::<Vec<_>>()` | コレクションとして具体化 |
| `sorted(iter)` | collectしてから`.sort()` | 遅延ソートイテレーターはない |
| `functools.reduce(f, iter)` | `.fold(init, f)`または`.reduce(f)` | 値を累積 |

### 主な違い
```text
Pythonのイテレーター：                 Rustのイテレーター：
─────────────────                     ──────────────
- デフォルトで遅延評価（ジェネレーター） - デフォルトで遅延評価（すべてのイテレーターチェーン）
- yieldでジェネレーターを作成            - impl Iterator { fn next() }
- StopIterationで終了                  - Noneで終了
- 1度だけ消費可能                      - 1度だけ消費可能
- 型安全性なし                         - 完全に型安全
- やや低速（インタープリター）           - ゼロコスト（コンパイル時に除去）
```

***


<!-- ch12a: マクロ -->
## Rustにマクロがある理由

Pythonにはマクロシステムがありません。メタプログラミングにはデコレーター、メタクラス、実行時イントロスペクションを使います。
Rustではマクロをコンパイル時のコード生成に使います。

### PythonのメタプログラミングとRustのマクロ
```python
# Python — メタプログラミングに使うデコレーターとメタクラス
from dataclasses import dataclass
from functools import wraps

@dataclass              # インポート時に__init__、__repr__、__eq__を生成
class Point:
    x: float
    y: float

# 独自デコレーター
def log_calls(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        print(f"Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper

@log_calls
def process(data):
    return data.upper()
```

```rust
// Rust — コード生成に使うderiveマクロと宣言的マクロ
#[derive(Debug, Clone, PartialEq)]  // コンパイル時にDebug、Clone、PartialEqの実装を生成
struct Point {
    x: f64,
    y: f64,
}

// 宣言的マクロ（テンプレートのようなもの）
macro_rules! log_call {
    ($func_name:expr, $body:expr) => {
        {
            println!("Calling {}", $func_name);
            $body
        }
    };
}

fn process(data: &str) -> String {
    log_call!("process", data.to_uppercase())
}
```

### よく使う組み込みマクロ
```rust
// これらのマクロはRustのあらゆる場所で使われる：

println!("Hello, {}!", name);           // Print with formatting
format!("Value: {}", x);               // Create formatted String
vec![1, 2, 3];                          // Create a Vec
assert_eq!(2 + 2, 4);                  // Test assertion
assert!(value > 0, "must be positive"); // Boolean assertion
dbg!(expression);                       // Debug print: prints expression AND value
todo!();                                // Placeholder — compiles but panics if reached
unimplemented!();                       // Mark code as unimplemented
panic!("something went wrong");         // Crash with message (like raise RuntimeError)

// なぜこれらは関数ではなくマクロなのか？
// - println!は可変個の引数を受け取れる（Rustの関数ではできない）
// - vec!は任意の型とサイズに対応するコードを生成する
// - assert_eq!は比較対象のソースコードを把握できる
// - dbg!はファイル名と行番号を把握できる
```

## macro_rules!で簡単なマクロを書く
```rust
// Pythonのdict()に相当
// Python: d = dict(a=1, b=2)
// Rust:   let d = hashmap!{ "a" => 1, "b" => 2 };

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    };
}

let scores = hashmap! {
    "Alice" => 100,
    "Bob" => 85,
    "Charlie" => 90,
};
```

## deriveマクロ — トレイトの自動実装
```rust
// #[derive(...)]はPythonの@dataclassデコレーターに相当する

// Python:
// @dataclass(frozen=True, order=True)
// class Student:
//     name: str
//     grade: int

// Rust:
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Student {
    name: String,
    grade: i32,
}

// よく使うderiveマクロ：
// Debug         → {:?}書式（__repr__に相当）
// Clone         → .clone()によるディープコピー
// Copy          → 暗黙のコピー（単純な型のみ）
// PartialEq, Eq → ==による比較（__eq__に相当）
// PartialOrd, Ord → <、>、ソート（__lt__などに相当）
// Hash          → HashMapのキーとして使用可能（__hash__に相当）
// Default       → MyType::default()（引数なしの__init__に相当）

// クレートが提供するderiveマクロ：
// Serialize, Deserialize (serde) → JSON/YAML/TOMLのシリアライズ
//                                  （Pythonのjson.dumps/loadsに似ているが型安全）
```

### PythonのデコレーターとRustのderive

| Pythonのデコレーター | Rustのderive | 用途 |
|-----------------|-------------|---------|
| `@dataclass` | `#[derive(Debug, Clone, PartialEq)]` | データクラス |
| `@dataclass(frozen=True)` | デフォルトで不変 | 不変性 |
| `@dataclass(order=True)` | `#[derive(Ord, PartialOrd)]` | 比較・ソート |
| `@total_ordering` | `#[derive(PartialOrd, Ord)]` | 全順序 |
| JSON `json.dumps(obj.__dict__)` | `#[derive(Serialize)]` | シリアライズ |
| JSON `MyClass(**json.loads(s))` | `#[derive(Deserialize)]` | デシリアライズ |

---

## 練習問題

<details>
<summary><strong>🏋️ 練習問題：deriveと独自Debug実装</strong>（クリックして展開）</summary>

**課題**：`name: String`、`email: String`、`password_hash: String`フィールドを持つ`User`構造体を作ってください。`Clone`と`PartialEq`をderiveし、名前とメールアドレスを表示しつつパスワードを伏せ字（`"***"`）にするよう、`Debug`を手動実装してください。

<details>
<summary>🔑 解答</summary>

```rust
use std::fmt;

#[derive(Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    password_hash: String,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("name", &self.name)
            .field("email", &self.email)
            .field("password_hash", &"***")
            .finish()
    }
}

fn main() {
    let user = User {
        name: "Alice".into(),
        email: "alice@example.com".into(),
        password_hash: "a1b2c3d4e5f6".into(),
    };
    println!("{user:?}");
    // 出力：User { name: "Alice", email: "alice@example.com", password_hash: "***" }
}
```

**要点**：Pythonの`__repr__`とは異なり、Rustでは`Debug`を簡単にderiveできます。また、機密フィールドについては独自実装で表示を変更できます。`print(user)`で誤って秘密情報が漏れる可能性のあるPythonより安全です。

</details>
</details>

***
