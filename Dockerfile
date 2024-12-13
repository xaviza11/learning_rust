# Usa la imagen oficial de Rust como base
FROM rust:latest

# Establece el directorio de trabajo en el contenedor
WORKDIR /app

# Copia los archivos desde "scr" al directorio de trabajo
COPY src/ /app/

# Compila la aplicación
RUN rustc main.rs -o app

# Comando para ejecutar la aplicación
CMD ["./app"]
