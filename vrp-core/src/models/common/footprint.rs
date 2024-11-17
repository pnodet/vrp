//! This module contains a logic to maintain a low-dimensional representation of the VRP Solution.

use crate::algorithms::structures::BitVec;
use crate::prelude::*;

/// Defines a low-dimensional representation of multiple solutions.
pub struct Footprint {
    /// repr here is the same adjacency matrix as in [Shadow], but instead of storing bits
    /// we store here the number of times the edge was present in multiple solutions.
    repr: Vec<u8>,
    dimension: usize,
}

impl Footprint {
    /// Creates a new instance of a `Snapshot`.
    pub fn new(problem: &Problem) -> Self {
        let dim = get_dimension(problem);
        Self { repr: vec![0; dim * dim], dimension: dim }
    }

    /// Returns an iterator over the low-dimensional representation.
    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
        (0..self.dimension).flat_map(move |from| {
            (0..self.dimension).map(move |to| ((from, to), self.repr[from * self.dimension + to]))
        })
    }

    /// Returns dimension of adjacency matrix.
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    // TODO use it or move to heuristic research crate
    #[allow(dead_code)]
    pub(crate) fn apply(&mut self, solution: &mut InsertionContext) {
        let shadow = Shadow::from(&*solution);
        self.memorize(&shadow);

        solution.solution.state.set_shadow(shadow);
    }

    fn memorize(&mut self, shadow: &Shadow) {
        self.repr.iter_mut().enumerate().for_each(|(index, value)| {
            let bit_value = shadow.repr.get(index).map(|bit| bit as u8).unwrap_or_default();
            *value = value.saturating_add(bit_value);
        });
    }
}

/// Specifies a maximum number of locations considered in the low-dimensional representation.
const MAX_REPRESENTATION_DIMENSION: usize = 1000;

// A state property to store the low-dimensional representation of the solution.
custom_solution_state!(Shadow typeof Shadow);

/// A low-dimensional representation of the VRP Solution.
/// Here, we use Bit Vector data structure to represent the adjacency matrix of the solution, where
/// each bit represents the presence of the edge between pair of locations in the given solution.
#[derive(Clone, Debug)]
pub struct Shadow {
    /// repr is adjusted matrix of size dim x dim, where dim is the minimum of MAX_REPRESENTATION_DIMENSION
    /// and number of locations present in the problem.
    repr: BitVec,
}

impl Shadow {
    /// Returns an iterator over the low-dimensional representation.
    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), bool)> + '_ {
        let size = (self.repr.len() as f64).sqrt() as usize;
        debug_assert!(size * size == self.repr.len());

        (0..size).flat_map(move |from| (0..size).map(move |to| ((from, to), self.repr[from * size + to])))
    }

    /// Returns dimension of the shadow.
    pub fn dimension(&self) -> usize {
        (self.repr.len() as f64).sqrt() as usize
    }
}

impl From<&InsertionContext> for Shadow {
    fn from(insertion_ctx: &InsertionContext) -> Self {
        let dim = get_dimension(&insertion_ctx.problem);
        let mut shadow = Shadow { repr: BitVec::new(dim * dim) };

        insertion_ctx.solution.routes.iter().for_each(|route_ctx| {
            route_ctx
                .route()
                .tour
                .legs()
                .filter_map(|(activities, _)| if let [from, to] = activities { Some((from, to)) } else { None })
                // NOTE apply % operator on locations. This is not optimal, but it is the simplest and
                // the fastest approach to keep memory usage quite low. Better, but slower approach would be
                // to apply some clustering algorithm for nearby locations and use the same index to them.
                .map(|(from, to)| (from.place.location % dim, to.place.location % dim))
                .for_each(|(from, to)| shadow.repr.set(from * dim + to, true));
        });

        shadow
    }
}

fn get_dimension(problem: &Problem) -> usize {
    problem.transport.size().min(MAX_REPRESENTATION_DIMENSION)
}