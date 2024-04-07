import asyncio
import concurrent.futures

def blocking_io():
    #deve-se utilizar operações ASSÍNCRONAS
    #event loop: operações que são executadas em uma thread pool
    with open('/dev/urandom', 'rb') as f:
        return f.read(100)
    
def cpu_bound():
    #operações limitadas pela CPU serão bloqueadas no event loop,
    #ou seja, não suportam serem executadas assincronamente
    return sum(i**2 for i in range(10**7))

async def main():
    loop = asyncio.get_running_loop()

    #opções:

    result = await loop.run_in_executor(
        None, blocking_io
    ) #running on default loop's executor
    print('default thread pool', result)

    with concurrent.futures.ThreadPoolExecutor() as pool:
        result = await loop.run_in_executor(
            pool, blocking_io
        )
        print('custom thread pool', result) #permite subir outra thread

    with concurrent.futures.ProcessPoolExecutor() as pool:
        result = await loop.run_in_executor(
            pool, cpu_bound
        )
        print('custom process pool', result) #permite operar paralelamente

asyncio.run(main())