#!/usr/bin/env python3
"""
================================================================================
NDH-SURREALISM: MACHINE TOOLPATH VISUALIZATION ENGINE
Document ID: NDH-SUR-VIS-V3
Purpose: Parses compiled G-code files and maps physical deposition trajectories
================================================================================
"""

import os
import re
import numpy as np
import matplotlib.pyplot as plt

def parse_gcode_matrix(file_path):
    """Parses standard G-code lines into multi-axis coordinate trajectories."""
    if not os.path.exists(file_path):
        raise FileNotFoundError(f"[-] Target G-code file missing at {file_path}. Run the compiler first.")

    # Tracking registers
    x_coords, y_coords, e_flows = [0.0], [0.0], [0.0]
    is_rest_phase = [False]

    # Regular expressions to catch coordinate metrics
    x_pattern = re.compile(r"X(-?\d+\.\d+)")
    y_pattern = re.compile(r"Y(-?\d+\.\d+)")
    e_pattern = re.compile(r"E(\d+\.\d+)")

    with open(file_path, "r") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith(";"):
                continue

            if "G1" in line or "G0" in line:
                x_match = x_pattern.search(line)
                y_match = y_pattern.search(line)
                e_match = e_pattern.search(line)

                # Keep previous position if axis didn't change
                curr_x = float(x_match.group(1)) if x_match else x_coords[-1]
                curr_y = float(y_match.group(1)) if y_match else y_coords[-1]
                curr_e = float(e_match.group(1)) if e_match else 0.0

                x_coords.append(curr_x)
                y_coords.append(curr_y)
                e_flows.append(curr_e)
                
                # Check if the line represents a balanced ternary '0' anti-pooling phase
                is_rest_phase.append("Ternary Rest Phase" in line or (curr_e == 0.0 and "G1" in line))

    return np.array(x_coords), np.array(y_coords), np.array(e_flows), np.array(is_rest_phase)

def render_toolpath_chart(file_path="output_deposition_paths.gcode"):
    print(f"[*] Parsing machine telemetry records from: {file_path}")
    try:
        x, y, e, rest = parse_gcode_matrix(file_path)
    except FileNotFoundError as err:
        print(err)
        return

    # Initialize plotting canvas
    plt.figure(figsize=(10, 8), dpi=150)
    plt.style.use('seaborn-v0_8-whitegrid' if 'seaborn-v0_8-whitegrid' in plt.style.available else 'default')

    # Draw segments individually to visually capture phase variations
    for i in range(len(x) - 1):
        if rest[i+1]:
            # Ternary 0 step (Nozzle closed, gantry stationary/slow-turning corner)
            plt.plot(x[i:i+2], y[i:i+2], color='#FF3366', linewidth=3, linestyle=':', zorder=4,
                     label='Ternary Rest Phase (E0.0000)' if 'Ternary Rest Phase (E0.0000)' not in plt.gca().get_legend_handles_labels()[1] else "")
            plt.scatter(x[i+1], y[i+1], color='#FF3366', edgecolor='black', s=50, zorder=5)
        else:
            # Active ternary ±1 steps (Nozzle printing, fluid discharging)
            plt.plot(x[i:i+2], y[i:i+2], color='#00FFCC', linewidth=2.5, zorder=3,
                     label='Active Deposition Path' if 'Active Deposition Path' not in plt.gca().get_legend_handles_labels()[1] else "")

    # Draw arrows indicating path direction
    for i in range(0, len(x) - 1, max(1, len(x) // 10)):
        dx = x[i+1] - x[i]
        dy = y[i+1] - y[i]
        if np.sqrt(dx**2 + dy**2) > 0:
            plt.annotate('', xy=(x[i+1], y[i+1]), xytext=(x[i], y[i]),
                         arrowprops=dict(arrowstyle="->", color='#333333', lw=1, alpha=0.6), zorder=2)

    # Global Datum Anchors
    plt.scatter(x[0], y[0], color='#FFCC00', edgecolor='black', s=100, zorder=6, label='Home Datum Axis (0,0)')

    # Labels, Legends, and Title Layout Configuration
    plt.title('AI-Guided Painting Rig: Toolpath Vector Visualization Trace', fontsize=12, fontweight='bold', pad=15)
    plt.xlabel('Gantry X Coordinate Position (mm)', fontsize=10)
    plt.ylabel('Gantry Y Coordinate Position (mm)', fontsize=10)
    plt.legend(loc='upper right', frameon=True, facecolor='white', shadow=True, fontsize=9)
    plt.axis('equal') # Keep 1:1 spatial aspect ratio intact
    plt.grid(True, which="both", ls="--", color='#ECECEC')

    output_img = "gcode_toolpath_visualization.png"
    plt.savefig(output_img, bbox_inches='tight')
    plt.close()
    print(f"[+] Spatial toolpath chart successfully rendered and saved to: {output_img}")

if __name__ == "__main__":
    # Fallback asset creation if the user hasn't generated a G-code file yet
    if not os.path.exists("output_deposition_paths.gcode"):
        print("[!] Local output_deposition_paths.gcode missing. Mocking file for test...")
        mock_gcode = [
            "G90", "G21", "T3",
            "G1 X0.0050 Y0.0050 E0.0006 F1200 ; Active Deposition",
            "G1 X0.0100 Y0.0100 E0.0006 F1200 ; Active Deposition",
            "G1 X0.0100 Y0.0100 E0.0000 F1200 ; Ternary Rest Phase",
            "G1 X0.0050 Y0.0100 E0.0006 F1200 ; Active Deposition",
            "G1 X0.0050 Y0.0050 E0.0006 F1200 ; Active Deposition"
        ]
        with open("output_deposition_paths.gcode", "w") as f:
            f.write("\n".join(mock_gcode))

    render_toolpath_chart()
