# atcoder-rust-snippets

AtCoder用Rustスニペット

## 使い方

スニペット生成（VSCode用）

```
cargo snippet -t vscode
```

テスト

```
cargo test
```

## Git hooks

Git hooks ファイルを`.githooks/`で管理している

設定：

```sh
git config core.hooksPath .githooks
```