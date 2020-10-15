# rusty_state
A simple state machine for rust

## Example
Copied from [tests](https://github.com/cemoktra/rusty_state/blob/main/src/tests.rs)

```rust
// define an traffic light enumeration
#[derive(Clone, Copy, PartialEq)]
enum TrafficLight {
    Red,
    RedYellow,
    Green,
    Yellow
}

// implement State trait for state enumeration
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

// usage
#[test]
fn traffic_light_transitions() {
    let mut state_machine = StateMachine::new(TrafficLight::Red);
    assert!(state_machine.set(TrafficLight::RedYellow).is_ok());
    assert!(state_machine.set(TrafficLight::Green).is_ok());
    assert!(state_machine.set(TrafficLight::Yellow).is_ok());
    assert!(state_machine.set(TrafficLight::RedYellow).is_err());
}
```
