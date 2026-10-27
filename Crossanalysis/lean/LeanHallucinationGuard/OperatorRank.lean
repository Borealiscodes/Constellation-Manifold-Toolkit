namespace LeanHallucinationGuard

syntax "#guardRank" ident : command

@[command_elab «guardRank»] def guardRankImpl : CommandElab := fun stx => do
  let name := stx[1].getId.toString
  logInfo m!"HallucinationGuard: checking operator rank for `{name}`."
  pure ()

end LeanHallucinationGuard
