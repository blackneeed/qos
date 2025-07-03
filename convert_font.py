file, out = "", ""
height = 16

with open(file, 'rb') as f:
    data = f.read()

if len(data) / height > 255.0:
    data = data[:255 * height]

data = height.to_bytes(length=1, signed=False, byteorder='little') + (len(data) // height).to_bytes(length=1, signed=False, byteorder='little') + data

with open(out, 'wb') as f:
    f.write(data)
