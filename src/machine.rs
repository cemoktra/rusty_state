/// Error returned when a state transition is not allowed
#[derive(Debug, Clone)]
pub struct TranstionNotAllowed;

/// Trait that must be implemented by state enumerations
pub trait State {
    fn transition_allowed(self, new_state: &Self) -> bool;
}

pub trait Feature<T: State> {
    fn allowed(self, state: &T) -> bool;
}

/// State machine
/// 
/// For an example look at the traffic lights test
#[derive(Clone, Copy)]
pub struct StateMachine<T> {
    state: T
}

impl<T: State + Copy> StateMachine<T> {
  /// Create a new state machine
  /// 
  /// # Arguments
  /// 
  /// * `state` - initial state
  pub fn new(state: T) -> StateMachine<T> {
    StateMachine {
      state: state
    }
  }

  pub fn feature_allowed<U: Feature<T> + Copy>(self, feature: &U) -> bool {
    feature.allowed(&self.state)
  }

  /// Starts a state transition
  /// 
  /// # Arguments
  /// 
  /// * `new_state` - try transtion to this state
  pub fn set(&mut self, new_state: &T) -> Result<(), TranstionNotAllowed> {
      if self.state.transition_allowed(&new_state) {
          self.state = *new_state;
          Ok(())
      } else {
          Err(TranstionNotAllowed {})
      }
  }
}