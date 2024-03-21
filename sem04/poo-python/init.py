class Item:   #instanciando objeto Item
    def __init__(self, name, price, quantity = 0): #default value 
        #construtor pelo magic method __init__
        self.name = name
        self.price = price
        self.quantity = quantity

    def calculate_total_price(self): #método
        total = self.price * self.quantity
        return total

    


item1 = Item("Notebook", 3500, 5)

item2 = Item("Celular", 1500, 5)

item3 = Item("Tablet", "2500", 5) #não impede de declarar tipos diferentes

item1.has_numpad = False #não impede de definir mais atributos fora do construtor

print(item3.calculate_total_price()) 