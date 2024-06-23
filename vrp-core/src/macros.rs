//! Provides some useful macros to avoid repetitive code.

/// A macro to define custom dimension on [crate::models::common::Dimensions].
macro_rules! custom_dimension {
    ($name:ident typeof $type:ty $(: $gen:ident)?) => {
        paste::paste! {
            #[doc = " Extends [Dimensions] within a new ["[<$name Dimension>]"]."]
            pub trait [<$name Dimension>] {
                #[doc = " Gets "$name " property."]
                fn [<get_ $name:snake:lower>]$(<$type : $gen>)?(&self) -> Option<&$type>;
                #[doc = " Sets "$name " property."]
                fn [<set_ $name:snake:lower>]$(<$type : $gen>)?(&mut self, value: $type) -> &mut Self;
            }

            // Define a dummy struct type which is used as a key.
            struct [<$name DimensionKey>];
            impl [<$name Dimension>] for Dimensions {
                fn [<get_ $name:snake:lower>]$(<$type : $gen>)?(&self) -> Option<&$type> {
                    self.get_value::<[<$name DimensionKey>], _>()
                }

                fn [<set_ $name:snake:lower>]$(<$type : $gen>)?(&mut self, value: $type) -> &mut Self {
                    self.set_value::<[<$name DimensionKey>], _>(value);
                    self
                }
            }
        }
    };
}

/// A macro to define custom activity state on [crate::construction::heuristics::RouteState].
macro_rules! custom_activity_state {
    ($name:ident typeof $type:ty $(: $gen:ident)?) => {
        paste::paste! {
            #[doc = " Extends [RouteState] within a new ["[<$name ActivityState>]"]."]
            pub trait [<$name ActivityState>] {
                #[doc = " Gets `"$name "` activity state."]
                fn [<get_ $name:snake:lower _at>]$(<$type : $gen>)?(&self, activity_idx: usize) -> Option<&$type>;
                #[doc = " Sets `"$name "` activity states."]
                fn [<set_ $name:snake:lower _states>]$(<$type : $gen>)?(&mut self, values: Vec<$type>);
            }

            // Define a dummy struct type which is used as a key.
            struct [<$name ActivityStateKey>];
            impl [<$name ActivityState>] for RouteState {
                fn [<get_ $name:snake:lower _at>]$(<$type : $gen>)?(&self, activity_idx: usize) -> Option<&$type> {
                    self.get_activity_state_ex::<[<$name ActivityStateKey>], _>(activity_idx)
                }

                fn [<set_ $name:snake:lower _states>]$(<$type : $gen>)?(&mut self, values: Vec<$type>) {
                    self.set_activity_states_ex::<[<$name ActivityStateKey>], _>(values);
                }
            }
        }
    };
}

/// A macro to define custom route state on [crate::construction::heuristics::RouteState].
macro_rules! custom_tour_state {
    ($name:ident typeof $type:ty $(: $gen:ident)?) => {
        paste::paste! {
            #[doc = " Extends [RouteState] within a new ["[<$name TourState>]"]."]
            pub trait [<$name TourState>] {
                #[doc = " Gets `"$name "` tour state."]
                fn [<get_ $name:snake:lower>]$(<$type : $gen>)?(&self) -> Option<&$type>;
                #[doc = " Sets `"$name "` tour state."]
                fn [<set_ $name:snake:lower>]$(<$type : $gen>)?(&mut self, value: $type);
            }

            // Define a dummy struct type which is used as a key
            struct [<$name TourStateKey>];
            impl [<$name TourState>] for RouteState {
                fn [<get_ $name:snake:lower>]$(<$type : $gen>)?(&self) -> Option<&$type> {
                    self.get_tour_state_ex::<[<$name TourStateKey>], _>()
                }

                fn [<set_ $name:snake:lower>]$(<$type : $gen>)?(&mut self, value: $type) {
                    self.set_tour_state_ex::<[<$name TourStateKey>], _>(value);
                }
            }
        }
    };
}

/// A macro to define custom route intervals state used within [crate::construction::enablers::RouteIntervals].
macro_rules! custom_route_intervals_state {
    // visibility modifier is provided
    ($(#[$meta:meta])* $vis:vis $name:ident) => {
        paste::paste! {
            custom_tour_state!($name typeof Vec<(usize, usize)>);
            $(#[$meta])*
            /// Provides access to route intervals implementation.
            $vis struct [<$name State>];
            impl RouteIntervalsState for [<$name State>] {
                fn get_route_intervals<'a>(&self, route_state: &'a RouteState) -> Option<&'a Vec<(usize, usize)>> {
                    route_state.[<get_ $name:snake:lower>]()
                }

                fn set_route_intervals(&self, route_state: &mut RouteState, values: Vec<(usize, usize)>) {
                    route_state.[<set_ $name:snake:lower>](values);
                }
            }
        }
    };

    // no visibility modifier is provided
    ($(#[$meta:meta])* $name:ident) => {
        custom_route_intervals_state!($(#[$meta])* pub(crate) $name);
    };
}