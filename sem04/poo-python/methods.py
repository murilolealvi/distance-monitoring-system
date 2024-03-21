class Item:   #instanciando objeto Item
    def calculate_total_price(self):
        total = self.price * self.quantity
        return total

    


item1 = Item()
item1.name = "Notebook"
item1.price = 3500
item1.quantity = 5

print(item1.calculate_total_price()) #objeto da classe Item