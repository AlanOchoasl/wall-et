## Purpose

Permite abrir una terminal externa en el directorio del proyecto para que el usuario ejecute OpenCode manualmente, sin chat integrado y sin acceso de la herramienta a los datos financieros.

## ADDED Requirements

### Requirement: Abrir una terminal externa en el directorio del proyecto

El sistema SHALL ofrecer un control visible que abra una terminal del sistema operativo en el directorio raíz del proyecto. La terminal MUST abrirse como proceso externo independiente de la aplicación y MUST iniciar en ese directorio sin requerir que el usuario lo escriba.

#### Scenario: Apertura exitosa

- **WHEN** el usuario activa el control de apertura de terminal
- **THEN** se abre una terminal externa cuyo directorio de trabajo es el directorio raíz del proyecto

#### Scenario: Terminal abierta junto a la aplicación

- **WHEN** la terminal externa está abierta
- **THEN** la aplicación sigue funcionando y la terminal opera como proceso independiente

#### Scenario: Directorio del proyecto no disponible

- **WHEN** el directorio configurado no existe o no es accesible
- **THEN** el sistema no abre la terminal e informa el problema

### Requirement: Ejecutar OpenCode de forma manual

El sistema SHALL limitarse a abrir la terminal y MUST NOT ejecutar `opencode` automáticamente ni enviar comandos a la terminal. El usuario MUST escribir y ejecutar el comando por su cuenta.

#### Scenario: Apertura sin ejecución automática

- **WHEN** el usuario abre la terminal desde la aplicación
- **THEN** la terminal queda disponible sin que la aplicación ejecute ningún comando

#### Scenario: OpenCode no instalado

- **WHEN** el usuario abre la terminal y `opencode` no está instalado en el sistema
- **THEN** la aplicación no falla y el resultado depende del intento manual del usuario en la terminal

### Requirement: Informar errores al abrir la terminal

El sistema SHALL informar de forma visible cuando no pueda abrir la terminal externa. El error MUST NOT afectar los datos financieros ni impedir el uso del resto de la aplicación.

#### Scenario: Fallo al iniciar el proceso

- **WHEN** el sistema operativo rechaza la apertura de la terminal
- **THEN** el sistema muestra un mensaje de error y la aplicación continúa operativa

#### Scenario: Error sin cambios en los datos

- **WHEN** ocurre un error al abrir la terminal
- **THEN** ningún dato de servicios, inversiones o preferencias se modifica

### Requirement: No incluir chat de IA ni acceso a los datos financieros

El sistema MUST NOT incluir un chat de IA integrado. La aplicación MUST NOT exponer a OpenCode ni a ningún proceso externo acceso de lectura o escritura a los datos financieros, ni permitir escritura directa sobre la base SQLite.

#### Scenario: Sin chat integrado

- **WHEN** el usuario abre la sección de OpenCode o la terminal
- **THEN** no se presenta ninguna interfaz de conversación con un modelo de IA

#### Scenario: Sin acceso a datos

- **WHEN** la terminal externa está abierta
- **THEN** la aplicación no le transfiere datos financieros ni credenciales

#### Scenario: Sin escritura directa en la base

- **WHEN** se abre la terminal desde la aplicación
- **THEN** la aplicación no habilita ni documenta un mecanismo de escritura directa sobre la base SQLite

### Requirement: Apertura segura del proceso externo

El sistema SHALL iniciar la terminal con un comando y un directorio definidos por la aplicación, sin interpolar texto ingresado por el usuario en el comando. La ruta del directorio MUST validarse antes de usarse como directorio de trabajo.

#### Scenario: Sin entrada de usuario en el comando

- **WHEN** el usuario activa el control de apertura de terminal
- **THEN** el comando ejecutado no contiene texto ingresado por el usuario

#### Scenario: Ruta validada

- **WHEN** la aplicación va a abrir la terminal
- **THEN** verifica que la ruta de trabajo corresponda a un directorio existente antes de iniciar el proceso
