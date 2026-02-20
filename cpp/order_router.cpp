#include <iostream>
#include <string>

extern "C" {

struct OrderResult {
    const char* order_id;
    double price;
    double quantity;
    bool success;
};

OrderResult send_limit_order(const char* symbol, bool buy, double price, double quantity) {
    std::cout << "Sending order: " << (buy ? "BUY" : "SELL") << " " << quantity << " of "
              << symbol << " at " << price << std::endl;
    // In production, this would connect to a FIX gateway.
    return OrderResult{"dummy123", price, quantity, true};
}

} // extern "C"
