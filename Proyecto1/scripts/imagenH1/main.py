import time
import random
var=5
while True:
    
    result = random.random()
    for i in range(10000):
        result += i / (i+1)

    time.sleep(0.1)