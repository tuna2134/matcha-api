import requests


res = requests.post("http://localhost:3000/synthesize", json={
    "text": "おはようございます、みなさん"
})
with open("output.wav", "wb") as f:
    f.write(res.content)