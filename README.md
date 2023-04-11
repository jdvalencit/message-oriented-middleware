# Diseño e Implementación de un Middleware que Implemente un Servicio de Mensajería Asincrónica entre Aplicaciones
#
### info de la materia: ST0263
### Estudiante(s): 
-  Juan David Valencia Torres, jdvalencit@eafit.edu.co; 
-  Tomas Atehortua Ceferino, tatehortuc@eafit.edu.co; 
-  Daniel Arango Hoyos, darangoh@eafit.edu.co
### Profesor: Edwin Nelson Montoya Munera, emontoya@eafit.edu.co
#
# 1. breve descripción de la actividad

El objetivo de este proyecto es el desarrollo de un Message-Oriented Middleware (MOM) utilizando el lenguaje de programación Rust, el cual sirve como middleware para el intercambio de mensajes de manera asíncrona entre un conjunto de clientes o servidores. Esto con el objetivo de experimentar con características clave de los sistemas distribuidos (como heterogeneidad, transparencia, seguridad y escalabilidad) que son necesarias para las aplicaciones y los subsistemas base.

Además, se creó un cliente que permite a los usuarios del MOM utilizar las funcionalidades con un buen nivel de abstracción. Este cliente realiza peticiones a una API desarrollada, la cual se comunica con el Message-Oriented Middleware a través de comunicación por gRPC, lo que permite una comunicación rápida y eficiente entre el cliente, la API y el MOM. Esto también facilita la integración de otros servicios en el servidor.

## 1.1. Que aspectos cumplió o desarrolló de la actividad propuesta por el profesor (requerimientos funcionales y no funcionales)

- Conexión y desconexión al servidor: El MOM fue diseñado e implementado para permitir que los clientes se conecten y desconecten al servidor. Esto se puede hacer tanto en modo con estado como sin estado, lo que significa que el MOM puede mantener la conexión de manera persistente o no.

- Ciclo de vida de colas: El MOM permite la creación, eliminación y listado de colas. Las colas permiten que los clientes envíen y reciban mensajes en orden.

- Envío de un mensaje a una cola: El MOM permite que los clientes envíen mensajes a una cola en particular. Los mensajes se agregan a la cola en el orden en que se reciben.

- Recepción de un mensaje de una cola: El MOM permite que los clientes reciban mensajes de una cola en particular. Los mensajes se entregan a los clientes en el orden en que se agregaron a la cola.

- La conexión y desconexión al servidor debe ser con usuarios autenticados. El MOM requiere que los clientes se autentiquen antes de establecer una conexión y enviar o recibir mensajes. Esto garantiza la seguridad y la privacidad de los datos transmitidos.

- Solo se puede borrar colas creados por el usuario que los creó. El MOM implementa un control de acceso que asegura que solo los usuarios autorizados puedan borrar canales o cola

- Todos los servicios son expuestos como un API REST hacia los clientes. El MOM implementa una API RESTful que los clientes pueden utilizar para interactuar con los servicios del MOM.

- El mecanismo de recepción de mensajes en modo pull está definido. El MOM implementa una funcionalidad de recepción de mensajes en modo pull, que permite a los clientes obtener mensajes de las colas en función de sus necesidades.

- Se aplica un concepto de particionamiento. El MOM utiliza un particionamiento de tipo horizontal para distribuir las colas en diferentes servidores, lo que permite mejorar la escalabilidad y la disponibilidad del sistema.


## 1.2. Que aspectos NO cumplió o desarrolló de la actividad propuesta por el profesor (requerimientos funcionales y no funcionales)

- Implementación de tópicos.

- Tolerancia a fallos.

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
