import numpy as np

def parse_ternary_block(block):
    # block: dict containing symbolic ternary arrays
    raw = block["trit_vector"]
    return np.array([{"-":-1, "0":0, "+":1}[t] for t in raw])

def embed_geometry(v):
    alpha = 0.1
    z = alpha * v
    metric = np.eye(len(v))
    connection = np.zeros((len(v), len(v), len(v)))
    curvature = np.zeros((len(v), len(v), len(v), len(v)))
    return z, metric, connection, curvature

def encode_diagnostics(v):
    drift = v * 0.05
    roughness = np.abs(v) * 0.02
    holonomy = np.cumsum(v) * 0.01
    collapse_grad = np.gradient(v)
    return drift, roughness, holonomy, collapse_grad

def encode_safety(v):
    membrane_env = np.clip(v, -1, +1)
    return membrane_env

def encode_flow(v):
    return np.roll(v, 1) - np.roll(v, -1)

def encode_telemetry(block):
    v = parse_ternary_block(block)
    z, metric, connection, curvature = embed_geometry(v)
    drift, roughness, holonomy, collapse_grad = encode_diagnostics(v)
    membrane_env = encode_safety(v)
    flow = encode_flow(v)

    return {
        "z": z,
        "metric": metric,
        "connection": connection,
        "curvature": curvature,
        "drift": drift,
        "roughness": roughness,
        "holonomy": holonomy,
        "collapse_grad": collapse_grad,
        "membrane_env": membrane_env,
        "flow": flow
    }
