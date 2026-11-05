#!/usr/bin/env python3
"""
test_sigma_delta.py: Matrix Collapse and Dither Diagnostic Testing Suite
Verifies the mathematical stability of 80D trajectories under quantization.
"""

import numpy as np

MANIFOLD_DIM = 80
ALPHA_CEILING = 4.5
LAMBDA_REGULARIZATION = 0.0015

def uniform_mid_tread_quantizer(accumulator):
    """Uniform two-threshold mid-tread quantization operator (Delta = 1.0)"""
    if accumulator > 0.5:
        return 1.0
    elif accumulator < -0.5:
        return -1.0
    else:
        return 0.0

def run_quantization_test(matrix_A, use_dither=False):
    """Simulates the 80D Sigma-Delta Loop across a tracking matrix frame"""
    quantized_matrix = np.zeros_like(matrix_A)
    error_feedback = np.zeros(MANIFOLD_DIM)
    
    # Static seed for reproducibility across testing epochs
    rng = np.random.default_rng(seed=42)
    
    for i in range(MANIFOLD_DIM):
        for j in range(MANIFOLD_DIM):
            x_n = matrix_A[i, j]
            
            # Apply Triangular Dither if toggled on
            dither_n = 0.5 * (rng.random() - rng.random()) if use_dither else 0.0
            
            dynamic_accumulator = x_n + error_feedback[j] + dither_n
            y_n = uniform_mid_tread_quantizer(dynamic_accumulator)
            
            quantized_matrix[i, j] = y_n
            # Save serialization error (decorrelated or systematic depending on dither status)
            error_feedback[j] = (x_n + error_feedback[j]) - y_n
            
    return quantized_matrix

def compute_tikhonov_condition(matrix_A, lam):
    """Computes the regularized condition number kappa_2"""
    # A_lambda = A^T * A + lambda * I_80
    A_lambda = np.dot(matrix_A.T, matrix_A) + lam * np.eye(MANIFOLD_DIM)
    # Extract regularized singular values via standard SVD
    singular_values = np.linalg.svd(A_lambda, compute_uv=False)
    return singular_values[0] / singular_values[-1]

if __name__ == "__main__":
    print("=" * 70)
    print(" CONSTELLATION-MANIFOLD V4.0 PYTHON NUMERICAL TESTING MATRIX")
    print("=" * 70)
    
    # Generate a smooth, continuous trajectory matrix representing human intent / AI design
    # Using an ill-conditioned random matrix base to simulate typical complex geometries
    base_matrix = np.random.default_rng(12345).standard_normal((MANIFOLD_DIM, MANIFOLD_DIM))
    u, s, vh = np.linalg.svd(base_matrix)
    s_smooth = np.linspace(10, 1, MANIFOLD_DIM) # Well-behaved initial singular spectrum
    A_continuous = np.dot(u * s_smooth, vh)
    
    raw_kappa = np.linalg.cond(A_continuous)
    print(f"[+] Initial Continuous Matrix Condition Number (κ₂): {raw_kappa:.4f}")
    
    # Test Block 1: Legacy Un-dithered Quantization Profile
    A_legacy_quantized = run_quantization_test(A_continuous, use_dither=False)
    kappa_legacy_raw = np.linalg.cond(A_legacy_quantized)
    print(f"[-] Legacy Un-dithered Raw Condition Number (κ₂): {kappa_legacy_raw:.4f}")
    if kappa_legacy_raw > ALPHA_CEILING:
        print(f"    🚨 EMERGENCY VALVE CRASH: Raw condition number breaches alpha limit ({ALPHA_CEILING})!")
        
    # Test Block 2: Updated Dithered Quantization Profile
    A_v4_quantized = run_quantization_test(A_continuous, use_dither=True)
    kappa_v4_raw = np.linalg.cond(A_v4_quantized)
    print(f"[+] Updated Dithered Raw Condition Number (κ₂): {kappa_v4_raw:.4f}")
    
    # Test Block 3: Tikhonov Regularized Safety Intercept
    kappa_v4_regularized = compute_tikhonov_condition(A_v4_quantized, LAMBDA_REGULARIZATION)
    print(f"[+] Updated Tikhonov-Regularized Condition Number (κ₂_λ): {kappa_v4_regularized:.4f}")
    
    if kappa_v4_regularized <= ALPHA_CEILING:
        print(f"    ✅ STATUS NOMINAL: Tikhonov framework clamped noise safely under ceiling!")
    else:
        print(f"    ❌ SAFETY FAULT: Regularization coefficients insufficient.")
    print("=" * 70)
