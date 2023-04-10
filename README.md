# info de la materia: ST0263
#
# Estudiante(s): 
-  Juan David Valencia Torres, jdvalencit@eafit.edu.co; 
-  Tomas Atehortua Ceferino, tatehortuc@eafit.edu.co; 
-  Daniel Arango Hoyos, darangoh@eafit.edu.co
#
# Profesor: Edwin Nelson Montoya Munera, emontoya@eafit.edu.co
#
# Diseño e Implementación de un Middleware que Implemente un Servicio de Mensajería Asincrónica entre Aplicaciones
#
# 1. breve descripción de la actividad
#
El objetivo de este proyecto es crear un middleware orientado a mensajes (MOM) para permitir que los clientes envíen y reciban mensajes. Esto con el objetivo de experimentar con características clave de los sistemas distribuidos (como heterogeneidad, transparencia, seguridad y escalabilidad) que son necesarias para las aplicaciones y los subsistemas base. El MOM se diseñará e implementará para manejar la complejidad y características del sistema distribuido, de modo que los clientes puedan usarlo de forma segura y transparente.
## 1.1. Que aspectos cumplió o desarrolló de la actividad propuesta por el profesor (requerimientos funcionales y no funcionales)

- Conexión y desconexión al servidor: El MOM fue diseñado e implementado para permitir que los clientes se conecten y desconecten al servidor. Esto se puede hacer tanto en modo con estado como sin estado, lo que significa que el MOM puede mantener la conexión de manera persistente o no.

- Ciclo de vida de tópicos: El MOM permite la creación, eliminación y listado de tópicos. Los tópicos son canales con nombres únicos que los clientes pueden utilizar para enviar y recibir mensajes relacionados con un tema en particular.

- Ciclo de vida de colas: El MOM permite la creación, eliminación y listado de colas. Las colas son estructuras de datos que permiten que los clientes envíen y reciban mensajes en orden.

- Envío de un mensaje a un tópico: El MOM permite que los clientes envíen mensajes a un tópico en particular. Cuando un cliente envía un mensaje a un tópico, todos los demás clientes suscritos al mismo tópico pueden recibir el mensaje.

- Envío de un mensaje a una cola: El MOM permite que los clientes envíen mensajes a una cola en particular. Los mensajes se agregan a la cola en el orden en que se reciben.

- Recepción de un mensaje de un tópico: El MOM permite que los clientes reciban mensajes de un tópico en particular. Cuando un mensaje se envía a un tópico, todos los clientes suscritos al mismo tópico pueden recibir el mensaje.

- Recepción de un mensaje de una cola: El MOM permite que los clientes reciban mensajes de una cola en particular. Los mensajes se entregan a los clientes en el orden en que se agregaron a la cola.


## 1.2. Que aspectos NO cumplió o desarrolló de la actividad propuesta por el profesor (requerimientos funcionales y no funcionales)

# 2. información general de diseño de alto nivel, arquitectura, patrones, mejores prácticas utilizadas.


# 3. Descripción del ambiente de desarrollo y técnico: lenguaje de programación, librerias, paquetes, etc, con sus numeros de versiones.

## como se compila y ejecuta.

## detalles del desarrollo.

## detalles técnicos

## descripción y como se configura los parámetros del proyecto (ej: ip, puertos, conexión a bases de datos, variables de ambiente, parámetros, etc)

## opcional - detalles de la organización del código por carpetas o descripción de algún archivo. (ESTRUCTURA DE DIRECTORIOS Y ARCHIVOS IMPORTANTE DEL PROYECTO, comando 'tree' de linux)


## 
## opcionalmente - si quiere mostrar resultados o pantallazos 

# 4. Descripción del ambiente de EJECUCIÓN (en producción) lenguaje de programación, librerias, paquetes, etc, con sus numeros de versiones.


# IP o nombres de dominio en nube o en la máquina servidor.


## una mini guia de como un usuario utilizaría el software o la aplicación


# 5. otra información que considere relevante para esta actividad.

# referencias:

#### versión README.md -> 1.0 (2022-agosto)
