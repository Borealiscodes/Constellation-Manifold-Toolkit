#!/usr/bin/env bash
# ==============================================================================
# NDH-SURREALISM: UNIFIED ARCHITECTURAL EXECUTION ORCHESTRATOR
# Document ID: NDH-SUR-OMNIBUS-SHELL
# Purpose: Compiles Rust primitives, runs Julia/Python tests, bakes G-code, 
#          and generates toolpath visual traces in a single execution tree.
# ==============================================================================

set -euo pipefail

# --- Color Output Triggers ---
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

log_info() { echo -e "${CYAN}[OMNIBUS_INFO]$(date +'%H:%M:%S')${NC} $1"; }
log_succ() { echo -e "${GREEN}[OMNIBUS_PASS]$(date +'%H:%M:%S')${NC} $1"; }
log_warn() { echo -e "${YELLOW}[OMNIBUS_WARN]$(date +'%H:%M:%S')${NC} $1"; }
log_err()  { echo -e "${RED}[OMNIBUS_FAIL]$(date +'%H:%M:%S')${NC} $1"; exit 1; }

# --- Step 1: Enforce Workspace Structure ---
log_info "Initializing workspace environment verification matrix..."
mkdir -p target/release

# --- Step 2: Compile Zero-Allocation Rust Engine ---
log_info "Compiling low-latency Rust v3 FFI primitive modules..."
if [ -f "prototypes/mod.rs" ]; then
    # Compile directly into a C-compatible dynamic library asset crossing the FFI border
    rustc --crate-type=cdylib prototypes/mod.rs -o target/release/libconstellation_manifold_toolkit.so
    log_succ "Bare-metal Rust compilation successful: target/release/libconstellation_manifold_toolkit.so"
elif [ -f "mod.rs" ]; then
    rustc --crate-type=cdylib mod.rs -o target/release/libconstellation_manifold_toolkit.so
    log_succ "Bare-metal Rust compilation successful: target/release/libconstellation_manifold_toolkit.so"
else
    log_warn "mod.rs namespace entry missing in path root. Attempting standard cargo release compilation..."
    if command -v cargo &> /dev/null; then
        cargo build --release
        log_succ "Cargo workspace assembly compilation completed."
    else
        log_err "Rust toolchain compiler targets unavailable. Aborting validation sequence."
    fi
fi

# --- Step 3: Run High-Performance Julia Benchmark Mode Toggle ---
log_info "Invoking Julia low-level CPU cycle profiling harness..."
if command -v julia &> /dev/null; then
    if [ -f "test_sigma_delta.jl" ]; then
        julia test_sigma_delta.jl
        log_succ "Julia high-frequency optimization assertions passed."
    elif [ -f "prototypes/test_sigma_delta.jl" ]; then
        julia prototypes/test_sigma_delta.jl
        log_succ "Julia high-frequency optimization assertions passed."
    else
        log_warn "Julia micro-benchmark validation scripts absent. Skipping profile trace."
    fi
else
    log_warn "Julia runtime environment not detected. Skipping multi-dispatch profile checks."
fi

# --- Step 4: Run Python Signal Enclave Validation Tests ---
log_info "Executing Python First-Order Sigma-Delta noise-shaping verification loop..."
if [ -f "test_sigma_delta.py" ]; then
    python3 test_sigma_delta.py
    log_succ "Python quantization and signal guard assertions verified."
elif [ -f "prototypes/test_sigma_delta.py" ]; then
    python3 prototypes/test_sigma_delta.py
    log_succ "Python quantization and signal guard assertions verified."
else
    log_err "Critical error: Python validation suite test_sigma_delta.py missing."
fi

# --- Step 5: Compile Balanced Ternary G-Code Instructions ---
log_info "Invoking G-code translation layer to compile toolpath trajectories..."
if [ -f "ternary_to_gcode.py" ]; then
    python3 ternary_to_gcode.py
    log_succ "Industrial step-motor instructions successfully baked to output_deposition_paths.gcode."
elif [ -f "prototypes/ternary_to_gcode.py" ]; then
    python3 prototypes/ternary_to_gcode.py
    log_succ "Industrial step-motor instructions successfully baked to output_deposition_paths.gcode."
else
    log_err "Critical error: G-code compilation engine script ternary_to_gcode.py missing."
fi

# --- Step 6: Render Spatial Toolpath Visualizations ---
log_info "Parsing G-code instructions to render spatial path chart maps..."
if [ -f "plot_deposition_paths.py" ]; then
    python3 plot_deposition_paths.py
    log_succ "Spatial chart rendered: gcode_toolpath_visualization.png"
elif [ -f "prototypes/plot_deposition_paths.py" ]; then
    python3 prototypes/plot_deposition_paths.py
    log_succ "Spatial chart rendered: gcode_toolpath_visualization.png"
else
    log_warn "Visualization engine plot_deposition_paths.py absent. Skipping PNG rendering."
fi

echo -e "\n================================================================================"
log_succ "🌌 NDH-SURREALISM: UNIFIED COMPILER PIPELINE RUN COMPLETED SUCCESSFULLY"
echo -e "================================================================================"
