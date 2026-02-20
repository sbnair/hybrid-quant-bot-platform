use std::ffi::CStr;
use std::os::raw::c_double;

/// Compute fair variance strike from discrete option prices.
/// Exposed to C for calling from Python.
#[no_mangle]
pub extern "C" fn calculate_var_swap_strike(
    strikes_ptr: *const c_double,
    option_prices_ptr: *const c_double,
    n: usize,
    time_to_maturity: c_double,
    forward_price: c_double,
    risk_free_rate: c_double,
) -> c_double {
    // Safety: caller must provide valid pointers and correct length.
    let strikes = unsafe { std::slice::from_raw_parts(strikes_ptr, n) };
    let prices = unsafe { std::slice::from_raw_parts(option_prices_ptr, n) };

    let mut integral = 0.0;
    for i in 0..n - 1 {
        let k_i = strikes[i];
        let k_ip1 = strikes[i + 1];
        let dk = k_ip1 - k_i;
        let avg_price = (prices[i] + prices[i + 1]) / 2.0;
        integral += (2.0 / (k_i * k_ip1)) * avg_price * dk;
    }

    let term1 = (2.0 / time_to_maturity)
        * (risk_free_rate * time_to_maturity
            - (forward_price / strikes[0] - 1.0)
            - (forward_price / strikes[0]).ln());
    let term2 = (2.0 / time_to_maturity) * integral;

    (term1 + term2).sqrt()
}
