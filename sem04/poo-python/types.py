class Item:   #instanciando objeto Item
    discount = 0.8 #desconto
    def __init__(self, name: str, price: float, quantity = 0): #default value 
        #validações
        assert price >= 0 #não usado em produção, apenas em debugging
        assert quantity >= 0
        #atributos
        self.name = name
        self.price = price
        self.quantity = quantity


    def calculate_total_price(self): #método
        total = self.price * self.quantity
        return total

    


item1 = Item("Notebook", 3500, 5)

item2 = Item("Celular", 1500, 5)

# item3 = Item("Tablet", "2500", 5) vai dar erro dado o assert

item1.has_numpad = False #não impede de definir mais atributos fora do construtor

print(Item.discount)
print(item1.discount) #definido em nível da classe
print(Item.__dict__)
