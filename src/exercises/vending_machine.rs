use std::io::{self, Write};

pub struct VendingMachine {
    one_credit_coin: u32,
    two_credit_coin: u32,
    five_credit_coin: u32,
    products: Vec<Product>,
    money_inserted: u32,
}

struct Product {
    name: String,
    price: u32,
}

impl VendingMachine {
    fn new(one_credit: u32, two_credit: u32, five_credit: u32) -> VendingMachine {
        VendingMachine {
            one_credit_coin: one_credit,
            two_credit_coin: two_credit,
            five_credit_coin: five_credit,
            products: vec![
                Product {
                    name: String::from("Soda"),
                    price: 5,
                },
                Product {
                    name: String::from("Chips"),
                    price: 3,
                },
                Product {
                    name: String::from("Candy Bar"),
                    price: 2,
                },
                Product {
                    name: String::from("Water"),
                    price: 1,
                },
            ],
            money_inserted: 0,
        }
    }

    fn display_products(&self) {
        println!("Products available for purchase:");
        for (index, product) in self.products.iter().enumerate() {
            println!("{}. {} - {} credits", index + 1, product.name, product.price);
        }
    }

    fn insert_coin(&mut self, coin_value: u32) -> Result<u32, String> {
        match coin_value {
            1 => {
                self.one_credit_coin += 1;
                self.money_inserted += 1;
                Ok(1)
            }
            2 => {
                self.two_credit_coin += 1;
                self.money_inserted += 2;
                Ok(2)
            }
            5 => {
                self.five_credit_coin += 1;
                self.money_inserted += 5;
                Ok(5)
            }
            _ => Err(String::from("Invalid coin value. Only 1, 2, and 5 are accepted.")),
        }
    }

    fn can_give_change(&self, change: u32) -> bool {
        let mut remaining_change = change;
        
        let five_coin_count = remaining_change / 5;
        let two_coin_count = (remaining_change % 5) / 2;
        let one_coin_count = (remaining_change % 5 % 2) / 1;
        
        if five_coin_count <= self.five_credit_coin
            && two_coin_count <= self.two_credit_coin
            && one_coin_count <= self.one_credit_coin {
            return true;
        }

        false
    }

    fn refund(&mut self) {
        println!("Refunding {} credits.", self.money_inserted);
        self.money_inserted = 0;
    }

    fn buy_product(&mut self, product_index: usize) -> Result<String, String> {
        if product_index < 1 || product_index > self.products.len() {
            return Err(String::from("Invalid product selection."));
        }

        let product = &self.products[product_index - 1];
        if self.money_inserted < product.price {
            return Err(String::from("Not enough money to buy this product."));
        }

        let change = self.money_inserted - product.price;

        if !self.can_give_change(change) {
            self.refund();
            return Err(String::from("Not enough change available. Your money has been refunded."));
        }

        let mut remaining_change = change;

        let five_coin_count = remaining_change / 5;
        self.five_credit_coin -= five_coin_count;
        remaining_change %= 5;

        let two_coin_count = remaining_change / 2;
        self.two_credit_coin -= two_coin_count;
        remaining_change %= 2;

        let one_coin_count = remaining_change;
        self.one_credit_coin -= one_coin_count;

        self.money_inserted = 0;

        Ok(format!(
            "You bought a {}. Your change is {} credits.",
            product.name, change
        ))
    }
}

pub fn main() {
    let mut machine = VendingMachine::new(12, 7, 10);
    println!("Welcome to the Vending Machine!");

    loop {
        println!("\nCurrent inserted money: {} credits", machine.money_inserted);
        println!("Choose an option:");
        println!("1. Display products");
        println!("2. Insert a coin (1, 2, or 5 credits)");
        println!("3. Buy a product");
        println!("4. Exit");

        let mut choice = String::new();
        print!("Your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                machine.display_products();
            }
            "2" => {
                println!("Enter coin value (1, 2, or 5):");
                let mut coin_value = String::new();
                io::stdin().read_line(&mut coin_value).unwrap();
                let coin_value: u32 = match coin_value.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Invalid coin value. Please enter 1, 2, or 5.");
                        continue;
                    }
                };

                match machine.insert_coin(coin_value) {
                    Ok(_) => println!("Coin inserted successfully!"),
                    Err(msg) => println!("{}", msg),
                }
            }
            "3" => {
                println!("Enter product number to buy:");
                let mut product_index = String::new();
                io::stdin().read_line(&mut product_index).unwrap();
                let product_index: usize = match product_index.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Invalid product number.");
                        continue;
                    }
                };

                match machine.buy_product(product_index) {
                    Ok(message) => println!("{}", message),
                    Err(message) => println!("{}", message),
                }
            }
            "4" => {
                println!("Thank you for using the Vending Machine!");
                break;
            }
            _ => {
                println!("Invalid option, please choose a valid one.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vending_machine() {
        let mut machine = VendingMachine::new(12, 7, 10);
        assert_eq!(machine.one_credit_coin, 12);
        assert_eq!(machine.two_credit_coin, 7);
        assert_eq!(machine.five_credit_coin, 10);
        assert_eq!(machine.money_inserted, 0);
        assert_eq!(machine.products.len(), 4);
        assert_eq!(machine.products[0].name, "Soda");
        assert_eq!(machine.products[0].price, 5);
    }

    #[test]
    fn test_insert_coin() {
        let mut machine = VendingMachine::new(12, 7, 10);
        let result = machine.insert_coin(2);
        assert_eq!(result, Ok(2));
        assert_eq!(machine.money_inserted, 2);
        assert_eq!(machine.two_credit_coin, 8);
    }

    #[test]
    fn test_invalid_coin_insertion() {
        let mut machine = VendingMachine::new(12, 7, 10);
        let result = machine.insert_coin(3);
        assert_eq!(result, Err("Invalid coin value. Only 1, 2, and 5 are accepted.".to_string()));
        assert_eq!(machine.money_inserted, 0);
    }

    #[test]
    fn test_buy_product_success() {
        let mut machine = VendingMachine::new(12, 7, 10);
        machine.insert_coin(5).unwrap();
        let result = machine.buy_product(1);
        assert_eq!(result, Ok("You bought a Soda. Your change is 0 credits.".to_string()));
        assert_eq!(machine.money_inserted, 0);
    }

    #[test]
    fn test_buy_product_not_enough_money() {
        let mut machine = VendingMachine::new(12, 7, 10);
        machine.insert_coin(2).unwrap();
        let result = machine.buy_product(1);
        assert_eq!(result, Err("Not enough money to buy this product.".to_string()));
        assert_eq!(machine.money_inserted, 2);
    }

    #[test]
    fn test_buy_product_no_change() {
        let mut machine = VendingMachine::new(0, 0, 0);
        machine.insert_coin(5).unwrap();
        let result = machine.buy_product(4);
        assert_eq!(result, Err("Not enough change available. Your money has been refunded.".to_string()));
        assert_eq!(machine.money_inserted, 0);
    }

    #[test]
    fn test_invalid_product_selection() {
        let mut machine = VendingMachine::new(12, 7, 10);
        let result = machine.buy_product(10);
        assert_eq!(result, Err("Invalid product selection.".to_string()));
    }

    #[test]
    fn test_can_give_change_enough() {
        let mut machine = VendingMachine::new(12, 7, 10);
        let result = machine.can_give_change(4);
        assert_eq!(result, true);
    }

    #[test]
    fn test_can_give_change_not_enough() {
        let mut machine = VendingMachine::new(12, 7, 10);
        machine.five_credit_coin = 0; 
        let result = machine.can_give_change(7); 
        assert_eq!(result, false);
    }
}
