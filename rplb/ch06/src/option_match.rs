#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    // rest of the states
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn test_coin() {
    let change = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("I have {} cents", change);
}
