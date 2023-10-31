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
* 遅延セグメント木を実装したい
* 関数の単体testを作成