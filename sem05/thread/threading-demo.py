import time
from threading import Thread

start = time.perf_counter() #Performance counter

def func():
    print('Sleeping 1 sec')
    time.sleep(1)
    print('Wake up!')

#func()

t1 = Thread(target=func)
t2 = Thread(target=func)

t1.start()
t2.start()
t1.join()
t2.join()

done = time.perf_counter()

print(f'Finished in {round(done-start,2)}s!')