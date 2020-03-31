let health = ./health.dhall
let types  = ./types.dhall
in
{
  default =
    { id = 0
    , r2 =
       { location = { x = 0.0, y = 0.0 }
       , velocity = 3.0
       , heading = 1.0
       }
    , health = health.default
    } : types.Person
}
