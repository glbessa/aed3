use std::cmp::Eq;

pub struct Graph<T: Eq> {
    adjacency_matrix: Vec<Vec<T>>
}

impl<T: Eq> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph {
            adjacency_matrix: Vec::<Vec<T>>::new()
        }
    }

    pub fn from(adjacency_matrix: Vec<Vec<T>>) -> Graph<T> {
        Graph {
            adjacency_matrix
        }
    }

    pub fn get_adjacency_matrix(&self) -> &Vec<Vec<T>> {
        &self.adjacency_matrix
    }

    pub fn is_squared(&self) -> bool {
        for i in 0..self.adjacency_matrix.len() {
            if self.adjacency_matrix[i].len() != self.adjacency_matrix.len() {
                return false;
            }
        }

        true
    }

    pub fn is_symmetric(&self) -> bool {
        if !self.is_squared(){
            return false;
        }

        for i in 0..self.adjacency_matrix.len() {
            for j in 0..self.adjacency_matrix.len() {
                if self.adjacency_matrix[i][j] != self.adjacency_matrix[j][i] {
                    return false;
                }
            }
        }

        true
    }


}