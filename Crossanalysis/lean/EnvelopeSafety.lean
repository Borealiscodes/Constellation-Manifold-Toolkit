import LeanHallucinationGuard.Guard

structure EnvelopeSafety :=
  (H : Type → Type → ℝ)
  (energy_bound : ℝ)
  (safe : Prop)

#guard EnvelopeSafety

namespace EnvelopeSafety

theorem envelope_safety (E : EnvelopeSafety) :
  E.safe :=
by
  admit

end EnvelopeSafety
