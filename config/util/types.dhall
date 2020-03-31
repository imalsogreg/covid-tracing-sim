let Condition = < Asthma : {} | Diabetes : {} | Opeisity : {} >

let ExposureLevel = < Low : {} | Medium : {} | High : {} >

let TimedInfection =
    { time : Text
    , exposure : ExposureLevel
    }

let Health =
    { birthdate  : Text
    , conditions : List Condition
    , infections : List TimedInfection
    }

let Phone =
    { current_id : Natural
    , past_ids   : List Natural
    , contacts   : List Natural
    }

let Location = { x : Double, y : Double }

let Person =
    { id       : Natural
    , r2 :
      { location : Location
      , velocity : Double
      , heading  : Double
      }
    , health   : Health
    }


let State =
    { people : List Person
    , now    : Text
    }

let EnvironmentElement =
    < BoundingBox :
        { BoundingBox : 
        { left   : Double
        , top    : Double
        , width  : Double
        , height : Double
        }}
    | Road : { points : List Location, capacity : Double }
    | Point : { a : Double }
    >

in
{ Condition          = Condition
, ExposureLevel      = ExposureLevel 
, Health             = Health
, TimedInfection     = TimedInfection
, Person             = Person
, State              = State
, EnvironmentElement = EnvironmentElement
}
