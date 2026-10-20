# Animation 3D
Par : Tristan Lemarchand, Andéol Vella

## Prérequis
Rust généralise le concepte de librairie et d'application exécutable par le mot _crate_.
Plusieurs crates dans un même projet implique de travailler dans un _workspace_, c'est le cas pour ce projet.
Chaque crate contient un fichier [Cargo.toml] qui spécifie les liens qu'il y a avec les autres crates.
Enfin, un workspace est caractérisé par un fichier [Cargo.toml] placé à la racine et qui contient un manifeste des différentes crates placées à l'intérieur.

## Organisation
Ce projet est un workspace contenant deux crates :
- __bvh-parser__ : Parser pour fichiers _.bvh_ (Librairie)
- __main__ : Application permettant de lancer l'animation 3D

## Compiler et Exécuter
Clonez le repos github avec la commande suivante :
````
```bash
git clone https://github.com/chimivari/Ensimag-Animation3D.git
cd Ensimag-Animation3D
```
Pour compiler et exécuter en même temps, exécutez la commande :
```bash
cargo run
```
Pour compiler uniquement :
```bash
cargo build
```

Les fichiers [.bvh] se situent dans le dossier [input].

## Sources
Le parser [.bvh] a été fait à l'aide de [cette page](https://research.cs.wisc.edu/graphics/Courses/cs-838-1999/Jeff/BVH.html).
