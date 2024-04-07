import asyncio

async def main(): 
    #o tempo para execução dos eventos do contexto podem ser reaproveitados
    #daí entra funções async
    print('murilo')
    #foo('testando') never awaited to return
    await foo('testando')

async def foo(text):
    print(text)
    await asyncio.sleep(1)

asyncio.run(main())