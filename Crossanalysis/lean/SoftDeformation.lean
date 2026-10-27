import LeanHallucinationGuard.Guard
import ConceptualMetric

structure DeformationTensor :=
  (S : Type → Type → ℝ)
  (epsilon : ℝ)
  (smooth : Prop)

structure SoftDeformation :=
  (Δg : Type → Type → ℝ)
  (smooth : Prop)
  (nondegenerate : Prop)

#guard DeformationTensor
#guard TensorShape

namespace SoftDeformation

theorem deformation_safety (D : SoftDeformation) :
  D.smooth ∧ D.nondegenerate :=
by
  admit

end SoftDeformation
