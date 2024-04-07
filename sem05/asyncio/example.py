import asyncio

async def fetch_data():
    print('start fetching...')
    await asyncio.sleep(2)
    print('done fetching')
    return {'data': 1}

async def print_numbers():
    for i in range(10):
        print(i)
        await asyncio.sleep(0.25)

async def main():
    task1 = asyncio.create_task(fetch_data()) #ou ensure_future
    task2 = asyncio.create_task(print_numbers()) 
    # um conjunto de tasks podem ser passadas a uma lista
    # e assim utilizar await async.gather(*tasks) p/ aguardar todas serem
    # concluídas e clock ser reaproveitado, assim obtêm-se concorrência


    # asyncio.run(foo()) pode ser usada para identificar o event loop
    # e fechar automaticamente quando o contexto for completado(sobe um thread)

    value = await task1 #ensure task1 is finished
    print(value) #task print_numbers is done until the 7th number
    await task2 #ensure task2 is finished and pass clock to others functionalities

asyncio.run(main())