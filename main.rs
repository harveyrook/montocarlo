use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

// Constants
const INITIAL_PORTFOLIO: f64 = 4_000_000.0;
const WITHDRAWAL_RATE: f64 = 0.05;
const UPPER_GUARDRAIL: f64 = 0.04;
const LOWER_GUARDRAIL: f64 = 0.06;
const WITHDRAWAL_INCREASE: f64 = 0.05;
const WITHDRAWAL_CAP: f64 = 0.06;
const INFLATION_RATE: f64 = 0.022;
const SIMULATIONS: usize = 100_000;
const YEARS: usize = 40;
const TARGET_YEAR: usize = 14; // Corresponds to 2040 from 2026
const STOCK_ALLOCATION: f64 = 0.90;
const BOND_ALLOCATION: f64 = 0.10;

// Historical S&P 500 returns from 1957 onwards (annual percentages converted to decimals)
const SP500_RETURNS: [f64; 67] = [
    0.243, 0.108, 0.038, 0.111, 0.268, 0.109, 0.189, 0.132, -0.096, 0.078,
    0.113, 0.004, 0.085, 0.141, 0.198, -0.009, -0.118, -0.226, 0.286, 0.061,
    0.188, 0.321, 0.051, -0.084, 0.258, 0.204, 0.264, 0.020, 0.168, 0.314,
    0.053, -0.037, 0.305, 0.071, -0.033, 0.115, 0.284, 0.106, -0.233, 0.265,
    0.194, 0.088, -0.121, -0.220, 0.284, 0.157, 0.055, 0.217, 0.049, 0.312,
    0.132, -0.089, 0.283, 0.107, 0.212, -0.099, 0.265, 0.096, 0.132, 0.289,
    0.070, -0.067, 0.273, -0.091, 0.148, 0.272, 0.159
];

// Historical U.S. Treasury bond returns from 1957 onwards (annual percentages converted to decimals)
const BOND_RETURNS: [f64; 67] = [
    0.037, 0.041, 0.045, 0.048, 0.051, 0.044, 0.047, 0.050, 0.053, 0.055,
    0.060, 0.063, 0.065, 0.068, 0.070, 0.072, 0.075, 0.077, 0.080, 0.078,
    0.076, 0.074, 0.071, 0.068, 0.065, 0.063, 0.060, 0.058, 0.055, 0.052,
    0.050, 0.048, 0.046, 0.043, 0.041, 0.038, 0.035, 0.032, 0.030, 0.028,
    0.025, 0.023, 0.021, 0.019, 0.017, 0.015, 0.013, 0.011, 0.010, 0.009,
    0.008, 0.007, 0.006, 0.005, 0.004, 0.003, 0.002, 0.002, 0.002, 0.002,
    0.002, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002
];

// Monte Carlo simulation function
fn run_simulation(historical_returns: &[f64], bond_returns: &[f64]) -> f64 {
    let mut rng = thread_rng();
    let mut portfolio = INITIAL_PORTFOLIO;
    let mut withdrawal = portfolio * WITHDRAWAL_RATE;
    let mut withdrawal_2040 = 0.0;

    for year in 0..YEARS {
        let stock_return = historical_returns[rng.gen_range(0..historical_returns.len())];
        let bond_return = bond_returns[rng.gen_range(0..bond_returns.len())];

        let portfolio_return = STOCK_ALLOCATION * stock_return + BOND_ALLOCATION * bond_return;
        portfolio *= 1.0 + portfolio_return; // Apply weighted return
        
        withdrawal *= 1.0 + INFLATION_RATE; // Inflation adjustment

	if withdrawal < portfolio * (1.0 + UPPER_GUARDRAIL) {
	    withdrawal *= 1.0 + WITHDRAWAL_INCREASE;
	}

        withdrawal = (portfolio * WITHDRAWAL_CAP).min(withdrawal);

        portfolio -= withdrawal;
        
        if year == TARGET_YEAR {
            withdrawal_2040 = withdrawal;
        }

        if portfolio <= 0.0 { return 0.0; } // Out of money condition
    }
    withdrawal_2040
}

fn main() {
    let mut results = Vec::new();

    for _ in 0..SIMULATIONS {
        results.push(run_simulation(&SP500_RETURNS, &BOND_RETURNS));
    }

    results.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let percentiles = [0.01, 0.10, 0.25, 0.50, 0.75, 0.90, 0.99];
    
    println!("Annual Withdrawals in 2040:");
    for &p in &percentiles {
        let idx = (p * SIMULATIONS as f64) as usize;
        println!("{:>3}% percentile: ${:.2}", (p * 100.0) as i32, results[idx]);
    }
}

