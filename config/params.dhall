let types = ./util/types.dhall
in
{ step_size_seconds = 1
, simulation_length_seconds = 70
, environment =
    [ types.EnvironmentElement.BoundingBox
        { BoundingBox = 
        { left   = -10.0
        , top    = -10.0
        , width  =  20.0
        , height =  20.0
        }}
    ] : List types.EnvironmentElement
}
