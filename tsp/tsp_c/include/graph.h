#ifndef GRAPH_H
#define GRAPH_H

#include <stdlib.h>
#include <stdio.h>

typedef struct Graph Graph;
struct Graph {
    unsigned int num_vertices;
    void *vertices;
    unsigned int **adjacency_matrix;
};

Graph *graph_create(unsigned int num_vertices) {

}

void graph_destroy(Graph *graph) {

}

void graph_add_edge(Graph *graph, unsigned int vertex1, unsigned int vertex2, unsigned int weight) {

}

void graph_remove_edge(Graph *graph, unsigned int vertex1, unsigned int vertex2) {

}

unsigned int graph_get_edge_weight(Graph *graph, unsigned int vertex1, unsigned int vertex2) {

}

unsigned int  graph_get_num_vertices(Graph *graph) {

}

unsigned int graph_get_num_edges(Graph *graph) {

}

unsigned int graph_get_vertex_degree(Graph *graph, unsigned int vertex) {

}

unsigned int graph_get_route_cost(Graph *graph, unsigned int *route, unsigned int route_length) {

}

void graph_print(Graph *graph) {

}

#endif