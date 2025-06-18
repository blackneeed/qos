with open('disk.img', 'wb') as f:
    for i in range(4096):
        if i % 2 == 0:
            f.write((i // 512).to_bytes(byteorder='big', length=1, signed=False))
        else:
            f.write((0).to_bytes(byteorder='big', length=1, signed=False))