import random

parts = [hex(random.randint(0, 255)).removeprefix("0x").zfill(2) for _ in range(6)]
print("{}:{}:{}:{}:{}:{}".format(*parts))
