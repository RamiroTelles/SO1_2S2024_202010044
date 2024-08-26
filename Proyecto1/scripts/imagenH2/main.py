import time
large_list = []

for i in range(100000):
    large_list.append([j for j in range(100)])
while True:
    time.sleep(1)