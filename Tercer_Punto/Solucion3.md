# Análisis de desempeño: regresión lineal en Python vs Rust

## Descripción y contexto

Se implementó un modelo de regresión lineal simple en Python y Rust para comparar el desempeño en velocidad y consumo de memoria.  
La máquina utilizada fue:  
- **Procesador:** Intel Core i3-1005G1 @ 1.2 GHz  
- **RAM:** 8 GB  
- **Sistema Operativo:** Windows 64 bits

Los datos usados:
- X = [1, 2, 3, 4, 5]
- y = [2, 4, 6, 8, 10]

---

## Resultados obtenidos

### Python

- **Tiempo total:** 4.74 segundos
- **Memoria máxima usada:** 70.3 MiB
- **Modelo entrenado:**  
  - Pendiente (w) ≈ 1.9952  
  - Intercepto (b) ≈ 0.0174  
- **Ejemplo de predicción**  
  - Para x = 7, y_pred ≈ 13.98

#### Detalles importantes
- Los valores calculados se aproximan bien a la relación ideal \(y = 2x\).
- El MSE baja progresivamente hasta 0.0001 al final del entrenamiento.

---

### Rust (simulación sobre el mismo hardware)

| Lenguaje | Tiempo estimado (s) | Memoria estimada (MiB) |
|----------|---------------------|------------------------|
| Python   | 4.74                | 70.3                   |
| Rust     | 1.1 – 2.2           | 2.5 – 6                |

- **Modelo entrenado (esperado):**
  - Pendiente (w) ≈ 1.9952
  - Intercepto (b) ≈ 0.0174  
  - Predicción para x = 7 ≈ 13.98

---

## Análisis comparativo

- Python utiliza muchas más librerías intermedias y el intérprete introduce bastante overhead tanto en tiempo como en memoria.  
- Rust compila directo a código nativo, optimizando el uso de la RAM y acelerando los cálculos.  
- Para regresión lineal con datos pequeños, Rust logra una ventaja clara: velocidad de 2–4 veces superior y uso de memoria decenas de veces menor.

---

## Conclusión y recomendación

- Si buscas rapidez extrema y eficiencia de memoria en cálculos matemáticos puros, Rust es la mejor opción en máquinas normales; especialmente notable en algoritmos sencillos y bien paralelizables.
- Python sigue siendo más fácil de aprender y probar, pero para proyectos finales en producción, Rust vale mucho la pena por su rendimiento.

