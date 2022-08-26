#[allow(dead_code)]
struct Weather<T, S> {
    temp: T,
    humidity: S
}
#[allow(dead_code)]
impl<T, S> Weather<T, S> {
    fn times(self) -> (T, S) {
        (self.temp, self.humidity)
    }
}

pub fn times<T: std::ops::Mul<Output = T>> (x: T, y: T) -> T { // generics
    let _test = Weather {
        temp: 10,
        humidity: 10
    };
    x * y
}

#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 { // match
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}