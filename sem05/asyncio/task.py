import asyncio

async def main(): 
    print('murilo')
    task = asyncio.create_task(foo('testando'))
    print('done') #feito durante execução do contexto de foo
    await task
    print('task awaited') #feito após execução(await)

async def foo(text):
    print(text)
    await asyncio.sleep(1)

asyncio.run(main())