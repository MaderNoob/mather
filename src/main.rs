use bigdecimal::{BigDecimal, FromPrimitive};
use num_bigint::BigInt;

mod interpreter;
fn main() {
    let x=BigDecimal::new(BigInt::from_i64(2543).unwrap(),-1);
    println!("{}",x);
}
