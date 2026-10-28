module Sandbox50D

export SandboxState, init_sandbox, embed_ndh, embed_8b, embed_sg

struct SandboxState
    x50::Vector{Float64}        # 50D neutral manifold coordinate
    xproto::Dict{String,Any}   # proto-atlas conceptual vector
end

function init_sandbox()
    SandboxState(zeros(50), Dict())
end

function embed_ndh(state::SandboxState, v::Vector{Float64})
    state.x50 .= v
    state.xproto["NDH"] = v
    return state
end

function embed_8b(state::SandboxState, v::Vector{Float64})
    state.x50 .= v
    state.xproto["8B"] = v
    return state
end

function embed_sg(state::SandboxState, v::Vector{Float64})
    state.x50 .= v
    state.xproto["SG"] = v
    return state
end

end # module
