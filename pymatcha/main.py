from pymatcha import Matcha


with open("model.onnx", "rb") as f:
    model = f.read()

with open("vocoder.onnx", "rb") as f:
    vocoder = f.read()


matcha = Matcha(model, vocoder)
symbols = matcha.preprocess("おはようございます")
mel, mel_lengths = matcha.synthesise(symbols)
wav = matcha.decode(mel, mel_lengths)


with open("output.wav", "wb") as f:
    f.write(wav)
