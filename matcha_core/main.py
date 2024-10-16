import onnxruntime


session = onnxruntime.InferenceSession("vocoder.onnx")


print([input.name for input in session.get_inputs()])
print([output.name for output in session.get_outputs()])