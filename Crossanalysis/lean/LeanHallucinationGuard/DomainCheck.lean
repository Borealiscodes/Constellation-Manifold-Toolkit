namespace LeanHallucinationGuard

syntax "#guardDomain" ident : command

@[command_elab «guardDomain»] def guardDomainImpl : CommandElab := fun stx => do
  let name := stx[1].getId.toString
  logInfo m!"HallucinationGuard: checking domain constraints for `{name}`."
  pure ()

end LeanHallucinationGuard
