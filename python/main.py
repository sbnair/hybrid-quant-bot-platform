import ctypes
import numpy as np
import time
import os

# Load Rust pricing library
lib_path = os.path.join(os.path.dirname(__file__), "..", "rust", "target", "release", "libpricing.so")
pricing_lib = ctypes.CDLL(lib_path)
pricing_lib.calculate_var_swap_strike.argtypes = [
    ctypes.POINTER(ctypes.c_double),
    ctypes.POINTER(ctypes.c_double),
    ctypes.c_size_t,
    ctypes.c_double,
    ctypes.c_double,
    ctypes.c_double,
]
pricing_lib.calculate_var_swap_strike.restype = ctypes.c_double

# Load C++ order router library
router_lib_path = os.path.join(os.path.dirname(__file__), "..", "cpp", "build", "liborderrouter.so")
router_lib = ctypes.CDLL(router_lib_path)
router_lib.send_limit_order.argtypes = [
    ctypes.c_char_p,
    ctypes.c_bool,
    ctypes.c_double,
    ctypes.c_double,
]
# For simplicity, treat returned struct as a Python object (in real code you'd define a proper Structure)
router_lib.send_limit_order.restype = ctypes.py_object

def get_market_data():
    """Mock function to simulate fetching options chain and forward price."""
    strikes = np.array([100, 110, 120, 130, 140, 150, 160], dtype=np.float64)
    # OTM call prices (for strikes above forward)
    option_prices = np.array([10, 8, 5, 3, 2, 1.5, 1.2], dtype=np.float64)
    forward_price = 125.0
    time_to_maturity = 30 / 365.0   # 30 days
    risk_free_rate = 0.05
    return strikes, option_prices, forward_price, time_to_maturity, risk_free_rate

def main():
    strikes, prices, F, T, r = get_market_data()

    # Call Rust function
    fair_strike = pricing_lib.calculate_var_swap_strike(
        strikes.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
        prices.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
        len(strikes),
        T,
        F,
        r,
    )
    print(f"Fair variance strike: {fair_strike:.4f}")

    # Compare with market implied (mock)
    market_implied = 0.25  # 25% volatility
    if fair_strike < market_implied * 0.95:  # 5% cheap
        print("Opportunity detected! Sending order...")
        symbol = b"ETH-30DEC23"
        # Call C++ function
        result = router_lib.send_limit_order(symbol, True, market_implied, 10.0)
        print(f"Order result: {result}")
    else:
        print("No opportunity.")

if __name__ == "__main__":
    main()
