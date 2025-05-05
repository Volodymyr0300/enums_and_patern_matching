fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => {
                println!("Lucky nickel!");
                5
            },
            Coin::Dime => {
                println!("Lucky dime!");
                10
            },
            Coin::Quarter => {
                println!("Lucky quarter!");
                25
            },
        }   
        
    }

    

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);
}
