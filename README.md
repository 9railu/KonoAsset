# KonoAsset

![GitHub deployments](https://img.shields.io/github/deployments/siloneco/KonoAsset/release?style=flat)
![GitHub Release](https://img.shields.io/github/v/release/siloneco/KonoAsset?label=Stable)
![GitHub Release](https://img.shields.io/github/v/release/siloneco/KonoAsset?include_prereleases&label=Pre-Release)

「このアセットにしよ！」 をもっと簡単にするための VRChat 向けアセット管理ツール

散らかりやすいアセットの整頓を最大限サポートします

## インストール

[Releases](https://github.com/siloneco/KonoAsset/releases/latest) を開き、`KonoAsset_X.X.X_x64-setup.exe` をクリックしてインストーラーをダウンロードして実行してください  
X.X.X はバージョン情報になっています

> [!WARNING]
> 有料のソフトウェア署名を行っていないため、インストーラー起動時に警告が出ることがあります  
> 続行する場合は「詳細情報」から「実行」を押してください

## 機能
- [x] アセット等を追加して管理
- [x] 追加したアセットのテキスト検索
- [x] BOOTHから商品情報の取得
- [x] アバター素体、アバター関連アセット、ワールドアセットを分けて管理
- [x] アセットにカテゴリ、タグ、メモ、依存関係などを設定可能
- [x] ファイルをドラッグ&ドロップして追加
- [x] zipファイルの自動展開
- [x] Google Drive 等のクラウド同期フォルダを使った複数PC間でのデータ共有

## 複数PC・クラウド同期での利用

このフォークでは、Google Drive・OneDrive・Dropbox などのクラウド同期フォルダにデータ保存先を置くことで、複数の PC から同じアセットデータを利用できます。

### セットアップ方法

1. クラウド同期クライアントのローカル同期フォルダ内に、KonoAsset 用のフォルダ（例: `KonoAssetData`）を作成する
2. KonoAsset の設定画面（歯車アイコン → データ保存先）で、保存先をそのフォルダに変更する
3. 他の PC でも同じクラウドサービスの同期フォルダ内の対応するパスをデータ保存先に指定する

これだけで、どのクラウドサービスでも同じ手順でセットアップできます

### 仕組みと注意点

- 各 PC は自分専用の metadata ファイル（`avatars__<device-id>.json` 等）にのみ書き込み、読み込み時に全 PC 分のファイルを統合して一覧表示します。そのため、**別々の PC がほぼ同時に別々のアセットを追加しても、互いの変更が上書きされることはありません**
- metadata ファイルの書き込みはアトミック（一時ファイル + リネーム）に行われるため、クラウド同期クライアントが書き込み途中のファイルを拾って壊れたコピーが同期される心配もありません
- 一覧画面右上の更新ボタン、または設定画面の「自動更新の間隔」で、他 PC がクラウド経由で加えた変更を取り込めます
- サイドバーの「登録デバイス」フィルタで、どの PC が登録したアセットかで絞り込めます
- Windows 環境では、クラウド未取得（プレースホルダー）のアセットにバッジが表示されます（正確な進捗率は OS の仕様上取得できないため、未取得 / 一部取得 / 取得済みの3段階表示です）

> [!NOTE]
> **同じアセットを複数 PC からほぼ同時に編集する運用は想定していません。** 編集は基本的に 1 台の PC で行い、他 PC では更新ボタンで反映を確認してから作業してください

## ライセンス
本アプリケーションは [MIT License](./LICENSE) で提供されています
