use tokio::time;
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let (tx_realized, rx_realized) = watch::channel(0.2);
    let (tx_depth, rx_depth) = watch::channel(1_000_000.0);

    // Simulate updating realized volatility
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let new_vol = 0.15 + rand::random::<f64>() * 0.1;
            let _ = tx_realized.send(new_vol);
        }
    });

    // Simulate updating DeFi pool depth
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(3));
        loop {
            interval.tick().await;
            let new_depth = 800_000.0 + rand::random::<f64>() * 400_000.0;
            let _ = tx_depth.send(new_depth);
        }
    });

    let monitor = GammaRiskMonitor {
        net_vega: 100_000.0,
        implied_vol: 0.20,
        realized_vol_rx: rx_realized,
        defi_pool_depth_rx: rx_depth,
        risk_limit: 50_000.0,
    };
    monitor.run().await;
}

struct GammaRiskMonitor {
    net_vega: f64,
    implied_vol: f64,
    realized_vol_rx: watch::Receiver<f64>,
    defi_pool_depth_rx: watch::Receiver<f64>,
    risk_limit: f64,
}

impl GammaRiskMonitor {
    async fn run(mut self) {
        let mut interval = time::interval(time::Duration::from_millis(100));
        loop {
            interval.tick().await;
            let realized = *self.realized_vol_rx.borrow();
            let depth = *self.defi_pool_depth_rx.borrow();

            let pnl_rate = self.net_vega * (realized.powi(2) - self.implied_vol.powi(2));

            if pnl_rate.abs() > self.risk_limit || depth < 500_000.0 {
                eprintln!(
                    "[RISK ALERT] PnL rate: {:.2}, Realized: {:.2}, Implied: {:.2}, Depth: {:.2}",
                    pnl_rate, realized, self.implied_vol, depth
                );
            }
        }
    }
}
