use crate::exercises::vending_machine::VendingMachine;

#[test]
fn test_create_vending_machine() {
    let machine = VendingMachine::new();
    assert_eq!(machine.money_inserted, 0);
    assert_eq!(machine.one_credit_coin, 12);
    assert_eq!(machine.two_credit_coin, 7);
    assert_eq!(machine.five_credit_coin, 10);
    assert_eq!(machine.products.len(), 4);
}

#[test]
fn test_insert_coin() {
    let mut machine = VendingMachine::new();
    let result = machine.insert_coin(2);
    assert_eq!(result, Ok(2));
    assert_eq!(machine.money_inserted, 2);
    assert_eq!(machine.two_credit_coin, 8);
}

#[test]
fn test_buy_product() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(5).unwrap();
    let result = machine.buy_product(1);
    assert_eq!(result, Ok("You bought a Soda. Your change is 0 credits.".to_string()));
    assert_eq!(machine.money_inserted, 0);
}

#[test]
fn test_not_enough_change() {
    let mut machine = VendingMachine::new();
    machine.insert_coin(5).unwrap();
    machine.insert_coin(2).unwrap();
    let result = machine.buy_product(1);
    assert_eq!(result, Err("Not enough change available. Your money has been refunded.".to_string()));
    assert_eq!(machine.money_inserted, 0);
}
