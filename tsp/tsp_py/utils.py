def read_tsp_file(filepath):
    content = None
    with open(filepath, "r") as writer:
        content = writer.readlines()
        
    content = [line.split(" ") for line in content]
    content = [[int(x) for x in line if x != ""] for line in content]
    return content

def print_help():
    print("Usage: python main.py <tsp_file> <algorithm>")
    print("    Algorithms: brute-force, 2opt")