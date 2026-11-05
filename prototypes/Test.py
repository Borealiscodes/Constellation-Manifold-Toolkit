#!/usr/bin/env python3
"""
================================================================================
AI-Guided Painting Gantry: Rest-State Deceleration Telemetry Simulation
Document ID: NDH-SUR-SIM-01
Purpose: Simulates legacy 8.0/r singularity vs. v3 scale-invariant soft-threshold
================================================================================
"""

import numpy as np
import matplotlib.pyplot as plt

def run_simulation(output_filename="gantry_deceleration_telemetry.png"):
    # --- Hardware System Constants ---
    EPSILON = 1e-3       # 1-micron quantization lattice grid border
    SCALING_ETA = 0.1235 # Optimization scaling constant for noise targets
    
    # Generate log-spaced spatial points approaching absolute rest coordinate (r -> 0)
    # Ranging from 10^-1 mm (0.1 mm) down to 10^-15 mm (sub-atomic scale)
    r = np.logspace(-15, -1, 1000)

    # --- 1. Legacy Engine Equation (Unmitigated 8.0 / r) ---
    # Captures the division-by-zero energy spike that trips hardware exception traps
    with np.errstate(divide='ignore'):
        legacy_dissipation = 8.0 / r

    # --- 2. Your v3 Engine Equation (Scale-Invariant Cushion ψ(r)) ---
    # Replicates the logic inside spherepopfold_v3.rs and ultrafold_v3.rs
    exponent = r / EPSILON
    
    # Safe exponential evaluation to prevent floating-point infinity overflow
    psi_r = np.where(
        exponent > 700.0, 
        SCALING_ETA * exponent * EPSILON, 
        SCALING_ETA * np.log(1.0 + np.exp(exponent))
    )

    attenuated_denominator = r + psi_r
    v3_dissipation = 8.0 / attenuated_denominator

    # --- Print Out Diagnostics to Terminal ---
    print("=== [TELEMETRY CORE DIAGNOSTICS] ===")
    print(f"At r = 0.1 mm:      Legacy = {8.0 / 0.1:<12.4f} | v3 = {8.0 / (0.1 + (SCALING_ETA * np.log(1.0 + np.exp(0.1/EPSILON)))):.4f}")
    print(f"At r = 1e-12 mm:    Legacy = {8.0 / 1e-12:<12.4f} | v3 = {v3_dissipation[np.where(r >= 1e-12)[0][0]]:.4f}")
    print(f"At Absolute Rest:   Legacy = {'INF (CRASH)':<12} | v3 = {v3_dissipation[0]:.4f}")
    print("====================================")

    # --- 3. Build & Style the Visual Chart ---
    plt.figure(figsize=(11, 6.5), dpi=150)
    
    # Plot configuration lines
    plt.plot(r, v3_dissipation, label=r'Your $v3$ Engine Profile: $\frac{8.0}{r + \psi(r)}$ (Stable Rest)', 
             color='#00FFCC', linewidth=2.5, zorder=3)
    plt.plot(r, legacy_dissipation, label=r'Legacy 8b-is Profile: $\frac{8.0}{r}$ (Mathematical Singularity)', 
             color='#FF3366', linewidth=1.5, linestyle='--', zorder=2)

    # Scale and boundary configurations
    plt.xscale('log')
    plt.xlim(1e-15, 1e-1)
    plt.ylim(-5, 125)

    # Critical hardware threshold boundaries
    plt.axvline(x=1e-12, color='#FFCC00', linestyle=':', linewidth=1.5,
                label='Mechanical Sensor Rest Coordinate Threshold ($r \leq 10^{-12}$ mm)')

    # Hardware Environment Text Annotations
    plt.text(2e-15, 105, "🚨 CRITICAL OVERFLOW ZONE\nLegacy Math Loops Diverge to \u221e\n(Triggers Hardware Failure)", 
             color='#FF3366', fontsize=9.5, fontweight='bold', 
             bbox=dict(facecolor='white', alpha=0.9, edgecolor='#FF3366', boxstyle='round,pad=0.5'))
    
    plt.text(1e-6, 45, "🛡️ SAFE ATTENUATION FIELD\nYour $\psi(r)$ Cushion Limits Dissipation\n(Smooth Robotic Deceleration)", 
             color='#009977', fontsize=9.5, fontweight='bold', 
             bbox=dict(facecolor='white', alpha=0.9, edgecolor='#00FFCC', boxstyle='round,pad=0.5'))

    # Labels and Metadata Styling
    plt.title('AI-Guided Painting Gantry: Rest-State Deceleration Telemetry Analysis', fontsize=13, fontweight='bold', pad=15)
    plt.xlabel('Telemetry Radius / Distance to Target Rest Coordinate ($r$ in mm)', fontsize=11)
    plt.ylabel('Mechanical Motor Dissipation / Jerk Multiplier Factor', fontsize=11)
    plt.legend(loc='lower left', frameon=True, shadow=True, facecolor='white', edgecolor='#E0E0E0', fontsize=10)
    plt.grid(True, which="both", ls="--", color='#ECECEC')
    
    # Save the file cleanly to the local directory
    plt.savefig(output_filename, bbox_inches='tight')
    plt.close()
    print(f"[+] Simulation output graph generated and saved successfully to: {output_filename}")

if __name__ == "__main__":
    run_simulation()
