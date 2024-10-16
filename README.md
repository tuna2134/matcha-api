# matcha-api
Matcha-TTS-JPの推論部分をRustに移植しました。

## Prepare onnx model
始める前にonnxモデルを準備しましょう。

### 1, Install vocoder model
vocoderモデルをインストールしましょう。
```bash
$ wget https://huggingface.co/tuna2134/uji-tts_jsut/resolve/main/vocoder.onnx
```

### 2. Bring Matcha(Uji)-TTS onnx model
Matcha-TTSのonnxモデルを持ってきて`model.onnx`に改名してください。

## Run API Server
APIサーバを実行します。
```sh
cargo run -r -p matcha_api # cudaで動かしたい場合-F cudaも追加する
```

## Todo:
- [x] vocoderの実装
- [x] preprocessの実装
- [x] matcha本体の実装
- [ ] C++ライブラリ作成