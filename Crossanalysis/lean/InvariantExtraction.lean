import LeanHallucinationGuard.Guard
import DriftSemigroup

structure InvariantExtraction :=
  (extract : Type → Type)
  (trace_invariant : Prop)
  (norm_invariant : Prop)
  (det_invariant : Prop)

#guard InvariantExtraction
#guard InvariantStability

namespace InvariantExtraction

theorem invariant_safety (I : InvariantExtraction) :
  I.trace_invariant ∧ I.norm_invariant ∧ I.det_invariant :=
by
  admit

end InvariantExtraction
