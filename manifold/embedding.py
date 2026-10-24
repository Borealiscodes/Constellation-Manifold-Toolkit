"""
Constellation Embedding Runtime
Altitude: A3 (Formal Membrane)
Author: Borealis S. Hedling

This module defines the operational embedding state E = (v, z, M)
and the reversible operator classes specified in specification.tex.
"""

import numpy as np

# ------------------------------------------------------------
# Embedding State
# ------------------------------------------------------------

class EmbeddingState:
    """
    E = (v, z, M)
    v : ternary vector {-1,0,+1}^n
    z : geometric embedding (numpy array)
    M : membrane envelope (numpy array)
    """

    def __init__(self, v, z, M):
        self.v = np.array(v, dtype=float)
        self.z = np.array(z, dtype=float)
        self.M = np.array(M, dtype=float)

    def copy(self):
        return EmbeddingState(self.v.copy(), self.z.copy(), self.M.copy())


# ------------------------------------------------------------
# Operators (Activation, Transition, Diagnostic, Guard, Reversal)
# ------------------------------------------------------------

class ActivationOperator:
    """
    Symbolic → Ternary
    Parses symbolic telemetry into numeric ternary vectors.
    """

    SYMBOLIC_MAP = {"-": -1, "0": 0, "+": 1}

    def __call__(self, symbolic_vector):
        v = [self.SYMBOLIC_MAP[s] for s in symbolic_vector]
        return np.array(v, dtype=float)


class TransitionOperator:
    """
    Ternary → Manifold
    Embeds ternary vectors into geometric coordinates.
    """

    def __init__(self, scale=0.1):
        self.scale = scale

    def __call__(self, v):
        z = self.scale * np.array(v, dtype=float)
        M = np.clip(v, -1, +1)  # membrane envelope
        return EmbeddingState(v, z, M)


class DiagnosticOperator:
    """
    Computes drift, roughness, holonomy, collapse gradients.
    """

    def __call__(self, E: EmbeddingState):
        v = E.v

        drift = v * 0.05
        roughness = np.abs(v) * 0.02
        holonomy = np.cumsum(v) * 0.01
        collapse_grad = np.gradient(v)

        diagnostics = {
            "drift": drift,
            "roughness": roughness,
            "holonomy": holonomy,
            "collapse_grad": collapse_grad,
        }

        return diagnostics


class GuardOperator:
    """
    Prevents membrane violation or illegal ternary states.
    """

    def __call__(self, E: EmbeddingState):
        E.v = np.clip(E.v, -1, +1)
        E.M = np.clip(E.M, -1, +1)
        return E


class ReversalOperator:
    """
    Restores a previous embedding state.
    """

    def __call__(self, E_prev: EmbeddingState):
        return E_prev.copy()


# ------------------------------------------------------------
# Runtime Controller
# ------------------------------------------------------------

class ConstellationRuntime:
    """
    High-level controller for embedding transitions.
    """

    def __init__(self):
        self.O_act = ActivationOperator()
        self.O_trans = TransitionOperator()
        self.O_diag = DiagnosticOperator()
        self.O_guard = GuardOperator()
        self.O_rev = ReversalOperator()

        self.provenance = []

    def log(self, event):
        self.provenance.append(event)

    def symbolic_to_embedding(self, symbolic_vector):
        v = self.O_act(symbolic_vector)
        E = self.O_trans(v)
        E = self.O_guard(E)
        self.log({"event": "activation+transition", "v": v.tolist()})
        return E

    def diagnose(self, E):
        diag = self.O_diag(E)
        self.log({"event": "diagnostic", "diag": diag})
        return diag

    def reverse(self, E_prev):
        E = self.O_rev(E_prev)
        self.log({"event": "reversal"})
        return E
