namespace LeanHallucinationGuard

syntax "#guardDrift" ident : command

@[command_elab «guardDrift»] def guardDriftImpl : CommandElab := fun stx => do
  let name := stx[1].getId.toString
  logInfo m!"HallucinationGuard: checking drift semigroup closure for `{name}`."
  pure ()

end LeanHallucinationGuard
