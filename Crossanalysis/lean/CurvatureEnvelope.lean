import LeanHallucinationGuard.Guard
import ConceptualMetric

structure CurvatureEnvelope :=
  (Riemann : Type → Type → Type → Type → ℝ)
  (scalar : Type → ℝ)
  (envelope : Type → ℝ)
  (smooth : Prop)

#guard CurvatureEnvelope
#guard CurvatureCheck

namespace CurvatureEnvelope

theorem envelope_safety (Φ : CurvatureEnvelope) :
  Φ.smooth :=
by
  admit

end CurvatureEnvelope
