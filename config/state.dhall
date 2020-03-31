let types = ./util/types.dhall
let person = ./util/person.dhall
in
{ people = [ person.default  ]
, now    = "2020-01-01T00:00:00"
} : types.State
