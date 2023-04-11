# Diseño e Implementación de un Middleware que Implemente un Servicio de Mensajería Asincrónica entre Aplicaciones
#
### info de la materia: ST0263
### Estudiante(s): 
-  Juan David Valencia Torres, jdvalencit@eafit.edu.co; 
-  Tomas Atehortua Ceferino, tatehortuc@eafit.edu.co; 
-  Daniel Arango Hoyos, darangoh@eafit.edu.co
### Profesor: 
-  Edwin Nelson Montoya Munera, emontoya@eafit.edu.co
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

![WhatsApp Image 2023-04-10 at 7 56 42 PM](https://user-images.githubusercontent.com/61467004/231029144-a80a0272-1dd8-431d-83a6-3d2260cc7b97.jpeg)
 
La estructura del proyecto se divide en tres partes principales: cliente, API y la capa del MOM que contiene la base de datos. El objetivo del cliente y la API es actuar como intermediarios para proporcionar una capa de abstracción y permitir la escalabilidad del sistema en el futuro. La comunicación entre el cliente y la API se realiza mediante REST, mientras que la comunicación entre la API y el MOM se establece a través de gRPC. Esta arquitectura permite una comunicación eficiente y un intercambio de datos rápido y seguro entre las diferentes partes del sistema.

La capa del MOM se desarrolló teniendo en cuenta el concepto de particionamiento, esto con el fin de permitir una mejor disponibilidad y escalabilidad del sistema.

# 3. Descripción del ambiente de desarrollo y técnico: lenguaje de programación, librerias, paquetes, etc, con sus numeros de versiones.

## como se compila y ejecuta.
Para compilar y ejecutar el proyecto primero es necesario instalar lo siguiente:
```
sudo apt-get update
sudo apt install build-essential
sudo apt install docker.io
sudo apt install docker-compose
sudo apt-get install protobuf-compiler
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install sqlx-cli --no-default-features --features rustls,postgres
```
Luego, diríjase a la carpeta api_gateway y ejecute lo siguiente:
```
cargo build
cargo run
```
Con esto descargará las dependencias necesarias para ejecutar la api gateway y también la pondrá en ejecución.
Luego diríjase a la carpeta docker para poder correr la base de datos con el siguiente comando:
```
sudo ./service.sh up
```
Con todo esto corriendo, diríjase hacia la carpeta grpc_server y escriba el siguiente comando:
```
sqlx migrate run
```
Ya con esto tendrá el proyecto funcionando.

## detalles del desarrollo.
El desarrollo del proyecto se llevó a cabo de manera ágil, en donde cada integrante se especializó en una capa del proyecto. A pesar de esta especialización, todos los miembros participaron activamente en el desarrollo de cada una de las capas y contribuyeron al excelente resultado obtenido al final. La división de trabajo permitió una mejor organización y eficiencia en el desarrollo del proyecto.

## detalles técnicos
## descripción y como se configura los parámetros del proyecto (ej: ip, puertos, conexión a bases de datos, variables de ambiente, parámetros, etc)
Para configurar los valores con los que se va a ejecutar el proyecto se utiliza un archivo de configuración .env para cada capa en la cual se especifican las direcciones o puertos dependiendo de en cuál capa se encuentre el lector. Por ejemplo, el grpc_server se utiliza para comunicar el MOM con la base de datos contiene el siguiente archivo .env:
```
DB_USER=postgres
DB_NAME=momdb
DB_HOST="127.0.0.1"
DB_PORT=5432
DB_PASS=secret

DATABASE_URL="postgresql://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
```
Con el cuál se puede configurar la IP y el puerto que conecta con la base de datos.

## detalles de la organización del código por carpetas o descripción de algún archivo.
```
└── message-oriented-middleware
    ├── api_gateway
    │   ├── build.rs
    │   ├── Cargo.toml
    │   ├── Rocket.toml
    │   └── src
    │       ├── grpc_client
    │       │   ├── grpc_client.rs
    │       │   └── mod.rs
    │       ├── lib.rs
    │       └── main.rs
    ├── docker
    │   ├── bash.sh
    │   ├── docker-compose.yml
    │   └── service.sh
    ├── grpc_server
    │   ├── build.rs
    │   ├── build-sql.sh
    │   ├── Cargo.toml
    │   ├── migrations
    │   │   └── 20230410030209_user_table.sql
    │   └── src
    │       ├── crud
    │       │   ├── connection.rs
    │       │   ├── mod.rs
    │       │   └── user.rs
    │       ├── lib.rs
    │       ├── main.rs
    │       └── queue.rs
    ├── protos
    │   └── protomom.proto
    └── README.md

```


## 
## opcionalmente - si quiere mostrar resultados o pantallazos 

# 4. Descripción del ambiente de EJECUCIÓN (en producción) lenguaje de programación, librerias, paquetes, etc, con sus numeros de versiones.
La aplicación se ejecuta en un ambiente de producción con el lenguaje de programación Rustc versión 1.68.2. Las librerías necesarias para el correcto funcionamiento de la aplicación se encuentran especificadas en el archivo "Cargo.toml" y se instalan mediante el manejador de paquetes de Rust, Cargo.

Además, la aplicación utiliza el framework web Rocket en su versión 0.5.0-rc.2, que proporciona una solución elegante y eficiente para el manejo de solicitudes y respuestas HTTP, así como la gestión de las rutas y la serialización de los datos. Otras librerías y paquetes que se utilizan en la aplicación incluyen grpc, que se utiliza para la comunicación entre los diferentes componentes de la aplicación, también se utiliza PostgreSQL como base de datos y varias bibliotecas de manejo de bases de datos para la persistencia de los datos. Todas las versiones específicas de estas librerías y paquetes están especificadas en el archivo "Cargo.toml".

# IP o nombres de dominio en nube o en la máquina servidor.


## una mini guia de como un usuario utilizaría el software o la aplicación.
El uso del sistema se puede comprobar mediante *métodos HTTP* para realizarle peticiones a la base de datos a través de un *Browser* o también se puede a través de una consola mediante la herramienta *curl*.
La siguiente guía será para la herramienta *curl*
Para poder realizar el registro del usuario, este deberá hacerlo mediante la petición:
```curl -X POST --user user:password 127.0.0.1:8000/register-user```

Una vez registrado, el usuario podrá crear colas:
```curl -X POST --user user:password 127.0.0.1:8000/crud/create-queue/<queuename>```

añadir contenido a la cola:
```curl -X PUT 127.0.0.1:8000/queue/put/<queuename> -H "Content-Type: text/html" -d "This is a new content"```

Consultar el contenido de la cola:
```curl 127.0.0.1:8000/crud/read-queue/<queuename>```

Y eliminar la cola:
```curl -X DELETE --user user:password 127.0.0.1:8000/crud/delete-queue/<queuename>```

# 5. otra información que considere relevante para esta actividad.

# referencias:

- https://www.rust-lang.org/es
- https://www.rincondelvago.com/
- https://chat.openai.com/
- https://www.youtube.com/

#### versión README.md -> 1.0 (2022-agosto)
