file = "VGA8.F16"
width = 1
height = 16
out = "VGA8.FONT"

with open(file, 'rb') as f:
    data = f.read()

if len(data) / (height * width) > 255.0:
    data = data[:255 * (height * width)]

data = width.to_bytes(length=1, signed=False, byteorder='little') + height.to_bytes(length=1, signed=False, byteorder='little') + (len(data) // (height * width)).to_bytes(length=1, signed=False, byteorder='little') + data

with open(out, 'wb') as f:
    f.write(data)
