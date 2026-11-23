# Requisitos — App de Finanzas Personales

**Versión:** 0.2
**Estado:** Base de la primera versión aprobada
**Tipo:** Aplicación personal de escritorio, local y extensible.

## 1. Objetivo

Desarrollar una aplicación de escritorio para administrar gastos recurrentes y visualizar una comparación entre los gastos personales y el rendimiento de inversiones en cauciones.

La primera versión debe proporcionar una base funcional y clara sobre la cual incorporar mejoras gradualmente. No busca resolver desde el inicio todos los escenarios financieros.

## 2. Alcance de la primera versión

- Aplicación personal de escritorio.
- Funcionamiento local.
- Alcance inicial intermedio, priorizando las funciones esenciales.
- Monedas de trabajo: pesos argentinos (ARS) y dólares estadounidenses (USD), sin conversión entre ellas.
- Tema claro y oscuro.
- Gestión de servicios mensuales.
- Gestión básica de inversiones en cauciones con datos ingresados manualmente.
- Botón para abrir una terminal externa en el directorio del proyecto y ejecutar OpenCode.
- Diseño preparado para poder ampliar el producto más adelante, sin implementar ahora conversión de monedas ni gestión histórica. D-024 admite ARS y USD desde v1; la conversión sigue fuera de alcance.

## 3. Requisitos funcionales

### RF-01 — Gestión de servicios

La aplicación permitirá administrar gastos recurrentes mediante una tabla.

#### Datos de un servicio

- Nombre del servicio.
- Costo total en ARS o USD, con selector explícito y valor predeterminado ARS.
- Moneda del servicio (ARS o USD).
- Cantidad de personas que comparten el gasto.
- Mi parte (calculada).
- Notas.
- Estado (activo o congelado).

Todos los servicios de la primera versión serán mensuales. No se incluye una columna de renovación anual; la renovación anual queda registrada como funcionalidad futura.

#### Reglas de cálculo

- La cantidad de personas incluye al usuario.
- La división es siempre en partes iguales.
- Fórmula: `Mi parte = Costo total / Cantidad de personas`, en la moneda del servicio.
- La cantidad de personas debe ser un entero mayor o igual a 1.
- `Mi parte` es un resultado calculado, no un campo editable.
- Para modificar el resultado se cambian el costo total o la cantidad de personas; luego se recalcula automáticamente.

#### Operaciones

- Agregar un servicio.
- Editar sus datos de origen y notas.
- Eliminar un servicio.
- Congelar un servicio sin eliminar sus datos.
- Consultar servicios congelados.
- Reactivar un servicio conservando sus datos anteriores.

Un servicio congelado:
- deja de mostrarse en la tabla de servicios activos;
- deja de participar en los totales;
- conserva sus datos;
- permanece congelado indefinidamente hasta que el usuario decida reactivarlo.

#### Totales

Mostrar por separado:
1. **Costo total de servicios:** suma de los costos completos de los servicios activos.
2. **Mi gasto:** suma de las partes correspondientes al usuario en los servicios activos.

Los importes se expresan en ARS o USD; cada servicio conserva su moneda y los totales se calculan separadamente por moneda.

### RF-02 — Gestión de inversiones

La aplicación permitirá cargar inversiones en cauciones y visualizar un resultado comparativo con los gastos.

Cada fila representa una inversión asociada a un mes.

#### Datos de la tabla

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

#### Ingreso de datos

- Los campos de entrada comienzan en cero.
- El usuario ingresa manualmente el capital y los parámetros de la inversión, incluyendo TNA, plazo e impuestos.
- No se precargan tasas, plazos ni porcentajes impositivos como valores reales.
- La moneda se selecciona explícitamente; no se convierte automáticamente entre ARS y USD.
- Los resultados calculados no se editan directamente: se modifican los datos de origen.

#### Cálculo provisional

La fórmula financiera definitiva, la convención de días y el tratamiento impositivo quedan fuera de esta etapa de definición. El usuario aprobó PV-01 a PV-03 para una implementación provisional, identificada como tal y reemplazable; esas fórmulas no sustituyen a la ruta definitiva, que sigue pendiente de las decisiones correspondientes.

Los resultados provisionales sirven para comprobar la interfaz y el flujo de datos; no deben presentarse como cálculos financieros exactos.

#### Cobertura

Comparar el rendimiento neto mostrado con los gastos del mes. La comparación requiere que ambos importes estén en la misma moneda; no se convierten importes:
- Si el rendimiento alcanza o supera los gastos: `¡Sí cubre!`.
- Si no alcanza: `Déficit de: $[importe faltante]`.

La cobertura de la primera versión se muestra con PV-03 mientras la ruta definitiva permanece pendiente. Debe quedar aislada para poder sustituirse por la fórmula definitiva.

### RF-03 — Relación entre servicios e inversiones

- El gasto personal de servicios activos alimenta inicialmente el dato `Gastos del mes` de las inversiones.
- Los cambios en servicios activos actualizan los totales.
- Los resultados de cobertura se actualizan cuando cambian los gastos o los datos de inversión.
- En esta primera versión, `Gastos del mes` considera los servicios administrados por el módulo Servicios. La incorporación de otras categorías de gastos podrá evaluarse posteriormente.
- La inversión y los gastos comparados deben compartir moneda; no se convierten importes.

### RF-04 — Apertura de OpenCode

La aplicación incluirá un botón para abrir una terminal externa ubicada en el directorio del proyecto.

Flujo esperado:
1. El usuario presiona el botón.
2. Se abre la terminal del sistema en la carpeta del proyecto.
3. El usuario ejecuta manualmente `opencode`.
4. El usuario interactúa con OpenCode desde esa terminal.

Este botón no representa una integración de chat dentro de la aplicación ni concede por sí mismo acceso estructurado a los datos financieros de la aplicación.

La posibilidad de que OpenCode consulte o modifique servicios e inversiones mediante operaciones controladas queda reservada para una etapa posterior.

### RF-05 — Apariencia

- Ofrecer tema claro y tema oscuro.
- Mantener legibilidad de tablas, formularios, totales e indicadores en ambos temas.

## 4. Requisitos no funcionales

- **Localidad:** las funciones principales deben operar localmente, sin depender de conexión permanente a Internet.
- **Persistencia:** conservar los datos entre ejecuciones mediante SQLite.
- **Integridad:** centralizar reglas y cálculos.
- **Mantenibilidad:** separar interfaz, aplicación, dominio y persistencia.
- **Usabilidad:** tablas legibles, edición sencilla y navegación clara.
- **Seguridad:** validar entradas y operaciones locales.
- **Extensibilidad:** permitir incorporar más adelante historial, conversión de monedas e integración avanzada sin imponer complejidad innecesaria a la primera versión.

## 5. Fuera del alcance de la primera versión

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

## 6. Extensiones futuras contempladas

Aunque no se implementan ahora, el diseño debe evitar bloquear:
1. Conversión entre ARS y USD, cotizaciones y visualización de monedas adicionales.
2. Renovación anual y prorrateo de servicios anuales.
3. Historial de cambios para que actualizar un precio o parámetro no altere la información de períodos anteriores.
4. Fórmula definitiva de rendimiento, impuestos y convención de días para cauciones.
5. Operaciones controladas para que OpenCode consulte y modifique datos de la aplicación.
6. Incorporación de otros tipos de gastos.

## 7. Cuestiones para definir durante futuras iteraciones

- Fórmula de rendimiento de cauciones, base anual y convención de días.
- Regla de cálculo y aplicación de impuestos.
- Precisión y redondeo de los resultados: definido en D-028.
- Política de confirmación y registro de modificaciones solicitadas por OpenCode.
- Estrategia de copias de seguridad y recuperación de SQLite.
- Alcance de una futura versión móvil.
