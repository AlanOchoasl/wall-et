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
    

No se debe asumir que una tarea autoriza modificaciones generales sobre el proyecto.

### Fase 2 — Definición del resultado esperado

Antes de implementar, elaborar una especificación concreta del resultado esperado.

Debe incluir:

**A. Resultado funcional**

Descripción de los comportamientos que deberán existir al finalizar.

**B. Resultado técnico**

Componentes, entidades, interfaces, servicios, repositorios o módulos que deberán crearse o modificarse.

**C. Criterios de aceptación**

Lista de condiciones verificables que deben cumplirse.

**D. Plan de pruebas**

Por cada criterio de aceptación, indicar qué prueba o verificación permitirá demostrar su cumplimiento.

**E. Archivos autorizados**

Lista explícita de archivos que pueden crearse o modificarse.

**F. Exclusiones**

Lista de funcionalidades, componentes o archivos que quedan fuera de la tarea.

**G. Riesgos y dependencias**

Identificación de problemas potenciales y dependencias con tareas anteriores o decisiones existentes.

No comenzar la implementación hasta que estos elementos sean coherentes con los requisitos del proyecto.

Si existen contradicciones, ambigüedades relevantes, decisiones pendientes o información insuficiente, detenerse y solicitar intervención.

No inventar decisiones para poder continuar.

Si el alcance está suficientemente definido, continuar autónomamente con la implementación.

### Fase 3 — Implementación

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

Se permite corregir errores propios de la implementación cuando:

- Están dentro del alcance autorizado.
    
- No modifican decisiones arquitectónicas.
    
- No alteran requisitos.
    
- No requieren ampliar el alcance.
    

Después de cada corrección deben repetirse las verificaciones afectadas.

Si el problema requiere cambiar una decisión existente, detenerse.

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