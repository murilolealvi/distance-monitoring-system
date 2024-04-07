import time
from multiprocessing import Process

start = time.perf_counter() #Performance counter

def func():
    print('Sleeping 1 sec')
    time.sleep(1)
    print('Wake up!')

#func()
p1 = Process(target=func)
p2 = Process(target=func)

p1.start()
p2.start()
p1.join()
p2.join()

done = time.perf_counter()

print(f'Finished in {round(done-start,2)}s!') #ambos terminam no mesmo tempo