class Item:   #instanciando objeto Item
    discount = 0.8 #desconto
    instances = list()
    def __init__(self, name: str, price: float, quantity = 0): #default value 
        #validações
        assert price >= 0 #não usado em produção, apenas em debugging
        assert quantity >= 0
        #atributos
        self.name = name
        self.price = price
        self.quantity = quantity
        #append instances
        Item.instances.append(self)

    def calculate_total_price(self): #método
        total = self.price * self.quantity
        return total
    
    def apply_discount(self):
        self.price = self.price * self.discount

    


item1 = Item("Notebook", 3500, 5)

item2 = Item("Celular", 1500, 5)

# item3 = Item("Tablet", "2500", 5) vai dar erro dado o assert

item3 = Item("Teclado", 50, 3)

item4 = Item("Mouse", 30, 2)

item1.has_numpad = False #não impede de definir mais atributos fora do construtor

item1.apply_discount()
print(item1.price, item1.discount)
item2.discount = 0.5
item2.apply_discount()
print(item2.price, item2.discount)

for instance in Item.instances:
    print(instance) #objetos da classe Item instanciados
    #não muito legíveis
