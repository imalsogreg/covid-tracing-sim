let types = ./util/types.dhall
let person = ./util/person.dhall
in
{ people = [ person.default
           , person.default //
             { r2 = { location = {x = 0.5, y = 0.0}
                    , heading = 2.0
                    , velocity = 0.1
                    }
             , health =
                    { birthdate = "1970-01-02"
                    , conditions = [] : List types.Condition
                    , infections =
                      [ { time = "2020-01-01T00:00:01"
                        , exposure = types.ExposureLevel.Medium
                        }
                      ] : List types.TimedInfection
                    }
             }
           ]
, now    = "2020-01-01T00:00:00"
} : types.State
