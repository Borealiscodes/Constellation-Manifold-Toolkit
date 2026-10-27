import LeanHallucinationGuard.Guard

structure ConceptualMetric :=
  (g : Type → Type → ℝ)
  (symmetric : Prop)
  (positive_definite : Prop)
  (det_nonzero : Prop)

#guard ConceptualMetric
#guard TensorShape

namespace ConceptualMetric

theorem metric_safety (g : ConceptualMetric) :
  g.symmetric ∧ g.positive_definite ∧ g.det_nonzero :=
by
  admit

end ConceptualMetric
