use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

/// (スラッグ、タイトル、説明、カテゴリ)
const BOOKS: &[(&str, &str, &str, &str)] = &[
    (
        "c-cpp-book",
        "C/C++プログラマーのためのRust",
        "ムーブセマンティクス、RAII、FFI、組み込み、no_std",
        "bridge",
    ),
    (
        "csharp-book",
        "C#プログラマーのためのRust",
        "Swift / C# / Javaの開発者に最適",
        "bridge",
    ),
    (
        "python-book",
        "PythonプログラマーのためのRust",
        "動的型付けから静的型付けへ、GILに依存しない並行処理",
        "bridge",
    ),
    (
        "async-book",
        "Async Rust：Futureから本番環境まで",
        "Tokio、ストリーム、キャンセル安全性",
        "deep-dive",
    ),
    (
        "rust-patterns-book",
        "Rust Patterns",
        "Pin、アロケーター、ロックフリー構造、unsafe",
        "advanced",
    ),
    (
        "type-driven-correctness-book",
        "Type-Driven Correctness",
        "型状態、ファントム型、ケイパビリティトークン",
        "expert",
    ),
    (
        "engineering-book",
        "Rust Engineering Practices",
        "ビルドスクリプト、クロスコンパイル、カバレッジ、CI/CD",
        "practices",
    ),
];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtaskはワークスペースのサブディレクトリに配置してください")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(),
        Some("serve") => {
            cmd_build();
            cmd_serve();
        }
        Some("deploy") => cmd_deploy(),
        Some("clean") => cmd_clean(),
        Some("--help" | "-h" | "help") | None => print_usage(0),
        Some(other) => {
            eprintln!("不明なコマンドです: {other}\n");
            print_usage(1);
        }
    }
}

fn print_usage(code: i32) {
    let stream: &mut dyn Write = if code == 0 {
        &mut std::io::stdout()
    } else {
        &mut std::io::stderr()
    };
    let _ = writeln!(
        stream,
        "\
使い方: cargo xtask <コマンド>

コマンド:
  build    すべての書籍をsite/にビルド（ローカルプレビュー用）
  serve    ビルドしてhttp://localhost:3000で配信
  deploy   すべての書籍をdocs/にビルド（GitHub Pages用）
  clean    site/とdocs/ディレクトリを削除"
    );
    std::process::exit(code);
}

// ── ビルド ───────────────────────────────────────────────────────────

fn cmd_build() {
    if !check_mdbook() {
        eprintln!("エラー: PATHに'mdbook'が見つかりません。インストールしてください: https://rust-lang.github.io/mdbook/guide/installation.html");
        std::process::exit(1);
    }
    build_to("site");
}

fn cmd_deploy() {
    if !check_mdbook() {
        eprintln!("エラー: PATHに'mdbook'が見つかりません。");
        std::process::exit(1);
    }
    build_to("docs");
    println!("\n公開するには、docs/をコミットし、GitHub Pagesで「Deploy from a branch」→「/docs」を選択してください。");
}

fn check_mdbook() -> bool {
    Command::new("mdbook")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn build_to(dir_name: &str) {
    let root = project_root();
    let out = root.join(dir_name);

    if out.exists() {
        fs::remove_dir_all(&out).expect("出力ディレクトリのクリーンアップに失敗しました");
    }
    fs::create_dir_all(&out).expect("出力ディレクトリの作成に失敗しました");

    println!("統合サイトを{dir_name}/にビルドしています。\n");

    let mut ok = 0u32;
    for &(slug, _, _, _) in BOOKS {
        let book_dir = root.join(slug);
        if !book_dir.is_dir() {
            eprintln!("  ✗ {slug}/が見つからないため、スキップします");
            continue;
        }
        let dest = out.join(slug);
        let status = Command::new("mdbook")
            .args(["build", "--dest-dir"])
            .arg(&dest)
            .current_dir(&book_dir)
            .status()
            .expect("mdbookの実行に失敗しました。インストールされていますか？");

        if status.success() {
            println!("  ✓ {slug}");
            ok += 1;
        } else {
            eprintln!("  ✗ {slug} ビルド失敗");
        }
    }
    println!("\n  書籍のビルド数: {}/{}", ok, BOOKS.len());

    write_landing_page(&out);

    // GitHub Pagesが出力をJekyllで処理しないようにする
    fs::write(out.join(".nojekyll"), "").expect(".nojekyllの作成に失敗しました");
    println!("\n完了しました。出力先: {dir_name}/");
}

fn category_label(cat: &str) -> &str {
    match cat {
        "bridge" => "入門",
        "deep-dive" => "詳解",
        "advanced" => "応用",
        "expert" => "上級",
        "practices" => "実践",
        _ => cat,
    }
}

fn write_landing_page(site: &Path) {
    let cards: String = BOOKS
        .iter()
        .map(|&(slug, title, desc, cat)| {
            let label = category_label(cat);
            format!(
                r#"    <a class="card cat-{cat}" href="{slug}/">
      <h2>{title} <span class="label">{label}</span></h2>
      <p>{desc}</p>
    </a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Rustトレーニングブック</title>
  <style>
    :root {{
      --bg: #1a1a2e;
      --card-bg: #16213e;
      --accent: #e94560;
      --text: #eee;
      --muted: #a8a8b3;
      --clr-bridge: #4ade80;
      --clr-deep-dive: #22d3ee;
      --clr-advanced: #fbbf24;
      --clr-expert: #c084fc;
      --clr-practices: #2dd4bf;
    }}
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
      background: var(--bg);
      color: var(--text);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 3rem 1rem;
    }}
    h1 {{ font-size: 2.5rem; margin-bottom: 0.5rem; }}
    h1 span {{ color: var(--accent); }}
    .subtitle {{ color: var(--muted); font-size: 1.1rem; margin-bottom: 1.2rem; }}

    /* Legend */
    .legend {{
      display: flex; flex-wrap: wrap; gap: 0.6rem 1.4rem;
      justify-content: center; margin-bottom: 2.2rem;
      font-size: 0.8rem; color: var(--muted);
    }}
    .legend-item {{ display: flex; align-items: center; gap: 0.35rem; }}
    .legend-dot {{
      width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0;
    }}

    /* Grid & Cards */
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 1.5rem;
      max-width: 1000px;
      width: 100%;
    }}
    .card {{
      background: var(--card-bg);
      border-radius: 12px;
      padding: 1.5rem 1.5rem 1.5rem 1.25rem;
      text-decoration: none;
      color: var(--text);
      transition: transform 0.15s, box-shadow 0.15s;
      border: 1px solid rgba(255,255,255,0.05);
      border-left: 4px solid var(--stripe);
    }}
    .card:hover {{
      transform: translateY(-4px);
      box-shadow: 0 8px 25px color-mix(in srgb, var(--stripe) 30%, transparent);
      border-color: rgba(255,255,255,0.08);
      border-left-color: var(--stripe);
    }}
    .card h2 {{ font-size: 1.2rem; margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap; }}
    .card p  {{ color: var(--muted); font-size: 0.9rem; line-height: 1.4; }}

    /* Category colours */
    .cat-bridge     {{ --stripe: var(--clr-bridge); }}
    .cat-deep-dive  {{ --stripe: var(--clr-deep-dive); }}
    .cat-advanced   {{ --stripe: var(--clr-advanced); }}
    .cat-expert     {{ --stripe: var(--clr-expert); }}
    .cat-practices  {{ --stripe: var(--clr-practices); }}

    /* Label pill */
    .label {{
      font-size: 0.55rem; font-weight: 700; letter-spacing: 0.08em;
      text-transform: uppercase; padding: 0.15em 0.55em;
      border-radius: 4px; white-space: nowrap; flex-shrink: 0;
      color: var(--bg); background: var(--stripe);
    }}

    footer {{ margin-top: 3rem; color: var(--muted); font-size: 0.85rem; }}
  </style>
</head>
<body>
  <h1>🦀 <span>Rust</span>トレーニングブック</h1>
  <p class="subtitle">ご自身の経験に合ったガイドをお選びください</p>

  <div class="legend">
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-bridge)"></span> 入門 &mdash; 他の言語からRustを学ぶ</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-deep-dive)"></span> 詳解</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-advanced)"></span> 応用</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-expert)"></span> 上級</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-practices)"></span> 実践</span>
  </div>

  <div class="grid">
{cards}
  </div>
  <footer><a href="https://rust-lang.github.io/mdBook/" style="color:var(--accent)">mdBook</a>で作成</footer>
</body>
</html>
"##
    );

    let path = site.join("index.html");
    fs::write(&path, html).expect("index.htmlの書き込みに失敗しました");
    println!("  ✓ index.html");
}

enum ResolveResult {
    File(PathBuf),
    Redirect(String),
    NotFound,
}

/// `request_target`（例: `/foo/bar?x=1`のようなHTTPリクエストパス）を`site_canon`配下のファイルに解決する。
/// 成功時は`ResolveResult::File`、ディレクトリに末尾のスラッシュが必要な場合は`Redirect`、
/// パストラバーサルの試行またはファイルが見つからない場合は`NotFound`を返す。
///
/// 注意: PR#18で導入された多層防御を維持し、さらに強化している。
/// 1. `percent_decode_path`によるパーセントデコード。
/// 2. NULバイトの拒否。
/// 3. パストラバーサル（`..`）の防止。
/// 4. 正規化とプレフィックス検証によるシンボリックリンク経由の脱出防止。
fn resolve_site_file(site_canon: &Path, request_target: &str) -> ResolveResult {
    let path_only = match request_target
        .split('?')
        .next()
        .and_then(|s| s.split('#').next())
    {
        Some(p) => p,
        None => return ResolveResult::NotFound,
    };

    // [セキュリティ] パーセントエンコーディングを処理し、NULバイトを拒否する（PR#18より）
    let decoded = percent_decode_path(path_only);
    if decoded.as_bytes().contains(&0) {
        return ResolveResult::NotFound;
    }

    let rel = decoded.trim_start_matches('/');
    let mut file_path = site_canon.to_path_buf();
    if !rel.is_empty() {
        for seg in rel.split('/').filter(|s| !s.is_empty()) {
            // [セキュリティ] ディレクトリトラバーサルを防止する（PR#18より）
            if seg == ".." {
                return ResolveResult::NotFound;
            }
            file_path.push(seg);
        }
    }

    if file_path.is_dir() {
        // ディレクトリを指していて末尾にスラッシュがない場合、相対リンクが機能するようリダイレクトする。
        if !request_target.ends_with('/') && !request_target.is_empty() {
            return ResolveResult::Redirect(format!("{path_only}/"));
        }
        file_path.push("index.html");
    }

    // [セキュリティ] パスを正規化し、引き続きsite_canon配下にあることを確認する（PR#18より）
    let real = match fs::canonicalize(&file_path) {
        Ok(r) => r,
        Err(_) => return ResolveResult::NotFound,
    };

    if !real.starts_with(site_canon) || !real.is_file() {
        return ResolveResult::NotFound;
    }

    ResolveResult::File(real)
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn percent_decode_path(input: &str) -> String {
    let mut decoded = Vec::with_capacity(input.len());
    let b = input.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex_val(b[i + 1]), hex_val(b[i + 2])) {
                decoded.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        decoded.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

// ── 配信 ─────────────────────────────────────────────────────────────

fn cmd_serve() {
    let site = project_root().join("site");
    let site_canon = fs::canonicalize(&site).expect(
        "site/が見つかりません。先に`cargo xtask build`を実行してください（`cargo xtask serve`ではビルドも自動実行されます）。",
    );
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).expect("ポート3000のバインドに失敗しました");

    // Ctrl+Cで正常終了し、cargoがエラーを報告しないようにする
    ctrlc_exit();

    println!("\nhttp://localhost:3000 で配信中です（終了するにはCtrl+Cを押してください）");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);
        let request = String::from_utf8_lossy(&buf[..n]);

        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        match resolve_site_file(&site_canon, path) {
            ResolveResult::File(file_path) => {
                let body = fs::read(&file_path).unwrap_or_default();
                let mime = guess_mime(&file_path);
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(&body);
            }
            ResolveResult::Redirect(new_path) => {
                let header = format!(
                    "HTTP/1.1 301 Moved Permanently\r\nLocation: {new_path}\r\nContent-Length: 0\r\n\r\n"
                );
                let _ = stream.write_all(header.as_bytes());
            }
            ResolveResult::NotFound => {
                let body = b"404 Not Found";
                let header = format!(
                    "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
                    body.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(body);
            }
        }
    }
}

/// OSに`STATUS_CONTROL_C_EXIT`で終了させず、正常終了コード（0）で終了するCtrl+Cハンドラーを設定する。
fn ctrlc_exit() {
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Ctrl+Cハンドラーの設定に失敗しました");
}

fn guess_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

// ── クリーンアップ ────────────────────────────────────────────────────

fn cmd_clean() {
    let root = project_root();
    for dir_name in ["site", "docs"] {
        let dir = root.join(dir_name);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("ディレクトリの削除に失敗しました");
            println!("{dir_name}/を削除しました");
        }
    }
}
