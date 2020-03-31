let types = ./types.dhall
in
{ 
  default =
    { birthdate = "1970-01-01"
    , conditions = [] : List types.Condition
    , infections = [] : List types.TimedInfection
    } : types.Health
}
