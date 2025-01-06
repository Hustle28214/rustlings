// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }


fn calculate_price_of_apples(quantity: i32) -> i32 {
    if quantity > 40 {
        return quantity;  // 如果购买数量超过40，每个苹果的价格是1 rustbuck
    } else {
        return quantity * 2;  // 否则每个苹果的价格是2 rustbucks
    }
}

fn main() {
    let price = calculate_price_of_apples(35);
    println!("The total price is {}", price);
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
