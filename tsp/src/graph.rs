use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::cmp::{Ordering, Reverse, Eq, Ord};
use std::convert::TryInto;
use std::clone::Clone;
use std::fmt::Display;

pub struct Graph<V: Eq + Display + Clone, M: Clone + Display> {
    vertices: Vec<V>,
    adjacency_matrix: Vec<Vec<M>>
}

impl<V: Eq + Display + Clone, M: Clone + Display + Eq + Ord> Graph<V, M> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::<V>::new(),
            adjacency_matrix: Vec::<Vec<M>>::new()
        }
    }

    pub fn from(vertices: Vec<V>, adjacency_matrix: Vec<Vec<M>>) -> Self {
        Graph {
            vertices: vertices,
            adjacency_matrix
        }
    }

    pub fn get_adjacency_matrix(&self) -> &Vec<Vec<M>> {
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

    pub fn insert_vertex(&mut self, vertex: V) {
        self.vertices.push(vertex);

        self.adjacency_matrix.push(vec![0; self.vertices.len()]);

        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].push(0);
        }
    }

    pub fn remove_vertex(&mut self, vertex_index: usize) -> Result<(), &'static str> {
        if self.vertices.len() <= vertex_index {
            return Err("Index out of range!");
        }

        self.vertices.remove(vertex_index);

        Ok(())
    }

    pub fn get_vertex(&self, vertex_idx: usize) -> Result<&V, &'static str> {
        if self.vertices.len() <= vertex_idx {
            return Err("Index out of range!");
        }

        Ok(&self.vertices[vertex_idx])
    }

    pub fn insert_edge(&mut self, src_idx: usize, dst_idx: usize, edge_weight: M, directed: bool) -> Result<(), &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        self.adjacency_matrix[src_idx][dst_idx] = edge_weight;
        
        if directed == true {
            self.adjacency_matrix[dst_idx][src_idx] = edge_weight;
        }

        Ok(())
    }

    pub fn remove_edge(&mut self, src_idx: usize, dst_idx: usize, directed: bool) -> Result<(), &'static str> {
        if self.num_vertices() <= src_idx || self.num_vertices() <= dst_idx {
            return Err("Index out of range!");
        }

        self.adjacency_matrix[src_idx][dst_idx] = 0;
        
        if directed == true {
            self.adjacency_matrix[dst_idx][src_idx] = 0;
        }        

        Ok(())
    }

    pub fn get_edge_weight(&self, src_idx: usize, dst_idx: usize) -> Result<M, &'static str> {
        if self.num_vertices() <= src_idx || self.num_vertices() <= dst_idx {
            return Err("Index out of range!");
        }

        let weight: M = self.adjacency_matrix[src_idx][dst_idx];

        Ok(weight)
    }

    pub fn get_adjacent_vertices(&self, vertex_idx: usize) -> Result<Vec<usize>, &'static str> {
        if self.num_vertices() <= vertex_idx {
            return Err("Index out of range!");
        }

        let mut adjacent_vertices: Vec<usize> = Vec::new();
        
        for i in 0..self.num_vertices() {
            if self.get_edge_weight(vertex_idx, i).unwrap() > 0 {
                adjacent_vertices.push(i);
            }
        }

        Ok(adjacent_vertices)
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_dijkstra_path(&self, src_idx: usize, dst_idx: usize) -> Result<VecDeque<usize>, &'static str> {
        if self.vertices.len() <= src_idx || self.vertices.len() <= dst_idx {
            return Err("Index out of range!");
        }

        let mut previous_vertex: Vec<Option<usize>> = vec![None; self.vertices.len()];
        let mut path_cost: Vec<Option<usize>> = vec![None; self.vertices.len()];
        let mut is_closed: Vec<bool> = vec![false; self.vertices.len()];
        let mut vert_to_visit: BinaryHeap<Reverse<(u64, usize)>> = BinaryHeap::new();
        let mut vertex_idx: usize;

        // custo do no inicial é 0
        path_cost[src_idx] = Some(0);
        // insere o no inicial na heap
        vert_to_visit.push(Reverse((0, src_idx)));

        loop {
            // remove o primeiro nó da heap
            vertex_idx = match vert_to_visit.pop() {
                Some(Reverse((_, v))) => v,
                None => break
            };

            // verifica se esse no ja nao esta fechado
            if is_closed[vertex_idx] {
                continue;
            }

            // fecha o no
            is_closed[vertex_idx] = true;

            // verifica os adjacentes desse no, atualiza seus pesos e adiciona na heap
            match self.get_adjacent_vertices(vertex_idx) {
                Ok(list) => {
                    for idx in list.iter() {
                        if is_closed[*idx] {
                            continue;
                        }

                        if self.get_edge_weight(vertex_idx, *idx).unwrap() < 0 {
                            continue;
                        }

                        let total_cost: usize = match path_cost[vertex_idx] {
                            Some(cost_v_idx) => cost_v_idx + self.get_edge_weight(vertex_idx, *idx).unwrap() as usize,
                            None => self.get_edge_weight(vertex_idx, *idx).unwrap() as usize
                        };

                        match path_cost[*idx] {
                            Some(cost) => {
                                if cost > total_cost {
                                    path_cost[*idx] = Some(total_cost);
                                    previous_vertex[*idx] = Some(vertex_idx);
                                    vert_to_visit.push(Reverse((total_cost.try_into().unwrap(), *idx)));
                                }
                            },
                            None => {
                                path_cost[*idx] = Some(total_cost);
                                previous_vertex[*idx] = Some(vertex_idx);
                                vert_to_visit.push(Reverse((total_cost.try_into().unwrap(), *idx)));
                            }
                        }
                    }
                },
                Err(msg) => return Err(msg)
            }
        }

        let mut path: VecDeque<usize> = VecDeque::new();
        path.push_front(dst_idx);
        let mut actual_vertex: usize = dst_idx;

        loop {
            actual_vertex = match previous_vertex[actual_vertex] {
                Some(v_idx) => v_idx,
                None => break
            };

            path.push_front(actual_vertex);
        }

        Ok(path)
    }

    // Algoritmo de Kruskal: o conjunto A é  uma floresta cujos vértices são todos os vértices do grafo e a aresta segura
    //   adicionada é sempre uma aresta de peso mínimo no grafo que conecta duas componentes distintas.
    pub fn get_mst_kruskal(&self) -> Self {
        let mut a: HashSet<(usize, usize)> = HashSet::new();
        let mut v_sets: Vec<HashSet<usize>> = Vec::new();
        let mut heap: BinaryHeap<Reverse<(u64, (usize, usize))>> = BinaryHeap::new();

        // Criando a floresta de conjuntos
        for i in 0..self.vertices.len() {
            v_sets.push(HashSet::from([i]));
        }

        // Ordenando as arestas
        for i in 0..self.adjacency_matrix.len() {
            for j in 0..self.adjacency_matrix.len() {
                if self.get_edge_weight(i, j).unwrap() != 0 {
                    heap.push(Reverse((self.get_edge_weight(i, j).unwrap(), (i, j))));
                }
            }
        }

        for _ in 0..heap.len() {
            // Remove a aresta de menor peso da heap
            let (u, v) = match heap.pop() {
                Some(Reverse((_, (u1, v1)))) => (u1, v1),
                None => (0, 0)
            };
            let (mut set1_idx, mut set2_idx): (usize, usize) = (0, 0);

            // Verifica em todos os sets se src ou dst estão inclusos neles
            for i in 0..v_sets.len() {
                if v_sets[i].contains(&u) {
                    set1_idx = i;
                }

                if v_sets[i].contains(&v) {
                    set2_idx = i;
                }
            }

            // Caso os dois não estejam no mesmo set
            if set1_idx != set2_idx {
                // Insere-se essa aresta
                a.insert((u, v));
                
                // E junta os sets
                let t = v_sets.remove(set2_idx);
                
                if set2_idx < set1_idx {
                    v_sets[set1_idx-1].extend(&t);
                }
                else {
                    v_sets[set1_idx].extend(&t);
                }
            }
        }

        let mut adjacency_matrix: Vec<Vec<u64>> = vec![vec![0; self.adjacency_matrix.len()]; self.adjacency_matrix.len()];
        for (src, dst) in a.into_iter() {
            adjacency_matrix[src][dst] = self.get_edge_weight(src, dst).unwrap();
        }

        return Graph::from(self.vertices.clone(), adjacency_matrix);
    }

    // Algoritmo de Prim: inicia adicionando ao conjunto A os vertices ligados pela aresta de menor custo
    //      e após vai adicionando os vertices que tiverem menor custo e estejam sejam adjacentes aos já existentes
    // https://pt.wikipedia.org/wiki/Algoritmo_de_Prim
    pub fn get_mst_prim(&self) -> Graph<V, u64> {
        let mut a: HashSet<usize> = HashSet::with_capacity(self.vertices.len());
        let mut heap: BinaryHeap<Reverse<(u64, (usize, usize))>> = BinaryHeap::new();
        let mut edges: Vec<(usize, usize)> = Vec::with_capacity(self.vertices.len() - 1);

        // Pega a aresta de menor valor diferente de zero
        let mut min: (usize, usize) = (0, 0);
        for i in 0..self.vertices.len() {
            for j in 0..self.vertices.len() {
                if self.get_edge_weight(i, j).unwrap() != 0 && self.get_edge_weight(i, j).unwrap() < self.get_edge_weight(min.0, min.1).unwrap() {
                    min = (i, j);
                }
            }
        }

        // Adicionando a primeira aresta na heap
        heap.push(Reverse((self.get_edge_weight(min.0, min.1).unwrap(), min)));
        a.insert(min.0);
        let adjacents = self.get_adjacent_vertices(min.0).unwrap();

        // Adicionando as arestas para adjacentes do primeiro vertice na heap ---
        for adj_vertex in adjacents.into_iter() {
            if a.contains(&adj_vertex) {
                continue;
            }

            heap.push(Reverse((self.get_edge_weight(min.0, adj_vertex).unwrap(), (min.0, adj_vertex))));
        }

        // Itera ate que todos os vertices estejam acessiveis
        loop {
            if a.len() == self.vertices.len() {
                break;
            }

            // Remove da heap
            let (priority, src, dst) = match heap.pop() {
                Some(Reverse((prio, (sr, ds)))) => (prio, sr, ds),
                None => { continue; }
            };

            if a.contains(&dst) {
                continue;
            }

            // Insere um vertice que nao havia sido explorado ainda
            a.insert(dst);
            // Adiciona a aresta no vetor de arestas
            edges.push((src, dst));
            // Pega os adjacentes
            let adjacents = self.get_adjacent_vertices(dst).unwrap();

            for adj_vertex in adjacents.into_iter() {
                if a.contains(&adj_vertex) {
                    continue;
                }

                // Popula a heap denovo
                heap.push(Reverse((self.get_edge_weight(dst, adj_vertex).unwrap(), (dst, adj_vertex))));
            }
        }

        // Transforma tudo em um novo grafo :)
        let mut adjacency_matrix: Vec<Vec<u64>> = vec![vec![0; self.adjacency_matrix.len()]; self.adjacency_matrix.len()];
        for (src, dst) in edges.into_iter() {
            adjacency_matrix[src][dst] = self.get_edge_weight(src, dst).unwrap();
        }

        return Graph::from(self.vertices.clone(), adjacency_matrix);
    }

    pub fn tsp_2_approx(&self) -> Self {
        todo!()
    }
}