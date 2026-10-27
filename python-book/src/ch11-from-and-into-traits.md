<a id="type-conversions-in-rust"></a>

## Rustの型変換

> **学習内容:** ゼロコストの型変換に使う`From`と`Into`トレイト、失敗する可能性のある変換に使う`TryFrom`、
> `impl From<A> for B`によって`Into`が自動生成される仕組み、文字列変換のパターン。
>
> **難易度:** 🟡 中級

Pythonではコンストラクター呼び出し（`int("42")`、`str(42)`、`float("3.14")`）で型変換します。Rustでは型安全な変換に`From`と`Into`トレイトを使います。

### Pythonの型変換
```python
# Python — 明示的なコンストラクターによる変換
x = int("42")           # str → int (can raise ValueError)
s = str(42)             # int → str
f = float("3.14")       # str → float
lst = list((1, 2, 3))   # tuple → list

# __init__またはクラスメソッドによる独自の変換
class Celsius:
    def __init__(self, temp: float):
        self.temp = temp

    @classmethod
    def from_fahrenheit(cls, f: float) -> "Celsius":
        return cls((f - 32.0) * 5.0 / 9.0)

c = Celsius.from_fahrenheit(212.0)  # 100.0°C
```

### RustのFrom/Into
```rust
// Rust — Fromトレイトで変換を定義する
// From<T>を実装するとInto<U>も自動的に使える！

struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

// どちらの書き方も使える：
let c1 = Celsius::from(Fahrenheit(212.0));    // 明示的なFrom
let c2: Celsius = Fahrenheit(212.0).into();   // Into（自動的に利用可能）

// 文字列変換：
let s: String = String::from("hello");         // &str → String
let s: String = "hello".to_string();           // Same thing
let s: String = "hello".into();                // Also works (From is implemented)

let num: i64 = 42i32.into();                   // i32 → i64（情報を失わないためFromがある）
// let small: i32 = 42i64.into();              // ❌ i64 → i32は情報を失う可能性があるためFromはない

// 失敗する可能性のある変換にはTryFromを使う：
let n: Result<i32, _> = "42".parse();          // str → i32 (might fail)
let n: i32 = "42".parse().unwrap();            // Panic if not a number
let n: i32 = "42".parse()?;                    // Propagate error with ?
```

### FromとIntoの関係

```mermaid
flowchart TB
    A["impl From&lt;A&gt; for B"] -->|"自動生成"| B["impl Into&lt;B&gt; for A"]
    C["Celsius::from(Fahrenheit(212.0))"] ---|"と同じ"| D["Fahrenheit(212.0).into()"]
    style A fill:#d4edda
    style B fill:#d4edda
```

> **原則**：常に`From`を実装し、`Into`を直接実装することは避けてください。`From<A> for B`を実装すれば、`Into<B> for A`も自動的に使えます。

***

### From/Intoを使う場面

```rust
// 型にFrom<T>を実装して、使いやすいAPIを設計する：

#[derive(Debug)]
struct UserId(i64);

impl From<i64> for UserId {
    fn from(id: i64) -> Self {
        UserId(id)
    }
}

// これで関数はUserIdに変換できる型なら何でも受け取れる：
fn find_user(id: impl Into<UserId>) -> Option<String> {
    let user_id = id.into();
    // ... 検索処理
    Some(format!("User #{:?}", user_id))
}

find_user(42i64);              // ✅ i64からUserIdへ自動変換
find_user(UserId(42));         // ✅ UserIdはそのまま使われる
```

***

## TryFrom — 失敗する可能性のある変換

すべての変換が成功するとは限りません。Pythonでは例外を送出しますが、Rustでは`Result`を返す`TryFrom`を使います。

```python
# Python — 失敗する変換では例外が発生する
try:
    port = int("not_a_number")   # ValueError
except ValueError as e:
    print(f"Invalid: {e}")

# __init__内での独自検証
class Port:
    def __init__(self, value: int):
        if not (1 <= value <= 65535):
            raise ValueError(f"Invalid port: {value}")
        self.value = value

try:
    p = Port(99999)  # ValueError at runtime
except ValueError:
    pass
```

```rust
use std::num::ParseIntError;

// 組み込み型に対するTryFrom
let n: Result<i32, ParseIntError> = "42".try_into();   // Ok(42)
let n: Result<i32, ParseIntError> = "bad".try_into();  // Err(...)

// 検証用の独自TryFrom
#[derive(Debug)]
struct Port(u16);

#[derive(Debug)]
enum PortError {
    Zero,
}

impl TryFrom<u16> for Port {
    type Error = PortError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Err(PortError::Zero),
            1..=65535 => Ok(Port(value)),
        }
    }
}

impl std::fmt::Display for PortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortError::Zero => write!(f, "port cannot be zero"),
        }
    }
}

// 使用例：
let p: Result<Port, _> = 8080u16.try_into();   // Ok(Port(8080))
let p: Result<Port, _> = 0u16.try_into();       // Err(PortError::Zero)
```

> **PythonからRustへの考え方**：`TryFrom`は、検証を行い失敗する可能性がある`__init__`に相当します。ただし例外を送出する代わりに`Result`を返すため、呼び出し側はエラーケースを**必ず**処理します。

***

## 文字列変換のパターン

Python開発者が変換で最も混乱しやすいのは文字列です。

```rust
// String → &str（借用、コストなし）
let s = String::from("hello");
let r: &str = &s;              // Automatic Deref coercion
let r: &str = s.as_str();     // Explicit

// &str → String（確保が必要、メモリーを消費）
let r: &str = "hello";
let s1 = String::from(r);     // From trait
let s2 = r.to_string();       // ToString trait (via Display)
let s3: String = r.into();    // Into trait

// 数値 → String
let s = 42.to_string();       // "42" — Pythonのstr(42)に相当
let s = format!("{:.2}", 3.14); // "3.14" — Pythonのf"{3.14:.2f}"に相当

// String → 数値
let n: i32 = "42".parse().unwrap();       // Pythonのint("42")に相当
let f: f64 = "3.14".parse().unwrap();     // Pythonのfloat("3.14")に相当

// 独自型 → String（Displayを実装）
use std::fmt;

struct Point { x: f64, y: f64 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1.0, y: 2.0 };
println!("{p}");                // (1, 2) — Pythonの__str__に相当
let s = p.to_string();         // これも使える！Displayを実装するとToStringも自動で使える。
```

### 変換の早見表

| Python | Rust | 備考 |
|--------|------|-------|
| `str(x)` | `x.to_string()` | `Display`の実装が必要 |
| `int("42")` | `"42".parse::<i32>()` | `Result`を返す |
| `float("3.14")` | `"3.14".parse::<f64>()` | `Result`を返す |
| `list(iter)` | `iter.collect::<Vec<_>>()` | 型注釈が必要 |
| `dict(pairs)` | `pairs.collect::<HashMap<_,_>>()` | 型注釈が必要 |
| `bool(x)` | 直接の相当物なし | 明示的な条件判定を使う |
| `MyClass(x)` | `MyClass::from(x)` | `From<T>`を実装 |
| `MyClass(x)`（検証あり） | `MyClass::try_from(x)?` | `TryFrom<T>`を実装 |

***

## 変換の連鎖とエラー処理

実際のコードでは複数の変換を連鎖させることがよくあります。それぞれの方法を比べてみましょう。

```python
# Python — try/exceptを使った変換の連鎖
def parse_config(raw: str) -> tuple[str, int]:
    try:
        host, port_str = raw.split(":")
        port = int(port_str)
        if not (1 <= port <= 65535):
            raise ValueError(f"Bad port: {port}")
        return (host, port)
    except (ValueError, AttributeError) as e:
        raise ConfigError(f"Invalid config: {e}") from e
```

```rust
fn parse_config(raw: &str) -> Result<(String, u16), String> {
    let (host, port_str) = raw
        .split_once(':')
        .ok_or_else(|| "missing ':' separator".to_string())?;

    let port: u16 = port_str
        .parse()
        .map_err(|e| format!("invalid port: {e}"))?;

    if port == 0 {
        return Err("port cannot be zero".to_string());
    }

    Ok((host.to_string(), port))
}

fn main() {
    match parse_config("localhost:8080") {
        Ok((host, port)) => println!("Connecting to {host}:{port}"),
        Err(e) => eprintln!("Config error: {e}"),
    }
}
```

> **重要なポイント**：`?`はそれぞれ目に見える終了ポイントです。Pythonでは`try`内のどの行でも例外が発生する可能性がありますが、Rustでは`?`で終わる行だけがエラーを伝播します。
>
> 📌 **関連項目**：[第9章 — エラー処理](ch09-error-handling.md)では、`Result`、`?`、`thiserror`による独自エラー型を詳しく扱います。

---

## 練習問題

<details>
<summary><strong>🏋️ 練習問題：温度変換ライブラリー</strong>（クリックして展開）</summary>

**課題**：小さな温度変換ライブラリーを作成してください。
1. `Celsius(f64)`、`Fahrenheit(f64)`、`Kelvin(f64)`構造体を定義する
2. `From<Celsius> for Fahrenheit`と`From<Celsius> for Kelvin`を実装する
3. 絶対零度未満（-273.15°C = 0K）の値を拒否する`TryFrom<f64> for Kelvin`を実装する
4. 3つの型すべてに`Display`を実装する（例：`"100.00°C"`）

<details>
<summary>🔑 解答</summary>

```rust
use std::fmt;

struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        Kelvin(c.0 + 273.15)
    }
}

#[derive(Debug)]
struct BelowAbsoluteZero;

impl fmt::Display for BelowAbsoluteZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "temperature below absolute zero")
    }
}

impl TryFrom<f64> for Kelvin {
    type Error = BelowAbsoluteZero;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 {
            Err(BelowAbsoluteZero)
        } else {
            Ok(Kelvin(value))
        }
    }
}

impl fmt::Display for Celsius    { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.2}°C", self.0) } }
impl fmt::Display for Fahrenheit { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.2}°F", self.0) } }
impl fmt::Display for Kelvin     { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:.2}K",  self.0) } }

fn main() {
    let boiling = Celsius(100.0);
    let f: Fahrenheit = Celsius(100.0).into();
    let k: Kelvin = Celsius(100.0).into();
    println!("{boiling} = {f} = {k}");

    match Kelvin::try_from(-10.0) {
        Ok(k) => println!("{k}"),
        Err(e) => println!("Error: {e}"),
    }
}
```

**要点**：`From`は失敗しない変換を扱います（摂氏から華氏への変換は常に成功）。`TryFrom`は失敗する可能性がある変換を扱います（負のケルビン温度は不可能）。Pythonでは両方を`__init__`で扱いますが、Rustでは型システム上で明確に区別します。

</details>
</details>

***
