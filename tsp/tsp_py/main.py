import time
import itertools
import sys
import utils

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

