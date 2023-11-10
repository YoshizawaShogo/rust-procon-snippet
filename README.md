# 準備

``` bash
    $ cargo install cargo-snippet --features="binaries"
```

# 使用方法

``` bash
    $ cargo snippet -t vscode > rust.code-snippets
    $ mkdir -p <競技プログラミングに使用するディレクトリ>/.vscode/
    $ cp rust.code-snippets <競技プログラミングに使用するディレクトリ>/.vscode/
```

# 次の目標
* 関数の単体testを作成
* 尺取り法

# 注意点
* テストしていません、動くかどうか知りません。
* 関数化していない部分があるため、このリポジトリでcargo buildが通りません。