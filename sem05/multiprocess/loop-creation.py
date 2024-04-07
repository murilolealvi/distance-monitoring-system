import time
from multiprocessing import Process

start = time.perf_counter() #Performance counter
Processes = list()

def func(seconds):
    print(f'Sleeping {seconds} seconds')
    time.sleep(seconds)
    print('Wake up!')

for _ in range(10):
    p = Process(target=func, args=[2]) #args são os argumentos da função target
    p.start()
    Processes.append(p)

for Process in Processes:
    Process.join()

done = time.perf_counter()

print(f'Finished in {round(done-start,2)}s!') #terminam juntos