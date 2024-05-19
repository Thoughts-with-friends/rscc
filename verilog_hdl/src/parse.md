# Parse

## 設計

syn クレートを使用して Rust コードをパースし、コード内のトークンを抽出し、それらの文字列のみを標準出力するプログラムを作成します。以下にその方法を示します。

### 1. Cargo.toml の設定

まず、`syn` と proc-macro2 クレートを依存関係に追加します。

toml
[package]
name = "rust_parser"
version = "0.1.0"
edition = "2018"

[dependencies]
syn = "2.0"
proc-macro2 = "1.0"

### 2. メインコード

次に、Rust コードをパースしてトークンを標準出力するプログラムを書きます。`main.rs` ファイルを作成し、以下のコードを追加します。

```rust
extern crate proc_macro2;
extern crate syn;

use proc_macro2::{TokenStream, TokenTree};
use std::str::FromStr;

fn main() {
    // サンプルのRustコード
    let code = r#"
        fn main() {
            println!("Hello, world!");
        }
    "#;

    // TokenStreamにパース
    let token_stream = TokenStream::from_str(code).expect("Failed to parse code");

    // トークンを標準出力
    for token in token_stream {
        print_token(token);
    }
}

fn print_token(token: TokenTree) {
    match token {
        TokenTree::Group(group) => {
            for token in group.stream() {
                print_token(token);
            }
        },
        _ => {
            print!("{}", token);
        }
    }
}
```

このプログラムは、以下の手順を実行します。

1. パースする Rust コードを文字列として定義します。
2. TokenStream 型にコードをパースします。
3. 各トークンを標準出力に出力します。

### 説明

- TokenStream は、Rust コードをトークンのシーケンスとして扱うデータ型です。
- 各トークンは proc_macro2::TokenTree 型で表現され、識別子 (`Ident`)、リテラル (`Literal`)、区切り記号 (`Punct`)、グループ (`Group`) などを含みます。
- print_token 関数では、トークンが Group であれば再帰的にその中身を処理し、それ以外のトークンはそのまま標準出力に出力します。

### 実行

次に、このプログラムを実行してみましょう。

sh
cargo run

実行結果は、コード内の各トークンが文字列として標準出力に表示されます。例えば、以下のような出力が得られるでしょう：

fn main() { println!("Hello, world!"); }

このプログラムでは、トークンの内容を再帰的に処理して出力することで、元のコードの文字列を忠実に再現しています。
