import time
import itertools
import sys
import utils
import numpy as np
import networkx as nx

def calcular_distancia(cidade1, cidade2):
    return np.linalg.norm(np.array(cidade1) - np.array(cidade2))

def calcular_distancia_total(caminho, matriz_adjacencia):
    distancia_total = 0
    for i in range(len(caminho) - 1):
        cidade_atual = caminho[i]
        proxima_cidade = caminho[i + 1]
        distancia_total += matriz_adjacencia[cidade_atual][proxima_cidade]
    distancia_total += matriz_adjacencia[caminho[-1]][caminho[0]]
    return distancia_total

def construir_caminho_aproximado(matriz_adjacencia):
    # Construir árvore geradora mínima
    # (Use uma implementação eficiente para grandes conjuntos de dados)
    # Aqui, estamos usando o algoritmo de Prim.
    num_cidades = len(matriz_adjacencia)
    arvore_geradora_minima = prim(matriz_adjacencia)

    # Encontrar vértices de grau ímpar na árvore geradora mínima
    vertices_grau_impar = [v for v in range(num_cidades) if arvore_geradora_minima.degree(v) % 2 != 0]

    # Encontrar emparelhamento perfeito mínimo no subgrafo induzido pelos vértices de grau ímpar
    emparelhamento_perfeito = emparelhamento_perfeito_minimo(matriz_adjacencia, vertices_grau_impar)

    # Construir circuito euleriano
    circuito_euleriano = construir_circuito_euleriano(arvore_geradora_minima, emparelhamento_perfeito)

    # Remover visitas repetidas no circuito euleriano
    circuito_final = remover_repeticoes(circuito_euleriano)

    return circuito_final

def prim(matriz_adjacencia):
    num_cidades = len(matriz_adjacencia)
    arvore_geradora_minima = nx.Graph()

    # Inicialização: Adiciona a primeira cidade ao conjunto da árvore geradora mínima
    conjunto_cidades = set([0])

    while len(conjunto_cidades) < num_cidades:
        menor_peso = float('inf')
        aresta_menor_peso = None

        for cidade_na_arvore in conjunto_cidades:
            for cidade_fora_arvore in range(num_cidades):
                if cidade_fora_arvore not in conjunto_cidades:
                    peso_aresta = matriz_adjacencia[cidade_na_arvore][cidade_fora_arvore]
                    if peso_aresta < menor_peso:
                        menor_peso = peso_aresta
                        aresta_menor_peso = (cidade_na_arvore, cidade_fora_arvore)

        arvore_geradora_minima.add_edge(*aresta_menor_peso)
        conjunto_cidades.add(aresta_menor_peso[1])

    return arvore_geradora_minima

def emparelhamento_perfeito_minimo(matriz_adjacencia, vertices_grau_impar):
    # Implementação simples para ilustração
    emparelhamento_perfeito = []
    vertices_disponiveis = set(vertices_grau_impar)

    while vertices_disponiveis:
        v = vertices_disponiveis.pop()
        u = encontrar_vizinho_com_peso_minimo(matriz_adjacencia, v, vertices_disponiveis)
        emparelhamento_perfeito.append((u, v))
        vertices_disponiveis.remove(u)

    return emparelhamento_perfeito

def encontrar_vizinho_com_peso_minimo(matriz_adjacencia, v, vertices_disponiveis):
    menor_peso = float('inf')
    vizinho = None

    for u in vertices_disponiveis:
        peso_aresta = matriz_adjacencia[v][u]
        if peso_aresta < menor_peso:
            menor_peso = peso_aresta
            vizinho = u

    return vizinho

def construir_circuito_euleriano(arvore_geradora_minima, emparelhamento_perfeito):
    grafo_auxiliar = arvore_geradora_minima.copy()

    for aresta in emparelhamento_perfeito:
        grafo_auxiliar.add_edge(*aresta)

    circuito_euleriano = list(nx.eulerian_circuit(grafo_auxiliar))

    return circuito_euleriano

def remover_repeticoes(circuito_euleriano):
    circuito_final = []
    visitados = set()

    for aresta in circuito_euleriano:
        if aresta[0] not in visitados:
            circuito_final.append(aresta[0])
            visitados.add(aresta[0])

    circuito_final.append(circuito_final[0])  # Volte para o ponto inicial

    return circuito_final

def caixeiro_viajante_forca_bruta(matriz_adjacencia):
    num_cidades = len(matriz_adjacencia)
    melhor_rota = None
    menor_distancia = float('inf')

    # Gerar todas as permutações possíveis das cidades
    todas_rotas = itertools.permutations(range(num_cidades))

    for rota in todas_rotas:
        distancia_total = 0
        for i in range(num_cidades - 1):
            cidade_atual, proxima_cidade = rota[i], rota[i+1]
            distancia_total += matriz_adjacencia[cidade_atual][proxima_cidade]

        # Adicionar a distância de volta à cidade de origem
        distancia_total += matriz_adjacencia[rota[-1]][rota[0]]

        # Atualizar a melhor rota se encontrarmos uma com menor distância
        if distancia_total < menor_distancia:
            menor_distancia = distancia_total
            melhor_rota = rota

    return melhor_rota, menor_distancia

def calcular_distancia_total(cidades, rota):
    distancia_total = 0
    for i in range(len(rota) - 1):
        distancia_total += cidades[rota[i]][rota[i+1]]
    distancia_total += cidades[rota[-1]][rota[0]]
    return distancia_total

def trocar_arcos(rota, i, k):
    return rota[:i] + rota[i:k+1][::-1] + rota[k+1:]

def two_opt(matriz_adjacencia):
    num_cidades = len(matriz_adjacencia)
    rota_inicial = list(range(num_cidades))  # Rota inicial: 0, 1, 2, ..., num_cidades-1
    melhor_rota = rota_inicial
    melhor_distancia = calcular_distancia_total(matriz_adjacencia, rota_inicial)

    melhoria = True
    while melhoria:
        melhoria = False
        for i in range(1, num_cidades - 2):
            for k in range(i + 1, num_cidades):
                nova_rota = trocar_arcos(melhor_rota, i, k)
                nova_distancia = calcular_distancia_total(matriz_adjacencia, nova_rota)
                if nova_distancia < melhor_distancia:
                    melhor_rota = nova_rota
                    melhor_distancia = nova_distancia
                    melhoria = True

    return melhor_rota, melhor_distancia

if __name__ == "__main__":
    if len(sys.argv) < 3:
        utils.print_help()
        sys.exit(1)

    tsp_file = sys.argv[1]
    algorithm_name = sys.argv[2]
    algorithm = None
    matriz_adjacencia = utils.read_tsp_file(tsp_file)

    if algorithm_name == "brute-force":
        algorithm = caixeiro_viajante_forca_bruta
    elif algorithm_name == "christ":
        algorithm = construir_caminho_aproximado
    else:
        algorithm = two_opt

    inicio = time.time()
    melhor_rota, menor_distancia = algorithm(matriz_adjacencia)
    fim = time.time()

    tempo_execucao = fim - inicio

    print("Arquivo:", tsp_file)
    print("Algoritmo:", algorithm_name)
    print("Melhor rota:", melhor_rota)
    print("Menor distância:", menor_distancia)
    print("Tempo de execução:", tempo_execucao, "segundos")

