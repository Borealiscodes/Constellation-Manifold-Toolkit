namespace LeanHallucinationGuard

syntax "#guard" ident : command

@[command_elab «guard»] def guardImpl : CommandElab := fun stx => do
  let name := stx[1].getId.toString
  logInfo m!"HallucinationGuard: validating `{name}` (placeholder implementation)."
  pure ()

end LeanHallucinationGuard
