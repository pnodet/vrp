use crate::construction::heuristics::{RouteContext, RouteState};
use crate::helpers::models::problem::*;
use crate::helpers::models::solution::test_actor;
use crate::models::common::{Duration, Location, Schedule, TimeWindow};
use crate::models::problem::{Actor, Fleet, Single};
use crate::models::solution::{Activity, Place, Route, Tour};
use std::sync::Arc;

pub const DEFAULT_ACTIVITY_SCHEDULE: Schedule = Schedule { departure: 0.0, arrival: 0.0 };

pub fn test_activity() -> Activity {
    test_activity_with_job(Arc::new(test_single()))
}

pub fn test_activity_with_location(location: Location) -> Activity {
    Activity {
        place: Place { idx: 0, location, duration: DEFAULT_JOB_DURATION, time: DEFAULT_ACTIVITY_TIME_WINDOW },
        schedule: Schedule::new(location as f64, location as f64 + DEFAULT_JOB_DURATION),
        job: Some(test_single_with_location(Some(location))),
        commute: None,
    }
}

pub fn test_activity_with_location_and_duration(location: Location, duration: Duration) -> Activity {
    Activity {
        place: Place { idx: 0, location, duration, time: DEFAULT_ACTIVITY_TIME_WINDOW },
        schedule: Schedule::new(location as f64, location as f64 + DEFAULT_JOB_DURATION),
        job: Some(test_single_with_location(Some(location))),
        commute: None,
    }
}

pub fn test_activity_with_location_and_tw(location: Location, tw: TimeWindow) -> Activity {
    Activity {
        place: Place { idx: 0, location, duration: DEFAULT_JOB_DURATION, time: tw },
        schedule: Schedule::new(location as f64, location as f64 + DEFAULT_JOB_DURATION),
        job: Some(test_single_with_location(Some(location))),
        commute: None,
    }
}

pub fn test_activity_with_location_tw_and_duration(location: Location, tw: TimeWindow, duration: Duration) -> Activity {
    Activity {
        place: Place { idx: 0, location, duration, time: tw },
        schedule: Schedule::new(location as f64, location as f64 + duration),
        job: Some(test_single_with_location(Some(location))),
        commute: None,
    }
}

pub fn test_activity_with_schedule(schedule: Schedule) -> Activity {
    Activity {
        place: Place {
            idx: 0,
            location: DEFAULT_JOB_LOCATION,
            duration: DEFAULT_JOB_DURATION,
            time: DEFAULT_ACTIVITY_TIME_WINDOW,
        },
        schedule,
        job: None,
        commute: None,
    }
}

pub fn test_activity_with_job(job: Arc<Single>) -> Activity {
    Activity {
        place: Place {
            idx: 0,
            location: DEFAULT_JOB_LOCATION,
            duration: DEFAULT_JOB_DURATION,
            time: DEFAULT_ACTIVITY_TIME_WINDOW,
        },
        schedule: DEFAULT_ACTIVITY_SCHEDULE,
        job: Some(job),
        commute: None,
    }
}

pub fn test_activity_without_job() -> Activity {
    Activity {
        place: Place {
            idx: 0,
            location: DEFAULT_JOB_LOCATION,
            duration: DEFAULT_JOB_DURATION,
            time: DEFAULT_ACTIVITY_TIME_WINDOW,
        },
        schedule: DEFAULT_ACTIVITY_SCHEDULE,
        job: None,
        commute: None,
    }
}

pub struct RouteContextBuilder(RouteContext);

impl Default for RouteContextBuilder {
    fn default() -> Self {
        Self(create_empty_route_ctx())
    }
}

impl RouteContextBuilder {
    pub fn with_route(&mut self, route: Route) -> &mut Self {
        *self.0.route_mut() = route;
        self
    }

    pub fn with_state(&mut self, state: RouteState) -> &mut Self {
        *self.0.state_mut() = state;
        self
    }

    pub fn build(&mut self) -> RouteContext {
        std::mem::replace(&mut self.0, create_empty_route_ctx())
    }
}

pub struct RouteBuilder(Route);

impl Default for RouteBuilder {
    fn default() -> Self {
        Self(create_route(test_actor(), Tour::default(), vec![]))
    }
}

impl RouteBuilder {
    /// Switches route to a vehicle with a given id from the fleet. Clears all changes in the tour done previously.
    pub fn with_vehicle(&mut self, fleet: &Fleet, vehicle_id: &str) -> &mut Self {
        let actor = get_test_actor_from_fleet(fleet, vehicle_id);
        let tour = Tour::new(actor.as_ref());

        self.0 = create_route(actor, tour, vec![]);
        self
    }

    /// Sets tour start. NOTE: clears all existing activities.
    pub fn with_start(&mut self, start: Activity) -> &mut Self {
        self.0.tour = Tour::default();
        self.0.tour.set_start(start);
        self
    }

    /// Sets tour end. NOTE: requires start to be set and no job activities are inserted.
    pub fn with_end(&mut self, end: Activity) -> &mut Self {
        self.0.tour.set_end(end);
        self
    }

    /// Adds activities with jobs to the tour.
    pub fn add_activities<I>(&mut self, activities: I) -> &mut Self
    where
        I: IntoIterator<Item = Activity>,
    {
        let start_idx = self.0.tour.job_activity_count() + 1;
        activities.into_iter().enumerate().for_each(|(index, a)| {
            self.0.tour.insert_at(a, start_idx + index);
        });
        self
    }

    pub fn add_activity(&mut self, activity: Activity) -> &mut Self {
        self.0.tour.insert_last(activity);
        self
    }

    pub fn with_activity<F>(&mut self, configure: F) -> &mut Self
    where
        F: FnOnce(&mut Activity),
    {
        let mut activity = test_activity();
        configure(&mut activity);
        self.0.tour.insert_last(activity);
        self
    }

    pub fn build(&mut self) -> Route {
        std::mem::replace(&mut self.0, RouteBuilder::default().0)
    }
}

pub struct ActivityBuilder(Activity);

impl Default for ActivityBuilder {
    fn default() -> Self {
        Self(test_activity())
    }
}

impl ActivityBuilder {
    pub fn place(&mut self, place: Place) -> &mut Self {
        self.0.place = place;
        self
    }

    pub fn schedule(&mut self, schedule: Schedule) -> &mut Self {
        self.0.schedule = schedule;
        self
    }

    pub fn job(&mut self, job: Option<Arc<Single>>) -> &mut Self {
        self.0.job = job;
        self
    }

    pub fn build(&mut self) -> Activity {
        std::mem::replace(&mut self.0, test_activity())
    }
}

fn create_empty_route_ctx() -> RouteContext {
    RouteContext::new_with_state(create_route(test_actor(), Tour::default(), vec![]), RouteState::default())
}

fn create_route(actor: Arc<Actor>, mut tour: Tour, activities: Vec<Activity>) -> Route {
    activities.into_iter().enumerate().for_each(|(index, a)| {
        tour.insert_at(a, index + 1);
    });

    Route { actor, tour }
}
