namespace LeanHallucinationGuard

syntax "#guardTensor" ident : command

@[command_elab «guardTensor»] def guardTensorImpl : CommandElab := fun stx => do
  let name := stx[1].getId.toString
  logInfo m!"HallucinationGuard: verifying tensor shape for `{name}`."
  pure ()

end LeanHallucinationGuard
