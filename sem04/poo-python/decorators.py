import csv

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
    
    @classmethod #decorator 
    def instantiate_from_csv(cls, csv: str): #cls refer to the Class
        with open (csv, 'r') as file:
            reader = csv.DictReader(file)
            items = list(reader)
        for item in items:
            Item(  #constructor
                name=item.get('name'),
                price=float(item.get('price')),
                quantity=int(item.get('quantity'))
            )

    @staticmethod #bound to a class rather than the objects
    #self -> CLASS
    def is_integer(num):
        if isinstance(num, float): #check if the object is an instance of a class
            return num.is_integer() #True or False
        elif isinstance(num, int):
            return True
        else:
            return False
        
    @property
    def name(self): #acesso apenas de leitura do atributo
        return self.name
    #name to _name turns the attribute privative
    #name to __name prevents to be visible in instance level

    @name.setter
    def name(self, value):
        self.name = value


Item.instantiate_from_csv("items.csv")

for instance in Item.instances:
    print(instance) #objetos da classe Item instanciados
