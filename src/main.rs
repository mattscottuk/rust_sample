//Financial Calculations Library
//03 October 2023
//Matt Scott

//use financial_calculations::calculate_interest_accrual;
//use financial_calculations::calculate_monthly_interest;

fn main() {
    // let a = calculate_interest_accrual(4999, 0.1675);
    // println!("OUTPUT A Daily Interest Accrual (Potential Revenue) £{a}");
    // let b = calculate_monthly_interest(a, 31);
    // println!("OUTPUT B Monthly Interest Accrual (Potential Revenue) £{b}");
    // let c = b * 12.0;
    // println!("OUTPUT C Yearly Estimated Interest Income £{c}");

    let mut balance_purchase: f64 = 850.799559;
    println!("Purchase Balance (FLOAT): £{balance_purchase}");

    balance_purchase = balance_purchase * 100.0;

    let balance_pur_int = balance_purchase as i64;

    //println!("DEBUG Balance INT: {balance_pur_int}");

    let large_val: i64 = balance_pur_int/100;
    let small_val: i64 = balance_pur_int - (large_val*100);

    let a = String::from("£");
    //println!("{}", a);
    let b = large_val.to_string();
    //println!("{}", b);
    let c = String::from(".");
    //println!("{}", c);
    let d = small_val.to_string();
    //println!("{}", d);
    let out: String = a + &b + &c + &d;

    println!("Purchase Balance (INT): {out}");
}
