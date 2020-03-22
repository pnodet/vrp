# Options

This section describes `vrp-cli` options.


## Scientific problem

At the moment, `vrp-cli` supports solving of two scientific problem sets: **solomon** and **lilim**.

The following command solves solomon problem defined in _RC1_10_1.txt_ and stores solution in _RC1_10_1_solution.txt_:

    vrp-cli solomon RC1_10_1.txt -o RC1_10_1_solution.txt

Optionally, you can specify initial solution to start with:

    vrp-cli solomon RC1_10_1.txt --init-solution RC1_10_1_solution_initial.txt -o RC1_10_1_solution_improved.txt

To run the problem from Li&Lim set, simply specify _lilim_ instead of _solomon_ as a type:

    vrp-cli lilim LC1_10_2.txt -o LC1_10_2_solution.txt


## Pragmatic problem

Pragmatic format requires at least one routing matrix passed as argument:

    vrp-cli pragmatic problem.json -m routing_matrix.json -o solution.json

If you have multiple, simply specify them one by one, in the order of `fleet.profiles`:

    vrp-cli pragmatic problem.json -m routing_matrix_car.json -m routing_matrix_truck.json


## Termination criteria

Termination criteria defines when refinement algorithm should stop and return best known solution. At the moment, there
are two types.

### Max time

Max time specifies duration of solving in seconds:

    vrp-cli pragmatic problem.json -m routing_matrix.json -o solution.json --max-time=600

### Max generations

Generation is one refinement step and it can be limited via _max-generations_ parameter:

    vrp-cli pragmatic problem.json -m routing_matrix.json -o solution.json --max-generations=1000


### Default behavior

By default termination criteria is max 2000 generations or 300 seconds.


## Writing solution to file

Writing solution into file is controlled by `-o` or `--out-result` setting. When it is omitted, then solution is written
in std out.

Pragmatic format supports option `-g` or `--geo-json` which writes solution in separate file in geojson format.


## Get unique locations list

List of unique locations can be received via `-l` or `--get-locations` setting. This list can be used to request routing
matrices needed by `pragmatic` format.