# AudiobookManager

## 環境構築手順

1. `yarn create @vitejs/app AudiobookManager --template svelte-ts`
2. `cd AudiobookManager`
3. `yarn`
4. `yarn add -D @tauri-apps/cli`
5. `yarn tauri init`
6. `yarn tauri info`
7. src-tauri/tauri.conf.jsonのbuild内に以下を記載
   1. `"devPath": "http://localhost:3000"`
   2. `"beforeDevCommand": "yarn dev"`
   3. `"beforeBuildCommand": "yarn build"`

## 各種コマンド

- 開発サーバー起動
  - `yarn tauri dev`