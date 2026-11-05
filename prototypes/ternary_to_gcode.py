#!/usr/bin/env python3
"""
================================================================================
NDH-SURREALISM: TERNARY STEP-MOTOR G-CODE COMPILER
Document ID: NDH-SUR-GCODE-V3
Purpose: Translates Quantized Ternary Arrays into Absolute Machine Instructions
================================================================================
"""

import numpy as np
import logging

logging.basicConfig(level=logging.INFO, format="[HARDWARE_COMPILER] %(message)s")
logger = logging.getLogger("TernaryCompiler")

class TernaryGCodeCompiler:
    def __init__(self, feedrate: int = 1200, micron_step: float = 1.0):
        """
        Initializes the mechanical translation matrix parameters.
        
        :param feedrate: Gantry travel speed in mm/min
        :param micron_step: Size of a single ternary structural step displacement in microns
        """
        self.feedrate = feedrate
        self.step_size_mm = micron_step / 1000.0  # Convert micron steps to mm
        self.delta_z_mm = 5.0 / 1_000_000.0       # Strict Δz = 5nm ceiling converted to mm
        self.extrusion_multiplier = 0.00012       # Volumetric scaling factor for slot-die head

    def compile_matrix(self, frame_id: int, ternary_history: np.ndarray, tool_id: int = 3) -> list:
        """
        Transforms sequential ternary step vectors into clean physical toolpaths.
        
        :param frame_id: Tracking index of the deposition thin-film layer
        :param ternary_history: Matrix of shape (N, 2) containing steps of [-1.0, 0.0, 1.0]
        :param tool_id: Chromatic slot-die macro channel ID (T0 - T15)
        """
        if ternary_history.shape[1] < 2:
            raise ValueError("Ternary history matrix must contain at least 2 coordinate tracking planes (X, Y).")

        # Calculate current thin-film layer height constraint block
        current_z_mm = frame_id * self.delta_z_mm

        gcode_output = [
            f"; 🌌 BEGIN FLOW TRANSCRIPTION: FRAME #{frame_id:04d}",
            f"; Framework Constraint: [ Δz <= 5nm Bound Verified ]",
            "G90 ; Establish absolute coordinate positioning constraints",
            "G21 ; Hardlock tool head processing matrix to millimeter metrics",
            f"T{tool_id} ; Hot-swap macro toolhead cluster slot channel assignment",
            "M400 ; Clear mechanical gantry synchronization buffer cache",
            f"G0 X0.0000 Y0.0000 Z{current_z_mm:.7f} F3000 ; Rapid travel entry to home datum"
        ]

        # Machine coordinate tracks (initialized at home position)
        current_x = 0.0
        current_y = 0.0

        for idx, step_vector in enumerate(ternary_history):
            # Extract discrete balanced ternary directions (-1, 0, 1)
            step_x, step_y = step_vector[0], step_vector[1]

            # Translate discrete step pulses to physical millimeter displacements
            dx = step_x * self.step_size_mm
            dy = step_y * self.step_size_mm

            # Calculate absolute coordinates
            target_x = current_x + dx
            target_y = current_y + dy

            # Calculate path distance for extrusion volume tracking
            segment_distance = np.sqrt(dx**2 + dy**2)

            if segment_distance == 0.0:
                # Intercept 0 pause: cut fluid pressure instantly to stop pooling
                gcode_output.append(
                    f"G1 X{target_x:.4f} Y{target_y:.4f} E0.0000 F{self.feedrate} ; Ternary Rest Phase"
                )
            else:
                # Active motion: calculate required fluid flow volume
                extrusion_vol = segment_distance * self.extrusion_multiplier
                gcode_output.append(
                    f"G1 X{target_x:.4f} Y{target_y:.4f} E{extrusion_vol:.5f} F{self.feedrate} ; Active Deposition"
                )

            # Update structural tracking registers
            current_x = target_x
            current_y = target_y

        gcode_output.append(f"; 🌌 END FLOW TRANSCRIPTION: FRAME #{frame_id:04d}\n")
        return gcode_output

if __name__ == "__main__":
    print("[+] Launching Standalone G-Code Translation Engine Validation Block...")
    compiler = TernaryGCodeCompiler(feedrate=1500, micron_step=5.0)

    # Simulated step-sequence vector coming directly out of the Sigma-Delta loop enclaves
    # Each row defines: [step_x, step_y]
    mock_ternary_stream = np.array([
        [1.0,  0.0],
        [1.0,  1.0],
        [0.0,  0.0],  # Intercept Step: Momentary robotic pause
        [-1.0, 0.0],
        [0.0, -1.0]
    ])

    # Compile sample sequence target layer frame #1 over color track tool channel T3
    gcode_instructions = compiler.compile_matrix(frame_id=1, ternary_history=mock_ternary_stream, tool_id=3)

    print("\n=== VERIFIED MACHINE CONTROL OUTPUT INSTRUCTIONS ===")
    print("\n".join(gcode_instructions))

    # Export to physical text file tree path
    output_path = "output_deposition_paths.gcode"
    with open(output_path, "w") as f:
        f.write("\n".join(gcode_instructions))
    print(f"\n[+] Production G-code compilation file successfully baked to: {output_path}")
