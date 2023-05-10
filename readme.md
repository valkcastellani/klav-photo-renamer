# Klav Photo Renamer

O Klav Photo Renamer é uma ferramenta simples de linha de comando projetada para renomear fotos em massa com base em uma convenção de nomenclatura específica. 
Essa ferramenta é útil para pessoas que precisam renomear um grande número de fotos de maneira consistente e eficiente.

## Instalação

O Klav Photo Renamer é um aplicativo escrito em Rust, portanto, você precisará ter o Rust instalado em seu computador para usá-lo. 
Você pode instalar o Rust no site oficial em https://www.rust-lang.org/tools/install.

Depois de ter o Rust instalado, você pode baixar o repositório do Klav Photo Renamer do GitHub usando o seguinte comando:

```
git clone https://github.com/valkcastellani/klav-photo-renamer.git
```

## Uso

Para usar o Klav Photo Renamer, navegue até o diretório onde suas fotos estão armazenadas usando o prompt de comando. 
Em seguida, execute o seguinte comando:

```
cargo run --release
```

A ferramenta solicitará a convenção de nomenclatura que você deseja usar. 
Você pode escolher entre várias opções padrão ou criar uma convenção de nomenclatura personalizada usando uma combinação de texto e espaços reservados.

Os espaços reservados disponíveis incluem:

- `%y`: ano (por exemplo, 2023)
- `%m`: mês (por exemplo, 05)
- `%d`: dia (por exemplo, 09)
- `%h`: hora (por exemplo, 12)
- `%n`: nome do arquivo original (por exemplo, IMG_1234.jpg)

Por exemplo, se você deseja renomear suas fotos para incluir a data e a hora em que foram tiradas, pode usar a seguinte convenção de nomenclatura:

```
%y-%m-%d_%h-%n
```

Isso resultaria em nomes de arquivo como:

```
2023-05-09_12-IMG_1234.jpg
```

## Licença

O Klav Photo Renamer é lançado sob a licença MIT. Consulte [LICENSE](https://github.com/klavinsm/Klav-Photo-Renamer/blob/main/LICENSE) para obter mais informações.

## Contribuições

Contribuições para o Klav Photo Renamer são sempre bem-vindas! Se você tiver uma sugestão para um novo recurso ou gostaria de relatar um bug, abra uma issue no GitHub. Se você gostaria de contribuir com código, faça um fork do repositório e envie um pull request.

==================================================================================

# Klav Photo Renamer

Klav Photo Renamer is a simple command-line tool written in Rust, designed to rename photos in bulk based on a specific naming convention. This tool is useful for people who need to rename a large number of photos in a consistent and efficient manner.

## Installation

Klav Photo Renamer is a Rust application, so you'll need to have Rust installed on your computer to use it. You can install Rust from the official website at https://www.rust-lang.org/tools/install.

Once you have Rust installed, you can download the Klav Photo Renamer repository from GitHub using the following command:

```
git clone https://github.com/klavinsm/Klav-Photo-Renamer.git
```

## Usage

To use Klav Photo Renamer, navigate to the directory where your photos are stored using the command prompt. Then, run the following command:

```
cargo run --release
```

The tool will prompt you for the naming convention you want to use. You can choose from several default options or create a custom naming convention using a combination of text and placeholders.

The available placeholders include:

- `%y`: year (e.g. 2023)
- `%m`: month (e.g. 05)
- `%d`: day (e.g. 09)
- `%h`: hour (e.g. 12)
- `%n`: original file name (e.g. IMG_1234.jpg)

For example, if you want to rename your photos to include the date and time they were taken, you can use the following naming convention:

```
%y-%m-%d_%h-%n
```

This would result in file names like:

```
2023-05-09_12-IMG_1234.jpg
```

## License

Klav Photo Renamer is released under the MIT license. See [LICENSE](https://github.com/klavinsm/Klav-Photo-Renamer/blob/main/LICENSE) for more information.

## Contributions

Contributions to Klav Photo Renamer are always welcome! If you have a suggestion for a new feature or would like to report a bug, open an issue on GitHub. If you would like to contribute code, fork the repository and submit a pull request.

==================================================================================

# Klav Photo Renamer

Klav Photo Renamer es una herramienta simple de línea de comando escrita en Rust, diseñada para renombrar fotos en masa en función de una convención de nomenclatura específica. Esta herramienta es útil para personas que necesitan renombrar un gran número de fotos de manera consistente y eficiente.

## Instalación

Klav Photo Renamer es una aplicación de Rust, por lo que necesitarás tener Rust instalado en tu computadora para usarlo. Puedes instalar Rust desde el sitio web oficial en https://www.rust-lang.org/tools/install.

Una vez que tengas Rust instalado, puedes descargar el repositorio de Klav Photo Renamer desde GitHub utilizando el siguiente comando:

```
git clone https://github.com/klavinsm/Klav-Photo-Renamer.git
```

## Uso

Para usar Klav Photo Renamer, navega hasta el directorio donde se encuentran tus fotos usando el símbolo del sistema. Luego, ejecuta el siguiente comando:

```
cargo run --release
```

La herramienta te solicitará la convención de nomenclatura que deseas usar. Puedes elegir entre varias opciones predeterminadas o crear una convención de nomenclatura personalizada usando una combinación de texto y marcadores de posición.

Los marcadores de posición disponibles incluyen:

- `%y`: año (por ejemplo, 2023)
- `%m`: mes (por ejemplo, 05)
- `%d`: día (por ejemplo, 09)
- `%h`: hora (por ejemplo, 12)
- `%n`: nombre del archivo original (por ejemplo, IMG_1234.jpg)

Por ejemplo, si deseas renombrar tus fotos para incluir la fecha y hora en que se tomaron, puedes usar la siguiente convención de nomenclatura:

```
%y-%m-%d_%h-%n
```

Esto resultaría en nombres de archivo como:

```
2023-05-09_12-IMG_1234.jpg
```

## Licencia

Klav Photo Renamer se distribuye bajo la licencia MIT. Consulta [LICENSE](https://github.com/klavinsm/Klav-Photo-Renamer/blob/main/LICENSE) para obtener más información.

## Contribuciones

¡Las contribuciones a Klav Photo Renamer son siempre bienvenidas! Si tienes una sugerencia para una nueva característica o quieres informar sobre un error, abre un issue en GitHub. Si deseas contribuir con código, haz un fork del repositorio y envía una solicitud de extracción.
