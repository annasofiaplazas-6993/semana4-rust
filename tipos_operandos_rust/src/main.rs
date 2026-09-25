fn calcular_oleadas(derrotados: i32, por_oleada: i32) -> i32 {
    derrotados / por_oleada
}

fn calcular_dano_critico(dano_base: i32, multiplicador: f64) -> f64 {
    dano_base as f64 * multiplicador
}

fn calcular_dano_promedio(dano_base: i32, por_oleada: i32) -> f64 {
    dano_base as f64 / por_oleada as f64
}

fn main() {
    let oleadas = calcular_oleadas(7, 2);
    let dano_critico = calcular_dano_critico(100, 1.5);
    let dano_promedio = calcular_dano_promedio(100, 4);

    println!("Oleadas: {}", oleadas);
    println!("Daño crítico: {}", dano_critico);
    println!("Daño promedio: {}", dano_promedio);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_calcular_oleadas() {
        assert_eq!(calcular_oleadas(7, 2), 3);
    }

    #[test]
    fn prueba_calcular_dano_critico() {
        assert_eq!(calcular_dano_critico(100, 1.5), 150.0);
    }

    #[test]
    fn prueba_calcular_dano_promedio() {
        assert_eq!(calcular_dano_promedio(100, 4), 25.0);
    }
}