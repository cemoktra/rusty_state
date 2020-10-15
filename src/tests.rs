#[cfg(test)]
mod tests {
  use crate::machine::{State, StateMachine};

  #[derive(Clone, Copy, PartialEq)]
  enum TrafficLight {
      Red,
      RedYellow,
      Green,
      Yellow
  }

  impl State<TrafficLight> for TrafficLight {
    fn transition_allowed(self, new_state: TrafficLight) -> bool {
        match (self, new_state) {
            (TrafficLight::Red, TrafficLight::RedYellow) => true,
            (TrafficLight::RedYellow, TrafficLight::Green) => true,
            (TrafficLight::Green, TrafficLight::Yellow) => true,
            (TrafficLight::Yellow, TrafficLight::Red) => true,
            _ => false
        }
    }
  }

  #[test]
  fn traffic_light_transitions() {
    let mut state_machine = StateMachine::new(TrafficLight::Red);
    assert!(state_machine.set(TrafficLight::RedYellow).is_ok());
    assert!(state_machine.set(TrafficLight::Green).is_ok());
    assert!(state_machine.set(TrafficLight::Yellow).is_ok());
    assert!(state_machine.set(TrafficLight::RedYellow).is_err());
  }

}