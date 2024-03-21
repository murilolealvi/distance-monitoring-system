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

    def __repr__(self): #maneira de representar o objeto
        return f"Item: {self.name} \tR${self.price} \t{self.quantity}"

class Phone(Item): #herança é feito ao passar como argumento a classe-mãe
    instances = list()
    def __init__(self, name: str, price:float, quantity=0, broken_phones=0):
        #validações
        assert price >= 0 #não usado em produção, apenas em debugging
        assert quantity >= 0
        assert broken_phones >= 0
        #atributos
        self.name = name
        self.price = price
        self.quantity = quantity
        self.broken_phones = broken_phones

        Phone.instances.append(self)

class Keyboard(Item):
    def __init__(self, name: str, price: float, quantity =0, abnt2=False):
        super().__init__( #herda atributos da classe mãe
            name, price, quantity
        )
        self.abnt2 = abnt2

        Keyboard.instances.append(self)
        


phone1 = Phone("J5 PRIME", 250, 2)
phone2 = Phone("MOTO G5", 350, 9)

print(phone1.calculate_total_price()) #method inherited

keyb1 = Keyboard("Logitech Wireless", 100, 2)

print(keyb1.calculate_total_price())

print(keyb1.instances) #repr and instances list inherited