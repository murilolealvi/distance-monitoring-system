import time
from threading import Thread

start = time.perf_counter() #Performance counter
threads = list()

def func(seconds):
    print(f'Sleeping {seconds} seconds')
    time.sleep(seconds)
    print('Wake up!')

for _ in range(10):
    t = Thread(target=func, args=[2]) #args são os argumentos da função target
    t.start()
    threads.append(t)

for thread in threads:
    thread.join()

done = time.perf_counter()

print(f'Finished in {round(done-start,2)}s!')