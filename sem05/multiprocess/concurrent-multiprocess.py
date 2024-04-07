import time
import concurrent.futures

start = time.perf_counter() #Performance counter
threads = list()

def func(seconds):
    print(f'Sleeping {seconds} seconds')
    time.sleep(seconds)
    return 'Wake up!'

with concurrent.futures.ProcessPoolExecutor() as executor:
    secs = [5, 4, 3, 2, 1]
    #f1 = executor.submit(func,1)
    #f2 = executor.submit(func,1)
    results = [executor.submit(func,sec) for sec in secs]
    for f in concurrent.futures.as_completed(results):
        print(f.result()) #resultados são em ordens de criação de threads

    #executor.map(func,secs) executa a função como argumento os elementos de sec como threads
    #print(f1.result())
    #print(f2.result())

done = time.perf_counter()

print(f'Finished in {round(done-start,2)}s!')