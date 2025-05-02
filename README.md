# bevy-timing-game

`bevy-timing-game`は、ゲームエンジン`Bevy`で作られたタイミングゲームです。

## ゲーム概要

左右交互に動くキューをタイミングよく真ん中にクリックするゲームです。

真ん中に近いほどより多くのポイントを得ることができ、
制限時間10秒の間に10ポイント集めてゲームクリアを目指します。

## ゲーム情報

ゲームタイトル `いっとくタイミングゲーム`

画面サイズ `640x480`

## 遊び方

リポジトリをクローンしてから、`cargo run`を実行することで遊ぶことができます。

## 操作方法

- ゲームを始める、タイミングを決める: 左クリック
- 画面を遷移する: キーボード

## Wasm変換

`./wasm.sh`を実行することでゲームを`Web Assembly`に変換することができます。

## クレジット

開発者 [ittokunvim](https://github.com/ittokunvim)

ゲームエンジン [Bevy](https://bevyengine.org)

タイルセット [SunnyLand](https://ansimuz.itch.io/sunny-land-pixel-game-art)

フォント [美咲フォント](https://littlelimit.net/misaki.htm)

タイミングボタン画像 [いらすとや](https://www.irasutoya.com/)

ポーズボタン画像 [ICOOON MONO](https://icooon-mono.com/)

効果音 [効果音ラボ](https://soundeffect-lab.info)

タイミングボイス音源 [ゆくも！](https://www.yukumo.net/)

画像編集 [Pixlr](https://pixlr.com)

Wasm変換 [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)
