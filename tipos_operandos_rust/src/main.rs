fn calcular_oleadas(derrotados: i32, por_oleada: i32) -> i32 {
    derrotados / por_oleada
}

fn main() {
    let resultado = calcular_oleadas(7, 2);
    println!("Oleadas: {}", resultado);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_calcular_oleadas() {
        assert_eq!(calcular_oleadas(7, 2), 3);
    }
}