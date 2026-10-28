module Firewall

export firewall_check

function firewall_check(op::Any; purity=true, bounded=true, nongenerative=true)
    if !purity
        return false
    end
    if !bounded
        return false
    end
    if !nongenerative
        return false
    end
    return true
end

end # module
