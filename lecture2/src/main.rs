struct State<'a> {
    name: String,
    edges: Vec<Edge<'a>>,
}

struct Edge<'a> {
    to: &'a State<'a>,
    reward: isize,
}

const GAMMA: usize = 1;
const STATES: [&str; 4] = ["Facebook", "Class1", "Class2", "Class3"];
fn main() {
    let mut Facebook = State {
        name: String::from("Facebook"),
        edges: Vec::new(),
    };
    let Class1 = State {
        name: String::from("Class1"),
        edges: Vec::new(),
    };
    let Class2 = State {
        name: String::from("Class2"),
        edges: Vec::new(),
    };
    let Class3 = State {
        name: String::from("Class3"),
        edges: Vec::new(),
    };
    Facebook.edges.push(Edge {to: &Facebook, reward: -1});
    Facebook.edges.push(Edge {to: &Class1, reward: 0});
    // Facebook.edges.push(Edge {to: &Facebook, reward: -1});
    let REWARDS: [Vec<isize>; 4] = [vec![-1, 0], vec![-2], vec![0, -2], vec![1, 10]];
    let ACTIONS: [Vec<&str>; 4] = [
        vec!["Facebook", "Quit"],
        vec!["Study"],
        vec!["Sleep", "Study"],
        vec!["Pub", "Study"],
    ];
}
