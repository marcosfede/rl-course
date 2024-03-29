use mdp::{ActionData, ActionOutcome, MDP};
use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::distributions::WeightedIndex;

fn first_action_policy<'a>(actions: &Vec<&'a ActionData>) -> &'a ActionData {
    actions.choose(&mut rand::thread_rng()).unwrap()
}

fn main() {
    let mut g = MDP::new();
    let facebook = g.add_state("Facebook");
    let class1 = g.add_state("Class1");
    let class2 = g.add_state("Class2");
    let class3 = g.add_state("Class3");
    let sleep = g.add_state("Sleep");
    g.add_action(facebook, vec![ActionOutcome::new(facebook)], -1.0);
    g.add_action(facebook, vec![ActionOutcome::new(class1)], 0.0);
    g.add_action(class1, vec![ActionOutcome::new(facebook)], -1.0);
    g.add_action(class1, vec![ActionOutcome::new(class2)], -2.0);
    g.add_action(class2, vec![ActionOutcome::new(sleep)], 0.0);
    g.add_action(class2, vec![ActionOutcome::new(class3)], -2.0);
    g.add_action(class3, vec![ActionOutcome::new(sleep)], 10.0);
    g.add_action(class3, vec![ActionOutcome::new_with_prob(class1, 0.2)], 1.0);
    g.add_action(class3, vec![ActionOutcome::new_with_prob(class2, 0.4)], 1.0);
    g.add_action(class3, vec![ActionOutcome::new_with_prob(class3, 0.4)], 1.0);

    let mut reward = 0.0;
    let mut current_state = class1;
    let mut timesteps = 0;
    loop {
        println!("current state: {:?}", g.get_state(current_state));
        let actions: Vec<&ActionData> = g.actions(current_state).collect();
        let action = first_action_policy(&actions);
        reward += action.reward;
        let dist = WeightedIndex::new(action.outcomes.iter().map(|outcome| outcome.probability)).unwrap();
        current_state = action.outcomes[dist.sample(&mut rand::thread_rng())].state;
        println!();
        timesteps+=1;
        if current_state == sleep {
            break
        }
    }
    println!("reward: {:?}, timesteps: {}", reward, timesteps);
}
