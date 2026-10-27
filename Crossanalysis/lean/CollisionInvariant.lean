import LeanHallucinationGuard.Guard
import CurvatureEnvelope
import DriftSemigroup
import InvariantExtraction
import EnvelopeSafety

structure CollisionInvariant :=
  (collision : Prop)
  (avoidance : Prop)
  (distance_safe : Prop)

#guard CollisionInvariant

namespace CollisionInvariant

theorem collision_safety (C : CollisionInvariant) :
  C.avoidance ∧ C.distance_safe :=
by
  admit

end CollisionInvariant
