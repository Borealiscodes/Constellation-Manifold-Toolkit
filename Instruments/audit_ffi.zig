// In eightb-agy-fanout (e.g., src/audit_ffi.zig)

pub const TrajectoryStep = extern struct {
    // A 512-dimension text/routing embedding vector from the agent state
    embedding: [512]f64, 
};

pub const AgentPathTrace = extern struct {
    steps: [*]const TrajectoryStep,
    len: usize,
};

// Exported C-ABI function that Julia can hook into natively
export fn audit_agent_trajectory(trace: AgentPathTrace) f64 {
    // This is where Julia will attach its hook
    return 0.0;
}
