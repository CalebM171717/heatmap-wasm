//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use heatmap_wasm::{profit_factor, profit_factor_batch};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn profit_factor_basic() {
    let pnls = vec![10.0, -5.0, 2.0, -1.0];
    // gross profit = 12, gross loss abs = 6 => PF = 2
    assert_eq!(profit_factor(&pnls), 2.0);
}

#[wasm_bindgen_test]
fn profit_factor_edge_cases() {
    assert_eq!(profit_factor(&[]), 0.0);
    assert_eq!(profit_factor(&[1.0, 2.0]), f64::INFINITY);
    assert_eq!(profit_factor(&[-1.0, -2.0]), 0.0);
}

#[wasm_bindgen_test]
fn profit_factor_batch_basic() {
    let gp = vec![12.0, 0.0, 5.0];
    let gl = vec![6.0, 10.0, 0.0];
    let out = profit_factor_batch(&gp, &gl);
    assert_eq!(out.len(), 3);
    assert_eq!(out[0], 2.0);
    assert_eq!(out[1], 0.0);
    assert_eq!(out[2], f64::INFINITY);
}
