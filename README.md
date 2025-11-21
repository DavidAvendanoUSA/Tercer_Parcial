# Tercer_Parcial

Este repositorio contiene los tres puntos del parcial, organizados en carpetas separadas:

## Primer_punto

**Usando el paradigma de concurrencia y cálculo PI, diseñe una solución que realice la regresión lineal.**  
> **NOTA:** Aquí solo encuentras el **diseño detallado**, no la implementación. El documento explica paso a paso cómo estructurar la solución usando hilos, canales o procesos, cómo repartir subcálculos de PI y tareas de regresión en paralelo, sincronización y cómo se integrarían todos los resultados para ajustar los coeficientes finales.  
> Se detalla: creación de tareas concurrentes, gestión de recursos, comunicación entre entidades, y flujo de datos entre el cálculo de PI y el modelo de regresión.

## Segundo_punto

**Usando el paradigma de Aspectos, diseñe una solución que realice la regresión lineal.**  
> **NOTA:** Igual que en el punto anterior, aquí verás un **diseño detallado**, usando la idea de aspectos para separar la lógica de regresión lineal de las funcionalidades cruzadas (logging, verificación de datos, manejo de errores, etc).  
> Incluye: definición de los aspectos principales, mapeo de cortes (“join points”), ejemplos de cross-cutting para entrenamiento, y cómo los aspectos modificarían el flujo de ejecución y validación del modelo.

## Tercer_punto

**Usando RUST implemente la regresión lineal y haga una comparación de desempeño entre python y RUST.**  
> Aquí está el **código fuente en Rust** que replica el modelo trabajado en Python y se acompaña de un análisis comparativo del desempeño en tiempo y memoria. El diseño permite ver claramente diferencias de eficiencia y uso de recursos entre ambos lenguajes.

---

Cada carpeta tiene el contenido correspondiente según lo solicitado y los diseños cuentan con las explicaciones de arquitectura, división de tareas y detalle de cada parte, siguiendo las indicaciones del parcial.
