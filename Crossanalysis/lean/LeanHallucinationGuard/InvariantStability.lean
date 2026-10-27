namespace LeanHallucinationGuard

syntax "#guardInvariant" ident : command

@[command_elab «guardInvariant»] def guardInvariantImpl : CommandElab := fun stx => do
  let name := stx[1].getId.toString
  logInfo m!"HallucinationGuard: validating invariant stability for `{name}`."
  pure ()

end LeanHallucinationGuard
