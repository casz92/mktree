use sha2::{Sha256, Digest};

// Función para hashear una hoja individual de datos.
// Se usa para hashear los datos iniciales antes de construir el árbol.
#[rustler::nif]
fn hash_leaf(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes()); // Hashear los bytes de la cadena
    format!("{:x}", hasher.finalize()) // Formatear el hash como una cadena hexadecimal
}

// Función auxiliar para hashear una hoja, idéntica a hash_leaf.
// Utilizada internamente por otras funciones para mayor claridad.
fn phash_leaf(data: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Construye un árbol Merkle completo a partir de una lista de hojas ya hasheadas.
// Retorna una lista de capas, donde cada capa es una lista de hashes.
// La primera capa son las hojas iniciales, la última es la raíz.
#[rustler::nif]
fn build_tree(hashed_leaves: Vec<String>) -> Vec<Vec<String>> {
    // Si no hay hojas, el árbol está vacío.
    if hashed_leaves.is_empty() {
        return vec![];
    }

    let mut layers = vec![hashed_leaves.clone()]; // La primera capa son las hojas hasheadas
    let mut current_nodes = hashed_leaves; // Nodos de la capa actual

    // Continuar construyendo capas hasta que solo quede un nodo (la raíz)
    while current_nodes.len() > 1 {
        let mut next_layer_nodes = vec![]; // Nodos de la siguiente capa (padres)
        let mut i = 0;

        // Iterar a través de los nodos de la capa actual en pares
        while i < current_nodes.len() {
            let left = &current_nodes[i]; // Nodo izquierdo del par

            // Determinar el nodo derecho:
            // Si existe un nodo en i + 1, ese es el derecho.
            // Si no (es decir, la capa tiene un número impar de nodos y este es el último),
            // el nodo derecho es el mismo que el izquierdo (duplicación).
            let right = if i + 1 < current_nodes.len() {
                &current_nodes[i + 1]
            } else {
                left // Duplicar el último nodo si la cantidad es impar
            };

            // Hashear el par de nodos (izquierda + derecha)
            let mut hasher = Sha256::new();
            hasher.update(left.as_bytes());
            hasher.update(right.as_bytes());
            next_layer_nodes.push(format!("{:x}", hasher.finalize()));

            i += 2; // Mover al siguiente par
        }
        layers.push(next_layer_nodes.clone()); // Añadir la nueva capa al árbol
        current_nodes = next_layer_nodes; // La nueva capa se convierte en la capa actual para la siguiente iteración
    }

    layers // Retornar todas las capas del árbol
}

// Función auxiliar para construir el árbol, idéntica a build_tree.
// Utilizada internamente por otras funciones para mayor claridad.
fn pbuild_tree(hashed_leaves: Vec<String>) -> Vec<Vec<String>> {
    if hashed_leaves.is_empty() {
        return vec![];
    }

    let mut layers = vec![hashed_leaves.clone()];
    let mut current_nodes = hashed_leaves;

    while current_nodes.len() > 1 {
        let mut next_layer_nodes = vec![];
        let mut i = 0;
        while i < current_nodes.len() {
            let left = &current_nodes[i];
            let right = if i + 1 < current_nodes.len() {
                &current_nodes[i + 1]
            } else {
                left
            };

            let mut hasher = Sha256::new();
            hasher.update(left.as_bytes());
            hasher.update(right.as_bytes());
            next_layer_nodes.push(format!("{:x}", hasher.finalize()));

            i += 2;
        }
        layers.push(next_layer_nodes.clone());
        current_nodes = next_layer_nodes;
    }

    layers
}

// Calcula la raíz Merkle a partir de una lista de hojas de datos (no hasheadas).
// Primero hashea las hojas y luego construye el árbol para obtener la raíz.
#[rustler::nif]
fn compute_root(leaves: Vec<String>) -> String {
    if leaves.is_empty() {
        return String::new(); // Raíz vacía si no hay hojas
    }

    // Hashear las hojas iniciales para obtener la primera capa de hashes
    let hashed_leaves: Vec<String> = leaves.iter().map(|leaf| phash_leaf(leaf.clone())).collect();

    let mut nodes = hashed_leaves; // Empezar con las hojas hasheadas
    // Continuar hasheando pares hasta que solo quede un nodo (la raíz)
    while nodes.len() > 1 {
        let mut parent_nodes = vec![];
        let mut i = 0;
        while i < nodes.len() {
            let left = &nodes[i];
            let right = if i + 1 < nodes.len() {
                &nodes[i + 1]
            } else {
                left // Duplicar el último nodo si la cantidad es impar
            };

            let mut hasher = Sha256::new();
            hasher.update(left.as_bytes());
            hasher.update(right.as_bytes());
            parent_nodes.push(format!("{:x}", hasher.finalize()));

            i += 2;
        }
        nodes = parent_nodes; // La nueva capa se convierte en la capa actual
    }

    nodes[0].clone() // La raíz es el único nodo que queda
}

// Verifica una prueba Merkle para una hoja dada.
// `leaf`: la hoja de datos original (no hasheada).
// `proof`: una lista de tuplas (hash_hermano, es_izquierdo).
// `root`: el hash de la raíz Merkle esperado.
// Retorna `true` si la prueba es válida, `false` en caso contrario.
#[rustler::nif]
fn verify_proof(leaf: String, proof: Vec<(String, bool)>, root: String) -> bool {
    // Hashear la hoja inicial para empezar la verificación
    let mut current_hash = phash_leaf(leaf);

    // Iterar a través de los elementos de la prueba
    for (sibling_hash, is_left_sibling) in proof {
        let mut hasher = Sha256::new();
        // Si el hermano está a la izquierda, se hashea primero el hermano, luego el hash actual.
        // De lo contrario, se hashea primero el hash actual, luego el hermano.
        // Esto mantiene el orden de concatenación consistente con la construcción del árbol.
        if is_left_sibling {
            hasher.update(sibling_hash.as_bytes());
            hasher.update(current_hash.as_bytes());
        } else {
            hasher.update(current_hash.as_bytes());
            hasher.update(sibling_hash.as_bytes());
        }
        current_hash = format!("{:x}", hasher.finalize()); // Actualizar el hash actual
    }

    // La prueba es válida si el hash final coincide con la raíz esperada
    current_hash == root
}

// Genera una prueba Merkle para una hoja de datos específica.
// `leaves`: la lista completa de hojas de datos originales (no hasheadas).
// `target_leaf`: la hoja de datos para la que se quiere generar la prueba.
// Retorna una lista de tuplas (hash_hermano, es_izquierdo) que forman la prueba.
#[rustler::nif]
fn generate_proof(leaves: Vec<String>, target_leaf: String) -> Vec<(String, bool)> {
    // Primero, hashear todas las hojas iniciales para formar la capa base del árbol Merkle.
    let hashed_leaves: Vec<String> = leaves.iter().map(|leaf| phash_leaf(leaf.clone())).collect();

    // Construir el árbol Merkle completo a partir de las hojas hasheadas.
    // pbuild_tree ahora maneja correctamente los números impares de nodos duplicando el último.
    let tree = pbuild_tree(hashed_leaves.clone());
    let mut proof = vec![];

    // Hashear la hoja objetivo para encontrar su posición en la primera capa del árbol.
    let target_leaf_hash = phash_leaf(target_leaf.clone());
    let mut current_index = match tree[0].iter().position(|x| x == &target_leaf_hash) {
        Some(idx) => idx,
        None => return vec![], // Si la hoja hasheada no se encuentra, no se puede generar la prueba.
    };

    // Iterar a través de cada capa del árbol, desde las hojas hasta la capa justo antes de la raíz.
    for layer_idx in 0..tree.len() - 1 {
        let current_layer = &tree[layer_idx];
        let sibling_hash;
        let is_left_sibling; // Verdadero si el hermano está a la izquierda del nodo actual, falso si está a la derecha.

        if current_index % 2 == 0 {
            // El nodo actual es un hijo izquierdo. Su hermano está a su derecha.
            // Verificar si el hermano derecho existe. Si no, significa que este nodo fue duplicado.
            if current_index + 1 < current_layer.len() {
                sibling_hash = current_layer[current_index + 1].clone();
                is_left_sibling = false; // El hermano está a la derecha
            } else {
                // Este caso ocurre cuando el nodo actual es el último nodo en una capa de tamaño impar.
                // En nuestra lógica de build_tree, este nodo fue duplicado y hasheado consigo mismo.
                // Por lo tanto, su "hermano" para la prueba es él mismo.
                sibling_hash = current_layer[current_index].clone();
                is_left_sibling = false; // El orden de concatenación no importa si es consigo mismo.
            }
        } else {
            // El nodo actual es un hijo derecho. Su hermano está a su izquierda.
            sibling_hash = current_layer[current_index - 1].clone();
            is_left_sibling = true; // El hermano está a la izquierda
        }
        proof.push((sibling_hash, is_left_sibling)); // Añadir el hermano y su posición a la prueba

        // Mover al índice del nodo padre en la siguiente capa.
        current_index /= 2;
    }

    proof // Retornar la prueba generada
}

// Inicialización de la librería Rustler para Elixir.
// Se exponen las funciones NIF al módulo Elixir.Mktree.
rustler::init!("Elixir.Mktree");