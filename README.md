`vaccine` is a command line application to test your REST APIs against complex patch requests.

Often as the number of elements of a REST resource increases, the validation rules for patching it become
more complex, to the point where hard to find bugs are introduced because of the increasing number of
combinations of these elements which might appear in a patch request.


`vaccine` ingests a JSON formatted schema of your REST API endpoints, and uses it to do two things:

1. It creates an object graph according to your schema.
2. It then sends randomized patch payloads to the endpoints representing these objects.

The randomized payloads contain two levels of randomness:
1. Elements to be part of the payload are selected at random
2. The values of these elements are then randomized

For example, given the following resource:

```json
{
  "name": "Jumbo",
  "species": "elephant",
  "weight_in_lbs": 1000.09,
  "carnivorous": false 
}
```

The patch payload generated will contain combination of these elements with random values for each element.

The responses for the patch requests are recorded when server error is encountered.

#### Building the object graph

The object graph is built by `POST`ing the full payload from your schema, using default_value elements
that you define in the schema. Before building a given object all of its dependencies are built first.

#### Data types

The following data types are supported:

1. String
2. Number
3. Float
4. Boolean
5. Datetime
6. Nested JSON objects

Lists are planned to be added.
