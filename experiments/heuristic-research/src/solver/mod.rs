mod proxies;
pub use self::proxies::*;

mod state;
pub use self::state::*;

mod vector;
pub use self::vector::*;

mod vrp;
pub use self::vrp::*;

use rosomaxa::population::*;
use rosomaxa::prelude::*;
use std::sync::Arc;

/// Gets proxy population of given type.
fn get_population<C, O, S>(
    context: C,
    population_type: &str,
    objective: Arc<O>,
    environment: Arc<Environment>,
    selection_size: usize,
) -> Box<dyn HeuristicPopulation<Objective = O, Individual = S> + Send + Sync>
where
    C: RosomaxaContext<Solution = S> + 'static,
    O: HeuristicObjective<Solution = S> + Alternative + 'static,
    S: RosomaxaSolution<Context = C> + 'static,
{
    match population_type {
        "greedy" => Box::new(ProxyPopulation::new(Greedy::new(objective, 1, None))),
        "elitism" => {
            Box::new(ProxyPopulation::new(Elitism::new(objective, environment.random.clone(), 2, selection_size)))
        }
        "rosomaxa" => Box::new(ProxyPopulation::new(
            Rosomaxa::new(context, objective, environment, RosomaxaConfig::new_with_defaults(selection_size))
                .expect("cannot create rosomaxa with default configuration"),
        )),
        _ => unreachable!(),
    }
}
