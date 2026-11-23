---

# Metodología de ejecución de tareas

## 1. Propósito

Este documento establece el procedimiento obligatorio que debe seguir OpenCode para ejecutar cualquier tarea de desarrollo del proyecto App de Finanzas.

El objetivo es mantener una metodología consistente que garantice:

- Comprensión del alcance antes de implementar.
    
- Respeto por los requisitos y decisiones arquitectónicas.
    
- Resultados esperados definidos y verificables.
    
- Implementaciones acotadas.
    
- Pruebas suficientes para demostrar el cumplimiento.
    
- Revisión de los cambios antes de integrarlos.
    
- Commits y push únicamente cuando se cumplan todos los requisitos.
    

Esta metodología se aplica a todas las tareas de desarrollo, independientemente de su fase o complejidad.

## 2. Principios obligatorios

1. Los requisitos y las decisiones arquitectónicas vigentes tienen prioridad sobre cualquier suposición.
    
2. Ninguna tarea comienza con la implementación sin un análisis previo.
    
3. Todo criterio de aceptación debe tener una forma concreta de verificación.
    
4. No se deben implementar funcionalidades pertenecientes a tareas posteriores.
    
5. No se deben modificar archivos ajenos al alcance sin justificación.
    
6. No se deben ocultar errores ni considerar exitosas verificaciones que no fueron ejecutadas.
    
7. No se deben introducir decisiones de negocio o arquitectura por iniciativa propia cuando requieran definición.
    
8. El código debe respetar las responsabilidades de cada capa.
    
9. La precisión de los cálculos financieros y las invariantes del dominio deben preservarse.
    
10. El commit y push forman parte del cierre normal de cada tarea, pero están condicionados al cumplimiento de todos los controles.
    

## 3. Procedimiento obligatorio

Cada tarea debe ejecutarse siguiendo las siguientes etapas, en orden.

- Aplicación personal de escritorio.
- Funcionamiento local.
- Alcance inicial intermedio, priorizando las funciones esenciales.
- Monedas de trabajo: pesos argentinos (ARS) y dólares estadounidenses (USD), sin conversión entre ellas.
- Tema claro y oscuro.
- Gestión de servicios mensuales.
- Gestión básica de inversiones en cauciones con datos ingresados manualmente.
- Botón para abrir una terminal externa en el directorio del proyecto y ejecutar OpenCode.
- Diseño preparado para poder ampliar el producto más adelante, sin implementar ahora conversión de monedas ni gestión histórica. D-024 admite ARS y USD desde v1; la conversión sigue fuera de alcance.
### Fase 1 — Análisis del alcance

Antes de modificar código:

1. Identificar la tarea solicitada en OpenSpec.
    
2. Leer su descripción, dependencias y criterios de aceptación.
    
3. Consultar los requisitos relacionados.
    
4. Revisar las decisiones arquitectónicas y técnicas vigentes.
    
5. Consultar la arquitectura y documentación pertinente.
    
6. Inspeccionar las implementaciones existentes relacionadas.
    
7. Revisar el estado de Git.
    
8. Identificar posibles contradicciones, riesgos y decisiones pendientes.
    

El análisis debe determinar:

- Qué problema resuelve la tarea.
    
- Qué comportamiento debe implementarse.
    
- Qué componentes existentes intervienen.
    
- Qué dependencias deben respetarse.
    
- Qué restricciones técnicas aplican.
    
- Qué elementos quedan fuera del alcance.
    

- Nombre del servicio.
- Costo total en ARS o USD, con selector explícito y valor predeterminado ARS.
- Moneda del servicio (ARS o USD).
- Cantidad de personas que comparten el gasto.
- Mi parte (calculada).
- Notas.
- Estado (activo o congelado).

Todos los servicios de la primera versión serán mensuales. No se incluye una columna de renovación anual; la renovación anual queda registrada como funcionalidad futura.
No se debe asumir que una tarea autoriza modificaciones generales sobre el proyecto.

### Fase 2 — Definición del resultado esperado

Antes de implementar, elaborar una especificación concreta del resultado esperado.

- La cantidad de personas incluye al usuario.
- La división es siempre en partes iguales.
- Fórmula: `Mi parte = Costo total / Cantidad de personas`, en la moneda del servicio.
- La cantidad de personas debe ser un entero mayor o igual a 1.
- `Mi parte` es un resultado calculado, no un campo editable.
- Para modificar el resultado se cambian el costo total o la cantidad de personas; luego se recalcula automáticamente.
Debe incluir:

**A. Resultado funcional**

Descripción de los comportamientos que deberán existir al finalizar.

**B. Resultado técnico**

Componentes, entidades, interfaces, servicios, repositorios o módulos que deberán crearse o modificarse.

**C. Criterios de aceptación**

Los importes se expresan en ARS o USD; cada servicio conserva su moneda y los totales se calculan separadamente por moneda.
Lista de condiciones verificables que deben cumplirse.

**D. Plan de pruebas**

Por cada criterio de aceptación, indicar qué prueba o verificación permitirá demostrar su cumplimiento.

**E. Archivos autorizados**

Lista explícita de archivos que pueden crearse o modificarse.

**F. Exclusiones**

Lista de funcionalidades, componentes o archivos que quedan fuera de la tarea.

- Mes.
- Moneda (ARS o USD, con selector explícito y valor predeterminado ARS).
- Capital invertido.
- TNA.
- Plazo en días.
- Impuestos.
- Rendimiento bruto (calculado).
- Gastos del mes.
- Rendimiento neto (calculado).
- Resultado de cobertura (calculado).
**G. Riesgos y dependencias**

Identificación de problemas potenciales y dependencias con tareas anteriores o decisiones existentes.

- Los campos de entrada comienzan en cero.
- El usuario ingresa manualmente el capital y los parámetros de la inversión, incluyendo TNA, plazo e impuestos.
- No se precargan tasas, plazos ni porcentajes impositivos como valores reales.
- La moneda se selecciona explícitamente; no se convierte automáticamente entre ARS y USD.
- Los resultados calculados no se editan directamente: se modifican los datos de origen.
No comenzar la implementación hasta que estos elementos sean coherentes con los requisitos del proyecto.

Si existen contradicciones, ambigüedades relevantes, decisiones pendientes o información insuficiente, detenerse y solicitar intervención.

La fórmula financiera definitiva, la convención de días y el tratamiento impositivo quedan fuera de esta etapa de definición. El usuario aprobó PV-01 a PV-03 para una implementación provisional, identificada como tal y reemplazable; esas fórmulas no sustituyen a la ruta definitiva, que sigue pendiente de las decisiones correspondientes.
No inventar decisiones para poder continuar.

Si el alcance está suficientemente definido, continuar autónomamente con la implementación.

### Fase 3 — Implementación

Comparar el rendimiento neto mostrado con los gastos del mes. La comparación requiere que ambos importes estén en la misma moneda; no se convierten importes:
- Si el rendimiento alcanza o supera los gastos: `¡Sí cubre!`.
- Si no alcanza: `Déficit de: $[importe faltante]`.

La cobertura de la primera versión se muestra con PV-03 mientras la ruta definitiva permanece pendiente. Debe quedar aislada para poder sustituirse por la fórmula definitiva.
Implementar exclusivamente el resultado esperado definido en la fase anterior.

Reglas:

- Respetar la arquitectura existente.
    
- Mantener la separación entre dominio, aplicación, infraestructura y presentación.
    
- Reutilizar abstracciones existentes cuando corresponda.
    
- Evitar duplicación de lógica.
    
- Evitar dependencias innecesarias.
    
- Respetar los contratos y tipos existentes.
    
- Gestionar errores explícitamente.
    
- Mantener las invariantes del dominio.
    
- No persistir valores derivados cuando la arquitectura indique que deben calcularse.
    
- No introducir funcionalidades de tareas posteriores.
    

- El gasto personal de servicios activos alimenta inicialmente el dato `Gastos del mes` de las inversiones.
- Los cambios en servicios activos actualizan los totales.
- Los resultados de cobertura se actualizan cuando cambian los gastos o los datos de inversión.
- En esta primera versión, `Gastos del mes` considera los servicios administrados por el módulo Servicios. La incorporación de otras categorías de gastos podrá evaluarse posteriormente.
- La inversión y los gastos comparados deben compartir moneda; no se convierten importes.
Si durante la implementación aparece una necesidad que modifica sustancialmente el alcance, detenerse y solicitar autorización.

### Fase 4 — Pruebas y validación técnica

Ejecutar todas las pruebas necesarias para demostrar los criterios de aceptación.

Las verificaciones generales del proyecto incluyen:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
openspec validate implement-finanzas-v1 --strict
git diff --check
```

Ejecutarlas desde el directorio correspondiente.

Además:

- Crear pruebas unitarias para las reglas del dominio.
    
- Crear pruebas de integración para las operaciones entre componentes.
    
- Probar casos límite y entradas inválidas.
    
- Comprobar el tratamiento de errores.
    
- Verificar que no existan regresiones.
    
- Comprobar que no se introduzcan operaciones aritméticas inseguras.
    
- Revisar las dependencias agregadas.
    
- Verificar que la implementación no contradiga decisiones vigentes.
    

Las pruebas generales deben complementarse con verificaciones específicas de la tarea.

No considerar suficiente que el proyecto compile: también deben cumplirse los comportamientos esperados.

#### Corrección de errores

- Conversión entre monedas y cotizaciones.
- Historial de precios, tasas, inversiones y resultados por períodos.
- Fórmula financiera definitiva de cauciones y parametrización fiscal completa.
- Sincronización en la nube.
- Conexión automática con bancos o plataformas financieras.
- Importación automática de movimientos bancarios.
- Editor de fórmulas de estilo Excel.
- División desigual de gastos compartidos.
- Chat de IA embebido.
- Integración avanzada para que OpenCode modifique datos financieros mediante una API local.
- Aplicación móvil terminada.
Se permite corregir errores propios de la implementación cuando:

- Están dentro del alcance autorizado.
    
- No modifican decisiones arquitectónicas.
    
- No alteran requisitos.
    
- No requieren ampliar el alcance.
    

Aunque no se implementan ahora, el diseño debe evitar bloquear:
1. Conversión entre ARS y USD, cotizaciones y visualización de monedas adicionales.
2. Renovación anual y prorrateo de servicios anuales.
3. Historial de cambios para que actualizar un precio o parámetro no altere la información de períodos anteriores.
4. Fórmula definitiva de rendimiento, impuestos y convención de días para cauciones.
5. Operaciones controladas para que OpenCode consulte y modifique datos de la aplicación.
6. Incorporación de otros tipos de gastos.
Después de cada corrección deben repetirse las verificaciones afectadas.

Si el problema requiere cambiar una decisión existente, detenerse.

- Fórmula de rendimiento de cauciones, base anual y convención de días.
- Regla de cálculo y aplicación de impuestos.
- Precisión y redondeo de los resultados (D-028, pendiente): en fase 3 se conservan fracciones exactas en dominio, no se persisten resultados derivados redondeados y la presentación puede mostrar dos decimales solo como formato. Falta decidir cómo representar resultados fraccionarios y cómo resolver diferencias entre importes individuales mostrados y totales.
- Política de confirmación y registro de modificaciones solicitadas por OpenCode.
- Estrategia de copias de seguridad y recuperación de SQLite.
- Alcance de una futura versión móvil.
### Fase 5 — Auditoría de aceptación

Comparar la implementación final con la especificación definida en la fase 2.

Utilizar una tabla:

|Criterio|Evidencia|Resultado|
|---|---|---|
|Criterio 1|Prueba o verificación|OK / FALLIDO|
|Criterio 2|Prueba o verificación|OK / FALLIDO|

Todos los criterios obligatorios deben estar en estado OK.

También verificar:

- Que solo se hayan modificado archivos autorizados.
    
- Que no existan cambios ajenos a la tarea.
    
- Que el diff completo haya sido inspeccionado.
    
- Que no existan archivos temporales o accidentales.
    
- Que las pruebas se hayan ejecutado después de las últimas modificaciones.
    

Si algún criterio falla, no avanzar al commit.

### Fase 6 — Actualización de OpenSpec

Una vez superados todos los criterios de aceptación:

1. Actualizar el checkbox de la tarea correspondiente en `tasks.md`.
    
2. No marcar tareas posteriores.
    
3. No modificar el estado de otras tareas.
    
4. Revisar que la actualización documental corresponda al resultado realmente implementado.
    

La actualización de `tasks.md` debe incluirse en el mismo commit de la tarea.

Después de modificarlo, comprobar nuevamente el diff y las verificaciones documentales correspondientes.

### Fase 7 — Commit y push

El cierre mediante Git está autorizado cuando todas las fases anteriores finalizaron correctamente.

Antes del commit:

1. Ejecutar `git status --short`.
    
2. Identificar todos los archivos modificados.
    
3. Comprobar que no haya cambios preexistentes ajenos a la tarea.
    
4. Preparar el staging exclusivamente con los archivos correspondientes.
    
5. Ejecutar `git diff --cached --check`.
    
6. Revisar `git diff --cached --name-only`.
    
7. Revisar `git diff --cached --stat`.
    
8. Inspeccionar el diff staged completo.
    

El commit debe utilizar Conventional Commits, con un mensaje descriptivo del cambio.

Ejemplos:

```
feat: add SQLite service repository
fix: handle invalid service currency
test: cover service repository persistence
refactor: simplify service domain validation
```

Después:

1. Crear el commit.
    
2. Realizar push hacia el upstream configurado de la rama actual.
    
3. No utilizar `--force`.
    
4. No modificar el historial existente.
    
5. No realizar commits vacíos.
    

Si el commit o push falla, informar el problema y detenerse. No realizar operaciones destructivas para intentar resolverlo.

### Fase 8 — Verificación del cierre

Después del commit y push:

Ejecutar:

```
git log -1 --oneline
git status --short
git status -sb
```

Confirmar:

- Hash completo del commit.
    
- Mensaje del commit.
    
- Push exitoso.
    
- Rama actual correcta.
    
- Upstream configurado correctamente.
    
- Sincronización local/remota.
    
- Working tree limpio.
    
- Tarea marcada como completada.
    

La tarea solo se considera cerrada cuando estas condiciones se verifican.

## 4. Condiciones de detención obligatoria

OpenCode debe detener la ejecución y solicitar intervención cuando:

- Existan contradicciones entre requisitos y decisiones.
    
- Falte una definición de negocio necesaria.
    
- Se requiera modificar una decisión arquitectónica.
    
- Sea necesario ampliar el alcance autorizado.
    
- Existan cambios previos del usuario que no puedan aislarse con seguridad.
    
- Una prueba revele un problema ajeno a la tarea.
    
- No pueda demostrarse un criterio de aceptación.
    
- Exista un conflicto de Git que requiera decisiones del usuario.
    
- El push falle y no exista una resolución segura.
    

Detenerse significa informar el problema, explicar qué decisión se necesita y no continuar con acciones que dependan de ella.

## 5. Restricciones generales

En cada ejecución:

- Trabajar exclusivamente sobre la tarea solicitada.
    
- No avanzar automáticamente a la siguiente tarea.
    
- No descartar cambios del usuario.
    
- No sobrescribir trabajo preexistente.
    
- No introducir cambios no justificados.
    
- No alterar requisitos para hacer pasar pruebas.
    
- No reducir la cobertura de pruebas para ocultar fallos.
    
- No marcar tareas como completadas antes de superar la auditoría.
    
- No realizar operaciones destructivas de Git.
    

## 6. Informe obligatorio

Al finalizar cada tarea, presentar un informe con esta estructura:

### 1. Análisis

- Objetivo.
    
- Dependencias.
    
- Decisiones aplicadas.
    
- Alcance autorizado.
    

### 2. Resultado esperado

- Criterios de aceptación.
    
- Pruebas asociadas.
    

### 3. Implementación

- Descripción de los cambios.
    
- Componentes creados o modificados.
    

### 4. Archivos

- Archivos creados.
    
- Archivos modificados.
    
- Justificación de cualquier desviación.
    

### 5. Verificaciones

- Comandos ejecutados.
    
- Resultados.
    
- Cantidad de pruebas aprobadas, fallidas e ignoradas.
    

### 6. Auditoría

- Estado de cada criterio de aceptación.
    
- Resultado de la revisión del diff.
    

### 7. Git

- Commit.
    
- Hash completo.
    
- Push.
    
- Rama y upstream.
    
- Estado final del repositorio.
    

### 8. Resultado final

Indicar uno de estos estados:

- **COMPLETADA:** todos los controles fueron superados y el commit y push fueron verificados.
    
- **BLOQUEADA:** requiere intervención o una decisión pendiente.
    
- **FALLIDA:** no se pudo cumplir uno o más criterios obligatorios.
    

## 7. Regla final

La autonomía de OpenCode está limitada por los requisitos, las decisiones vigentes y el alcance de cada tarea.

El objetivo es ejecutar cada tarea de principio a fin sin necesitar aprobación manual de cada comando, pero nunca sacrificar la integridad del proyecto para conseguir una ejecución automática.

**Primero definir qué significa terminar correctamente. Después implementar. Finalmente demostrar que se cumplió antes de integrar los cambios.**
