use rand::Rng;
use tch::{nn, nn::Module, nn::OptimizerConfig, Tensor, Device, Kind};

#[derive(Debug, Clone)]
struct WormState {
    rel_x_to_food: f32,
    rel_y_to_food: f32,
    food_amount: f32,
    life: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

struct DQNetwork {
    model: nn::Sequential,
}

impl DQNetwork {
    fn new(vs: &nn::Path) -> Self {
        let model = nn::seq()
            .add(nn::linear(vs / "layer1", 5, 64, Default::default()))
            .add_fn(|x| x.relu())
            .add(nn::linear(vs / "layer2", 64, 4, Default::default()));
        Self { model }
    }

    fn predict(&self, state: &WormState) -> Tensor {
        let input = Tensor::of_slice(&[
            state.rel_x_to_food,
            state.rel_y_to_food,
            state.food_amount,
            state.life,
            state.speed,
        ])
            .view([1, 5]); // batch size 1, input dimension 5
        self.model.forward(&input)
    }
}

fn choose_action(q_values: &Tensor, epsilon: f32) -> Action {
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() < epsilon {
        let random_action = rng.gen_range(0..4);
        return match random_action {
            0 => Action::MoveLeft,
            1 => Action::MoveRight,
            2 => Action::MoveUp,
            _ => Action::MoveDown,
        };
    }
    match q_values.argmax(1, false).int64_value(&[0]) {
        0 => Action::MoveLeft,
        1 => Action::MoveRight,
        2 => Action::MoveUp,
        _ => Action::MoveDown,
    }
}

fn train_dqn() {
    let device = Device::cuda_if_available();
    let vs = nn::VarStore::new(device);
    let net = DQNetwork::new(&vs.root());
    let mut opt = nn::Adam::default().build(&vs, 1e-3).unwrap();

    let mut epsilon = 1.0;
    let epsilon_decay = 0.995;
    let gamma = 0.9;

    for episode in 0..1000 {
        let mut state = WormState {
            rel_x_to_food: 0.5, // these would come from simulation logic
            rel_y_to_food: -0.3,
            food_amount: 0.8,
            life: 0.5,
            speed: 0.6,
        };
        let mut total_reward = 0.0;

        for _ in 0..100 {
            // Get Q-values for current state
            let q_values = net.predict(&state);
            let action = choose_action(&q_values, epsilon);

            // Simulate taking action (dummy reward & next state)
            let (next_state, reward) = match action {
                Action::MoveLeft => (state.clone(), -0.1),   // dummy values
                Action::MoveRight => (state.clone(), 0.5),   // replace with real game logic
                Action::MoveUp => (state.clone(), 1.0),      // positive for moving closer to food
                Action::MoveDown => (state.clone(), -0.1),   // negative for moving away
            };

            total_reward += reward;

            // Update Q-value using Bellman equation
            let q_target = reward + gamma * net.predict(&next_state).max().double_value(&[]);
            let q_update = q_values.gather(1, &Tensor::of_slice(&[action as i64]), false);
            let loss = (q_update - q_target).pow(
                &Tensor::of_slice(&[2.0]),
            )
                .mean(Kind::Float);

            opt.backward_step(&loss);

            state = next_state; // move to next state

            if epsilon > 0.1 {
                epsilon *= epsilon_decay;
            }
        }
        println!("Episode {}: Total reward = {}", episode, total_reward);
    }
}
fn main() {
    train_dqn();
}
