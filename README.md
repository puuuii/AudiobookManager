# AudiobookManager

## 環境構築手順

1. `yarn create @vitejs/app AudiobookManager --template svelte-ts`
2. `cd AudiobookManager`
3. `yarn`
4. `yarn add -D @tauri-apps/cli`
5. `yarn add @tauri-apps/api`
6. `yarn tauri init`
7. `yarn tauri info`
8. src-tauri/tauri.conf.jsonのbuild内に以下を記載
   1. `"devPath": "http://localhost:3000"`
   2. `"beforeDevCommand": "yarn dev"`
   3. `"beforeBuildCommand": "yarn build"`
9. `yarn add -D sass`

## 各種コマンド

- 開発サーバー起動
  - `yarn tauri dev`