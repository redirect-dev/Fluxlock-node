use std::fs::File;
use std::io::Write;
use rand::Rng;

#[derive(Debug)]
struct Node {
    id: usize,
    stake: f64,
    trust: f64,
    decision: f64, // Node's current vote/decision value
}

impl Node {
    fn new(id: usize, stake: f64) -> Self {
        Node {
            id,
            stake,
            trust: 1.0, // initial trust factor
            decision: 0.0,
        }
    }

    fn make_decision(&mut self) {
        // Decision influenced by some randomness
        let mut rng = rand::thread_rng();
        self.decision = rng.gen_range(0.0..1.0);
    }

    fn update_trust(&mut self, consensus: f64) {
        // Simple trust model: if decision close to consensus, trust increases
        let diff = (self.decision - consensus).abs();
        if diff < 0.1 {
            self.trust += 0.05; // reward aligned nodes
        } else {
            self.trust -= 0.05; // penalize misaligned nodes
        }
        if self.trust < 0.1 {
            self.trust = 0.1; // min trust
        }
        if self.trust > 2.0 {
            self.trust = 2.0; // max trust
        }
    }
}

fn main() {
    let num_nodes = 5;
    let num_ticks = 100;
    let mut nodes: Vec<Node> = (0..num_nodes)
        .map(|i| Node::new(i, 1.0 + i as f64)) // example stake
        .collect();

    let mut log_files = Vec::new();
    for i in 0..num_nodes {
        let file = File::create(format!("node{}_log.csv", i + 1)).expect("Cannot create log");
        let mut writer = csv::Writer::from_writer(file);
        writer
            .write_record(&["tick", "node", "decision", "weighted_decision", "trust"])
            .unwrap();
        log_files.push(writer);
    }

    for tick in 0..num_ticks {
        // Step 1: each node makes a decision
        for node in nodes.iter_mut() {
            node.make_decision();
        }

        // Step 2: compute weighted consensus
        let total_weight: f64 = nodes.iter().map(|n| n.stake * n.trust).sum();
        let weighted_consensus: f64 = nodes
            .iter()
            .map(|n| n.decision * n.stake * n.trust)
            .sum::<f64>()
            / total_weight;

        // Step 3: update trust and log
        for (i, node) in nodes.iter_mut().enumerate() {
            node.update_trust(weighted_consensus);

            log_files[i]
                .write_record(&[
                    tick.to_string(),
                    node.id.to_string(),
                    format!("{:.4}", node.decision),
                    format!("{:.4}", node.decision * node.stake * node.trust),
                    format!("{:.4}", node.trust),
                ])
                .unwrap();
            log_files[i].flush().unwrap();
        }
    }

    println!("Simulation complete. Logs saved to nodeX_log.csv");
}
