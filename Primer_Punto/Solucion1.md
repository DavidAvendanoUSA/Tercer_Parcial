# 1. Usando el paradigma de concurrencia y calculo PI, diseñe una solución que realice la regresión lineal. (NOTA: es el diseño no la implementación. Sea muy detallista con este diseño)

## Solución: 
### Pseudocódigo: 
 ```bash
(ν datos_1, datos_2, ..., datos_N, grad, pedir_param, resp_param, actualizar_param, metrica, iniciar_epoca, fin, barrera, ok)
(
  # Carga datos y reparte a los trabajadores
  ! CargarDatos = 
    iniciar_epoca().
      ( datos_1<Batch1>.datos_2<Batch2>...datos_N<BatchN>.0 ).
    CargarDatos

  |
  # Cada trabajador recibe datos, pide parámetros, calcula gradientes y manda resultados 
  ! Trabajador_i = 
    datos_i(lote).
      ( pedir_param<i>.resp_param(w,b,epoca).
        y_pred = w*lote.X + b
        err = y_pred - lote.y
        dw = promedio(err * lote.X)
        db = promedio(err)
        grad<{id=i, dw=dw, db=db, epoca=epoca}>.
        ok<{id=i}>.
        Trabajador_i
      )

  |
  # Servidor de parámetros: actualiza usando todos los gradientes (sincronía básica)
  ! ServidorParam = 
    grad(msg).
      ( # acumula gradientes
        if cuantos_gradientes(msg.epoca) < N then
          ServidorParam
        else
          dw_prom = promedio(grads.dw)
          db_prom = promedio(grads.db)
          w = w - lr*dw_prom
          b = b - lr*db_prom
          actualizar_param<{w=w, b=b, epoca=msg.epoca+1}>
          metrica<{epoca=msg.epoca, mse=calcular_mse(...)}>
          borrar_gradientes(msg.epoca)
          ServidorParam
      )

  |
  # Controlador: inicia, recibe métricas y decide cuando parar
  ! Controlador =
    iniciar_epoca().
    ( metrica(m). 
        if paro(m) then
          fin<{epoca=m.epoca}>
        else
          iniciar_epoca().Controlador
    )

  |
  # Servicio de parámetros: responde a las peticiones puntuales de los trabajadores
  ! ServicioParam = 
    pedir_param(i).resp_param<{w=w, b=b, epoca=actual_epoca}>.ServicioParam

)
```
### Explicación: 

#### **1. ¿Qué hace?**
Distribuye el cálculo de regresión lineal entre varios procesos (workers). Cada worker calcula gradientes, el servidor de parámetros los agrupa y actualiza los coeficientes para la recta de predicción. Todo coordinado por un controlador que maneja “epochs” (iteraciones).

---

#### Sintaxis clave π-cálculo (muy simple)

- **canal(x).** : recibir x por canal y seguir  
- **canal\<x\>.** : enviar x por canal y seguir  
- **! Proceso = ...** : definición de proceso replicable (loop)  
- **|** : procesos ejecutándose a la vez (concurrentes)  
- **(ν canales...)** : canales privados/locales  

---

#### Componentes

- **Controller:** inicia cada ciclo, manda a repartir datos, espera y decide cuándo terminar.
- **DataLoader:** toma la data (X, y), parte en N lotes, los reparte a cada worker.
- **Workers:** cada uno:
    1. Toma su batch (lote de datos)
    2. Pide los parámetros actuales (w, b)
    3. Calcula su gradiente (dw, db) usando su batch
    4. Envía el resultado al Parameter Server
    5. Espera la nueva versión de parámetros
- **Parameter Server (PS):**
    - Junta los gradientes.
    - Cuando tiene N, promedia y actualiza w, b.
    - Informa a todos los workers y al Controller.
- **Servicio parámetros:** responde (opcional) si le piden (pull, para sincronizar).

---

#### Comunicación

- `Batch = {X_chunk, y_chunk, batch_id}`
- `Gradiente = {worker_id, dw, db, batch_id}`
- `ParamUpdate = {w, b, epoch}`
- `Métrica = {epoch, mse}`

---

#### ¿Cómo corre?

1. Controller inicia un epoch.  
2. DataLoader reparte los batches.  
3. Cada worker toma su batch, calcula predicción, calcula error/dw/db, manda gradiente.  
4. PS espera todos los gradientes, actualiza w y b con promedio, informa resultado.  
5. Controller ve si ya está bien (converge) o sigue repitiendo pasos.