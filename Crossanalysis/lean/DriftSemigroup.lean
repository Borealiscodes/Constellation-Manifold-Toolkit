import LeanHallucinationGuard.Guard

structure PseudoKraus :=
  (K : Type → Type)
  (adjoint : Type → Type)
  (trace_preserving : Prop)

structure DriftSemigroup :=
  (evolve : Type → Type)
  (closure : Prop)
  (bounded : Prop)

#guard DriftSemigroup
#guard DriftClosure

namespace DriftSemigroup

theorem semigroup_safety (Φ : DriftSemigroup) :
  Φ.closure ∧ Φ.bounded :=
by
  admit

end DriftSemigroup
