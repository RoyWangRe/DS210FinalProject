use std::collections::{HashMap, HashSet, VecDeque};
use csv::Reader;
use std::error::Error;

#[derive(Debug)]
struct VideoGame {
    name: String,
    genre: String,
    publisher: String,
    criticScore: Option<f32>,
    userScore: Option<f32>,
}

struct GameGraph {
    adjList: HashMap<String, HashSet<String>>,
}

impl GameGraph {
    fn new() -> Self {
        GameGraph {
            adjList: HashMap::new(),
        }
    }

    fn addEdge(&mut self, game1: &str, game2: &str) {
        self.adjList.entry(game1.to_string()).or_insert(HashSet::new()).insert(game2.to_string());
        self.adjList.entry(game2.to_string()).or_insert(HashSet::new()).insert(game1.to_string());
    }

    fn bfs(&self, start: &str) -> HashMap<String, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        distances.insert(start.to_string(), 0);
        queue.push_back(start.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.adjList.get(&current) {
                for neighbor in neighbors {
                    if !distances.contains_key(neighbor) {
                        distances.insert(neighbor.clone(), distances[&current] + 1);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        return distances
    }

    fn degreeDistribution(&self) -> HashMap<usize, usize> {
        let mut distribution = HashMap::new();
        for neighbors in self.adjList.values() {
            let degree = neighbors.len();
            *distribution.entry(degree).or_insert(0) += 1;
        }
        return distribution
    }

    fn degreeCentrality(&self) -> HashMap<String, usize> {
        let mut centrality = HashMap::new();
        for (node, neighbors) in &self.adjList {
            centrality.insert(node.clone(), neighbors.len());
        }
        return centrality
    }
}

fn buildGraph(filePath: &str) -> Result<GameGraph, Box<dyn Error>> {
    let mut rdr = Reader::from_path(filePath)?;
    let mut games = HashMap::new();
    let mut graph = GameGraph::new();

    for result in rdr.records() {
        let record = result?;
        let name = record.get(0).unwrap_or("").to_string();
        let genre = record.get(3).unwrap_or("").to_string();
        let publisher = record.get(4).unwrap_or("").to_string();
        let criticScore = record.get(6).and_then(|s| s.parse::<f32>().ok());
        let userScore = record.get(7).and_then(|s| s.parse::<f32>().ok());

        let game = VideoGame {
            name: name.clone(),
            genre: genre.clone(),
            publisher: publisher.clone(),
            criticScore,
            userScore,
        };
        games.insert(name.clone(), game);

        for otherName in games.keys() {
            if otherName != &name {
                let otherGame = &games[otherName];
                if otherGame.genre == genre || otherGame.publisher == publisher {
                    graph.addEdge(&name, otherName);
                }
            }
        }
    }
    Ok(graph)
}

fn main() -> Result<(), Box<dyn Error>> {
    let filePath = "/opt/app-root/src/Final/Final/Video_Games_Sales_as_at_22_Dec_2016.csv";
    let graph = buildGraph(filePath)?;

    let startGame = "The Legend of Zelda: Breath of the Wild";
    if graph.adjList.contains_key(startGame) {
        let distances = graph.bfs(startGame);
        let maxDistance = distances.values().max().unwrap_or(&0);
        println!("Max distance from {}: {}", startGame, maxDistance);
    }

    let degreeDistribution = graph.degreeDistribution();
    println!("Degree distribution summary: Total degrees: {}, Max degree: {}",degreeDistribution.values().sum::<usize>(),degreeDistribution.keys().max().unwrap_or(&0));

    let centrality = graph.degreeCentrality();
    let mostCentralGame = centrality.iter().max_by_key(|(_, &degree)| degree).map(|(game, degree)| (game, degree));
    if let Some((game, degree)) = mostCentralGame {
        println!("Most central game is {} with degree {}", game, degree);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testAddEdge() {
        let mut graph = GameGraph::new();
        graph.addEdge("Game1", "Game2");
        graph.addEdge("Game2", "Game3");

        assert_eq!(graph.adjList["Game1"].contains("Game2"), true);
        assert_eq!(graph.adjList["Game2"].contains("Game1"), true);
        assert_eq!(graph.adjList["Game2"].contains("Game3"), true);
        assert_eq!(graph.adjList["Game3"].contains("Game2"), true);
    }

    #[test]
    fn testBfs() {
        let mut graph = GameGraph::new();
        graph.addEdge("Game1", "Game2");
        graph.addEdge("Game2", "Game3");
        graph.addEdge("Game3", "Game4");

        let distances = graph.bfs("Game1");
        assert_eq!(distances["Game1"], 0);
        assert_eq!(distances["Game2"], 1);
        assert_eq!(distances["Game3"], 2);
        assert_eq!(distances["Game4"], 3);
    }

    #[test]
    fn testDegreeDistribution() {
        let mut graph = GameGraph::new();
        graph.addEdge("Game1", "Game2");
        graph.addEdge("Game2", "Game3");
        graph.addEdge("Game3", "Game4");
        graph.addEdge("Game4", "Game1");

        let degreeDistribution = graph.degreeDistribution();
        assert_eq!(degreeDistribution.get(&2).copied().unwrap_or(0), 4);
    }

    #[test]
    fn testDegreeCentrality() {
        let mut graph = GameGraph::new();
        graph.addEdge("Game1", "Game2");
        graph.addEdge("Game1", "Game3");
        graph.addEdge("Game1", "Game4");

        let centrality = graph.degreeCentrality();
        assert_eq!(centrality["Game1"], 3);
        assert_eq!(centrality["Game2"], 1);
        assert_eq!(centrality["Game3"], 1);
        assert_eq!(centrality["Game4"], 1);
    }
}