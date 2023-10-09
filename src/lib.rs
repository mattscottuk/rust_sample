//Financial Calculations Library
//03 October 2023
//Matt Scott

pub fn calculate_interest_accrual(principal: i64, interest: f64) -> f64 {
    let days = 365;
    let days_float = days as f64;
    let principal_float = principal as f64;
    let daily_interest: f64 = interest / days_float;
    let ret_val = daily_interest * principal_float;
    return ret_val;
}

pub fn calculate_monthly_interest(daily_interest: f64, month_days: i64) -> f64 {
    let month_days_float = month_days as f64;
    let monthly_interest = daily_interest * month_days_float;
    return monthly_interest;
}

pub fn calculate_available_credit(  purchase_balance: i64,
                                    cash_balance: i64,
                                    bt_balance: i64,
                                    mt_balance: i64,
                                    other_balance: i64,
                                    credit_limit: i64) -> bool {
    
    let total_balance = purchase_balance + cash_balance + bt_balance + mt_balance + other_balance;
    
    //let status:bool = false;

    if total_balance > credit_limit {
        println!("DEBUG: Decline - Insufficient Funds");
        return false;
    }
    else {
        println!("DEBUG: Approve - Within Credit Limit");
        return true;
    }

}

pub fn calculate_available_balance() {}

pub fn apply_rt_transaction_fee() {}

pub fn apply_nrt_transaction_fee() {}

pub fn convert_foreign_currency_value() {}

pub fn format_value_float2int() {}

pub fn format_value_int2string() {}