mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, heatmap-wasm!");
}

#[inline]
fn profit_factor_from_sums(gross_profit: f64, gross_loss_abs: f64) -> f64 {
    // Profit factor is typically defined as:
    //   PF = gross_profit / gross_loss_abs
    //
    // Where gross_loss_abs is the absolute magnitude of losses (positive number).
    // Common edge-case handling for optimization/fitness:
    // - no losses + some profit => +inf
    // - no losses + no profit   => 0
    if gross_loss_abs == 0.0 {
        if gross_profit == 0.0 {
            return 0.0;
        }
        return f64::INFINITY;
    }
    if gross_profit == 0.0 {
        return 0.0;
    }
    gross_profit / gross_loss_abs
}

/// Compute profit factor from a list of per-trade (or per-bar) PnL values.
///
/// - Positive PnL contributes to gross profit.
/// - Negative PnL contributes to gross loss by its absolute value.
#[wasm_bindgen]
pub fn profit_factor(pnls: &[f64]) -> f64 {
    utils::set_panic_hook();

    let mut gross_profit = 0.0;
    let mut gross_loss_abs = 0.0;

    for &p in pnls {
        if p > 0.0 {
            gross_profit += p;
        } else if p < 0.0 {
            gross_loss_abs += -p;
        }
    }

    profit_factor_from_sums(gross_profit, gross_loss_abs)
}

/// Compute profit factor for many parameter combinations at once.
///
/// Your worker can precompute `gross_profits[i]` and `gross_losses_abs[i]`
/// for each (min,max) pyramiding combination (or any other parameter combo),
/// and this function returns profit factor per index.
///
/// Notes:
/// - `gross_losses_abs` should be positive magnitudes (abs of summed losses).
/// - If you accidentally pass negative values, they will be abs()'d.
#[wasm_bindgen]
pub fn profit_factor_batch(gross_profits: &[f64], gross_losses_abs: &[f64]) -> Vec<f64> {
    utils::set_panic_hook();

    let n = gross_profits.len().min(gross_losses_abs.len());
    let mut out = Vec::with_capacity(n);

    for i in 0..n {
        let gp = gross_profits[i];
        let gl = gross_losses_abs[i].abs();
        out.push(profit_factor_from_sums(gp, gl));
    }

    out
}
